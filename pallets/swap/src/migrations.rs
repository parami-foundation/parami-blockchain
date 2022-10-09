use crate::Account;
use crate::Pallet;
use crate::StorageVersion;
use frame_support::traits::OnRuntimeUpgrade;
use frame_support::weights::Weight;

pub mod v1 {
    use frame_support::assert_ok;
    use parami_traits::Swaps;

    use crate::AccountOf;

    use super::*;

    pub struct ResetHeight<T>(sp_std::marker::PhantomData<T>);

    impl<T: crate::Config> OnRuntimeUpgrade for ResetHeight<T> {
        fn on_runtime_upgrade() -> Weight {
            let version = StorageVersion::get::<Pallet<T>>();
            if version != 0 {
                return 0;
            }

            for (account, asset, _claimed_at) in Account::<T>::iter() {
                let result = <Pallet<T> as Swaps<AccountOf<T>>>::burn(
                    account,
                    asset,
                    0u32.into(),
                    0u32.into(),
                );
                assert_ok!(result);
            }

            StorageVersion::put::<Pallet<T>>(&StorageVersion::new(1));
            1
        }

        #[cfg(feature = "try-runtime")]
        fn pre_upgrade() -> Result<(), &'static str> {
            use frame_support::log::info;

            let count = Account::<T>::iter().count();
            info!("accounts: {:?}", count);

            Ok(())
        }

        #[cfg(feature = "try-runtime")]
        fn post_upgrade() -> Result<(), &'static str> {
            let count = Account::<T>::iter().count();
            assert_eq!(count, 0);

            Ok(())
        }
    }
}

pub mod v2 {

    use crate::{Metadata, SwapOf};

    use super::*;

    pub struct ResetHeight<T>(sp_std::marker::PhantomData<T>);

    impl<T: crate::Config> OnRuntimeUpgrade for ResetHeight<T> {
        fn on_runtime_upgrade() -> Weight {
            let version = StorageVersion::get::<Pallet<T>>();
            if version != 1 {
                return 0;
            }

            Metadata::<T>::translate_values(|m| {
                Some(SwapOf::<T> {
                    created: 0u32.into(),
                    ..m
                })
            });

            StorageVersion::put::<Pallet<T>>(&StorageVersion::new(2));
            1
        }

        #[cfg(feature = "try-runtime")]
        fn pre_upgrade() -> Result<(), &'static str> {
            use frame_support::log::info;

            let count: u32 = Metadata::<T>::iter_values()
                .filter(|m| m.created != 0u32.into())
                .map(|_| 1u32)
                .sum();

            info!("non zero count: {:?}", count);

            Ok(())
        }

        #[cfg(feature = "try-runtime")]
        fn post_upgrade() -> Result<(), &'static str> {
            let count: u32 = Metadata::<T>::iter_values()
                .filter(|m| m.created != 0u32.into())
                .map(|_| 1u32)
                .sum();
            assert_eq!(count, 0);

            Ok(())
        }
    }
}

pub mod v3 {

    use crate::{Metadata, SwapOf};

    use super::*;

    pub struct AddLiquidityShare<T>(sp_std::marker::PhantomData<T>);

    impl<T: crate::Config> OnRuntimeUpgrade for AddLiquidityShare<T> {
        fn on_runtime_upgrade() -> Weight {
            let version = StorageVersion::get::<Pallet<T>>();
            if version != 2 {
                return 0;
            }

            let current_block = <frame_system::Pallet<T>>::block_number();

            Metadata::<T>::translate_values(|m: SwapOf<T>| {
                Some(SwapOf::<T> {
                    updated_at: current_block,
                    liquidity_contribution: Pallet::<T>::calculate_liquidity_contribution(
                        current_block - m.created,
                        m.liquidity,
                    ),
                    ..m
                })
            });

            StorageVersion::put::<Pallet<T>>(&StorageVersion::new(3));
            1
        }

        #[cfg(feature = "try-runtime")]
        fn pre_upgrade() -> Result<(), &'static str> {
            let version = StorageVersion::get::<Pallet<T>>();
            assert_eq!(version, 2);
        }

        #[cfg(feature = "try-runtime")]
        fn post_upgrade() -> Result<(), &'static str> {
            use log::info;

            let version = StorageVersion::get::<Pallet<T>>();
            assert_eq!(version, 3);

            for (asset_id, swap) in Metadata::<T>::iter() {
                info!("asset_id: {:?}, swap: {:?}", asset_id, swap);
            }
        }
    }
}
