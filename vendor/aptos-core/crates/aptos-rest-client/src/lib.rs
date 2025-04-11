// Copyright © Aptos Foundation
// Parts of the project are originally copyright © Meta Platforms, Inc.
// SPDX-License-Identifier: Apache-2.0

pub mod aptos;
pub mod client_builder;
pub mod error;
pub mod response;
pub mod state;
pub mod types;

use anyhow::anyhow;
use aptos_api_types::{
    mime_types::BCS_SIGNED_TRANSACTION, AptosError, Block, IndexResponse, PendingTransaction,
};
use aptos_types::transaction::SignedTransaction;
use client_builder::{AptosBaseUrl, ClientBuilder};
use error::RestError;
use move_core_types::account_address::AccountAddress;
use reqwest::{header::CONTENT_TYPE, Client as ReqwestClient};
use response::Response;
use serde::de::DeserializeOwned;
pub use state::State;
use types::Account;
use url::Url;

pub const DEFAULT_VERSION_PATH_BASE: &str = "v1/";
const X_APTOS_SDK_HEADER_VALUE: &str = concat!("aptos-rust-sdk/", env!("CARGO_PKG_VERSION"));

type AptosResult<T> = Result<T, RestError>;

#[derive(Clone, Debug)]
pub struct Client {
    inner: ReqwestClient,
    base_url: Url,
    version_path_base: String,
}

impl Client {
    pub fn builder(aptos_base_url: AptosBaseUrl) -> ClientBuilder {
        ClientBuilder::new(aptos_base_url)
    }

    pub fn new(base_url: Url) -> Self {
        Self::builder(AptosBaseUrl::Custom(base_url)).build()
    }

    pub fn path_prefix_string(&self) -> String {
        self.base_url
            .join(&self.version_path_base)
            .map(|path| path.to_string())
            .unwrap_or_else(|_| "<bad_base_url>".to_string())
    }

    /// Set a different version path base, e.g. "v1/" See
    /// DEFAULT_VERSION_PATH_BASE for the default value.
    pub fn version_path_base(mut self, version_path_base: String) -> AptosResult<Self> {
        if !version_path_base.ends_with('/') {
            return Err(anyhow!("version_path_base must end with '/', e.g. 'v1/'").into());
        }
        self.version_path_base = version_path_base;
        Ok(self)
    }

    pub fn build_path(&self, path: &str) -> AptosResult<Url> {
        Ok(self.base_url.join(&self.version_path_base)?.join(path)?)
    }

    /// Gets the balance of a specific asset type for an account.
    /// The `asset_type` parameter can be either:
    /// * A coin type (e.g. "0x1::aptos_coin::AptosCoin")
    /// * A fungible asset metadata address (e.g. "0xa")
    ///   For more details, see: https://aptos.dev/en/build/apis/fullnode-rest-api-reference#tag/accounts/GET/accounts/{address}/balance/{asset_type}
    pub async fn get_account_balance(
        &self,
        address: AccountAddress,
        asset_type: &str,
    ) -> AptosResult<Response<u64>> {
        let url = self.build_path(&format!(
            "accounts/{}/balance/{}",
            address.to_hex(),
            asset_type
        ))?;
        let response = self.inner.get(url).send().await?;
        self.json(response).await
    }

    pub async fn get_index(&self) -> AptosResult<Response<IndexResponse>> {
        self.get(self.build_path("")?).await
    }

    pub async fn get_account(&self, address: AccountAddress) -> AptosResult<Response<Account>> {
        let url = self.build_path(&format!("accounts/{}", address.to_hex()))?;
        let response = self.inner.get(url).send().await?;
        self.json(response).await
    }

    async fn get<T: DeserializeOwned>(&self, url: Url) -> AptosResult<Response<T>> {
        self.json(self.inner.get(url).send().await?).await
    }

    pub async fn submit(
        &self,
        txn: &SignedTransaction,
    ) -> AptosResult<Response<PendingTransaction>> {
        let txn_payload = bcs::to_bytes(txn)?;
        let url = self.build_path("transactions")?;

        let response = self
            .inner
            .post(url)
            .header(CONTENT_TYPE, BCS_SIGNED_TRANSACTION)
            .body(txn_payload)
            .send()
            .await?;

        self.json::<PendingTransaction>(response).await
    }

    pub async fn get_block_by_height(
        &self,
        height: u64,
        with_transactions: bool,
    ) -> AptosResult<Response<Block>> {
        self.get(self.build_path(&format!(
            "blocks/by_height/{}?with_transactions={}",
            height, with_transactions
        ))?)
        .await
    }

    async fn json<T: serde::de::DeserializeOwned>(
        &self,
        response: reqwest::Response,
    ) -> AptosResult<Response<T>> {
        let (response, state) = self.check_response(response).await?;
        let json = response.json().await.map_err(anyhow::Error::from)?;
        Ok(Response::new(json, state))
    }

    async fn check_response(
        &self,
        response: reqwest::Response,
    ) -> AptosResult<(reqwest::Response, State)> {
        if !response.status().is_success() {
            Err(parse_error(response).await)
        } else {
            let state = parse_state(&response)?;

            Ok((response, state))
        }
    }
}

fn parse_state(response: &reqwest::Response) -> AptosResult<State> {
    Ok(State::from_headers(response.headers())?)
}

fn parse_state_optional(response: &reqwest::Response) -> Option<State> {
    State::from_headers(response.headers())
        .map(Some)
        .unwrap_or(None)
}

async fn parse_error(response: reqwest::Response) -> RestError {
    let status_code = response.status();
    let maybe_state = parse_state_optional(&response);
    match response.json::<AptosError>().await {
        Ok(error) => (error, maybe_state, status_code).into(),
        Err(e) => RestError::Http(status_code, e),
    }
}

// If the user provided no version in the path, use the default. If the
// provided version has no trailing slash, add it, otherwise url.join
// will ignore the version path base.
pub fn get_version_path_with_base(base_url: Url) -> String {
    match base_url.path() {
        "/" => DEFAULT_VERSION_PATH_BASE.to_string(),
        path => {
            if !path.ends_with('/') {
                format!("{}/", path)
            } else {
                path.to_string()
            }
        }
    }
}
