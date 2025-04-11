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

use super::{client::AAClient, EngineAdapter};
use crate::config::engine::RemoteEngineConfig;
use anyhow::{anyhow, Ok, Result};
use aptos_global_constants::{GAS_UNIT_PRICE, MAX_GAS_AMOUNT};
use aptos_rest_client::{types::Account, Client};
use reqwest::{Response, StatusCode, Url};
use serde::de::DeserializeOwned;
use std::{borrow::Cow, marker::PhantomData};

#[derive(Debug, Clone)]
pub struct RemoteEngineAdapter {
    coin_type: Cow<'static, str>,
    client: AAClient,
}

impl RemoteEngineAdapter {
    pub fn new(
        coin_type: String,
        auth_func: String,
        entry_func: String,
        config: RemoteEngineConfig,
    ) -> Self {
        let node_url = Url::parse(config.endpoint()).expect("Failed parse adapter url");
        let client = AAClient::new(
            Client::new(node_url),
            auth_func,
            entry_func,
            config.chain_id(),
            config.timeout(),
        );

        Self {
            coin_type: Cow::Owned(coin_type),
            client,
        }
    }
}

#[async_trait::async_trait]
impl EngineAdapter for RemoteEngineAdapter {
    fn coin_type(&self) -> &str {
        &self.coin_type
    }

    async fn get_ledger_info(&self) -> Result<aptos_api_types::IndexResponse> {
        Ok(self.client.api_client.get_index().await?.into_inner())
    }

    async fn submit_transaction(
        &self,
        sender: move_core_types::account_address::AccountAddress,
        tx: Vec<u8>,
    ) -> Result<aptos_api_types::PendingTransaction> {
        let account = self.get_account(sender).await?;

        self.client
            .submit_transaction(
                sender,
                tx,
                account.sequence_number,
                MAX_GAS_AMOUNT,
                GAS_UNIT_PRICE,
            )
            .await
    }

    async fn get_block_by_height(
        &self,
        height: u64,
        with_transactions: bool,
    ) -> Result<aptos_api_types::Block> {
        Ok(self
            .client
            .api_client
            .get_block_by_height(height, with_transactions)
            .await?
            .into_inner())
    }

    async fn get_account(
        &self,
        address: move_core_types::account_address::AccountAddress,
    ) -> Result<Account> {
        Ok(self
            .client
            .api_client
            .get_account(address)
            .await?
            .into_inner())
    }

    async fn get_account_balance(
        &self,
        address: move_core_types::account_address::AccountAddress,
    ) -> Result<u64> {
        Ok(self
            .client
            .api_client
            .get_account_balance(address, &self.coin_type)
            .await?
            .into_inner())
    }
}

pub struct ResponseHandler<R> {
    _marker: PhantomData<R>,
    error: &'static str,
}

impl<R> ResponseHandler<R>
where
    R: DeserializeOwned,
{
    pub fn new(error: &'static str) -> Self {
        Self {
            _marker: Default::default(),
            error,
        }
    }

    pub async fn handle(&self, response: Response) -> Result<R> {
        if response.status().is_success() {
            let result = response.json::<R>().await?;
            Ok(result)
        } else {
            Err(Self::handle_error(
                self.error,
                response.status(),
                response.text().await?,
            ))
        }
    }

    fn handle_error(message: &'static str, status: StatusCode, error: String) -> anyhow::Error {
        tracing::warn!("{}: status={}, message={}", message, status, error);
        anyhow!(message)
    }
}
