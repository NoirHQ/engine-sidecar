// Copyright © Aptos Foundation
// SPDX-License-Identifier: Apache-2.0

use serde::{Deserialize, Serialize};

/// An `Ability` classifies what operations are permitted for a given type
#[repr(u8)]
#[derive(Debug, Clone, Eq, Copy, Hash, Ord, PartialEq, PartialOrd)]
// #[cfg_attr(any(test, feature = "fuzzing"), derive(proptest_derive::Arbitrary))]
// #[cfg_attr(feature = "fuzzing", derive(arbitrary::Arbitrary))]
pub enum Ability {
    /// Allows values of types with this ability to be copied, via CopyLoc or ReadRef
    Copy = 0x1,
    /// Allows values of types with this ability to be dropped, via Pop, WriteRef, StLoc, Eq, Neq,
    /// or if left in a local when Ret is invoked
    /// Technically also needed for numeric operations (Add, BitAnd, Shift, etc), but all
    /// of the types that can be used with those operations have Drop
    Drop = 0x2,
    /// Allows values of types with this ability to exist inside a struct in global storage
    Store = 0x4,
    /// Allows the type to serve as a key for global storage operations: MoveTo, MoveFrom, etc.
    Key = 0x8,
}

impl Ability {
    fn from_u8(u: u8) -> Option<Self> {
        match u {
            0x1 => Some(Ability::Copy),
            0x2 => Some(Ability::Drop),
            0x4 => Some(Ability::Store),
            0x8 => Some(Ability::Key),
            _ => None,
        }
    }
}

/// A set of `Ability`s
#[derive(Clone, Eq, Copy, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
// #[cfg_attr(
//     any(test, feature = "fuzzing"),
//     derive(arbitrary::Arbitrary, dearbitrary::Dearbitrary)
// )]
pub struct AbilitySet(u8);

pub struct AbilitySetIterator {
    set: AbilitySet,
    idx: u8,
}

impl Iterator for AbilitySetIterator {
    type Item = Ability;

    fn next(&mut self) -> Option<Self::Item> {
        while self.idx <= 0x8 {
            let next = Ability::from_u8(self.set.0 & self.idx);
            self.idx <<= 1;
            if next.is_some() {
                return next;
            }
        }
        None
    }
}

impl IntoIterator for AbilitySet {
    type IntoIter = AbilitySetIterator;
    type Item = Ability;

    fn into_iter(self) -> Self::IntoIter {
        AbilitySetIterator {
            idx: 0x1,
            set: self,
        }
    }
}
