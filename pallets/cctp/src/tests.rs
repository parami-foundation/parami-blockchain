use crate::{mock::*, Error};
use frame_support::traits::tokens::fungibles::{Inspect, Mutate};
use frame_support::{assert_noop, assert_ok};
use sp_core::crypto::AccountId32;
use sp_core::{ByteArray, Pair};

const CCTP_ASSET: u64 = 10000;
const CHAIN_DOMAIN: u64 = 1;
const DOMAIN_1: u64 = 2;
const DOMAIN_1_USER: &[u8; 4] = b"user";

#[test]
fn deposit_can_success() {
    new_test_ext().execute_with(|| {
        assert_ok!(Cctp::set_signer(Origin::root(), SIGNING_DID));
        assert_ok!(Cctp::register_asset(
            Origin::signed(SIGNING_ACCOUNT),
            CCTP_ASSET,
            ASSET_1
        ));
        let before_balance = Assets::balance(ASSET_1, &ALICE);
        let total_issuance = Assets::total_issuance(ASSET_1);
        assert_ok!(Cctp::deposit(
            Origin::signed(ALICE),
            CCTP_ASSET,
            100,
            DOMAIN_1,
            DOMAIN_1_USER.as_slice().into(),
        ));
        assert_eq!(Assets::balance(ASSET_1, &ALICE), before_balance - 100);
        assert_eq!(Assets::total_issuance(ASSET_1), total_issuance - 100);

        assert_events(vec![Event::Cctp(crate::Event::Deposited(
            0,
            CCTP_ASSET,
            100,
            CHAIN_DOMAIN,
            DID_ALICE,
            DOMAIN_1,
            DOMAIN_1_USER.as_slice().into(),
        ))]);
    });
}

#[test]
fn balance_wont_change_if_fail() {
    new_test_ext().execute_with(|| {
        assert_ok!(Cctp::set_signer(Origin::root(), SIGNING_DID));

        let before_balance = Assets::balance(ASSET_1, &ALICE);
        let total_issuance = Assets::total_issuance(ASSET_1);
        assert_noop!(
            Cctp::deposit(
                Origin::signed(ALICE),
                CCTP_ASSET,
                100,
                DOMAIN_1,
                DOMAIN_1_USER.as_slice().into(),
            ),
            Error::<Test>::AssetNotRegistered
        );
        assert_eq!(Assets::balance(ASSET_1, &ALICE), before_balance);
        assert_eq!(Assets::total_issuance(ASSET_1), total_issuance);
    });
}

#[test]
fn deposit_will_fail_if_not_regisrer_asset() {
    new_test_ext().execute_with(|| {
        assert_ok!(Cctp::set_signer(Origin::root(), SIGNING_DID));
        assert_noop!(
            Cctp::deposit(
                Origin::signed(ALICE),
                CCTP_ASSET,
                100,
                DOMAIN_1,
                DOMAIN_1_USER.as_slice().into(),
            ),
            Error::<Test>::AssetNotRegistered
        );
    });
}

#[test]
fn despoit_fails_if_not_enough_balance() {
    new_test_ext().execute_with(|| {
        assert_ok!(Cctp::set_signer(Origin::root(), SIGNING_DID));
        assert_ok!(Cctp::register_asset(
            Origin::signed(SIGNING_ACCOUNT),
            CCTP_ASSET,
            ASSET_1
        ));
        assert_noop!(
            Cctp::deposit(
                Origin::signed(ALICE),
                CCTP_ASSET,
                100000000,
                DOMAIN_1,
                DOMAIN_1_USER.as_slice().into(),
            ),
            Error::<Test>::NotEnoughBalance
        );
    });
}

#[test]
fn register_asset_fails_if_no_signer() {
    new_test_ext().execute_with(|| {
        assert_noop!(
            Cctp::register_asset(Origin::signed(ALICE), CCTP_ASSET, ASSET_1),
            Error::<Test>::InvalidSigner
        );
    });
}

