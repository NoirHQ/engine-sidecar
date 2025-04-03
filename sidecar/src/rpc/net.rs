use alloy_primitives::U64;
use jsonrpsee::core::RpcResult as Result;
use reth_rpc_api::NetApiServer;

/// `Net` API implementation.
///
/// This type provides the functionality for handling `net` related requests.
pub struct NetApi;

impl NetApiServer for NetApi {
    /// Handler for `net_version`
    fn version(&self) -> Result<String> {
        Ok(U64::from_le_slice(&hex::decode("deadbeef").unwrap()).to_string())
    }
}
