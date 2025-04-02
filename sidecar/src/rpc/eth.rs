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

use alloy_json_rpc::RpcObject;
use alloy_primitives::{Bytes, B256};
use jsonrpsee::core::RpcResult;
use reth_rpc_eth_api::EthApiServer;

pub struct EthApi;

#[async_trait::async_trait]
impl<T: RpcObject, B: RpcObject, R: RpcObject, H: RpcObject> EthApiServer<T, B, R, H> for EthApi {
    async fn send_raw_transaction(&self, bytes: Bytes) -> RpcResult<B256> {
        Ok(Default::default())
    }
}
