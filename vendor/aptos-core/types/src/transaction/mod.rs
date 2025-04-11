// Copyright © Aptos Foundation
// Parts of the project are originally copyright © Meta Platforms, Inc.
// SPDX-License-Identifier: Apache-2.0

pub mod script;

use crate::chain_id::ChainId;
use aptos_crypto_derive::{BCSCryptoHash, CryptoHasher};
use move_core_types::account_address::AccountAddress;
use script::EntryFunction;
use serde::{Deserialize, Serialize};

/// Different kinds of transactions.
#[derive(Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub enum TransactionPayload {
    // /// A transaction that executes code.
    // Script(Script),
    // /// Deprecated.
    // ModuleBundle(DeprecatedPayload),
    /// A transaction that executes an existing entry function published on-chain.
    EntryFunction(EntryFunction),
    // /// A multisig transaction that allows an owner of a multisig account to execute a pre-approved
    // /// transaction as the multisig account.
    // Multisig(Multisig),
}

/// RawTransaction is the portion of a transaction that a client signs.
#[derive(
    Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize, CryptoHasher, BCSCryptoHash,
)]
pub struct RawTransaction {
    /// Sender's address.
    sender: AccountAddress,

    /// Sequence number of this transaction. This must match the sequence number
    /// stored in the sender's account at the time the transaction executes.
    sequence_number: u64,

    /// The transaction payload, e.g., a script to execute.
    payload: TransactionPayload,

    /// Maximal total gas to spend for this transaction.
    max_gas_amount: u64,

    /// Price to be paid per gas unit.
    gas_unit_price: u64,

    /// Expiration timestamp for this transaction, represented
    /// as seconds from the Unix Epoch. If the current blockchain timestamp
    /// is greater than or equal to this time, then the transaction has
    /// expired and will be discarded. This can be set to a large value far
    /// in the future to indicate that a transaction does not expire.
    expiration_timestamp_secs: u64,

    /// Chain ID of the Aptos network this transaction is intended for.
    chain_id: ChainId,
}

impl RawTransaction {
    /// Create a new `RawTransaction` with a payload.
    ///
    /// It can be either to publish a module, to execute a script, or to issue a writeset
    /// transaction.
    pub fn new(
        sender: AccountAddress,
        sequence_number: u64,
        payload: TransactionPayload,
        max_gas_amount: u64,
        gas_unit_price: u64,
        expiration_timestamp_secs: u64,
        chain_id: ChainId,
    ) -> Self {
        RawTransaction {
            sender,
            sequence_number,
            payload,
            max_gas_amount,
            gas_unit_price,
            expiration_timestamp_secs,
            chain_id,
        }
    }
}
