// This file is part of Noir.

// Copyright (c) Haderech Pte. Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use crate::engine::adapter::EngineAdapter;
use alloy_consensus::transaction::Recovered;
use alloy_dyn_abi::TypedData;
use alloy_eips::{BlockId, BlockNumberOrTag};
use alloy_network::Ethereum;
use alloy_rpc_types_eth::{
    simulate::{SimulatePayload, SimulatedBlock},
    state::StateOverride,
    AccessListResult, BlockOverrides, Bundle, EIP1186AccountProofResponse, EthCallResponse,
    FeeHistory, Index, StateContext, SyncStatus, TransactionRequest, Work,
};
use alloy_serde::JsonStorageKey;
use jsonrpsee::{
    core::RpcResult,
    types::{error::INTERNAL_ERROR_CODE, ErrorObjectOwned},
};
use reth_ethereum_primitives::TransactionSigned;
use reth_rpc_eth_api::{EthApiServer, RpcBlock};
use reth_rpc_eth_types::utils::recover_raw_transaction;

pub struct EthApi<Adapter> {
    adapter: Adapter,
}

impl<Adapter> EthApi<Adapter>
where
    Adapter: EngineAdapter + Send + Sync + 'static,
{
    pub fn new(adapter: Adapter) -> Self {
        Self { adapter }
    }
}

