use crate::{mock::*, StakingActivityStore};
use frame_support::{
    assert_noop, assert_ok,
    traits::{tokens::fungibles::Mutate as FungMutate, Currency},
};
use sp_runtime::{AccountId32, MultiAddress};

#[test]
pub fn get_profit_will_never_exceed_total_reward_amount() {
    new_test_ext().execute_with(|| {
        let asset_id = 9;

        let reward_total_amount = 7_000_000u128 * 10u128.pow(18);

        assert_ok!(Assets::force_create(Origin::root(), asset_id, BOB, true, 1));

        assert_ok!(Stake::start(asset_id, reward_total_amount));

        let activity = <StakingActivityStore<Test>>::get(asset_id).unwrap();
        for _ in 0..12060 {
            let cur_blocknum = <frame_system::Pallet<Test>>::block_number();
            System::set_block_number(cur_blocknum + 540);
            assert_ok!(Stake::get_reward(&activity, &BOB));
        }

        let activity = <StakingActivityStore<Test>>::get(asset_id).unwrap();
        let balance = Assets::balance(asset_id, activity.reward_pot);
        assert!(
            balance < reward_total_amount,
            "reward in pot {:} should be less than reward_total_amount {:}",
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
