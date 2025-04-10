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

pub mod engine;
pub mod server;

use engine::EngineConfig;
use serde::Deserialize;
use server::ServerConfig;
use std::{fs, path::Path};

#[derive(Debug, Clone, Deserialize, Default)]
pub struct Config {
    pub server: Option<ServerConfig>,
    pub engine: Option<EngineConfig>,
}

impl Config {
    pub fn load_from_path(path: Option<impl AsRef<Path>>) -> Self {
        if let Some(path) = path {
            let config_str = fs::read_to_string(path).expect("Failed to read config file");
            toml::from_str::<Config>(&config_str).expect("Failed to parse config file")
        } else {
            Config::default()
        }
    }
}
