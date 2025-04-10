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
use crate::config::engine::RemoteEngineConfig;
use anyhow::{anyhow, Ok, Result};
use reqwest::{Response, StatusCode, Url};
use serde::de::DeserializeOwned;
use std::{borrow::Cow, marker::PhantomData};

#[derive(Debug, Clone)]
pub struct RemoteEngineAdapter {
    coin_type: Cow<'static, str>,
    endpoint: Url,
    client: reqwest::Client,
    auth_func: Cow<'static, str>,
    entry_func: Cow<'static, str>,
}

impl RemoteEngineAdapter {
    pub fn new(
        coin_type: String,
        auth_func: String,
        entry_func: String,
        config: RemoteEngineConfig,
    ) -> Self {
        let client = reqwest::ClientBuilder::new()
            .timeout(config.timeout())
            .build()
            .expect("Failed to build reqwest client");

        Self {
            coin_type: Cow::Owned(coin_type),
            endpoint: Url::parse(config.endpoint()).expect("Failed parse adapter url"),
            client,
            auth_func: Cow::Owned(auth_func),
            entry_func: Cow::Owned(entry_func),
        }
    }
}

#[async_trait::async_trait]
impl EngineAdapter for RemoteEngineAdapter {
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
        let response = self.client.get(self.endpoint.clone()).send().await?;

        ResponseHandler::<aptos_api_types::IndexResponse>::new("Failed to get ledger info")
            .handle(response)
            .await
    }

    async fn submit_transaction(
        &self,
        transaction: aptos_api_types::SubmitTransactionRequest,
    ) -> Result<aptos_api_types::PendingTransaction> {
        let response = self
            .client
            .post(format!("{}/transactions", self.endpoint))
            .json(&transaction)
            .send()
            .await?;

        ResponseHandler::<aptos_api_types::PendingTransaction>::new("Failed to submit transaction")
            .handle(response)
            .await
    }

    async fn get_block_by_height(
        &self,
        block_height: u64,
        _with_transactions: bool,
    ) -> Result<aptos_api_types::Block> {
        let response = self
            .client
            .get(format!(
                "{}/blocks/by_height/{}",
                self.endpoint, block_height
            ))
            .send()
            .await?;

        ResponseHandler::<aptos_api_types::Block>::new("Failed to get block by height")
            .handle(response)
            .await
    }

    async fn get_account(
        &self,
        address: aptos_api_types::Address,
    ) -> Result<aptos_api_types::AccountData> {
        let response = self
            .client
            .get(format!("{}/accounts/{}", self.endpoint, address))
            .send()
            .await?;

        ResponseHandler::<aptos_api_types::AccountData>::new("Failed to get account data")
            .handle(response)
            .await
    }

    async fn get_account_balance(&self, address: aptos_api_types::Address) -> Result<u64> {
        let response = self
            .client
            .get(format!(
                "{}/accounts/{}/balance/{}",
                self.endpoint,
                address,
                self.coin_type(),
            ))
            .send()
            .await?;

        ResponseHandler::<u64>::new("Failed to get balance")
            .handle(response)
            .await
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
