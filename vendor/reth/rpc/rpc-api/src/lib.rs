//! Reth RPC interface definitions
//!
//! Provides all RPC interfaces.
//!
//! ## Feature Flags
//!
//! - `client`: Enables JSON-RPC client support.

mod net;

/// re-export of all server traits
pub use servers::*;

/// Aggregates all server traits.
pub mod servers {
    pub use crate::net::NetApiServer;
}
