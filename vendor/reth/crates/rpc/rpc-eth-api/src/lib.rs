//! Reth RPC `eth_` API implementation
//!
//! ## Feature Flags
//!
//! - `client`: Enables JSON-RPC client support.

pub mod core;
pub mod types;

pub use core::EthApiServer;
pub use types::{RpcBlock, RpcHeader, RpcReceipt, RpcTransaction};
