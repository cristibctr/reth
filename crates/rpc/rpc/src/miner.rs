use alloy_primitives::{Bytes, U128};
use async_trait::async_trait;
use jsonrpsee::core::RpcResult;
use reth_rpc_api::MinerApiServer;

/// `miner` API implementation.
///
/// This type provides the functionality for handling `miner` related requests.
#[derive(Clone, Debug, Default)]
pub struct MinerApi {}

#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
impl MinerApiServer for MinerApi {
    fn set_extra(&self, _record: Bytes) -> RpcResult<bool> {
        Ok(false)
    }

    fn set_gas_price(&self, _gas_price: U128) -> RpcResult<bool> {
        Ok(false)
    }

    fn set_gas_limit(&self, _gas_price: U128) -> RpcResult<bool> {
        Ok(false)
    }
}
