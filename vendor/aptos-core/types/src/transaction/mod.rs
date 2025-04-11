// Copyright © Aptos Foundation
// Parts of the project are originally copyright © Meta Platforms, Inc.
// SPDX-License-Identifier: Apache-2.0

pub mod authenticator;
pub mod script;
pub mod user_transaction_context;

use crate::chain_id::ChainId;
use aptos_crypto::hash::HashValue;
use aptos_crypto_derive::{BCSCryptoHash, CryptoHasher};
use authenticator::{AccountAuthenticator, TransactionAuthenticator};
use move_core_types::account_address::AccountAddress;
use once_cell::sync::OnceCell;
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

/// A transaction that has been signed.
///
/// A `SignedTransaction` is a single transaction that can be atomically executed. Clients submit
/// these to validator nodes, and the validator and executor submits these to the VM.
///
/// **IMPORTANT:** The signature of a `SignedTransaction` is not guaranteed to be verified. For a
/// transaction whose signature is statically guaranteed to be verified, see
/// [`SignatureCheckedTransaction`].
#[derive(Clone, Eq, Serialize, Deserialize)]
pub struct SignedTransaction {
    /// The raw transaction
    raw_txn: RawTransaction,

    /// Public key and signature to authenticate
    authenticator: TransactionAuthenticator,

    /// A cached size of the raw transaction bytes.
    /// Prevents serializing the same transaction multiple times to determine size.
    #[serde(skip)]
    raw_txn_size: OnceCell<usize>,

    /// A cached size of the authenticator.
    /// Prevents serializing the same authenticator multiple times to determine size.
    #[serde(skip)]
    authenticator_size: OnceCell<usize>,

    /// A cached hash of the transaction.
    #[serde(skip)]
    committed_hash: OnceCell<HashValue>,
}

/// PartialEq ignores the cached OnceCell fields that may or may not be initialized.
impl PartialEq for SignedTransaction {
    fn eq(&self, other: &Self) -> bool {
        self.raw_txn == other.raw_txn && self.authenticator == other.authenticator
    }
}

impl SignedTransaction {
    pub fn new_signed_transaction(
        raw_txn: RawTransaction,
        authenticator: TransactionAuthenticator,
    ) -> SignedTransaction {
        SignedTransaction {
            raw_txn,
            authenticator,
            raw_txn_size: OnceCell::new(),
            authenticator_size: OnceCell::new(),
            committed_hash: OnceCell::new(),
        }
    }

    pub fn new_single_sender(
        raw_txn: RawTransaction,
        authenticator: AccountAuthenticator,
    ) -> SignedTransaction {
        Self::new_signed_transaction(
            raw_txn,
            TransactionAuthenticator::single_sender(authenticator),
        )
    }
}
