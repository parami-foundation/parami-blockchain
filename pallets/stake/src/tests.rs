use crate::{mock::*, StakingActivityStore};
use frame_support::{
    assert_noop, assert_ok,
    traits::{tokens::fungibles::Mutate as FungMutate, Currency},
};

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

#[test]
pub fn invariant_holds_after_multi_stake_and_multi_withdraw() {}

#[test]
pub fn no_two_assets_staking_activity_pot_will_conflict() {}
