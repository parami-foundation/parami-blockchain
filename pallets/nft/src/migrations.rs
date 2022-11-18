mod v4 {
    use frame_support::migration::move_prefix;
    use frame_support::traits::fungibles::Transfer;
    use frame_support::traits::OnRuntimeUpgrade;
    use frame_support::weights::Weight;

    use crate::Config;
    use crate::Deposit;
    use crate::Pallet;
    use crate::StorageVersion;
    use crate::{BalanceOf, IcoMeta, IcoMetaOf, Metadata};
    use parami_primitives::constants::DOLLARS;

    #[derive(Debug)]
    pub enum Error {
        NumberConversionFailed,
    }

    pub struct MigrateIcoMeta<T>(std::marker::PhantomData<T>);

    impl<T: Config> OnRuntimeUpgrade for MigrateIcoMeta<T> {
        fn on_runtime_upgrade() -> Weight {
            let version = StorageVersion::get::<Pallet<T>>();

            if version != 3 {
                return 0;
            }

            let token_num: BalanceOf<T> = TryInto::try_into(10_000_000 * DOLLARS)
                .map_err(|_e| Error::NumberConversionFailed)
                .unwrap();

            let transfer_num: BalanceOf<T> = TryInto::try_into(1_000_000 * DOLLARS)
                .map_err(|_e| Error::NumberConversionFailed)
                .unwrap();

            for (nft_id, meta) in Metadata::<T>::iter() {
                if meta.minted {
                    if IcoMetaOf::<T>::get(nft_id).is_none() {
                        log::info!("start to migrate nft_id {:?}", nft_id);
                        let deposit = Deposit::<T>::get(nft_id).unwrap();

                        IcoMetaOf::<T>::insert(
                            nft_id,
                            IcoMeta::<T> {
                                done: true,
                                expected_currency: deposit,
                                offered_tokens: token_num,
                                pot: Pallet::<T>::generate_ico_pot(&nft_id),
                            },
                        );

                        let owner_account =
                            parami_did::Pallet::<T>::lookup_did(meta.owner).unwrap();

                        let result = T::Assets::transfer(
                            nft_id,
                            &meta.pot,
                            &owner_account,
                            transfer_num,
                            false,
                        );

                        if result.is_err() {
                            log::error!("token transfer error {:?}", nft_id);
                            panic!("token tranfer error");
                        }

                        log::info!("end to migrate nft_id {:?}", nft_id);
                    }
                }
            }

            StorageVersion::put::<Pallet<T>>(&StorageVersion::new(4));
            0
        }

        #[cfg(feature = "try-runtime")]
        fn pre_upgrade() -> Result<(), &'static str> {
            use frame_support::migration::storage_key_iter;
            let storage_version = StorageVersion::get::<Pallet<T>>();
            assert!(storage_version == 3, "current storage version should be 3");

            let mut key_count = 0;

            for _ in <Metadata<T>>::iter() {
                key_count += 1;
            }

            log::info!("metadata key count: {:?}", key_count);
            Ok(())
        }

        #[cfg(feature = "try-runtime")]
        fn post_upgrade() -> Result<(), &'static str> {
            let storage_version = StorageVersion::get::<Pallet<T>>();
            assert!(storage_version == 4, "current storage version should be 4");

            let mut key_count = 0;
            for _ in <IcoMetaOf<T>>::iter() {
                key_count += 1;
            }

            log::info!("ico meta key count: {:?}", key_count);
            Ok(())
        }
    }
}
