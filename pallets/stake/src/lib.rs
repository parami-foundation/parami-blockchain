#[cfg(test)]
mod mock;
#[cfg(test)]
mod tests;

use codec::{Decode, Encode, MaxEncodedLen};
use frame_support::traits::{
    tokens::fungibles::{
        InspectMetadata as FungMeta, Mutate as FungMutate, Transfer as FungTransfer,
    },
    Currency,
};
use scale_info::TypeInfo;
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};
use sp_runtime::traits::{AtLeast32BitUnsigned, Bounded, Saturating, Zero};
use sp_runtime::RuntimeDebug;
use sp_std::prelude::*;

type AssetIdOf<T> = <T as pallet::Config>::AssetId;
type AccountOf<T> = <T as frame_system::Config>::AccountId;
type HeightOf<T> = <T as frame_system::Config>::BlockNumber;
type BalanceOf<T> = <<T as pallet::Config>::Currency as Currency<AccountOf<T>>>::Balance;

#[derive(Clone, Decode, Default, Encode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct AssetRewardActivity<A, H, B> {
    pub asset_id: A,
    pub start_block_num: H,
    pub halve_time: H,
    pub lastblock: H,
    pub total_supply: B,
    pub earnings_per_share: B,
    pub daily_output: B,
}

type StakingActivityOf<T> = AssetRewardActivity<AssetIdOf<T>, HeightOf<T>, BalanceOf<T>>;

//7 days
const DURATION: u32 = 7 * 24 * 60 * 5;

/**
 * This const is the normalized INIT_DAILY_OUTPUT, and the normalized total amount is 1_000_000.
 *
 * In English:
 *
 * As summation of proportional series, we can resolve the INIT_DAILY_OUTPUT
 *
 * Assumptions:
 * 1. x = INIT_DAILY_OUTPUT
 * 2. n = 365/7 * 3 = 156 weeks
 * 3. S = 1_000_000
 *
 * x + x/2 + x / 2^2 + ... + x/2^156 = 1_000_000
 *
 * as summation of proportional series says: S * (1 - q) = a1 - a_n+1
 *
 * bring in the variables:
 *
 * 1_000_000 * (1 - 1/2) = x - x * (1 / 2^157)
 *
 * so x = 500_000
 *
 *
 * In Chinese
 * 根据等比数列求和公式，求解一下等比数列的和
 * 1. 设 x 为第一周释放量
 * 2. 三年总共365/7 * 3 = 156周
 * 3. 按照3年释放100W币来做归一化，方便各中值的计算
 *
 * x + x/2 + x / 2^2 + ... + x/2^156 = 1_000_000
 *
 * 根据等比数列规律: (1 - q) * S = a1 - a_n+1，即S = (a1 - a_n+1) / 1 - q
 *
 * 带入该公式：1_000_000 = (x - x/2^157) / (1 - 1/2) -> x = 500_000 / (1 - 1/2^157) ~ 500_000
 */
const ONE_MILLION_NORMALIZED_INIT_DAILY_OUTPUT: u128 = 500_000u128 * 10 ^ 18;

#[frame_support::pallet]
pub mod pallet {
    use super::*;
    use frame_support::{pallet_prelude::*, Twox64Concat};
    use sp_runtime::DispatchError;

    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// The currency trait
        type Currency: Currency<AccountOf<Self>>;

        /// Fungible token ID type
        type AssetId: Parameter
            + Member
            + MaybeSerializeDeserialize
            + AtLeast32BitUnsigned
            + Default
            + Bounded
            + Copy
            + MaxEncodedLen;

