// Copyright © Aptos Foundation
// SPDX-License-Identifier: Apache-2.0

// The data in IndexResponse is flattened into a single JSON map to offer
// easier parsing for clients.

use crate::U64;
use aptos_config::config::RoleType;
use serde::{Deserialize, Serialize};

/// The struct holding all data returned to the client by the
/// index endpoint (i.e., GET "/").  Only for responding in JSON
#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize)]
pub struct IndexResponse {
    /// Chain ID of the current chain
    pub chain_id: u8,
    pub epoch: U64,
    pub ledger_version: U64,
    pub oldest_ledger_version: U64,
    pub ledger_timestamp: U64,
    pub node_role: RoleType,
    pub oldest_block_height: U64,
    pub block_height: U64,
    // This must be optional to be backwards compatible
    /// Git hash of the build of the API endpoint.  Can be used to determine the exact
    /// software version used by the API endpoint.
    pub git_hash: Option<String>,
}
