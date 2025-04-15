// Copyright © Aptos Foundation
// SPDX-License-Identifier: Apache-2.0

use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Formatter};

/// Move type `0x1::jwks::UnsupportedJWK` in rust.
/// See its doc in Move for more details.
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UnsupportedJWK {
    pub id: Vec<u8>,
    pub payload: Vec<u8>,
}

impl Debug for UnsupportedJWK {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("UnsupportedJWK")
            .field("id", &hex::encode(self.id.as_slice()))
            .field("payload", &String::from_utf8(self.payload.clone()))
            .finish()
    }
}
