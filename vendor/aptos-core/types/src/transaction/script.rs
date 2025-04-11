// Copyright © Aptos Foundation
// Parts of the project are originally copyright © Meta Platforms, Inc.
// SPDX-License-Identifier: Apache-2.0

use crate::serde_helper::vec_bytes;
use move_core_types::{
    identifier::Identifier,
    language_storage::{ModuleId, TypeTag},
};
use serde::{Deserialize, Serialize};

/// Call a Move entry function.
#[derive(Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct EntryFunction {
    module: ModuleId,
    function: Identifier,
    ty_args: Vec<TypeTag>,
    #[serde(with = "vec_bytes")]
    args: Vec<Vec<u8>>,
}
