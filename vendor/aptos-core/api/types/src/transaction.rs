// Copyright © Aptos Foundation
// Parts of the project are originally copyright © Meta Platforms, Inc.
// SPDX-License-Identifier: Apache-2.0

use crate::{
    address::Address,
    move_types::{
        EntryFunctionId, HexEncodedBytes, MoveFunction, MoveModule, MoveResource, MoveType, U64,
    },
    wrappers::EventGuid,
    HashValue, MoveModuleId, MoveStructTag,
};
use aptos_types::jwks::jwk::JWK;
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

/// Enum of the different types of transactions in Aptos
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
// #[oai(one_of, discriminator_name = "type", rename_all = "snake_case")]
pub enum Transaction {
    PendingTransaction(PendingTransaction),
    UserTransaction(UserTransaction),
    GenesisTransaction(GenesisTransaction),
    BlockMetadataTransaction(BlockMetadataTransaction),
    StateCheckpointTransaction(StateCheckpointTransaction),
    BlockEpilogueTransaction(BlockEpilogueTransaction),
    ValidatorTransaction(ValidatorTransaction),
}

/// A more API-friendly representation of the on-chain `aptos_types::jwks::ProviderJWKs`.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExportedProviderJWKs {
    pub issuer: String,
    pub version: u64,
    pub jwks: Vec<JWK>,
}

/// A more API-friendly representation of the on-chain `aptos_types::jwks::QuorumCertifiedUpdate`.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExportedQuorumCertifiedUpdate {
    pub update: ExportedProviderJWKs,
    pub multi_sig: ExportedAggregateSignature,
}

/// A more API-friendly representation of the on-chain `aptos_types::aggregate_signature::AggregateSignature`.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExportedAggregateSignature {
    pub signer_indices: Vec<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sig: Option<HexEncodedBytes>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct JWKUpdateTransaction {
    #[serde(flatten)]
    // #[oai(flatten)]
    pub info: TransactionInfo,
    pub events: Vec<Event>,
    pub timestamp: U64,
    pub quorum_certified_update: ExportedQuorumCertifiedUpdate,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "validator_transaction_type", rename_all = "snake_case")]
