#![cfg_attr(not(feature = "std"), no_std)]

use codec::Codec;
use parami_primitives::BalanceWrapper;
use sp_runtime::traits::{MaybeDisplay, MaybeFromStr};
use sp_runtime::DispatchError;

pub type ApiResult<T> = Result<T, DispatchError>;

sp_api::decl_runtime_apis! {
    pub trait ClockInRuntimeApi<NftId>
    where
        NftId: Codec,
    {
        // Check if clock in is enabled
        fn is_clock_in_enabled(nft_id: NftId) -> ApiResult<bool>;
    }
}
