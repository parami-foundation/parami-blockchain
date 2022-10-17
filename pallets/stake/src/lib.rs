use codec::{Decode, Encode, MaxEncodedLen};
use frame_support::traits::{Currency, Get};
use scale_info::TypeInfo;
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};
use sp_runtime::traits::{AtLeast32BitUnsigned, Bounded, Saturating, Zero};
use sp_runtime::RuntimeDebug;

type AssetIdOf<T> = <T as pallet::Config>::AssetId;
type AccountOf<T> = <T as frame_system::Config>::AccountId;
type HeightOf<T> = <T as frame_system::Config>::BlockNumber;
type BalanceOf<T> = <<T as pallet::Config>::Currency as Currency<AccountOf<T>>>::Balance;

#[derive(Clone, Decode, Default, Encode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct AssetRewardActivity<H, B> {
    pub start_block_num: H,
    pub total_supply: B,
    pub earnings_per_share: B,
}

type AssetRewardActivityOf<T> = AssetRewardActivity<HeightOf<T>, BalanceOf<T>>;

#[frame_support::pallet]
pub mod pallet {
    use super::*;
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;

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
    }

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    #[pallet::hooks]
    impl<T: Config> Hooks<T::BlockNumber> for Pallet<T> {}

    #[pallet::storage]
    pub(super) type StakingActivity<T: Config> = StorageMap<
        _,
        Twox64Concat,
        AssetIdOf<T>, // Asset ID
        AssetRewardActivityOf<T>,
    >;

    #[pallet::storage]
    pub(super) type StakingRewards<T: Config> = StorageDoubleMap<
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
        ACIVITY_NOT_EXISTS,
        NOT_STARTED,
        INVALID_AMOUNT,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {}

    impl<T: Config> Pallet<T> {
        pub fn start(assetId: AssetIdOf<T>) {}

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
        ) -> Result<(), BalanceOf<T>> {
            // 1. call make_profit first
            Self::make_profit();
            let reward_activity =
                <StakingActivity<T>>::get(asset_id).ok_or(Error::<T>::ACIVITY_NOT_EXISTS)?;
            // for now, swap's created time is reward's start time

            let cur_block = <frame_system::Pallet<T>>::block_number();
            ensure!(
                cur_block >= reward_activity.start_block_num,
                Error::<T>::NOT_STARTED
            );
            ensure!(amount > Zero::zero(), Error::<T>::INVALID_AMOUNT);

            if (reward_activity.earnings_per_share == Zero::zero()) {
                <StakingRewards<T>>::mutate(asset_id, account, |rewards| {
                    rewards.set_zero();
                });
            } else {
                <StakingRewards<T>>::mutate(asset_id, account, |rewards| {
                    rewards.saturating_accrue(reward_activity.earnings_per_share * amount)
                });
            }

            Self::stake_inner(asset_id, amount);

            Ok(())
            // emit Staked(msg.sender, amount);
        }

        fn make_profit() {
            //TODO(ironman_ch):
        }

        /**
        function stake(uint256 amount) public {
            _totalSupply = _totalSupply.add(amount);
            _balances[msg.sender] = _balances[msg.sender].add(amount);
            y.safeTransferFrom(msg.sender, address(this), amount);
        }
        */
        fn stake_inner(asset_id: AssetIdOf<T>, amount: BalanceOf<T>) {
            <StakingActivity<T>>::mutate(asset_id, |activity| {
                activity.total_supply.saturating_accrue(amount)
            });
        }
    }
}
