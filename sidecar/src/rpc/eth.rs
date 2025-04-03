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

use std::default;

use alloy_consensus::transaction::Recovered;
use alloy_eips::BlockNumberOrTag;
use alloy_network::Ethereum;
use alloy_primitives::{Bytes, B256, U256, U64};
use jsonrpsee::core::RpcResult;
use reth_ethereum_primitives::TransactionSigned;
use reth_rpc_eth_api::{EthApiServer, RpcBlock};
use reth_rpc_eth_types::utils::recover_raw_transaction;

pub struct EthApi;

#[async_trait::async_trait]
impl EthApiServer<(), RpcBlock<Ethereum>, (), ()> for EthApi {
    async fn send_raw_transaction(&self, bytes: Bytes) -> RpcResult<B256> {
        tracing::debug!("Ethereum transaction: {}", hex::encode(&bytes));

        let recovered: Recovered<TransactionSigned> = recover_raw_transaction(&bytes)?;
        let signer = recovered.signer();

        tracing::debug!("Signer: {:?}", signer);

        Ok(*recovered.hash())
    }

    fn block_number(&self) -> RpcResult<U256> {
        Ok(U256::from(1))
    }

    async fn chain_id(&self) -> RpcResult<Option<U64>> {
        Ok(Some(U64::from_le_slice(&hex::decode("deadbeef").unwrap())))
    }

    async fn block_by_hash(&self, hash: B256, full: bool) -> RpcResult<Option<RpcBlock<Ethereum>>> {
        Ok(Some(RpcBlock::<Ethereum>::default()))
    }

    async fn block_by_number(
        &self,
        number: BlockNumberOrTag,
        full: bool,
    ) -> RpcResult<Option<RpcBlock<Ethereum>>> {
        Ok(Some(RpcBlock::<Ethereum>::default()))
    }
}
