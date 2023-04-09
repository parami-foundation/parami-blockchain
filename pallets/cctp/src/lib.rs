#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;
/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://substrate.dev/docs/en/knowledgebase/runtime/frame>

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

type AccountOf<T> = <T as frame_system::Config>::AccountId;
type CctpAssetOf<T> = <T as Config>::CctpAssetId;
type BalanceOf<T> = <T as pallet_balances::Config>::Balance;
type AssetOf<T> = <T as pallet_assets::Config>::AssetId;
type DidOf<T> = <T as parami_did::Config>::DecentralizedId;
type DomainOf<T> = <T as Config>::DomainId;
type NonceOf<T> = <T as Config>::Nonce;

#[frame_support::pallet]
pub mod pallet {
    use super::*;
    use frame_support::traits::tokens::fungibles::{Create, Inspect, Mutate, Transfer};
    use frame_support::traits::Currency;
    use frame_support::PalletId;
    use frame_support::{dispatch::DispatchResult, pallet_prelude::*};
    use frame_system::pallet_prelude::*;
    use sp_core::crypto::AccountId32;
    use sp_io::hashing::keccak_256;
    use sp_runtime::traits::{AtLeast32BitUnsigned, Verify};
    use sp_runtime::MultiSignature;
    use sp_std::prelude::*;

