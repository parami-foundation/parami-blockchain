use crate::Runtime;
use crate::VERSION;
use crate::{AccountId, Balances};
use frame_support::assert_ok;
use frame_support::storage::migration::{remove_storage_prefix, storage_key_iter};
use frame_support::storage::PrefixIterator;
use frame_support::traits::Currency;
use frame_support::traits::OnRuntimeUpgrade;
use frame_support::traits::OriginTrait;
use frame_support::weights::Weight;
use sp_core::{H160, H256};
use sp_runtime::app_crypto::UncheckedFrom;
use sp_runtime::{AccountId32, MultiAddress};
use sp_std::prelude::*;
use sp_std::str::FromStr;

const DEPRECATED_PALLETS: &'static [&'static [u8]] = &[
    b"Staking",
    b"Authorship",
    b"Session",
    b"ImOnline",
    b"AuthorityDiscovery",
    b"Offences",
    b"Historical",
    b"BagsList",
    b"ChildBounties",
    b"PhragmenElection",
    b"Bounties",
    b"Contracts",
    b"ElectionProviderMultiPhase",
    b"RandomnessCollectiveFlip",
    b"Recovery",
    b"Society",
    b"Vesting",
    b"NominationPools",
];

pub struct RemoveDeprecatedPallets;

impl OnRuntimeUpgrade for RemoveDeprecatedPallets {
    fn on_runtime_upgrade() -> Weight {
        if VERSION.spec_version > 334 {
            return 0;
        }

        for module in DEPRECATED_PALLETS {
            let key = sp_io::hashing::twox_128(module);
            let result = frame_support::storage::unhashed::kill_prefix(&key, None);
            match result {
                sp_io::KillStorageResult::AllRemoved(i) => log::info!("all removed, {:?}", i),
                sp_io::KillStorageResult::SomeRemaining(i) => log::info!("some remain, {:?}", i),
            }
        }
        1
    }

    #[cfg(feature = "try-runtime")]
    fn pre_upgrade() -> Result<(), &'static str> {
        use core::str;
        let modules: Vec<&[u8]> = vec![b"Staking"];

        if VERSION.spec_version > 334 {
            return Ok(());
        }

        for module in DEPRECATED_PALLETS {
            log::info!(
                "RemoveDeprecatedPallet, module = {:?}, key_count: {:?}",
                str::from_utf8(module),
                pallet_key_count(module),
            );
        }
        Ok(())
    }

    #[cfg(feature = "try-runtime")]
    fn post_upgrade() -> Result<(), &'static str> {
        if VERSION.spec_version > 334 {
            return Ok(());
        }

        for module in DEPRECATED_PALLETS {
            assert_eq!(pallet_key_count(module), 0);
        }
        Ok(())
    }
}

pub fn pallet_key_count(module: &[u8]) -> usize {
    let mut prefix = Vec::new();
    let key = sp_io::hashing::twox_128(module);
    prefix.extend_from_slice(&key);

    let previous_key = prefix.clone();
    let closure = |_raw_key_without_prefix: &[u8], mut _raw_value: &[u8]| Ok(());
    PrefixIterator::<()>::new(prefix, previous_key, closure).count()
}

pub struct TransferOwnership<T>(sp_std::marker::PhantomData<T>);

impl OnRuntimeUpgrade for TransferOwnership<Runtime> {
    fn on_runtime_upgrade() -> Weight {
        let current_root = pallet_sudo::Pallet::<Runtime>::key();
        let old_root =
            H256::from_str("0x129716ea04c6374cc31b3f39e25c2fd7279e1501f4664ad32af4ceb4d2e67d12")
                .unwrap();
        let old_root = AccountId32::unchecked_from(old_root);

        if current_root.map_or(true, |r| r != old_root) {
            return 0;
        }

        let new_root =
            H256::from_str("0x8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48")
                .unwrap();
        let new_root = AccountId32::unchecked_from(new_root);

        let result = pallet_sudo::Pallet::<Runtime>::set_key(
            <Runtime as frame_system::Config>::Origin::signed(old_root.clone()),
            MultiAddress::Id(new_root),
        );
        assert_ok!(result);

        let new_token_owner =
            H256::from_str("0x0c98c49f1861d5f6ed9ea27230796a76878abbfbfb9716c64b2c7479a2197435")
                .unwrap();
        let new_token_owner = AccountId32::unchecked_from(new_token_owner);

        let balance =
            <pallet_balances::Pallet<Runtime> as Currency<AccountId>>::free_balance(&old_root);

        let result = <pallet_balances::Pallet<Runtime> as Currency<AccountId>>::transfer(
            &old_root,
            &new_token_owner,
            balance,
            frame_support::traits::ExistenceRequirement::AllowDeath,
        );
        assert_ok!(result);

        0
    }

    #[cfg(feature = "try-runtime")]
    fn pre_upgrade() -> Result<(), &'static str> {
        use log::info;

        let new_token_owner =
            H256::from_str("0x0c98c49f1861d5f6ed9ea27230796a76878abbfbfb9716c64b2c7479a2197435")
                .unwrap();
        let new_token_owner = AccountId32::unchecked_from(new_token_owner);
        let new_root =
            H256::from_str("0x8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48")
                .unwrap();
        let new_root = AccountId32::unchecked_from(new_root);

        let old_root =
            H256::from_str("0x129716ea04c6374cc31b3f39e25c2fd7279e1501f4664ad32af4ceb4d2e67d12")
                .unwrap();
        let old_root = AccountId32::unchecked_from(old_root);

        info!("new_token_owner is {:?}", new_token_owner);
        info!("old_root is {:?}", old_root);
        info!("new_root is {:?}", new_root);
        Ok(())
    }

    #[cfg(feature = "try-runtime")]
    fn post_upgrade() -> Result<(), &'static str> {
        use log::info;
        let new_root = pallet_sudo::Pallet::<Runtime>::key();
        info!("new root is {:?}", new_root);

        let old_root =
            H256::from_str("0x129716ea04c6374cc31b3f39e25c2fd7279e1501f4664ad32af4ceb4d2e67d12")
                .unwrap();
        let old_root = AccountId32::unchecked_from(old_root.clone());
        let balance =
            <pallet_balances::Pallet<Runtime> as Currency<AccountId>>::free_balance(&old_root);
        assert_eq!(balance, 0);
        Ok(())
    }
}
