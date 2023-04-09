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

#[frame_support::pallet]
pub mod pallet {
    use super::*;
    use frame_support::traits::tokens::fungibles::{Create, Inspect, Mutate, Transfer};
    use frame_support::traits::Currency;
    use frame_support::PalletId;
    use frame_support::{dispatch::DispatchResult, pallet_prelude::*};
    use frame_system::pallet_prelude::*;
    use sp_core::U256;
    use sp_runtime::traits::{AccountIdConversion, AtLeast32BitUnsigned};
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
    pub type Nonce<T> = StorageValue<_, u64, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn used_nonces_of)]
    pub type NoncesUsed<T> =
        StorageDoubleMap<_, Blake2_128Concat, DomainOf<T>, Blake2_128Concat, u64, bool, ValueQuery>;

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
            u64,
            CctpAssetOf<T>,
            BalanceOf<T>,
            DomainOf<T>,
            DidOf<T>,
            DomainOf<T>,
            Vec<u8>,
        ),
    }

    // Errors inform users that something went wrong.
    #[pallet::error]
    pub enum Error<T> {
        CctpAssetIdNotRegistered,
        AccountBalanceNotEnough,
        IncorrectSignerSetting,
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
            let asset_id =
                Self::asset_for(cctp_asset_id).ok_or(Error::<T>::CctpAssetIdNotRegistered)?;
            let account_balance = T::Assets::balance(asset_id, &account);
            ensure!(
                account_balance >= amount,
                Error::<T>::AccountBalanceNotEnough
            );

            let nonce = Nonce::<T>::get();
            Nonce::<T>::put(nonce + 1);

            T::Assets::transfer(asset_id, &account, &Self::account_id(), amount, false)?;
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
            nonce: U256,
            cctp_asset_id: CctpAssetOf<T>,
            amount: BalanceOf<T>,
            source_domain: DomainOf<T>,
            depositer: Vec<u8>,
            recipient: DidOf<T>,
            signature: Vec<u8>,
        ) -> DispatchResult {
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
                Error::<T>::IncorrectSignerSetting
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
        fn account_id() -> AccountOf<T> {
            <T as Config>::PalletId::get().into_sub_account_truncating(b"")
        }
    }
}
