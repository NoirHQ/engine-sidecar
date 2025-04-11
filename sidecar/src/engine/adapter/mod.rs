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

pub mod client;
pub mod local;
pub mod remote;

use anyhow::Result;

#[async_trait::async_trait]
pub trait EngineAdapter {
    fn coin_type(&self) -> &str;

    async fn get_ledger_info(&self) -> Result<aptos_api_types::IndexResponse>;

    async fn submit_transaction(
        &self,
        sender: move_core_types::account_address::AccountAddress,
        transaction: Vec<u8>,
    ) -> Result<aptos_api_types::PendingTransaction>;

    async fn get_block_by_height(
        &self,
        height: u64,
        with_transactions: bool,
    ) -> Result<aptos_api_types::Block, anyhow::Error>;

    async fn get_account(
        &self,
        address: move_core_types::account_address::AccountAddress,
    ) -> Result<aptos_rest_client::types::Account>;

    async fn get_account_balance(
        &self,
        address: move_core_types::account_address::AccountAddress,
    ) -> Result<u64, anyhow::Error>;
}
