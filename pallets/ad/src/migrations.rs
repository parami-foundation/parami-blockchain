use crate::StorageVersion;
use crate::{Config, Pallet};
use crate::{DeadlineOf, EndtimeOf, HeightOf, Metadata, Payout, SlotOf};
use frame_support::traits::OnRuntimeUpgrade;
use frame_support::weights::Weight;

pub fn migrate<T: Config>() -> Weight {
    let _version = StorageVersion::get::<Pallet<T>>();
    let weight: Weight = 0;

    weight
}

pub mod v4 {
    use crate::MetaOf;
    use crate::SlotMetaOf;

    use super::*;

    pub struct ResetHeight<T>(sp_std::marker::PhantomData<T>);

    impl<T: crate::Config> OnRuntimeUpgrade for ResetHeight<T> {
        fn on_runtime_upgrade() -> Weight {
            let version = StorageVersion::get::<Pallet<T>>();
            if version != 3 {
                return 0;
            }

            StorageVersion::put::<Pallet<T>>(&StorageVersion::new(4));

            Payout::<T>::translate_values(|_h: HeightOf<T>| Some(0u32.into()));
            DeadlineOf::<T>::translate_values(|_h: HeightOf<T>| Some(0u32.into()));
            EndtimeOf::<T>::translate_values(|_h: HeightOf<T>| Some(0u32.into()));
            Metadata::<T>::translate_values(|m| {
                Some(MetaOf::<T> {
                    created: 0u32.into(),
                    ..m
                })
            });
            SlotOf::<T>::translate_values(|s| {
                Some(SlotMetaOf::<T> {
                    created: 0u32.into(),
                    ..s
                })
            });

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
