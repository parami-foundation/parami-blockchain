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

            StorageVersion::put::<Pallet<T>>(&StorageVersion::new(1));

            for (account, asset, _claimed_at) in Account::<T>::iter() {
                let result = <Pallet<T> as Swaps<AccountOf<T>>>::burn(
                    account,
                    asset,
                    0u32.into(),
                    0u32.into(),
                );
                assert_ok!(result);
            }

            1
        }

        #[cfg(feature = "try-runtime")]
        fn pre_upgrade() -> Result<(), &'static str> {
            Ok(())
        }

        #[cfg(feature = "try-runtime")]
        fn post_upgrade() -> Result<(), &'static str> {
            Ok(())
        }
    }
}