    /// Configure the pallet by specifying the parameters and types on which it depends.
    #[pallet::config]
    pub trait Config:
        frame_system::Config + pallet_balances::Config + pallet_assets::Config + parami_did::Config
    {
        /// Because this pallet emits events, it depends on the runtime's definition of an event.
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
        type CctpAssetId: AtLeast32BitUnsigned + Parameter + Default + Copy + MaxEncodedLen;
        type DomainId: AtLeast32BitUnsigned + Parameter + Default + Copy + MaxEncodedLen;
        type Nonce: AtLeast32BitUnsigned + Parameter + Default + Copy + MaxEncodedLen;

        type Currency: Currency<<Self as frame_system::Config>::AccountId>;
        type Assets: Transfer<AccountOf<Self>, AssetId = AssetOf<Self>, Balance = BalanceOf<Self>>
            + Mutate<AccountOf<Self>, AssetId = AssetOf<Self>, Balance = BalanceOf<Self>>
            + Create<AccountOf<Self>, AssetId = AssetOf<Self>>
            + Inspect<AccountOf<Self>, AssetId = AssetOf<Self>, Balance = BalanceOf<Self>>;

        type CallOrigin: EnsureOrigin<Self::Origin, Success = (DidOf<Self>, AccountOf<Self>)>;

        #[pallet::constant]
        type ChainDomain: Get<Self::DomainId>;

        #[pallet::constant]
        type PalletId: Get<PalletId>;
    }

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    #[pallet::storage]
    #[pallet::getter(fn nonce)]
    pub type CurrentNonce<T> = StorageValue<_, NonceOf<T>, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn used_nonces_of)]
    pub type NoncesUsed<T> = StorageDoubleMap<
        _,
        Blake2_128Concat,
        DomainOf<T>,
        Blake2_128Concat,
        NonceOf<T>,
        bool,
        ValueQuery,
    >;

    #[pallet::storage]
    #[pallet::getter(fn asset_for)]
    pub type AssetMap<T> = StorageMap<_, Blake2_128Concat, CctpAssetOf<T>, AssetOf<T>, OptionQuery>;

    #[pallet::storage]
    #[pallet::getter(fn signing_did)]
    pub type Signer<T> = StorageValue<_, DidOf<T>, OptionQuery>;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// Asset Deposited. [nonce, asset_id, amount, sourceDomain, sourceDid, destinationDomain, destinationAddress]
        Deposited(
            NonceOf<T>,
            CctpAssetOf<T>,
            BalanceOf<T>,
            DomainOf<T>,
            DidOf<T>,
            DomainOf<T>,
            Vec<u8>,
        ),

        /// Asset Withdrawn. [nonce, asset_id, amount, sourceDomain, source_address, destination_domain, destination_did]
        Withdrawn(
            NonceOf<T>,
            CctpAssetOf<T>,
            BalanceOf<T>,
            DomainOf<T>,
            Vec<u8>,
            DomainOf<T>,
            DidOf<T>,
        ),
    }

    // Errors inform users that something went wrong.
    #[pallet::error]
    pub enum Error<T> {
        AssetNotRegistered,
        NotEnoughBalance,
        InvalidSigner,
        InvalidSignature,
        UsedNonce,
        InvalidRecipient,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::weight(0)]
        pub fn deposit(
            origin: OriginFor<T>,
            cctp_asset_id: CctpAssetOf<T>,
            amount: BalanceOf<T>,
            destination_domain: DomainOf<T>,
            destination_address: Vec<u8>,
        ) -> DispatchResult {
            let (did, account) = T::CallOrigin::ensure_origin(origin)?;
            let asset_id = Self::asset_for(cctp_asset_id).ok_or(Error::<T>::AssetNotRegistered)?;
            let account_balance = T::Assets::balance(asset_id, &account);
            ensure!(account_balance >= amount, Error::<T>::NotEnoughBalance);

            let nonce = CurrentNonce::<T>::get();
            CurrentNonce::<T>::put(nonce + 1u32.into());

            T::Assets::burn_from(asset_id, &account, amount)?;
            Self::deposit_event(Event::Deposited(
                nonce,
                cctp_asset_id,
                amount,
                T::ChainDomain::get(),
                did,
                destination_domain,
                destination_address,
            ));

            Ok(())
        }

        #[pallet::weight(0)]
        pub fn withdraw(
            origin: OriginFor<T>,
            nonce: NonceOf<T>,
            cctp_asset_id: CctpAssetOf<T>,
            amount: BalanceOf<T>,
            source_domain: DomainOf<T>,
            depositer: Vec<u8>,
            recipient: DidOf<T>,
            signature: MultiSignature,
        ) -> DispatchResult {
            let (did, _account) = T::CallOrigin::ensure_origin(origin)?;

            let signer = Self::signing_did().ok_or(Error::<T>::InvalidSigner)?;
            let signer_account: AccountOf<T> =
                parami_did::Pallet::<T>::lookup_did(signer).ok_or(Error::<T>::InvalidSigner)?;

            Self::verify_signature(
                nonce,
                cctp_asset_id,
                amount,
                source_domain,
                &depositer,
                recipient,
                signature,
                signer_account,
            )?;

            ensure!(
                !NoncesUsed::<T>::contains_key(source_domain, nonce),
                Error::<T>::UsedNonce
            );

            let asset_id = Self::asset_for(cctp_asset_id).ok_or(Error::<T>::AssetNotRegistered)?;

            let recipient_account = parami_did::Pallet::<T>::lookup_did(recipient)
                .ok_or(Error::<T>::InvalidRecipient)?;

            NoncesUsed::<T>::insert(source_domain, nonce, true);
            T::Assets::mint_into(asset_id, &recipient_account, amount)?;

            Self::deposit_event(Event::Withdrawn(
                nonce,
                cctp_asset_id,
                amount,
                source_domain,
                depositer,
                T::ChainDomain::get(),
                did,
            ));

            Ok(())
        }

        #[pallet::weight(0)]
        pub fn register_asset(
            origin: OriginFor<T>,
            cctp_asset_id: CctpAssetOf<T>,
            asset_id: AssetOf<T>,
        ) -> DispatchResult {
            let (did, _) = T::CallOrigin::ensure_origin(origin)?;
            ensure!(
                !Signer::<T>::get().is_none() && Signer::<T>::get().unwrap() == did,
                Error::<T>::InvalidSigner
            );
            AssetMap::<T>::insert(cctp_asset_id, asset_id);
            Ok(())
        }

        #[pallet::weight(0)]
        pub fn set_signer(origin: OriginFor<T>, did: DidOf<T>) -> DispatchResult {
            ensure_root(origin)?;
            Signer::<T>::put(did);
            Ok(())
        }
    }

    impl<T: Config> Pallet<T> {
        pub fn verify_signature(
            nonce: NonceOf<T>,
            cctp_asset_id: CctpAssetOf<T>,
            amount: BalanceOf<T>,
            source_domain: DomainOf<T>,
            depositer: &Vec<u8>,
            recipient: DidOf<T>,
            signature: MultiSignature,
            signer_account: AccountOf<T>,
        ) -> DispatchResult {
            let sig_message = Self::construct_msg(
                nonce,
                cctp_asset_id,
                amount,
                source_domain,
                depositer,
                recipient,
            );

            let signer_bytes: [u8; 32] = match signer_account.encode().try_into() {
                Ok(bytes) => bytes,
                Err(_) => return Err(Error::<T>::InvalidSigner.into()),
            };

            let signer_account = AccountId32::from(signer_bytes);
            ensure!(
                signature.verify(sig_message.as_slice(), &signer_account),
                Error::<T>::InvalidSignature
            );

            Ok(())
        }

        pub fn construct_msg(
            nonce: NonceOf<T>,
            cctp_asset_id: CctpAssetOf<T>,
            amount: BalanceOf<T>,
            source_domain: DomainOf<T>,
            depositer: &Vec<u8>,
            recipient: DidOf<T>,
        ) -> [u8; 32] {
            let mut msg_vec: Vec<u8> = Vec::new();
            msg_vec.extend_from_slice(&nonce.encode());
            msg_vec.extend_from_slice(&cctp_asset_id.encode());
            msg_vec.extend_from_slice(&amount.encode());
            msg_vec.extend_from_slice(&source_domain.encode());
            msg_vec.extend_from_slice(depositer);
            msg_vec.extend_from_slice(&T::ChainDomain::get().encode());
            msg_vec.extend_from_slice(&recipient.encode());
            keccak_256(&msg_vec.as_slice())
        }
    }
}
