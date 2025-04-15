// Copyright © Aptos Foundation
// SPDX-License-Identifier: Apache-2.0

use serde::{Deserialize, Serialize};

/// Move type `0x1::jwks::RSA_JWK` in rust.
/// See its doc in Move for more details.
#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct RSA_JWK {
    pub kid: String,
    pub kty: String,
    pub alg: String,
    pub e: String,
    pub n: String,
}
