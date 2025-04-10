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

use axum::{extract::State, http::StatusCode, Json};
use jsonrpsee::{
    core::JsonValue as Value,
    types::{ErrorCode, ErrorObject},
    RpcModule,
};

pub async fn handle_rpc(
    State(module): State<RpcModule<()>>,
    Json(payload): Json<Value>,
) -> (StatusCode, Json<Value>) {
    let raw_request = serde_json::to_string(&payload).unwrap();

    match module.raw_json_request(&raw_request, 1).await {
        Ok((response, _)) => (
            StatusCode::OK,
            serde_json::from_str::<Value>(&response).map(Json).unwrap(),
        ),
        Err(e) => (
            StatusCode::BAD_REQUEST,
            Json(
                serde_json::to_value(ErrorObject::owned(
                    ErrorCode::ParseError.code(),
                    e.to_string(),
                    None::<()>,
                ))
                .unwrap(),
            ),
        ),
    }
}
