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

pub mod adapter;

use adapter::EngineAdapter;
use anyhow::Result;

pub struct EngineClient {
    inner: Box<dyn EngineAdapter + Send + Sync>,
}

impl EngineClient {
    pub fn new(adapter: Box<dyn EngineAdapter + Send + Sync>) -> Self {
        EngineClient { inner: adapter }
    }
}

#[async_trait::async_trait]
impl EngineAdapter for EngineClient {
    fn coin_type(&self) -> &str {
        self.inner.coin_type()
    }

    fn auth_func(&self) -> &str {
        self.inner.auth_func()
    }

    fn entry_func(&self) -> &str {
        self.inner.entry_func()
    }

    async fn get_ledger_info(&self) -> Result<aptos_api_types::IndexResponse> {
        self.inner.get_ledger_info().await
    }

    async fn submit_transaction(
        &self,
        transaction: aptos_api_types::SubmitTransactionRequest,
    ) -> Result<aptos_api_types::PendingTransaction> {
        self.inner.submit_transaction(transaction).await
    }

    async fn get_block_by_height(
        &self,
        block_height: u64,
        with_transactions: bool,
    ) -> Result<aptos_api_types::Block> {
        self.inner
            .get_block_by_height(block_height, with_transactions)
            .await
    }

    async fn get_account(
        &self,
        address: aptos_api_types::Address,
    ) -> Result<aptos_api_types::AccountData> {
        self.inner.get_account(address).await
    }

    async fn get_account_balance(&self, address: aptos_api_types::Address) -> Result<u64> {
        self.inner.get_account_balance(address).await
    }
}
