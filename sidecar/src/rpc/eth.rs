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

use alloy_consensus::transaction::Recovered;
use alloy_eips::{BlockId, BlockNumberOrTag};
use alloy_network::Ethereum;
use alloy_primitives::{Address, Bytes, B256, U256, U64};
use alloy_rpc_types_eth::{state::StateOverride, TransactionRequest};
use jsonrpsee::core::RpcResult;
use reth_ethereum_primitives::TransactionSigned;
use reth_rpc_eth_api::{EthApiServer, RpcBlock};
use reth_rpc_eth_types::utils::recover_raw_transaction;

pub struct EthApi;

#[async_trait::async_trait]
impl EthApiServer<(), RpcBlock<Ethereum>, (), ()> for EthApi {
    async fn send_raw_transaction(&self, bytes: Bytes) -> RpcResult<B256> {
        tracing::debug!("send_raw_transaction rpc request received: bytes={}", bytes);

        let recovered: Recovered<TransactionSigned> = recover_raw_transaction(&bytes)?;
        let signer = recovered.signer();

        tracing::debug!("Signer: {:?}", signer);

        Ok(*recovered.hash())
    }

    fn block_number(&self) -> RpcResult<U256> {
        tracing::debug!("block_number rpc request received");
        Ok(U256::from(1))
    }

    async fn chain_id(&self) -> RpcResult<Option<U64>> {
        tracing::debug!("chain_id rpc request received");
        Ok(Some(U64::from_be_slice(&hex::decode("deadbeef").unwrap())))
    }

    async fn block_by_hash(&self, hash: B256, full: bool) -> RpcResult<Option<RpcBlock<Ethereum>>> {
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

    async fn balance(&self, address: Address, block_number: Option<BlockId>) -> RpcResult<U256> {
        tracing::debug!(
            "balance rpc request received: address={}, block_number={:?}",
            address,
            block_number
        );
        Ok(U256::from_str_radix("1000000000000000000", 10).unwrap())
    }

    async fn estimate_gas(
        &self,
        request: TransactionRequest,
        block_number: Option<BlockId>,
        state_override: Option<StateOverride>,
    ) -> RpcResult<U256> {
        tracing::debug!("estimate_gas rpc request received: request={:?}, block_number={:?}, state_override={:?}", request, block_number, state_override);
        Ok(U256::default())
    }
}
