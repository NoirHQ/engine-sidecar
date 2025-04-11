// Copyright © Aptos Foundation
// Parts of the project are originally copyright © Meta Platforms, Inc.
// SPDX-License-Identifier: Apache-2.0

pub use aptos_api_types::deserialize_from_string;
use aptos_types::transaction::authenticator::AuthenticationKey;
use serde::{Deserialize, Deserializer};
use std::str::FromStr;

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct Account {
    #[serde(deserialize_with = "deserialize_from_prefixed_hex_string")]
    pub authentication_key: AuthenticationKey,
    #[serde(deserialize_with = "deserialize_from_string")]
    pub sequence_number: u64,
}

pub fn deserialize_from_prefixed_hex_string<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    D: Deserializer<'de>,
    T: FromStr,
    <T as FromStr>::Err: std::fmt::Display,
{
    use serde::de::Error;

    let s = <String>::deserialize(deserializer)?;
    s.trim_start_matches("0x")
        .parse::<T>()
        .map_err(D::Error::custom)
}
