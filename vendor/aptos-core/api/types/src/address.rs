// Copyright © Aptos Foundation
// Parts of the project are originally copyright © Meta Platforms, Inc.
// SPDX-License-Identifier: Apache-2.0

use move_core_types::account_address::AccountAddress;
use serde::{de::Error as _, Deserialize, Deserializer, Serialize, Serializer};
use std::{fmt, str::FromStr};

/// The address of an account
///
/// This is represented in a string as a 64 character hex string, sometimes
/// shortened by stripping leading 0s, and adding a 0x.
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Address(AccountAddress);

impl fmt::Display for Address {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // While the inner type, AccountAddress, has a Display impl already, we don't
        // use it. As part of the AIP-40 migration, the Display impl of the inner
        // AccountAddress was changed to conform to AIP-40, but doing that for the API
        // would constitute a breaking change. So we keep an explicit display impl
        // here that maintains the existing address formatting behavior.
        write!(f, "{}", self.0.to_hex_literal())
    }
}

impl FromStr for Address {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Self, anyhow::Error> {
        Ok(Self(AccountAddress::from_str(s).map_err(|e| {
            anyhow::format_err!("Invalid account address: {:#}", e)
        })?))
    }
}

impl Serialize for Address {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.to_string().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Address {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let address = <String>::deserialize(deserializer)?;
        address.parse().map_err(D::Error::custom)
    }
}

impl From<AccountAddress> for Address {
    fn from(address: AccountAddress) -> Self {
        Self(address)
    }
}

impl From<Address> for AccountAddress {
    fn from(address: Address) -> Self {
        address.0
    }
}

impl From<&Address> for AccountAddress {
    fn from(address: &Address) -> Self {
        address.0
    }
}

impl From<Address> for move_core_types::value::MoveValue {
    fn from(d: Address) -> Self {
        move_core_types::value::MoveValue::Address(d.0)
    }
}
