// Copyright © Aptos Foundation
// SPDX-License-Identifier: Apache-2.0

use super::{rsa::RSA_JWK, unsupported::UnsupportedJWK};
use serde::{Deserialize, Serialize};

/// The JWK type that can be converted from/to `JWKMoveStruct` but easier to use in rust.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum JWK {
    RSA(RSA_JWK),
    Unsupported(UnsupportedJWK),
}