#[async_trait::async_trait]
impl<Adapter> EthApiServer<(), RpcBlock<Ethereum>, (), ()> for EthApi<Adapter>
where
    Adapter: EngineAdapter + Send + Sync + 'static,
{
    /// Returns the protocol version encoded as a string.
    async fn protocol_version(&self) -> RpcResult<alloy_primitives::U64> {
        unimplemented!();
    }

    /// Returns an object with data about the sync status or false.
    fn syncing(&self) -> RpcResult<SyncStatus> {
        unimplemented!();
    }

    /// Returns the client coinbase address.
    async fn author(&self) -> RpcResult<alloy_primitives::Address> {
        unimplemented!();
    }

    /// Returns a list of addresses owned by client.
    fn accounts(&self) -> RpcResult<Vec<alloy_primitives::Address>> {
        unimplemented!();
    }

    /// Returns the number of most recent block.
    fn block_number(&self) -> RpcResult<alloy_primitives::U256> {
        unimplemented!();
    }

    /// Returns the chain ID of the current network.
    async fn chain_id(&self) -> RpcResult<Option<alloy_primitives::U64>> {
        tracing::debug!("chain_id rpc request received");

        let ledger_info = self
            .adapter
            .get_ledger_info()
            .await
            .map_err(|e| internal_error(e.to_string()))?;

        Ok(Some(alloy_primitives::U64::from(ledger_info.chain_id)))
    }

    /// Returns information about a block by hash.
    async fn block_by_hash(
        &self,
        hash: alloy_primitives::B256,
        full: bool,
    ) -> RpcResult<Option<RpcBlock<Ethereum>>> {
        unimplemented!();
    }

    /// Returns information about a block by number.
    async fn block_by_number(
        &self,
        number: BlockNumberOrTag,
        full: bool,
    ) -> RpcResult<Option<RpcBlock<Ethereum>>> {
        unimplemented!();
    }

    /// Returns the number of transactions in a block from a block matching the given block hash.
    async fn block_transaction_count_by_hash(
        &self,
        hash: alloy_primitives::B256,
    ) -> RpcResult<Option<alloy_primitives::U256>> {
        unimplemented!();
    }

    /// Returns the number of transactions in a block matching the given block number.
    async fn block_transaction_count_by_number(
        &self,
        number: BlockNumberOrTag,
    ) -> RpcResult<Option<alloy_primitives::U256>> {
        unimplemented!();
    }

    /// Returns the number of uncles in a block from a block matching the given block hash.
    async fn block_uncles_count_by_hash(
        &self,
        hash: alloy_primitives::B256,
    ) -> RpcResult<Option<alloy_primitives::U256>> {
        unimplemented!();
    }

    /// Returns the number of uncles in a block with given block number.
    async fn block_uncles_count_by_number(
        &self,
        number: BlockNumberOrTag,
    ) -> RpcResult<Option<alloy_primitives::U256>> {
        unimplemented!();
    }

    /// Returns all transaction receipts for a given block.
    async fn block_receipts(&self, block_id: BlockId) -> RpcResult<Option<Vec<()>>> {
        unimplemented!();
    }

    /// Returns an uncle block of the given block and index.
    async fn uncle_by_block_hash_and_index(
        &self,
        hash: alloy_primitives::B256,
        index: Index,
    ) -> RpcResult<Option<RpcBlock<Ethereum>>> {
        unimplemented!();
    }

    /// Returns an uncle block of the given block and index.
    async fn uncle_by_block_number_and_index(
        &self,
        number: BlockNumberOrTag,
        index: Index,
    ) -> RpcResult<Option<RpcBlock<Ethereum>>> {
        unimplemented!();
    }

    /// Returns the EIP-2718 encoded transaction if it exists.
    ///
    /// If this is a EIP-4844 transaction that is in the pool it will include the sidecar.
    async fn raw_transaction_by_hash(
        &self,
        hash: alloy_primitives::B256,
    ) -> RpcResult<Option<alloy_primitives::Bytes>> {
        unimplemented!();
    }

    /// Returns the information about a transaction requested by transaction hash.
    async fn transaction_by_hash(&self, hash: alloy_primitives::B256) -> RpcResult<Option<()>> {
        unimplemented!();
    }

    /// Returns information about a raw transaction by block hash and transaction index position.
    async fn raw_transaction_by_block_hash_and_index(
        &self,
        hash: alloy_primitives::B256,
        index: Index,
    ) -> RpcResult<Option<alloy_primitives::Bytes>> {
        unimplemented!();
    }

    /// Returns information about a transaction by block hash and transaction index position.
    async fn transaction_by_block_hash_and_index(
        &self,
        hash: alloy_primitives::B256,
        index: Index,
    ) -> RpcResult<Option<()>> {
        unimplemented!();
    }

    /// Returns information about a raw transaction by block number and transaction index
    /// position.
    async fn raw_transaction_by_block_number_and_index(
        &self,
        number: BlockNumberOrTag,
        index: Index,
    ) -> RpcResult<Option<alloy_primitives::Bytes>> {
        unimplemented!();
    }

    /// Returns information about a transaction by block number and transaction index position.
    async fn transaction_by_block_number_and_index(
        &self,
        number: BlockNumberOrTag,
        index: Index,
    ) -> RpcResult<Option<()>> {
        unimplemented!();
    }

    /// Returns information about a transaction by sender and nonce.
    async fn transaction_by_sender_and_nonce(
        &self,
        address: alloy_primitives::Address,
        nonce: alloy_primitives::U64,
    ) -> RpcResult<Option<()>> {
        unimplemented!();
    }

    /// Returns the receipt of a transaction by transaction hash.
    async fn transaction_receipt(&self, hash: alloy_primitives::B256) -> RpcResult<Option<()>> {
        unimplemented!();
    }

    /// Returns the balance of the account of given address.
    async fn balance(
        &self,
        address: alloy_primitives::Address,
        block_number: Option<BlockId>,
    ) -> RpcResult<alloy_primitives::U256> {
        tracing::debug!(
            "balance rpc request received: address={}, block_number={:?}",
            address,
            block_number
        );

        let aptos_address = to_aptos_address(&address);
        let balance = self
            .adapter
            .get_account_balance(aptos_address)
            .await
            .map_err(|e| internal_error(e.to_string()))?;

        Ok(alloy_primitives::U256::from(balance))
    }

    /// Returns the value from a storage position at a given address
    async fn storage_at(
        &self,
        address: alloy_primitives::Address,
        index: JsonStorageKey,
        block_number: Option<BlockId>,
    ) -> RpcResult<alloy_primitives::B256> {
        unimplemented!();
    }

    /// Returns the number of transactions sent from an address at given block number.
    async fn transaction_count(
        &self,
        address: alloy_primitives::Address,
        block_number: Option<BlockId>,
    ) -> RpcResult<alloy_primitives::U256> {
        unimplemented!();
    }

    /// Returns code at a given address at given block number.
    async fn get_code(
        &self,
        address: alloy_primitives::Address,
        block_number: Option<BlockId>,
    ) -> RpcResult<alloy_primitives::Bytes> {
        unimplemented!();
    }

    /// Returns the block's header at given number.
    async fn header_by_number(&self, hash: BlockNumberOrTag) -> RpcResult<Option<()>> {
        unimplemented!();
    }

    /// Returns the block's header at given hash.
    async fn header_by_hash(&self, hash: alloy_primitives::B256) -> RpcResult<Option<()>> {
        unimplemented!();
    }

    /// `eth_simulateV1` executes an arbitrary number of transactions on top of the requested state.
    /// The transactions are packed into individual blocks. Overrides can be provided.
    async fn simulate_v1(
        &self,
        opts: SimulatePayload,
        block_number: Option<BlockId>,
    ) -> RpcResult<Vec<SimulatedBlock<RpcBlock<Ethereum>>>> {
        unimplemented!();
    }

    /// Executes a new message call immediately without creating a transaction on the block chain.
    async fn call(
        &self,
        request: TransactionRequest,
        block_number: Option<BlockId>,
        state_overrides: Option<StateOverride>,
        block_overrides: Option<Box<BlockOverrides>>,
    ) -> RpcResult<alloy_primitives::Bytes> {
        unimplemented!();
    }

    /// Simulate arbitrary number of transactions at an arbitrary blockchain index, with the
    /// optionality of state overrides
    async fn call_many(
        &self,
        bundle: Bundle,
        state_context: Option<StateContext>,
        state_override: Option<StateOverride>,
    ) -> RpcResult<Vec<EthCallResponse>> {
        unimplemented!();
    }

    /// Generates an access list for a transaction.
    ///
    /// This method creates an [EIP2930](https://eips.ethereum.org/EIPS/eip-2930) type accessList based on a given Transaction.
    ///
    /// An access list contains all storage slots and addresses touched by the transaction, except
    /// for the sender account and the chain's precompiles.
    ///
    /// It returns list of addresses and storage keys used by the transaction, plus the gas
    /// consumed when the access list is added. That is, it gives you the list of addresses and
    /// storage keys that will be used by that transaction, plus the gas consumed if the access
    /// list is included. Like eth_estimateGas, this is an estimation; the list could change
    /// when the transaction is actually mined. Adding an accessList to your transaction does
    /// not necessary result in lower gas usage compared to a transaction without an access
    /// list.
    async fn create_access_list(
        &self,
        request: TransactionRequest,
        block_number: Option<BlockId>,
    ) -> RpcResult<AccessListResult> {
        unimplemented!();
    }

    /// Generates and returns an estimate of how much gas is necessary to allow the transaction to
    /// complete.
    async fn estimate_gas(
        &self,
        request: TransactionRequest,
        block_number: Option<BlockId>,
        state_override: Option<StateOverride>,
    ) -> RpcResult<alloy_primitives::U256> {
        unimplemented!();
    }

    /// Returns the current price per gas in wei.
    async fn gas_price(&self) -> RpcResult<alloy_primitives::U256> {
        unimplemented!();
    }

    /// Returns the account details by specifying an address and a block number/tag
    async fn get_account(
        &self,
        address: alloy_primitives::Address,
        block: BlockId,
    ) -> RpcResult<Option<alloy_rpc_types_eth::Account>> {
        unimplemented!();
    }

    /// Introduced in EIP-1559, returns suggestion for the priority for dynamic fee transactions.
    async fn max_priority_fee_per_gas(&self) -> RpcResult<alloy_primitives::U256> {
        unimplemented!();
    }

    /// Introduced in EIP-4844, returns the current blob base fee in wei.
    async fn blob_base_fee(&self) -> RpcResult<alloy_primitives::U256> {
        unimplemented!();
    }

    /// Returns the Transaction fee history
    ///
    /// Introduced in EIP-1559 for getting information on the appropriate priority fee to use.
    ///
    /// Returns transaction base fee per gas and effective priority fee per gas for the
    /// requested/supported block range. The returned Fee history for the returned block range
    /// can be a subsection of the requested range if not all blocks are available.
    async fn fee_history(
        &self,
        block_count: alloy_primitives::U64,
        newest_block: BlockNumberOrTag,
        reward_percentiles: Option<Vec<f64>>,
    ) -> RpcResult<FeeHistory> {
        unimplemented!();
    }

    /// Returns whether the client is actively mining new blocks.
    async fn is_mining(&self) -> RpcResult<bool> {
        unimplemented!();
    }

    /// Returns the number of hashes per second that the node is mining with.
    async fn hashrate(&self) -> RpcResult<alloy_primitives::U256> {
        unimplemented!();
    }

    /// Returns the hash of the current block, the seedHash, and the boundary condition to be met
    /// (“target”)
    async fn get_work(&self) -> RpcResult<Work> {
        unimplemented!();
    }

    /// Used for submitting mining hashrate.
    ///
    /// Can be used for remote miners to submit their hash rate.
    /// It accepts the miner hash rate and an identifier which must be unique between nodes.
    /// Returns `true` if the block was successfully submitted, `false` otherwise.
    async fn submit_hashrate(
        &self,
        hashrate: alloy_primitives::U256,
        id: alloy_primitives::B256,
    ) -> RpcResult<bool> {
        unimplemented!();
    }

    /// Used for submitting a proof-of-work solution.
    async fn submit_work(
        &self,
        nonce: alloy_primitives::B64,
        pow_hash: alloy_primitives::B256,
        mix_digest: alloy_primitives::B256,
    ) -> RpcResult<bool> {
        unimplemented!();
    }

    /// Sends transaction; will block waiting for signer to return the
    /// transaction hash.
    async fn send_transaction(
        &self,
        request: TransactionRequest,
    ) -> RpcResult<alloy_primitives::B256> {
        unimplemented!();
    }

    /// Sends signed transaction, returning its hash.
    async fn send_raw_transaction(
        &self,
        bytes: alloy_primitives::Bytes,
    ) -> RpcResult<alloy_primitives::B256> {
        tracing::debug!("send_raw_transaction rpc request received: bytes={}", bytes);

        let recovered: Recovered<TransactionSigned> = recover_raw_transaction(&bytes)?;
        let signer = recovered.signer();

        let sender = to_aptos_address(&signer);
        let pending = self
            .adapter
            .submit_transaction(sender, bytes.0.to_vec())
            .await
            .map_err(|e| internal_error(e.to_string()))?;

        tracing::debug!("Submitted transaction: {:?}", pending);

        Ok(*recovered.hash())
    }

    /// Returns an Ethereum specific signature with: sign(keccak256("\x19Ethereum Signed Message:\n"
    /// + len(message) + message))).
    async fn sign(
        &self,
        address: alloy_primitives::Address,
        message: alloy_primitives::Bytes,
    ) -> RpcResult<alloy_primitives::Bytes> {
        unimplemented!();
    }

    /// Signs a transaction that can be submitted to the network at a later time using with
    /// `sendRawTransaction.`
    async fn sign_transaction(
        &self,
        transaction: TransactionRequest,
    ) -> RpcResult<alloy_primitives::Bytes> {
        unimplemented!();
    }

    /// Signs data via [EIP-712](https://github.com/ethereum/EIPs/blob/master/EIPS/eip-712.md).
    async fn sign_typed_data(
        &self,
        address: alloy_primitives::Address,
        data: TypedData,
    ) -> RpcResult<alloy_primitives::Bytes> {
        unimplemented!();
    }

    /// Returns the account and storage values of the specified account including the Merkle-proof.
    /// This call can be used to verify that the data you are pulling from is not tampered with.
    async fn get_proof(
        &self,
        address: alloy_primitives::Address,
        keys: Vec<JsonStorageKey>,
        block_number: Option<BlockId>,
    ) -> RpcResult<EIP1186AccountProofResponse> {
        unimplemented!();
    }
}

pub fn to_aptos_address(
    address: &alloy_primitives::Address,
) -> move_core_types::account_address::AccountAddress {
    let mut bytes: [u8; 32] = [0u8; 32];
    bytes[12..].copy_from_slice(address.0.as_slice());

    move_core_types::account_address::AccountAddress::new(bytes)
}

pub fn internal_error(message: impl Into<String>) -> ErrorObjectOwned {
    ErrorObjectOwned::owned(INTERNAL_ERROR_CODE, message, None::<()>)
}

#[cfg(test)]
pub mod tests {
    use super::to_aptos_address;
    use alloy_primitives::hex::FromHex;

    #[test]
    fn to_bytes32_test() {
        let eth_address =
            alloy_primitives::Address::from_hex("0xC96aAa54E2d44c299564da76e1cD3184A2386B8D")
                .unwrap();
        let aptos_address = to_aptos_address(&eth_address);

        assert_eq!(
            aptos_address,
            move_core_types::account_address::AccountAddress::from_hex(
                "000000000000000000000000C96aAa54E2d44c299564da76e1cD3184A2386B8D"
            )
            .unwrap()
        );
    }
}
