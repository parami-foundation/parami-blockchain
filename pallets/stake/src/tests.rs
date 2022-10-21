use crate::{mock::*, StakingActivityStore};
use frame_support::{
    assert_noop, assert_ok,
    traits::{tokens::fungibles::Mutate as FungMutate, Currency},
};

/*Profit Invariants Start */
#[test]
pub fn get_profit_will_never_exceed_total_reward_amount_in_138240_block_gap() {
    get_profit_will_never_exceed_total_reward_amount(138240);
}

#[test]
pub fn get_profit_will_never_exceed_total_reward_amount_in_69120_block_gap() {
    get_profit_will_never_exceed_total_reward_amount(69120);
}

#[test]
pub fn get_profit_will_never_exceed_total_reward_amount_in_34560_block_gap() {
    get_profit_will_never_exceed_total_reward_amount(34560);
}

#[test]
pub fn get_profit_will_never_exceed_total_reward_amount_in_17280_block_gap() {
    get_profit_will_never_exceed_total_reward_amount(17280);
}

#[test]
pub fn get_profit_will_never_exceed_total_reward_amount_in_8640_block_gap() {
    get_profit_will_never_exceed_total_reward_amount(8640);
}

#[test]
pub fn get_profit_will_never_exceed_total_reward_amount_in_4320_block_gap() {
    get_profit_will_never_exceed_total_reward_amount(4320);
}

#[test]
pub fn get_profit_will_never_exceed_total_reward_amount_in_2160_block_gap() {
    get_profit_will_never_exceed_total_reward_amount(2160);
}

#[test]
pub fn get_profit_will_never_exceed_total_reward_amount_in_1080_block_gap() {
    get_profit_will_never_exceed_total_reward_amount(1080);
}

#[test]
pub fn get_profit_will_never_exceed_total_reward_amount_in_540_block_gap() {
    get_profit_will_never_exceed_total_reward_amount(540);
}

#[test]
pub fn get_profit_will_never_exceed_total_reward_amount_in_270_block_gap() {
    let block_gap_of_get_reward = 270;
    get_profit_will_never_exceed_total_reward_amount(270);
}

#[test]
pub fn get_profit_will_never_exceed_total_reward_amount_in_90_block_gap() {
    get_profit_will_never_exceed_total_reward_amount(90);
}

pub fn get_profit_will_never_exceed_total_reward_amount(block_gap_of_get_reward: u64) {
    //7884000 is block num in 3 years.
    let loop_count = 7884000 / block_gap_of_get_reward;
    new_test_ext().execute_with(|| {
        let asset_id = 9;

        let reward_total_amount = 7_000_000u128 * 10u128.pow(18);

        assert_ok!(Assets::force_create(Origin::root(), asset_id, BOB, true, 1));

        assert_ok!(Stake::start(asset_id, reward_total_amount));

        let activity = <StakingActivityStore<Test>>::get(asset_id).unwrap();

        //7884000
        for _ in 0..loop_count {
            let cur_blocknum = <frame_system::Pallet<Test>>::block_number();
            System::set_block_number(cur_blocknum + block_gap_of_get_reward);
            assert_ok!(Stake::get_reward(&activity, &BOB));
        }

        let activity = <StakingActivityStore<Test>>::get(asset_id).unwrap();
        let balance = Assets::balance(asset_id, activity.reward_pot);
        assert!(
            balance < reward_total_amount * 11 / 10,
            "reward in pot {:} should be less than reward_total_amount {:}",
            balance,
            reward_total_amount
        );

        assert!(
            balance > reward_total_amount * 2 / 3,
            "reward in pot {:} should be more than 2 / 3 of reward_total_amount {:} ",
            balance,
            reward_total_amount
        );
        print!(
            "balance of pot is {:?}, reward_total_amount is {:?}",
            balance, reward_total_amount
        );
    });
}

/*Profit Invariants End */

/* For Invariants Start */
#[test]
pub fn invariant_holds_after_multi_stake_and_multi_withdraw() {
    new_test_ext().execute_with(|| {
        let asset_id = 9;

        let reward_total_amount = 7_000_000u128 * 10u128.pow(18);

        assert_ok!(Assets::force_create(Origin::root(), asset_id, BOB, true, 1));

        assert_ok!(Stake::start(asset_id, reward_total_amount));

        let activity = <StakingActivityStore<Test>>::get(asset_id).unwrap();

        assert_ok!(Stake::stake(20, asset_id, &ALICE));

        System::set_block_number(20);

        assert_ok!(Stake::stake(20, asset_id, &ALICE));

        System::set_block_number(40);

        assert_ok!(Stake::stake(20, asset_id, &CHARLIE));

        System::set_block_number(60);

        assert_ok!(Stake::stake(20, asset_id, &CHARLIE));

        let reward_pot_balance = Assets::balance(asset_id, activity.reward_pot);

        let activity = <StakingActivityStore<Test>>::get(asset_id).unwrap();
        let earned_total = Stake::earned(&activity, &ALICE) + Stake::earned(&activity, &CHARLIE);
        assert_eq!(reward_pot_balance, earned_total,);
    });
}
/* For Invariants End */

#[test]
pub fn no_two_assets_staking_activity_pot_will_conflict() {
    use std::collections::HashSet;
    let mut exists_pots = HashSet::new();

    new_test_ext().execute_with(|| {
        for asset_id in 1..400_000 {
            assert_ok!(Assets::force_create(Origin::root(), asset_id, BOB, true, 1));
            assert_ok!(Stake::start(asset_id, 7_000_000u128 * 10u128.pow(18)));

            let activity = <StakingActivityStore<Test>>::get(asset_id).unwrap();
            assert_eq!(exists_pots.contains(&activity.reward_pot), false);

            exists_pots.insert(activity.reward_pot);
        }
    });
}

/*
 * For Staking Activity Start
 */
#[test]
pub fn total_supply_should_change_exactly_after_withdraw() {}

#[test]
pub fn total_supply_should_change_after_stake() {}

#[test]
pub fn total_supply_and_user_balance_should_change_exactly_after_stake() {}

#[test]
pub fn total_supply_and_user_balance_should_not_change_after_get_reward() {}

#[test]
pub fn earnings_per_share_should_change_exatly_after_make_profit() {}

#[test]
pub fn reward_total_remains_should_change_exactly_after_make_profit() {}

#[test]
pub fn halve_time_and_daily_output_should_change_after_hit_halve_time_in_make_profit() {}

#[test]
pub fn last_block_should_change_after_make_profit() {}

/*
 * For Staking Activity End
 */

/*
For User State Start
 */
#[test]
pub fn earned_and_user_balance_should_decrease_exactly_after_withdraw() {}

#[test]
pub fn earned_should_decrease_exactly_after_get_reward() {}

#[test]
pub fn earned_and_user_balance_should_increase_exactly_after_stake() {}

#[test]
pub fn earned_should_be_zero_when_stake_in_the_same_block_with_start() {
    new_test_ext().execute_with(|| {
        let asset_id = 9;

        let reward_total_amount = 7_000_000u128 * 10u128.pow(18);

        assert_ok!(Assets::force_create(Origin::root(), asset_id, BOB, true, 1));

        assert_ok!(Stake::start(asset_id, reward_total_amount));

        let activity = <StakingActivityStore<Test>>::get(asset_id).unwrap();

        Stake::stake(20, asset_id, &ALICE);

        let activity = <StakingActivityStore<Test>>::get(asset_id).unwrap();
        let earned = Stake::earned(&activity, &ALICE);

        assert_eq!(earned, 0);
    });
}
/*
For User State End
*/
