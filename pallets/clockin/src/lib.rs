#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

mod types;

use frame_support::{
    dispatch::DispatchResult,
    ensure,
    traits::{
        tokens::{
            fungibles::{
                metadata::Mutate as FungMetaMutate, Create as FungCreate, Inspect,
                Mutate as FungMutate, Transfer as FungTransfer,
            },
            nonfungibles::{Create as NftCreate, Mutate as NftMutate},
        },
        Currency, EnsureOrigin,
        ExistenceRequirement::{self, KeepAlive},
        Get, StorageVersion,
    },
    PalletId,
};
use frame_system::offchain::SendTransactionTypes;
use parami_did::EnsureDid;
use sp_core::U512;
use sp_runtime::{
    traits::{AccountIdConversion, AtLeast32BitUnsigned, Bounded, One, Saturating, Zero},
    DispatchError, RuntimeDebug,
};
use sp_std::{
    convert::{TryFrom, TryInto},
    prelude::*,
};

type AccountOf<T> = <T as frame_system::Config>::AccountId;
type BalanceOf<T> = <<T as parami_did::Config>::Currency as Currency<AccountOf<T>>>::Balance;
type DidOf<T> = <T as parami_did::Config>::DecentralizedId;
type HeightOf<T> = <T as frame_system::Config>::BlockNumber;
type NftOf<T> = <T as parami_nft::Config>::AssetId;
type MetaOf<T> = types::Metadata<HeightOf<T>, AccountOf<T>, BalanceOf<T>>;

const STORAGE_VERSION: StorageVersion = StorageVersion::new(0);

#[frame_support::pallet]
pub mod pallet {
    use super::*;
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;

    #[pallet::config]
    pub trait Config:
        frame_system::Config
        + parami_did::Config
        + parami_nft::Config
        + SendTransactionTypes<Call<Self>>
    {
        /// The overarching event type
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

        /// The pallet id, used for deriving "pot" accounts to receive donation
        #[pallet::constant]
        type PalletId: Get<PalletId>;
    }

    /// Metadata
    #[pallet::storage]
    pub(super) type Metadata<T: Config> = StorageMap<_, Twox64Concat, NftOf<T>, MetaOf<T>>;

    #[pallet::storage]
    pub(super) type LastClockIn<T: Config> =
        StorageDoubleMap<_, Twox64Concat, NftOf<T>, Twox64Concat, DidOf<T>, i32, ValueQuery>;