// #[oai(
//     one_of,
//     discriminator_name = "validator_transaction_type",
//     rename_all = "snake_case"
// )]
pub enum ValidatorTransaction {
    ObservedJwkUpdate(JWKUpdateTransaction),
    DkgResult(DKGResultTransaction),
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct DKGResultTransaction {
    #[serde(flatten)]
    // #[oai(flatten)]
    pub info: TransactionInfo,
    pub events: Vec<Event>,
    pub timestamp: U64,
    pub dkg_transcript: ExportedDKGTranscript,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExportedDKGTranscript {
    pub epoch: U64,
    pub author: Address,
    pub payload: HexEncodedBytes,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct BlockEndInfo {
    pub block_gas_limit_reached: bool,
    pub block_output_limit_reached: bool,
    pub block_effective_block_gas_units: u64,
    pub block_approx_output_size: u64,
}

/// A block epilogue transaction
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct BlockEpilogueTransaction {
    #[serde(flatten)]
    // #[oai(flatten)]
    pub info: TransactionInfo,
    pub timestamp: U64,
    pub block_end_info: Option<BlockEndInfo>,
}

/// A state checkpoint transaction
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct StateCheckpointTransaction {
    #[serde(flatten)]
    // #[oai(flatten)]
    pub info: TransactionInfo,
    pub timestamp: U64,
}

/// A block metadata transaction
///
/// This signifies the beginning of a block, and contains information
/// about the specific block
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct BlockMetadataTransaction {
    #[serde(flatten)]
    // #[oai(flatten)]
    pub info: TransactionInfo,
    pub id: HashValue,
    pub epoch: U64,
    pub round: U64,
    /// The events emitted at the block creation
    pub events: Vec<Event>,
    /// Previous block votes
    pub previous_block_votes_bitvec: Vec<u8>,
    pub proposer: Address,
    /// The indices of the proposers who failed to propose
    pub failed_proposer_indices: Vec<u32>,
    pub timestamp: U64,

    /// If some, it means the internal txn type is `aptos_types::transaction::Transaction::BlockMetadataExt`.
    /// Otherwise, it is `aptos_types::transaction::Transaction::BlockMetadata`.
    ///
    /// NOTE: we could have introduced a new APT txn type to represent the corresponding internal type,
    /// but that is a breaking change to the ecosystem.
    ///
    /// NOTE: `oai` does not support `flatten` together with `skip_serializing_if`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    // #[oai(default, skip_serializing_if = "Option::is_none")]
    pub block_metadata_extension: Option<BlockMetadataExtension>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
// #[oai(one_of, discriminator_name = "type", rename_all = "snake_case")]
pub enum BlockMetadataExtension {
    V0(BlockMetadataExtensionEmpty),
    V1(BlockMetadataExtensionRandomness),
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct BlockMetadataExtensionEmpty {}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct BlockMetadataExtensionRandomness {
    randomness: Option<HexEncodedBytes>,
}

/// The genesis transaction
///
/// This only occurs at the genesis transaction (version 0)
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct GenesisTransaction {
    #[serde(flatten)]
    // #[oai(flatten)]
    pub info: TransactionInfo,
    pub payload: GenesisPayload,
    /// Events emitted during genesis
    pub events: Vec<Event>,
}

/// An event from a transaction
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Event {
    // The globally unique identifier of this event stream.
    pub guid: EventGuid,
    // The sequence number of the event
    pub sequence_number: U64,
    #[serde(rename = "type")]
    // #[oai(rename = "type")]
    pub typ: MoveType,
    /// The JSON representation of the event
    pub data: serde_json::Value,
}

/// The writeset payload of the Genesis transaction
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
// #[oai(one_of, discriminator_name = "type", rename_all = "snake_case")]
pub enum GenesisPayload {
    WriteSetPayload(WriteSetPayload),
}

/// A writeset payload, used only for genesis
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct WriteSetPayload {
    pub write_set: WriteSet,
}

/// The associated writeset with a payload
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
// #[oai(one_of, discriminator_name = "type", rename_all = "snake_case")]
pub enum WriteSet {
    ScriptWriteSet(ScriptWriteSet),
    DirectWriteSet(DirectWriteSet),
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct DirectWriteSet {
    pub changes: Vec<WriteSetChange>,
    pub events: Vec<Event>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScriptWriteSet {
    pub execute_as: Address,
    pub script: ScriptPayload,
}

/// Payload which runs a script that can run multiple functions
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScriptPayload {
    pub code: MoveScriptBytecode,
    /// Type arguments of the function
    pub type_arguments: Vec<MoveType>,
    /// Arguments of the function
    pub arguments: Vec<serde_json::Value>,
}

/// Move script bytecode
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct MoveScriptBytecode {
    pub bytecode: HexEncodedBytes,
    // We don't need deserialize MoveModule as it should be serialized
    // from `bytecode`.
    #[serde(skip_deserializing)]
    pub abi: Option<MoveFunction>,
}

/// A transaction submitted by a user to change the state of the blockchain
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserTransaction {
    #[serde(flatten)]
    // #[oai(flatten)]
    pub info: TransactionInfo,
    #[serde(flatten)]
    // #[oai(flatten)]
    pub request: UserTransactionRequest,
    /// Events generated by the transaction
    pub events: Vec<Event>,
    pub timestamp: U64,
}

/// Information related to how a transaction affected the state of the blockchain
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TransactionInfo {
    pub version: U64,
    pub hash: HashValue,
    pub state_change_hash: HashValue,
    pub event_root_hash: HashValue,
    pub state_checkpoint_hash: Option<HashValue>,
    pub gas_used: U64,
    /// Whether the transaction was successful
    pub success: bool,
    /// The VM status of the transaction, can tell useful information in a failure
    pub vm_status: String,
    pub accumulator_root_hash: HashValue,
    /// Final state of resources changed by the transaction
    pub changes: Vec<WriteSetChange>,
    /// Block height that the transaction belongs in, this field will not be present through the API
    // #[oai(skip)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_height: Option<U64>,
    /// Epoch of the transaction belongs in, this field will not be present through the API
    // #[oai(skip)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub epoch: Option<U64>,
}

/// A final state change of a transaction on a resource or module
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
// #[oai(one_of, discriminator_name = "type", rename_all = "snake_case")]
pub enum WriteSetChange {
    DeleteModule(DeleteModule),
    DeleteResource(DeleteResource),
    DeleteTableItem(DeleteTableItem),
    WriteModule(WriteModule),
    WriteResource(WriteResource),
    WriteTableItem(WriteTableItem),
}

/// Change set to write a table item
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct WriteTableItem {
    pub state_key_hash: String,
    pub handle: HexEncodedBytes,
    pub key: HexEncodedBytes,
    pub value: HexEncodedBytes,
    // This is optional, and only possible to populate if the table indexer is enabled for this node
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub data: Option<DecodedTableData>,
}

/// Decoded table data
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct DecodedTableData {
    /// Key of table in JSON
    pub key: serde_json::Value,
    /// Type of key
    pub key_type: String,
    /// Value of table in JSON
    pub value: serde_json::Value,
    /// Type of value
    pub value_type: String,
}

/// Write a resource or update an existing one
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct WriteResource {
    pub address: Address,
    /// State key hash
    pub state_key_hash: String,
    pub data: MoveResource,
}

/// Delete a module
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeleteModule {
    pub address: Address,
    /// State key hash
    pub state_key_hash: String,
    pub module: MoveModuleId,
}

/// Delete a resource
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeleteResource {
    pub address: Address,
    /// State key hash
    pub state_key_hash: String,
    pub resource: MoveStructTag,
}

/// Delete a table item
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeleteTableItem {
    pub state_key_hash: String,
    pub handle: HexEncodedBytes,
    pub key: HexEncodedBytes,
    // This is optional, and only possible to populate if the table indexer is enabled for this node
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub data: Option<DeletedTableData>,
}

/// Deleted table data
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeletedTableData {
    /// Deleted key
    pub key: serde_json::Value,
    /// Deleted key type
    pub key_type: String,
}

/// Write a new module or update an existing one
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct WriteModule {
    pub address: Address,
    /// State key hash
    pub state_key_hash: String,
    pub data: MoveModuleBytecode,
}

/// Move module bytecode along with it's ABI
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct MoveModuleBytecode {
    pub bytecode: HexEncodedBytes,
    // We don't need deserialize MoveModule as it should be serialized
    // from `bytecode`.
    #[serde(skip_deserializing)]
    pub abi: Option<MoveModule>,
}
