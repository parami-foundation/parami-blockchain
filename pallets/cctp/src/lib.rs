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
    use frame_support::{dispatch::DispatchResult, pallet_prelude::*};
    use frame_system::pallet_prelude::*;
    use sp_core::U256;
    use sp_runtime::traits::AtLeast32BitUnsigned;

    /// Configure the pallet by specifying the parameters and types on which it depends.
    #[pallet::config]
    pub trait Config:
        frame_system::Config + pallet_balances::Config + pallet_assets::Config + parami_did::Config
    {
        /// Because this pallet emits events, it depends on the runtime's definition of an event.
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
        type CctpAssetId: AtLeast32BitUnsigned + Parameter + Default + Copy;
        type DomainId: AtLeast32BitUnsigned + Parameter + Default + Copy;

        type Currency: Currency<<Self as frame_system::Config>::AccountId>;
        type Assets: Transfer<AccountOf<Self>, AssetId = AssetOf<Self>, Balance = BalanceOf<Self>>
            + Mutate<AccountOf<Self>, AssetId = AssetOf<Self>, Balance = BalanceOf<Self>>
            + Create<AccountOf<Self>, AssetId = AssetOf<Self>>
            + Inspect<AccountOf<Self>, AssetId = AssetOf<Self>, Balance = BalanceOf<Self>>;
    }

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    // The pallet's runtime storage items.
    // https://substrate.dev/docs/en/knowledgebase/runtime/storage
    #[pallet::storage]
    #[pallet::getter(fn something)]
    // Learn more about declaring storage items:
    // https://substrate.dev/docs/en/knowledgebase/runtime/storage#declaring-storage-items
    pub type Something<T> = StorageValue<_, u32>;

    // Pallets use events to inform users when important changes are made.
    // https://substrate.dev/docs/en/knowledgebase/runtime/events
    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// Event documentation should end with an array that provides descriptive names for event
        /// parameters. [something, who]
        SomethingStored(u32, T::AccountId),
    }

    // Errors inform users that something went wrong.
    #[pallet::error]
    pub enum Error<T> {
        /// Error names should be descriptive.
        NoneValue,
        /// Errors should have helpful documentation associated with them.
        StorageOverflow,
    }

    // Dispatchable functions allows users to interact with the pallet and invoke state changes.
    // These functions materialize as "extrinsics", which are often compared to transactions.
    // Dispatchable functions must be annotated with a weight and must return a DispatchResult.
    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// An example dispatchable that takes a singles value as a parameter, writes the value to
        /// storage and emits an event. This function must be dispatched by a signed extrinsic.
        #[pallet::weight(0)]
        pub fn deposit(
            origin: OriginFor<T>,
            cctp_asset_id: CctpAssetOf<T>,
            amount: BalanceOf<T>,
            destination_domain: DomainOf<T>,
            destination_address: Vec<u8>,
        ) -> DispatchResult {
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
            Ok(())
        }
    }
}
