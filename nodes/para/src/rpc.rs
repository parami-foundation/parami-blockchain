//! A collection of node-specific RPC methods.
//! Substrate provides the `sc-rpc` crate, which defines the core RPC layer
//! used by Substrate nodes. This file extends those RPC definitions with
//! capabilities that are specific to this project's runtime configuration.

#![warn(missing_docs)]

use std::sync::Arc;

use jsonrpsee::RpcModule;
use parami_para_runtime::{
    opaque::Block, AccountId, AssetId, Balance, BlockNumber, DecentralizedId, Hash, Index as Nonce,
    NftId,
};

use sc_client_api::AuxStore;
pub use sc_rpc::{DenyUnsafe, SubscriptionTaskExecutor};
use sc_transaction_pool_api::TransactionPool;
use sp_api::ProvideRuntimeApi;
use sp_block_builder::BlockBuilder;
use sp_blockchain::{Error as BlockChainError, HeaderBackend, HeaderMetadata};

/// Full client dependencies
pub struct FullDeps<C, P, B> {
    /// The backend instance to use.
    pub backend: Arc<B>,
    /// The client instance to use.
    pub client: Arc<C>,
    /// Transaction pool instance.
    pub pool: Arc<P>,
    /// Whether to deny unsafe calls
    pub deny_unsafe: DenyUnsafe,
}

/// Instantiate all Full RPC extensions.
pub fn create_full<C, P, B>(
    deps: FullDeps<C, P, B>,
) -> Result<RpcModule<()>, Box<dyn std::error::Error + Send + Sync>>
where
    C: ProvideRuntimeApi<Block>
        + HeaderBackend<Block>
        + AuxStore
        + HeaderMetadata<Block, Error = BlockChainError>
        + Sync
        + Send
        + 'static,
    C::Api: substrate_frame_rpc_system::AccountNonceApi<Block, AccountId, Nonce>,
    C::Api: pallet_contracts_rpc::ContractsRuntimeApi<Block, AccountId, Balance, BlockNumber, Hash>,
    C::Api: pallet_mmr_rpc::MmrRuntimeApi<Block, <Block as sp_runtime::traits::Block>::Hash>,
    C::Api: pallet_transaction_payment_rpc::TransactionPaymentRuntimeApi<Block, Balance>,
    C::Api: parami_swap_rpc::SwapRuntimeApi<Block, AssetId, Balance>,
    C::Api: parami_nft_rpc::NftRuntimeApi<Block, NftId, DecentralizedId, Balance>,
    C::Api: BlockBuilder<Block>,
    P: TransactionPool + 'static,
    B: sc_client_api::Backend<Block> + Send + Sync + 'static,
    B::State: sc_client_api::backend::StateBackend<sp_runtime::traits::HashFor<Block>>,
{
    use pallet_contracts_rpc::{Contracts, ContractsApiServer};
    use pallet_mmr_rpc::{Mmr, MmrApiServer};
    use pallet_transaction_payment_rpc::{TransactionPayment, TransactionPaymentApiServer};
    use parami_did_rpc::{DidApiServer, DidRpcHandler};
    use parami_nft_rpc::{NftApiServer, NftRpcHandler};
    use parami_swap_rpc::{SwapApiServer, SwapsRpcHandler};
    use substrate_frame_rpc_system::{System, SystemApiServer};

    let mut io = RpcModule::new(());
    let FullDeps {
        backend,
        client,
        pool,
        deny_unsafe,
    } = deps;

    io.merge(System::new(client.clone(), pool, deny_unsafe).into_rpc())?;
    // Making synchronous calls in light client freezes the browser currently,
    // more context: https://github.com/paritytech/substrate/pull/3480
    // These RPCs should use an asynchronous caller instead.
    io.merge(Contracts::new(client.clone()).into_rpc())?;
    io.merge(Mmr::new(client.clone()).into_rpc())?;
    io.merge(TransactionPayment::new(client.clone()).into_rpc())?;

    if let Some(did_rpc) = backend
        .offchain_storage()
        .map(|storage| DidRpcHandler::<_, DecentralizedId>::new(storage).into_rpc())
    {
        io.merge(did_rpc)?;
    }
    io.merge(SwapsRpcHandler::new(client.clone()).into_rpc())?;
    io.merge(NftRpcHandler::new(client.clone()).into_rpc())?;

    Ok(io)
}