#[test]
fn signature_can_verify() {
    new_test_ext().execute_with(|| {
        let bob_secret_pair: sp_core::sr25519::Pair =
            sp_core::sr25519::Pair::from_string(BOB_SEED, None).unwrap();
        let bob_account = AccountId32::new(bob_secret_pair.public().as_array_ref().clone());

        let msg = Cctp::construct_msg(
            1,
            CCTP_ASSET,
            100,
            DOMAIN_1,
            &(DOMAIN_1_USER.as_slice().into()),
            DID_ALICE,
        );

        let sig = bob_secret_pair.sign(&msg);
        assert_ok!(Cctp::verify_signature(
            1,
            CCTP_ASSET,
            100,
            DOMAIN_1,
            &(DOMAIN_1_USER.as_slice().into()),
            DID_ALICE,
            sig.into(),
            bob_account,
        ));
    });
}

#[test]
fn wrong_signature_fails() {
    new_test_ext().execute_with(|| {
        let bob_secret_pair: sp_core::sr25519::Pair =
            sp_core::sr25519::Pair::from_string(BOB_SEED, None).unwrap();
        let bob_account = AccountId32::new(bob_secret_pair.public().as_array_ref().clone());

        let msg = Cctp::construct_msg(
            2,
            CCTP_ASSET,
            100,
            DOMAIN_1,
            &(DOMAIN_1_USER.as_slice().into()),
            DID_ALICE,
        );

        let sig = bob_secret_pair.sign(&msg);
        assert_noop!(
            Cctp::verify_signature(
                1,
                CCTP_ASSET,
                100,
                DOMAIN_1,
                &(DOMAIN_1_USER.as_slice().into()),
                DID_ALICE,
                sig.into(),
                bob_account,
            ),
            Error::<Test>::InvalidSignature
        );
    });
}

#[test]
fn withdraw_can_success() {
    new_test_ext().execute_with(|| {
        let bob_secret_pair: sp_core::sr25519::Pair =
            sp_core::sr25519::Pair::from_string(BOB_SEED, None).unwrap();
        let bob_account = AccountId32::new(bob_secret_pair.public().as_array_ref().clone());

        assert_ok!(Cctp::set_signer(Origin::root(), DID_BOB));
        assert_ok!(Cctp::register_asset(
            Origin::signed(bob_account),
            CCTP_ASSET,
            ASSET_1
        ));

        let msg = Cctp::construct_msg(
            1,
            CCTP_ASSET,
            100,
            DOMAIN_1,
            &(DOMAIN_1_USER.as_slice().into()),
            DID_ALICE,
        );

        let sig = bob_secret_pair.sign(&msg);

        let before_balance = Assets::balance(ASSET_1, &ALICE);
        let total_issuance = Assets::total_issuance(ASSET_1);
        assert_ok!(Cctp::withdraw(
            Origin::signed(ALICE),
            1,
            CCTP_ASSET,
            100,
            DOMAIN_1,
            DOMAIN_1_USER.as_slice().into(),
            DID_ALICE,
            sig.into()
        ));
        assert_eq!(Assets::balance(ASSET_1, &ALICE), before_balance + 100);
        assert_eq!(Assets::total_issuance(ASSET_1), total_issuance + 100);

        assert_events(vec![Event::Cctp(crate::Event::Withdrawn(
            1,
            CCTP_ASSET,
            100,
            DOMAIN_1,
            DOMAIN_1_USER.as_slice().into(),
            CHAIN_DOMAIN,
            DID_ALICE,
        ))]);
    });
}

