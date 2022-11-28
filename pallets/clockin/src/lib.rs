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
    }

    #[pallet::pallet]
    #[pallet::storage_version(STORAGE_VERSION)]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {}

    #[pallet::error]
    pub enum Error<T> {}

    #[pallet::call]
    impl<T: Config> Pallet<T> {}

    #[pallet::genesis_config]
    pub struct GenesisConfig<T: Config> {
        _marker: sp_std::marker::PhantomData<T>,
    }

    #[cfg(feature = "std")]
    impl<T: Config> Default for GenesisConfig<T> {
        fn default() -> Self {
            Self::default()
        }
    }

    #[pallet::genesis_build]
    impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
        fn build(&self) {}
    }
}
