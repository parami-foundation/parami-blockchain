use crate::*;
use codec::{Decode, Encode, MaxEncodedLen};
use frame_support::dispatch::DispatchResult;
use frame_support::traits::fungibles::{Inspect, Mutate};
use frame_support::traits::tokens::Imbalance;
use frame_support::traits::Currency;
use frame_support::traits::{ExistenceRequirement, WithdrawReasons};
use scale_info::TypeInfo;
use sp_runtime::RuntimeDebug;
use sp_std::prelude::*;

#[derive(Clone, Decode, Encode, Eq, PartialEq, RuntimeDebug, MaxEncodedLen, TypeInfo)]
pub enum CurrencyOrAsset<AssetOf> {
    Currency,
    Asset(AssetOf),
}

pub trait AssetMutate<T: Config> {
    fn balance(&self, account: &AccountOf<T>) -> BalanceOf<T>;
    fn mint_into(&self, account: &AccountOf<T>, amount: BalanceOf<T>) -> DispatchResult;
    fn burn_from(&self, account: &AccountOf<T>, amount: BalanceOf<T>) -> DispatchResult;
}

impl<T: Config> AssetMutate<T> for CurrencyOrAsset<AssetOf<T>> {
    fn balance(&self, account: &AccountOf<T>) -> BalanceOf<T> {
        match self {
            CurrencyOrAsset::Currency => <T as crate::Config>::Currency::free_balance(account),
            CurrencyOrAsset::Asset(asset) => T::Assets::balance(*asset, &account),
        }
    }

    fn mint_into(&self, account: &AccountOf<T>, amount: BalanceOf<T>) -> DispatchResult {
        match self {
            CurrencyOrAsset::Currency => {
                let mut b = <T as crate::Config>::Currency::deposit_creating(account, amount);
            }
            CurrencyOrAsset::Asset(asset) => {
                T::Assets::mint_into(*asset, account, amount)?;
            }
        }
        Ok(())
    }

    fn burn_from(&self, account: &AccountOf<T>, amount: BalanceOf<T>) -> DispatchResult {
        match self {
            CurrencyOrAsset::Currency => {
                let mut imbalance = <T as crate::Config>::Currency::withdraw(
                    account,
                    amount,
                    WithdrawReasons::all(),
                    ExistenceRequirement::AllowDeath,
                )?;
            }
            CurrencyOrAsset::Asset(asset) => {
                T::Assets::burn_from(*asset, account, amount)?;
            }
        }
        Ok(())
    }
}