#[test]
fn withdraw_fail_if_withdrawn_twice() {
    new_test_ext().execute_with(|| {
        let bob_secret_pair: sp_core::sr25519::Pair =
            sp_core::sr25519::Pair::from_string(BOB_SEED, None).unwrap();
        let bob_account = AccountId32::new(bob_secret_pair.public().as_array_ref().clone());

        assert_ok!(Cctp::set_signer(Origin::root(), DID_BOB));
        assert_ok!(Cctp::register_asset(
            Origin::signed(bob_account),
            CCTP_ASSET,
            ASSET_1
        ));

        let msg = Cctp::construct_msg(
            1,
            CCTP_ASSET,
            100,
            DOMAIN_1,
            &(DOMAIN_1_USER.as_slice().into()),
            DID_ALICE,
        );

        let sig = bob_secret_pair.sign(&msg);

        assert_ok!(Cctp::withdraw(
            Origin::signed(ALICE),
            1,
            CCTP_ASSET,
            100,
            DOMAIN_1,
            DOMAIN_1_USER.as_slice().into(),
            DID_ALICE,
            sig.clone().into()
        ));

        let before_balance = Assets::balance(ASSET_1, &ALICE);
        let total_issuance = Assets::total_issuance(ASSET_1);

        assert_noop!(
            Cctp::withdraw(
                Origin::signed(ALICE),
                1,
                CCTP_ASSET,
                100,
                DOMAIN_1,
                DOMAIN_1_USER.as_slice().into(),
                DID_ALICE,
                sig.into()
            ),
            Error::<Test>::UsedNonce
        );

        assert_eq!(Assets::balance(ASSET_1, &ALICE), before_balance);
        assert_eq!(Assets::total_issuance(ASSET_1), total_issuance);
    });
}

#[test]
fn withdraw_fails_if_asset_not_registered() {
    new_test_ext().execute_with(|| {
        let bob_secret_pair: sp_core::sr25519::Pair =
            sp_core::sr25519::Pair::from_string(BOB_SEED, None).unwrap();
        let bob_account = AccountId32::new(bob_secret_pair.public().as_array_ref().clone());

        assert_ok!(Cctp::set_signer(Origin::root(), DID_BOB));

        let msg = Cctp::construct_msg(
            1,
            CCTP_ASSET,
            100,
            DOMAIN_1,
            &(DOMAIN_1_USER.as_slice().into()),
            DID_ALICE,
        );

        let sig = bob_secret_pair.sign(&msg);

        assert_noop!(
            Cctp::withdraw(
                Origin::signed(ALICE),
                1,
                CCTP_ASSET,
                100,
                DOMAIN_1,
                DOMAIN_1_USER.as_slice().into(),
                DID_ALICE,
                sig.into()
            ),
            Error::<Test>::AssetNotRegistered
        );
    });
}

#[test]
fn withdraw_fails_if_signature_is_invalid() {
    new_test_ext().execute_with(|| {
        let bob_secret_pair: sp_core::sr25519::Pair =
            sp_core::sr25519::Pair::from_string(BOB_SEED, None).unwrap();
        let bob_account = AccountId32::new(bob_secret_pair.public().as_array_ref().clone());

        assert_ok!(Cctp::set_signer(Origin::root(), DID_BOB));
        assert_ok!(Cctp::register_asset(
            Origin::signed(bob_account),
            CCTP_ASSET,
            ASSET_1
        ));

        let msg = Cctp::construct_msg(
            2,
            CCTP_ASSET,
            100,
            DOMAIN_1,
            &(DOMAIN_1_USER.as_slice().into()),
            DID_ALICE,
        );

        let sig = bob_secret_pair.sign(&msg);

        assert_noop!(
            Cctp::withdraw(
                Origin::signed(ALICE),
                1,
                CCTP_ASSET,
                100,
                DOMAIN_1,
                DOMAIN_1_USER.as_slice().into(),
                DID_ALICE,
                sig.into()
            ),
            Error::<Test>::InvalidSignature
        );
    });
}

pub fn assert_events(mut expected: Vec<Event>) {
    let mut actual: Vec<Event> = frame_system::Pallet::<Test>::events()
        .iter()
        .map(|e| e.event.clone())
        .collect();

    expected.reverse();

    for evt in expected {
        let next = actual.pop().expect("event expected");
        assert_eq!(next, evt.into(), "Events don't match");
    }
}
