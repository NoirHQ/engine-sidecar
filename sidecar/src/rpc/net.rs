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

use alloy_primitives::U64;
use jsonrpsee::core::RpcResult as Result;
use reth_rpc_api::NetApiServer;

/// `Net` API implementation.
///
/// This type provides the functionality for handling `net` related requests.
pub struct NetApi;

impl NetApiServer for NetApi {
    /// Handler for `net_version`
    fn version(&self) -> Result<String> {
        tracing::debug!("version rpc request received");
        Ok(U64::from_be_slice(&hex::decode("deadbeef").unwrap()).to_string())
    }

    /// Handler for `net_peerCount`
    fn peer_count(&self) -> Result<U64> {
        // Ok(U64::from(self.network.num_connected_peers()))
        unimplemented!();
    }

    /// Handler for `net_listening`
    fn is_listening(&self) -> Result<bool> {
        Ok(true)
    }
}
