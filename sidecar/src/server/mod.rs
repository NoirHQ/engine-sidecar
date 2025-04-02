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

pub mod cors;
pub mod router;
pub mod rpc;

use crate::config::server::{ItemOrList, ServerConfig};
use axum::{error_handling::HandleErrorLayer, http::StatusCode};
use cors::cors_layer;
use jsonrpsee::RpcModule;
use std::{net::SocketAddr, time::Duration};
use tokio::signal;
use tower::{BoxError, ServiceBuilder};
use tower_http::ServiceBuilderExt;

#[derive(Debug, Clone)]
pub struct Server {
    pub addr: SocketAddr,
    pub request_timeout_seconds: Duration,
    pub cors: Option<ItemOrList<String>>,
}

impl Server {
    pub fn new(config: ServerConfig) -> Self {
        Server {
            addr: config.addr(),
            request_timeout_seconds: config.request_timeout(),
            cors: config.cors,
        }
    }

    pub async fn start(&self) {
        let listener = tokio::net::TcpListener::bind(self.addr)
            .await
            .expect("Failed to bind to address");

        let middleware = ServiceBuilder::new()
            .layer(HandleErrorLayer::new(|err: BoxError| async move {
                if err.is::<tower::timeout::error::Elapsed>() {
                    StatusCode::REQUEST_TIMEOUT
                } else {
                    StatusCode::INTERNAL_SERVER_ERROR
                }
            }))
            .timeout(self.request_timeout_seconds)
            .trace_for_http()
            .layer(cors_layer(self.cors.clone()).expect("Failed to create CORS layer"));

        let module = RpcModule::new(());
        let app = router::create_router(module).layer(middleware.into_inner());

        tracing::info!("Starting server at {}", self.addr);

        axum::serve(listener, app)
            .with_graceful_shutdown(shutdown_signal())
            .await
            .unwrap();
    }
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("Failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("Failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}