        /// The assets trait to create, mint, and transfer fungible tokens
        type Assets: FungMeta<AccountOf<Self>, AssetId = AssetIdOf<Self>>
            + FungMutate<AccountOf<Self>, AssetId = Self::AssetId, Balance = BalanceOf<Self>>
            + FungTransfer<AccountOf<Self>, AssetId = AssetIdOf<Self>, Balance = BalanceOf<Self>>;
    }

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    #[pallet::hooks]
    impl<T: Config> Hooks<T::BlockNumber> for Pallet<T> {}

    #[pallet::storage]
    pub(super) type StakingActivityStore<T: Config> = StorageMap<
        _,
        Twox64Concat,
        AssetIdOf<T>, // Asset ID
        StakingActivityOf<T>,
    >;

    #[pallet::storage]
    pub(super) type UserStakingRewardStore<T: Config> = StorageDoubleMap<
        _,
        Twox64Concat,
        AssetIdOf<T>,
        Twox64Concat,
        AccountOf<T>,
        BalanceOf<T>,
        ValueQuery,
    >;

    #[pallet::storage]
    pub(super) type UserStakingBalanceStore<T: Config> = StorageDoubleMap<
        _,
        Twox64Concat,
        AssetIdOf<T>,
        Twox64Concat,
        AccountOf<T>,
        BalanceOf<T>,
        ValueQuery,
    >;

    #[pallet::error]
    pub enum Error<T> {
        ActivityNotExists,
        ActivityAlreadyExists,
        ActivityNotStarted,
        InvalidAmount,
        TypeCastError,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {}

    impl<T: Config> Pallet<T> {
        pub fn start(asset_id: AssetIdOf<T>) -> Result<(), DispatchError> {
            let already_exists = <StakingActivityStore<T>>::contains_key(asset_id);
            ensure!(!already_exists, Error::<T>::ActivityAlreadyExists);

            let cur_blocknum = <frame_system::Pallet<T>>::block_number();
            let duration = HeightOf::<T>::from(DURATION);
            let daily_output = BalanceOf::<T>::try_from(ONE_MILLION_NORMALIZED_INIT_DAILY_OUTPUT)
                .map_err(|_| Error::<T>::TypeCastError)?;

            <StakingActivityStore<T>>::insert(
                asset_id,
                AssetRewardActivity {
                    asset_id,
                    start_block_num: cur_blocknum,
                    halve_time: cur_blocknum.saturating_add(duration),
                    lastblock: HeightOf::<T>::zero(),
                    total_supply: BalanceOf::<T>::zero(),
                    earnings_per_share: BalanceOf::<T>::zero(),
                    daily_output,
                },
            );
            Ok(())
        }

        /**
        * function getPerBlockOutput() public view returns (uint256) {
               return DailyOutput.div(6646);// 13秒1个区块,每天大概是6646个区块 //https://etherscan.io/chart/blocktime
           }
        */
        pub fn get_per_block_output(asset_id: AssetIdOf<T>) -> Result<BalanceOf<T>, DispatchError> {
            let activity =
                <StakingActivityStore<T>>::get(asset_id).ok_or(Error::<T>::ActivityNotExists)?;
            //one block per 12 seconds, so 1 day has 7200 blocks
            Ok(activity.daily_output / 7200u32.into())
        }

        /**
         *function getprofit() private returns (uint256) {
            if (block.timestamp > Halvetime){
                DailyOutput = DailyOutput.div(2); //减半
                Halvetime = block.timestamp + DURATION;
            }
            uint256 new_blocknum = block.number;
            if (new_blocknum <= lastblock) {
                return 0;
            }
            uint256 diff = new_blocknum.sub(lastblock);
            lastblock = new_blocknum;
            uint256 profit = diff.mul(getPerBlockOutput());
            return profit;
        }
         */
        fn get_profit(activity: &StakingActivityOf<T>) -> Result<BalanceOf<T>, DispatchError> {
            let cur_block_num = <frame_system::Pallet<T>>::block_number();
            if cur_block_num > activity.halve_time {
                <StakingActivityStore<T>>::mutate(activity.asset_id, |activity| {
                    if let Some(activity) = activity {
                        activity.daily_output = activity.daily_output / 2u32.into();
                        activity.halve_time = cur_block_num + DURATION.into();
                    }
                });
            }
            let new_blocknum = cur_block_num;
            if new_blocknum <= activity.lastblock {
                return Ok(Zero::zero());
            }

            let diff: u32 = new_blocknum
                .saturating_sub(activity.lastblock)
                .try_into()
                .map_err(|_| Error::<T>::TypeCastError)?;

            <StakingActivityStore<T>>::mutate(activity.asset_id, |activity| {
                if let Some(activity) = activity {
                    activity.lastblock = new_blocknum;
                }
            });
            let per_block_output = Self::get_per_block_output(activity.asset_id)?;
            let profit = per_block_output.saturating_mul(diff.into());
            Ok(profit)
        }

        /**
        * modifier make_profit() {
               uint256 amount = getprofit();
               if (amount > 0) {
                   yfi.mint(address(this), amount);
                   if (totalSupply() == 0){
                       earnings_per_share = 0;
                   }else{
                       earnings_per_share = earnings_per_share.add(
                       amount.div(totalSupply())
                   );
                   }

               }
               _;
           }
        */
        fn make_profit(asset_id: AssetIdOf<T>) -> Result<(), DispatchError> {
            let activity =
                <StakingActivityStore<T>>::get(asset_id).ok_or(Error::<T>::ActivityNotExists)?;
            let amount = Self::get_profit(&activity)?;
            if amount > Zero::zero() {
                if activity.total_supply == Zero::zero() {
                    <StakingActivityStore<T>>::mutate(asset_id, |activity| {
                        if let Some(activity) = activity {
                            activity.earnings_per_share = Zero::zero();
                        }
                    });
                } else {
                    <StakingActivityStore<T>>::mutate(asset_id, |activity| {
                        if let Some(activity) = activity {
                            activity.earnings_per_share += amount / activity.total_supply;
                        }
                    });
                }
            }
            Ok(())
        }

        /**
        refer to YearnRewards's stake implementation:

        require(block.timestamp >starttime,"not start");
        require(amount > 0, "Cannot stake 0");
        if (earnings_per_share == 0){
            rewards[msg.sender] = 0;
        }else{
            rewards[msg.sender] = rewards[msg.sender].add(
                earnings_per_share.mul(amount)
            );
        }
        super.stake(amount);
        emit Staked(msg.sender, amount);
         */
        pub fn stake(
            amount: BalanceOf<T>,
            asset_id: AssetIdOf<T>,
            account: AccountOf<T>,
        ) -> Result<(), sp_runtime::DispatchError> {
            // 1. call make_profit first
            Self::make_profit(asset_id)?;

            // Others
            let reward_activity =
                <StakingActivityStore<T>>::get(asset_id).ok_or(Error::<T>::ActivityNotExists)?;

            let cur_block = <frame_system::Pallet<T>>::block_number();
            ensure!(
                cur_block >= reward_activity.start_block_num,
                Error::<T>::ActivityNotStarted
            );
            ensure!(amount > Zero::zero(), Error::<T>::InvalidAmount);

            if reward_activity.earnings_per_share == Zero::zero() {
                <UserStakingRewardStore<T>>::mutate(asset_id, &account, |rewards| {
                    rewards.set_zero();
                });
            } else {
                <UserStakingRewardStore<T>>::mutate(asset_id, &account, |rewards| {
                    rewards.saturating_accrue(reward_activity.earnings_per_share * amount)
                });
            }

            Self::stake_inner(asset_id, &account, amount);

            Ok(())
            // TODO(ironman_ch): emit Staked(msg.sender, amount);
        }

        /**
        * function withdraw(uint256 amount) public make_profit
           {
               require(amount > 0, "Cannot withdraw 0");
               getReward();

               rewards[msg.sender] = rewards[msg.sender].sub(
                   earnings_per_share.mul(amount)
               );
               super.withdraw(amount);
               emit Withdrawn(msg.sender, amount);
           }
        */
        pub fn withdraw(
            asset_id: AssetIdOf<T>,
            account: &AccountOf<T>,
            amount: BalanceOf<T>,
        ) -> Result<(), sp_runtime::DispatchError> {
            // 1. call make_profit();
            Self::make_profit(asset_id)?;

            ensure!(amount > Zero::zero(), Error::<T>::InvalidAmount);
            //

            let activity =
                <StakingActivityStore<T>>::get(asset_id).ok_or(Error::<T>::ActivityNotExists)?;

            Self::get_reward(&activity, &account)?;

            <UserStakingRewardStore<T>>::mutate(asset_id, &account, |user_staking_reward| {
                user_staking_reward.saturating_accrue(activity.earnings_per_share * amount)
            });

            Self::withdraw_inner(asset_id, &account, amount);
            Ok(())
        }

        /**
        * function exit() external {
               withdraw(balanceOf(msg.sender));
           }
        */
        pub fn exit(asset_id: AssetIdOf<T>, account: &AccountOf<T>) -> Result<(), DispatchError> {
            let amount = <UserStakingBalanceStore<T>>::get(asset_id, account);
            Self::withdraw(asset_id, account, amount)?;
            Ok(())
        }

        /**
        * function getReward() public make_profit  {
               uint256 reward = earned(msg.sender);
               if (reward > 0) {
                   rewards[msg.sender] = earnings_per_share.mul(balanceOf(msg.sender));
                   yfi.safeTransfer(msg.sender, reward);
                   emit RewardPaid(msg.sender, reward);
               }
           }
        */
        pub fn get_reward(
            activity: &StakingActivityOf<T>,
            account: &AccountOf<T>,
        ) -> Result<BalanceOf<T>, DispatchError> {
            //1. make_profit first
            Self::make_profit(activity.asset_id)?;

            //Others
            let reward = Self::earned(&activity, account);
            if reward > Zero::zero() {
                <UserStakingRewardStore<T>>::insert(
                    activity.asset_id,
                    account,
                    activity.earnings_per_share
                        * Self::staking_balance_of_inner(activity.asset_id, account),
                );
                Self::transfer_to(activity.asset_id, account, reward)?;
                //TODO(ironman_ch): emit RewardPaid(msg.sender, reward);
            }

            Ok(reward)
        }

        /**
        * function earned(address account) public view returns (uint256) {
               uint256 _cal = earnings_per_share.mul(balanceOf(account));
               if (_cal < rewards[msg.sender]) {
                   return 0;
               } else {
                   return _cal.sub(rewards[msg.sender]);
               }
           }
        */
        fn earned(activity: &StakingActivityOf<T>, account: &AccountOf<T>) -> BalanceOf<T> {
            let cal = activity.earnings_per_share
                * Self::staking_balance_of_inner(activity.asset_id, account);

            let cur_reward_of_user = <UserStakingRewardStore<T>>::get(activity.asset_id, account);

            if cal < cur_reward_of_user {
                return Zero::zero();
            } else {
                return cal.saturating_sub(cur_reward_of_user);
            }
        }

        /**
        function stake(uint256 amount) public {
            _totalSupply = _totalSupply.add(amount);
            _balances[msg.sender] = _balances[msg.sender].add(amount);
            y.safeTransferFrom(msg.sender, address(this), amount);
        }
        */
        fn stake_inner(asset_id: AssetIdOf<T>, account: &AccountOf<T>, amount: BalanceOf<T>) {
            <StakingActivityStore<T>>::mutate(asset_id, |activity| {
                if let Some(activity) = activity {
                    activity.total_supply.saturating_accrue(amount)
                }
            });

            <UserStakingBalanceStore<T>>::mutate(asset_id, account, |user_balance| {
                user_balance.saturating_accrue(amount)
            })
        }

        fn withdraw_inner(asset_id: AssetIdOf<T>, account: &AccountOf<T>, amount: BalanceOf<T>) {
            <StakingActivityStore<T>>::mutate(asset_id, |activity| {
                if let Some(activity) = activity {
                    activity.total_supply.saturating_sub(amount);
                }
            });

            <UserStakingBalanceStore<T>>::mutate(asset_id, account, |user_balance| {
                user_balance.saturating_sub(amount)
            });
        }

        fn staking_balance_of_inner(
            asset_id: AssetIdOf<T>,
            account: &AccountOf<T>,
        ) -> BalanceOf<T> {
            <UserStakingBalanceStore<T>>::get(asset_id, account)
        }

        fn transfer_to(
            asset_id: AssetIdOf<T>,
            account: &AccountOf<T>,
            amount: BalanceOf<T>,
        ) -> Result<(), DispatchError> {
            T::Assets::mint_into(asset_id, account, amount)?;
            Ok(())
        }
    }
}
