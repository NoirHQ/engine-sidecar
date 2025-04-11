// Copyright © Aptos Foundation
// Parts of the project are originally copyright © Meta Platforms, Inc.
// SPDX-License-Identifier: Apache-2.0

mod account;
mod address;
mod block;
mod error;
mod hash;
mod headers;
mod index;
pub mod mime_types;
mod move_types;
pub mod transaction;
mod wrappers;

pub use account::AccountData;
pub use address::Address;
pub use block::Block;
pub use error::{AptosError, AptosErrorCode};
pub use hash::HashValue;
pub use headers::*;
pub use index::IndexResponse;
pub use move_types::{
    EntryFunctionId, HexEncodedBytes, MoveModuleId, MoveStructTag, MoveType, U64,
};
use serde::{Deserialize, Deserializer};
use std::str::FromStr;
pub use transaction::{PendingTransaction, SubmitTransactionRequest, TransactionPayload};
pub use wrappers::IdentifierWrapper;

pub fn deserialize_from_string<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    D: Deserializer<'de>,
    T: FromStr,
    <T as FromStr>::Err: std::fmt::Display,
{
    use serde::de::Error;

    let s = <String>::deserialize(deserializer)?;
    s.parse::<T>().map_err(D::Error::custom)
}
