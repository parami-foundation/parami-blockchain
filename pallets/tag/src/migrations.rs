use crate::MetaOf;
use crate::Metadata;
use crate::{Config, Pallet};
use frame_support::traits::OnRuntimeUpgrade;
use frame_support::traits::StorageVersion;
use frame_support::{traits::Get, weights::Weight};
use sp_runtime::traits::Saturating;

pub fn migrate<T: Config>() -> Weight {
    let version = StorageVersion::get::<Pallet<T>>();
    let mut weight: Weight = 0;

    if version < 1 {
        weight.saturating_accrue(v1::migrate::<T>());
        StorageVersion::new(1).put::<Pallet<T>>();
    }

    weight
}

mod v1 {
    use super::*;
    use crate::{types::Score, InfluencesOf, PersonasOf};

    pub fn migrate<T: Config>() -> Weight {
        let mut weight: Weight = 0;

        <PersonasOf<T>>::translate_values(|score| {
            Some(Pallet::<T>::accrue(&Score::default(), score))
        });
        <InfluencesOf<T>>::translate_values(|score| {
            Some(Pallet::<T>::accrue(&Score::default(), score))
        });

        weight.saturating_accrue(T::DbWeight::get().reads_writes(1, 1));

        weight
    }
}

pub mod v2 {
    use super::*;
    pub struct ResetHeight<T>(sp_std::marker::PhantomData<T>);

    impl<T: crate::Config> OnRuntimeUpgrade for ResetHeight<T> {
        fn on_runtime_upgrade() -> Weight {
            let version = StorageVersion::get::<Pallet<T>>();
            if version != 1 {
                return 0;
            }

            StorageVersion::put::<Pallet<T>>(&StorageVersion::new(2));
            Metadata::<T>::translate_values(|m| {
                Some(MetaOf::<T> {
                    created: 0u32.into(),
                    ..m
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
