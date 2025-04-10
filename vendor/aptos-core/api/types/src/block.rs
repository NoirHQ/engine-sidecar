// Copyright © Aptos Foundation
// SPDX-License-Identifier: Apache-2.0

use crate::{HashValue, U64};
use serde::{Deserialize, Serialize};

/// A Block with or without transactions
///
/// This contains the information about a transactions along with
/// associated transactions if requested
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    pub block_height: U64,
    pub block_hash: HashValue,
    pub block_timestamp: U64,
    pub first_version: U64,
    pub last_version: U64,
    // /// The transactions in the block in sequential order
    // #[serde(skip_serializing_if = "Option::is_none")]
    // pub transactions: Option<Vec<Transaction>>,
}
