use crate::mock::*;
use crate::types::*;
use crate::{LastClockIn, Metadata, TagsOf};
use frame_support::traits::fungibles::{Create, Mutate};
use frame_support::{assert_noop, assert_ok};
use sp_runtime::traits::One;

#[test]
fn should_enable_clockin() {
    new_test_ext().execute_with(|| {
        let nft_id = 1;
        System::set_block_number(10);
        <Assets as Create<<Test as frame_system::Config>::AccountId>>::create(
            nft_id,
            ALICE,
            true,
            One::one(),
        )
        .unwrap();
        <Test as parami_nft::Config>::Assets::mint_into(nft_id, &ALICE, 100).unwrap();

        assert_ok!(ClockIn::enable_clock_in(
            Origin::signed(ALICE),
            nft_id,
            10,
            5,
            20,
            b"test".to_vec(),
            vec![b"tag1".to_vec(), b"tag2".to_vec(), b"tag3".to_vec()],
            50,
        ));
        let meta = Metadata::<Test>::get(nft_id).unwrap();
        assert_eq!(meta.start_at, 10);
        assert_eq!(meta.asset_id, 1);
        assert_eq!(meta.payout_base, 10);
        assert_eq!(meta.payout_max, 20);
        assert_eq!(meta.payout_min, 5);
        assert_eq!(meta.bucket_size, 10);
        assert_eq!(meta.metadata, b"test".to_vec());

        let pot = meta.pot;
        assert_eq!(Assets::balance(nft_id, pot), 50);

        let mut tag_count = 0;
        for _ in TagsOf::<Test>::iter_prefix_values(nft_id) {
            tag_count += 1;
        }
        assert_eq!(tag_count, 3);
        assert_ok!(ClockIn::add_token_reward(Origin::signed(ALICE), nft_id, 49));
    });
}

#[test]
fn should_add_token_reward() {
    new_test_ext().execute_with(|| {
        let nft_id = 1;
        System::set_block_number(10);
        <Assets as Create<<Test as frame_system::Config>::AccountId>>::create(
            nft_id,
            ALICE,
            true,
            One::one(),
        )
        .unwrap();
        <Test as parami_nft::Config>::Assets::mint_into(nft_id, &ALICE, 100).unwrap();

        assert_ok!(ClockIn::enable_clock_in(
            Origin::signed(ALICE),
            nft_id,
            10,
            5,
            20,
            b"test".to_vec(),
            vec![],
            50,
        ));

        let meta = Metadata::<Test>::get(nft_id).unwrap();
        let pot = meta.pot;
        assert_eq!(Assets::balance(nft_id, pot.clone()), 50);
        assert_ok!(ClockIn::add_token_reward(Origin::signed(ALICE), nft_id, 49));
        assert_eq!(Assets::balance(nft_id, pot), 99);
    });
}

#[test]
fn should_update_clockin() {
    new_test_ext().execute_with(|| {
        let nft_id = 1;
        System::set_block_number(10);
        <Assets as Create<<Test as frame_system::Config>::AccountId>>::create(
            nft_id,
            ALICE,
            true,
            One::one(),
        )
        .unwrap();
        <Test as parami_nft::Config>::Assets::mint_into(nft_id, &ALICE, 100).unwrap();

        assert_ok!(ClockIn::enable_clock_in(
            Origin::signed(ALICE),
            nft_id,
            10,
            5,
            20,
            b"test".to_vec(),
            vec![],
            50,
        ));
        assert_ok!(ClockIn::update_clock_in(
            Origin::signed(ALICE),
            nft_id,
            20,
            10,
            30,
            b"test1".to_vec(),
            vec![b"tag1".to_vec(), b"tag2".to_vec(), b"tag3".to_vec()],
        ));

        let meta = Metadata::<Test>::get(nft_id).unwrap();
        assert_eq!(meta.payout_base, 20);
        assert_eq!(meta.payout_max, 30);
        assert_eq!(meta.payout_min, 10);
        assert_eq!(meta.bucket_size, 10);
        assert_eq!(meta.metadata, b"test1".to_vec());

        let mut tag_count = 0;
        for _ in TagsOf::<Test>::iter_prefix_values(nft_id) {
            tag_count += 1;
        }
        assert_eq!(tag_count, 3);
    });
}

#[test]
fn should_disable_clockin() {
    new_test_ext().execute_with(|| {
        let nft_id = 1;
        System::set_block_number(10);
        <Assets as Create<<Test as frame_system::Config>::AccountId>>::create(
            nft_id,
            ALICE,
            true,
            One::one(),
        )
        .unwrap();
        <Test as parami_nft::Config>::Assets::mint_into(nft_id, &ALICE, 100).unwrap();

        assert_ok!(ClockIn::enable_clock_in(
            Origin::signed(ALICE),
            nft_id,
            10,
            5,
            20,
            b"test".to_vec(),
            vec![b"tag1".to_vec(), b"tag2".to_vec(), b"tag3".to_vec()],
            50,
        ));
        assert_ok!(ClockIn::disable_clock_in(Origin::signed(ALICE), nft_id));

        let meta = Metadata::<Test>::get(nft_id);
        assert_eq!(meta, None);

        let mut tag_count = 0;
        for _ in TagsOf::<Test>::iter_prefix_values(nft_id) {
            tag_count += 1;
        }
        assert_eq!(tag_count, 0);
    });
}
