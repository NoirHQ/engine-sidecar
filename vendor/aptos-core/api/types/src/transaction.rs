// Copyright © Aptos Foundation
// Parts of the project are originally copyright © Meta Platforms, Inc.
// SPDX-License-Identifier: Apache-2.0

use crate::{
    address::Address,
    move_types::{EntryFunctionId, HexEncodedBytes, MoveType, U64},
    HashValue,
};
use serde::{Deserialize, Serialize};

/// A request to submit a transaction
///
/// This requires a transaction and a signature of it
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SubmitTransactionRequest {
    #[serde(flatten)]
    pub user_transaction_request: UserTransactionRequestInner,
    pub signature: TransactionSignature,
}

// TODO: Rename this to remove the Inner when we cut over.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserTransactionRequestInner {
    pub sender: Address,
    pub sequence_number: U64,
    pub max_gas_amount: U64,
    pub gas_unit_price: U64,
    pub expiration_timestamp_secs: U64,
    pub payload: TransactionPayload,
}

/// An enum of the possible transaction payloads
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum TransactionPayload {
    EntryFunctionPayload(EntryFunctionPayload),
    // ScriptPayload(ScriptPayload),
    // // Deprecated. We cannot remove the enum variant because it breaks the
    // // ordering, unfortunately.
    // ModuleBundlePayload(DeprecatedModuleBundlePayload),

    // MultisigPayload(MultisigPayload),
}

/// Payload which runs a single entry function
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct EntryFunctionPayload {
    pub function: EntryFunctionId,
    /// Type arguments of the function
    pub type_arguments: Vec<MoveType>,
    /// Arguments of the function
    pub arguments: Vec<serde_json::Value>,
}

/// An enum representing the different transaction signatures available
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum TransactionSignature {
    // Ed25519Signature(Ed25519Signature),
    // MultiEd25519Signature(MultiEd25519Signature),
    // MultiAgentSignature(MultiAgentSignature),
    // FeePayerSignature(FeePayerSignature),
    SingleSender(AccountSignature),
    // NoAccountSignature(NoAccountSignature),
}

/// Account signature scheme
///
/// The account signature scheme allows you to have two types of accounts:
///
///   1. A single Ed25519 key account, one private key
///   2. A k-of-n multi-Ed25519 key account, multiple private keys, such that k-of-n must sign a transaction.
///   3. A single Secp256k1Ecdsa key account, one private key
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum AccountSignature {
    // Ed25519Signature(Ed25519Signature),
    // MultiEd25519Signature(MultiEd25519Signature),
    // SingleKeySignature(SingleKeySignature),
    // MultiKeySignature(MultiKeySignature),
    // NoAccountSignature(NoAccountSignature),
    AbstractionSignature(AbstractionSignature),
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AbstractionSignature {
    pub function_info: String,
    pub auth_data: HexEncodedBytes,
}

/// A transaction waiting in mempool
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PendingTransaction {
    pub hash: HashValue,
    #[serde(flatten)]
    pub request: UserTransactionRequest,
}

// TODO: Remove this when we cut over.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserTransactionRequest {
    pub sender: Address,
    pub sequence_number: U64,
    pub max_gas_amount: U64,
    pub gas_unit_price: U64,
    pub expiration_timestamp_secs: U64,
    pub payload: TransactionPayload,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature: Option<TransactionSignature>,
}
