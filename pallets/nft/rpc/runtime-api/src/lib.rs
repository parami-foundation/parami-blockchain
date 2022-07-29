#![cfg_attr(not(feature = "std"), no_std)]

use codec::Codec;
use parami_primitives::BalanceWrapper;
use sp_runtime::traits::{MaybeDisplay, MaybeFromStr};
use sp_runtime::DispatchError;

pub type ApiResult<T> = Result<T, DispatchError>;

sp_api::decl_runtime_apis! {
    pub trait NftRuntimeApi<NftId, DecentralizedId, Balance>
    where
        NftId: Codec,
        DecentralizedId: Codec,
        Balance: Codec + MaybeDisplay + MaybeFromStr,
    {
        /// TODO(ironman_ch): add more comment here.
        fn get_claim_info(nft_id: NftId, claimer: DecentralizedId) -> ApiResult<(BalanceWrapper<Balance>, BalanceWrapper<Balance>, BalanceWrapper<Balance>)>;
    }
}
