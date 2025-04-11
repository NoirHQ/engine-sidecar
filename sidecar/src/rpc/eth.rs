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
use alloy_eips::{BlockId, BlockNumberOrTag};
use alloy_network::Ethereum;
use alloy_rpc_types_eth::{state::StateOverride, TransactionRequest};
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

    async fn block_number(&self) -> RpcResult<alloy_primitives::U256> {
        tracing::debug!("block_number rpc request received");

        let ledger_info = self
            .adapter
            .get_ledger_info()
            .await
            .map_err(|e| internal_error(e.to_string()))?;

        Ok(alloy_primitives::U256::from(ledger_info.block_height.0))
    }

    async fn chain_id(&self) -> RpcResult<Option<alloy_primitives::U64>> {
        tracing::debug!("chain_id rpc request received");

        let ledger_info = self
            .adapter
            .get_ledger_info()
            .await
            .map_err(|e| internal_error(e.to_string()))?;

        Ok(Some(alloy_primitives::U64::from(ledger_info.chain_id)))
    }

    async fn block_by_hash(
        &self,
        hash: alloy_primitives::B256,
        full: bool,
    ) -> RpcResult<Option<RpcBlock<Ethereum>>> {
        tracing::debug!(
            "block_by_hash rpc request received: hash={}, full={}",
            hash,
            full
        );
        Ok(Some(RpcBlock::<Ethereum>::default()))
    }

    async fn block_by_number(
        &self,
        number: BlockNumberOrTag,
        full: bool,
    ) -> RpcResult<Option<RpcBlock<Ethereum>>> {
        tracing::debug!(
            "block_by_hash rpc request received: number={}, full={}",
            number,
            full
        );
        Ok(Some(RpcBlock::<Ethereum>::default()))
    }

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

    async fn estimate_gas(
        &self,
        request: TransactionRequest,
        block_number: Option<BlockId>,
        state_override: Option<StateOverride>,
    ) -> RpcResult<alloy_primitives::U256> {
        tracing::debug!("estimate_gas rpc request received: request={:?}, block_number={:?}, state_override={:?}", request, block_number, state_override);

        // TODO: simulate tx

        Ok(alloy_primitives::U256::default())
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
                "0x000000000000000000000000C96aAa54E2d44c299564da76e1cD3184A2386B8D"
            )
            .unwrap()
        );
    }
}