    #[pallet::pallet]
    #[pallet::storage_version(STORAGE_VERSION)]
    #[pallet::generate_store(pub(super) trait Store)]
    #[pallet::without_storage_info]
    pub struct Pallet<T>(_);

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        ClockInEnabled(NftOf<T>),
        ClockInDisabled(NftOf<T>),
    }

    #[pallet::error]
    pub enum Error<T> {
        NftNotExists,
        NotNftOwner,
        NftNotMinted,
        InsufficientToken,
        ClockInNotExists,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::weight(0)]
        pub fn enable_clock_in(
            origin: OriginFor<T>,
            nft_id: NftOf<T>,
            payout_base: BalanceOf<T>,
            payout_min: BalanceOf<T>,
            payout_max: BalanceOf<T>,
            metadata: Vec<u8>,
            tags: Vec<Vec<u8>>,
            total_reward_token: BalanceOf<T>,
        ) -> DispatchResult {
            let (did, who) = parami_did::EnsureDid::<T>::ensure_origin(origin)?;
            let nft_meta = parami_nft::Pallet::<T>::meta(nft_id).ok_or(Error::<T>::NftNotExists)?;
            ensure!(nft_meta.owner == did, Error::<T>::NotNftOwner);
            ensure!(nft_meta.minted, Error::<T>::NftNotMinted);
            ensure!(
                T::Assets::balance(nft_meta.token_asset_id, &who) >= total_reward_token,
                Error::<T>::InsufficientToken
            );

            // TODO: handle tags

            let pot = Self::generate_reward_pot(&nft_id);
            T::Assets::transfer(
                nft_meta.token_asset_id,
                &who,
                &pot,
                total_reward_token,
                true,
            )?;

            let start_at = <frame_system::Pallet<T>>::block_number();
            Metadata::<T>::insert(
                nft_id,
                MetaOf::<T> {
                    payout_base,
                    payout_min,
                    payout_max,
                    metadata,
                    start_at,
                    pot,
                },
            );

            Self::deposit_event(Event::<T>::ClockInEnabled(nft_id));
            Ok(())
        }

        #[pallet::weight(0)]
        pub fn add_token_reward(
            origin: OriginFor<T>,
            nft_id: NftOf<T>,
            reward_token: BalanceOf<T>,
        ) -> DispatchResult {
            let (did, who) = parami_did::EnsureDid::<T>::ensure_origin(origin)?;
            let nft_meta = parami_nft::Pallet::<T>::meta(nft_id).ok_or(Error::<T>::NftNotExists)?;
            ensure!(nft_meta.owner == did, Error::<T>::NotNftOwner);
            ensure!(nft_meta.minted, Error::<T>::NftNotMinted);
            let meta = Metadata::<T>::get(nft_id).ok_or(Error::<T>::ClockInNotExists)?;

            T::Assets::transfer(nft_meta.token_asset_id, &who, &meta.pot, reward_token, true)?;

            Ok(())
        }

        #[pallet::weight(0)]
        pub fn update_clock_in(
            origin: OriginFor<T>,
            nft_id: NftOf<T>,
            payout_base: BalanceOf<T>,
            payout_min: BalanceOf<T>,
            payout_max: BalanceOf<T>,
            metadata: Vec<u8>,
            tags: Vec<Vec<u8>>,
        ) -> DispatchResult {
            let (did, _who) = parami_did::EnsureDid::<T>::ensure_origin(origin)?;
            let nft_meta = parami_nft::Pallet::<T>::meta(nft_id).ok_or(Error::<T>::NftNotExists)?;
            ensure!(nft_meta.owner == did, Error::<T>::NotNftOwner);
            ensure!(nft_meta.minted, Error::<T>::NftNotMinted);
            let meta = Metadata::<T>::get(nft_id).ok_or(Error::<T>::ClockInNotExists)?;

            // TODO: handle tags

            Metadata::<T>::insert(
                nft_id,
                MetaOf::<T> {
                    payout_base,
                    payout_min,
                    payout_max,
                    metadata,
                    ..meta
                },
            );

            Ok(())
        }

        #[pallet::weight(0)]
        pub fn disable_clock_in(origin: OriginFor<T>, nft_id: NftOf<T>) -> DispatchResult {
            let (did, who) = parami_did::EnsureDid::<T>::ensure_origin(origin)?;
            let nft_meta = parami_nft::Pallet::<T>::meta(nft_id).ok_or(Error::<T>::NftNotExists)?;
            ensure!(nft_meta.owner == did, Error::<T>::NotNftOwner);
            let metadata = Metadata::<T>::get(nft_id).ok_or(Error::<T>::ClockInNotExists)?;

            // TODO: handle tags

            let balance = T::Assets::balance(nft_id, &metadata.pot);
            T::Assets::transfer(nft_meta.token_asset_id, &metadata.pot, &who, balance, false)?;

            Self::deposit_event(Event::<T>::ClockInDisabled(nft_id));
            Ok(())
        }
    }

    impl<T: Config> Pallet<T> {
        fn generate_reward_pot(nft_id: &NftOf<T>) -> AccountOf<T> {
            return <T as crate::Config>::PalletId::get().into_sub_account_truncating(&nft_id);
        }
    }

    #[pallet::genesis_config]
    pub struct GenesisConfig<T: Config> {
        _marker: sp_std::marker::PhantomData<T>,
    }

    #[cfg(feature = "std")]
    impl<T: Config> Default for GenesisConfig<T> {
        fn default() -> Self {
            Self {
                _marker: sp_std::marker::PhantomData::<T>,
            }
        }
    }

    #[pallet::genesis_build]
    impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
        fn build(&self) {}
    }
}
