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

use super::EngineAdapter;
use anyhow::Result;
use std::borrow::Cow;

#[derive(Debug, Clone)]
pub struct LocalEngineAdapter {
    coin_type: Cow<'static, str>,
    auth_func: Cow<'static, str>,
    entry_func: Cow<'static, str>,
}

impl LocalEngineAdapter {
    pub fn new(coin_type: String, auth_func: String, entry_func: String) -> Self {
        Self {
            coin_type: Cow::Owned(coin_type),
            auth_func: Cow::Owned(auth_func),
            entry_func: Cow::Owned(entry_func),
        }
    }
}

#[async_trait::async_trait]
impl EngineAdapter for LocalEngineAdapter {
    fn coin_type(&self) -> &str {
        &self.coin_type
    }

    fn auth_func(&self) -> &str {
        &self.auth_func
    }

    fn entry_func(&self) -> &str {
        &self.entry_func
    }

    async fn get_ledger_info(&self) -> Result<aptos_api_types::IndexResponse> {
        unimplemented!();
    }

    async fn submit_transaction(
        &self,
        _transaction: aptos_api_types::SubmitTransactionRequest,
    ) -> Result<aptos_api_types::PendingTransaction> {
        unimplemented!();
    }

    async fn get_block_by_height(
        &self,
        _block_height: u64,
        _with_transactions: bool,
    ) -> Result<aptos_api_types::Block> {
        unimplemented!();
    }

    async fn get_account(
        &self,
        _address: aptos_api_types::Address,
    ) -> Result<aptos_api_types::AccountData> {
        unimplemented!();
    }

    async fn get_account_balance(&self, _address: aptos_api_types::Address) -> Result<u64> {
        unimplemented!();
    }
}
