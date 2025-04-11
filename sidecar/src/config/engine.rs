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

use crate::engine::adapter::{
    local::LocalEngineAdapter, remote::RemoteEngineAdapter, EngineAdapter,
};
use aptos_types::chain_id::NamedChain;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize, Default)]
pub struct EngineConfig {
    pub basic: Option<EngineBasicConfig>,
    pub adapter: Option<AdapterConfig>,
}

impl EngineConfig {
    pub fn basic(&self) -> EngineBasicConfig {
        self.basic.clone().unwrap_or_default()
    }

    pub fn adapter(&self) -> AdapterConfig {
        self.adapter.clone().unwrap_or_default()
    }
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct EngineBasicConfig {
    pub coin_type: Option<String>,
    pub auth_func: Option<String>,
    pub entry_func: Option<String>,
}

impl EngineBasicConfig {
    pub fn coin_type(&self) -> String {
        self.coin_type
            .clone()
            .unwrap_or_else(|| "0x1::aptos_coin::AptosCoin".into())
    }

    pub fn auth_func(&self) -> String {
        self.auth_func
            .clone()
            .unwrap_or_else(|| "0x100::evm::authenticate".into())
    }

    pub fn entry_func(&self) -> String {
        self.auth_func
            .clone()
            .unwrap_or_else(|| "0x100::evm::transact".into())
    }
}

#[derive(Debug, Clone, Deserialize)]
pub enum AdapterConfig {
    Remote(RemoteEngineConfig),
    Local,
}

impl Default for AdapterConfig {
    fn default() -> Self {
        Self::Remote(RemoteEngineConfig::default())
    }
}

impl AdapterConfig {
    pub fn build_adapter(&self, config: EngineBasicConfig) -> Box<dyn EngineAdapter + Send + Sync> {
        let coin_type = config.coin_type();
        let auth_func = config.auth_func();
        let entry_func = config.entry_func();

        match self {
            AdapterConfig::Remote(remote) => Box::new(RemoteEngineAdapter::new(
                coin_type,
                auth_func,
                entry_func,
                remote.clone(),
            )),
            AdapterConfig::Local => Box::new(LocalEngineAdapter::new(coin_type)),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct RemoteEngineConfig {
    pub endpoint: Option<String>,
    pub timeout: Option<u64>,
    pub chain_id: Option<u8>,
}

impl RemoteEngineConfig {
    pub fn endpoint(&self) -> &str {
        self.endpoint
            .as_deref()
            .unwrap_or("http://127.0.0.1:8080/v1")
    }

    pub fn timeout(&self) -> u64 {
        self.timeout.unwrap_or(10)
    }

    pub fn chain_id(&self) -> u8 {
        self.chain_id.unwrap_or(NamedChain::TESTING.id())
    }
}
