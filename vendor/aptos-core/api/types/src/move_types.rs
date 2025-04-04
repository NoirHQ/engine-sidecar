// Copyright © Aptos Foundation
// Parts of the project are originally copyright © Meta Platforms, Inc.
// SPDX-License-Identifier: Apache-2.0

use anyhow::format_err;
use move_core_types::{
    language_storage::{StructTag, TypeTag},
    parser::{parse_struct_tag, parse_type_tag},
};
use serde::{de::Error as _, Deserialize, Deserializer, Serialize, Serializer};
use std::{fmt, fmt::Display, str::FromStr};

use crate::{address::Address, wrapper::IdentifierWrapper};

macro_rules! define_integer_type {
    ($n:ident, $t:ty, $d:literal) => {
        #[doc = $d]
        #[doc = "Encoded as a string to encode into JSON."]
        #[derive(Clone, Debug, Default, Eq, PartialEq, Copy)]
        pub struct $n(pub $t);

        impl $n {
            pub fn inner(&self) -> &$t {
                &self.0
            }
        }

        impl From<$t> for $n {
            fn from(d: $t) -> Self {
                Self(d)
            }
        }

        impl From<$n> for $t {
            fn from(d: $n) -> Self {
                d.0
            }
        }

        impl From<$n> for move_core_types::value::MoveValue {
            fn from(d: $n) -> Self {
                move_core_types::value::MoveValue::$n(d.0)
            }
        }

        impl fmt::Display for $n {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "{}", &self.0)
            }
        }

        impl Serialize for $n {
            fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
                self.0.to_string().serialize(serializer)
            }
        }

        impl<'de> Deserialize<'de> for $n {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                let s = <String>::deserialize(deserializer)?;
                s.parse().map_err(D::Error::custom)
            }
        }

        impl FromStr for $n {
            type Err = anyhow::Error;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                let data = s.parse::<$t>().map_err(|e| {
                    format_err!(
                        "Parsing {} string {:?} failed, caused by error: {}",
                        stringify!($t),
                        s,
                        e
                    )
                })?;

                Ok($n(data))
            }
        }
    };
}

define_integer_type!(U64, u64, "A string encoded U64.");
define_integer_type!(U128, u128, "A string encoded U128.");

/// Hex encoded bytes to allow for having bytes represented in JSON
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HexEncodedBytes(pub Vec<u8>);

impl FromStr for HexEncodedBytes {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Self, anyhow::Error> {
        let hex_str = if let Some(hex) = s.strip_prefix("0x") {
            hex
        } else {
            s
        };
        Ok(Self(hex::decode(hex_str).map_err(|e| {
            format_err!(
                "decode hex-encoded string({:?}) failed, caused by error: {}",
                s,
                e
            )
        })?))
    }
}

impl fmt::Display for HexEncodedBytes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "0x{}", hex::encode(&self.0))?;
        Ok(())
    }
}

impl Serialize for HexEncodedBytes {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.to_string().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HexEncodedBytes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = <String>::deserialize(deserializer)?;
        s.parse().map_err(D::Error::custom)
    }
}

/// A Move module Id
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct MoveModuleId {
    pub address: Address,
    pub name: IdentifierWrapper,
}

impl fmt::Display for MoveModuleId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}::{}", self.address, self.name)
    }
}

impl FromStr for MoveModuleId {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((address, name)) = s.split_once("::") {
            return Ok(Self {
                address: address.parse().map_err(|_| invalid_move_module_id(s))?,
                name: name.parse().map_err(|_| invalid_move_module_id(s))?,
            });
        }
        Err(invalid_move_module_id(s))
    }
}

#[inline]
fn invalid_move_module_id<S: Display + Sized>(s: S) -> anyhow::Error {
    format_err!("Invalid Move module ID: {}", s)
}

impl Serialize for MoveModuleId {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.to_string().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for MoveModuleId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let module_id = <String>::deserialize(deserializer)?;
        module_id.parse().map_err(D::Error::custom)
    }
}

/// Entry function id
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EntryFunctionId {
    pub module: MoveModuleId,
    pub name: IdentifierWrapper,
}

impl FromStr for EntryFunctionId {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((module, name)) = s.rsplit_once("::") {
            return Ok(Self {
                module: module.parse().map_err(|_| invalid_entry_function_id(s))?,
                name: name.parse().map_err(|_| invalid_entry_function_id(s))?,
            });
        }
        Err(invalid_entry_function_id(s))
    }
}

#[inline]
fn invalid_entry_function_id<S: Display + Sized>(s: S) -> anyhow::Error {
    format_err!("Invalid entry function ID {}", s)
}

impl Serialize for EntryFunctionId {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.to_string().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for EntryFunctionId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let entry_fun_id = <String>::deserialize(deserializer)?;
        entry_fun_id.parse().map_err(D::Error::custom)
    }
}

impl fmt::Display for EntryFunctionId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}::{}", self.module, self.name)
    }
}

/// An enum of Move's possible types on-chain
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MoveType {
    /// A bool type
    Bool,
    /// An 8-bit unsigned int
    U8,
    /// A 16-bit unsigned int
    U16,
    /// A 32-bit unsigned int
    U32,
    /// A 64-bit unsigned int
    U64,
    /// A 128-bit unsigned int
    U128,
    /// A 256-bit unsigned int
    U256,
    /// A 32-byte account address
    Address,
    /// An account signer
    Signer,
    /// A Vector of [`MoveType`]
    Vector { items: Box<MoveType> },
    /// A struct of [`MoveStructTag`]
    Struct(MoveStructTag),
    /// A generic type param with index
    GenericTypeParam { index: u16 },
    /// A reference
    Reference { mutable: bool, to: Box<MoveType> },
    /// A move type that couldn't be parsed
    ///
    /// This prevents the parser from just throwing an error because one field
    /// was unparsable, and gives the value in it.
    Unparsable(String),
}

impl fmt::Display for MoveType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MoveType::U8 => write!(f, "u8"),
            MoveType::U16 => write!(f, "u16"),
            MoveType::U32 => write!(f, "u32"),
            MoveType::U64 => write!(f, "u64"),
            MoveType::U128 => write!(f, "u128"),
            MoveType::U256 => write!(f, "u256"),
            MoveType::Address => write!(f, "address"),
            MoveType::Signer => write!(f, "signer"),
            MoveType::Bool => write!(f, "bool"),
            MoveType::Vector { items } => write!(f, "vector<{}>", items),
            MoveType::Struct(s) => write!(f, "{}", s),
            MoveType::GenericTypeParam { index } => write!(f, "T{}", index),
            MoveType::Reference { mutable, to } => {
                if *mutable {
                    write!(f, "&mut {}", to)
                } else {
                    write!(f, "&{}", to)
                }
            }
            MoveType::Unparsable(string) => write!(f, "unparsable<{}>", string),
        }
    }
}

// This function cannot handle the full range of types that MoveType can
// represent. Internally, it uses parse_type_tag, which cannot handle references
// or generic type parameters. This function adds nominal support for references
// on top of parse_type_tag, but it still does not work for generic type params.
// For that, we have the Unparsable variant of MoveType, so the deserialization
// doesn't fail when dealing with these values.
impl FromStr for MoveType {
    type Err = anyhow::Error;

    fn from_str(mut s: &str) -> Result<Self, Self::Err> {
        let mut is_ref = false;
        let mut is_mut = false;
        if s.starts_with('&') {
            s = &s[1..];
            is_ref = true;
        }
        if is_ref && s.starts_with("mut ") {
            s = &s[4..];
            is_mut = true;
        }
        // Previously this would just crap out, but this meant the API could
        // return a serialized version of an object and not be able to
        // deserialize it using that same object.
        let inner = match parse_type_tag(s) {
            Ok(inner) => inner.into(),
            Err(_e) => MoveType::Unparsable(s.to_string()),
        };
        if is_ref {
            Ok(MoveType::Reference {
                mutable: is_mut,
                to: Box::new(inner),
            })
        } else {
            Ok(inner)
        }
    }
}

impl From<TypeTag> for MoveType {
    fn from(tag: TypeTag) -> Self {
        match tag {
            TypeTag::Bool => MoveType::Bool,
            TypeTag::U8 => MoveType::U8,
            TypeTag::U16 => MoveType::U16,
            TypeTag::U32 => MoveType::U32,
            TypeTag::U64 => MoveType::U64,
            TypeTag::U256 => MoveType::U256,
            TypeTag::U128 => MoveType::U128,
            TypeTag::Address => MoveType::Address,
            TypeTag::Signer => MoveType::Signer,
            TypeTag::Vector(v) => MoveType::Vector {
                items: Box::new(MoveType::from(*v)),
            },
            TypeTag::Struct(v) => MoveType::Struct((*v).into()),
        }
    }
}

impl Serialize for MoveType {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.to_string().serialize(serializer)
    }
}

// This deserialization has limitations, see the FromStr impl for MoveType.
impl<'de> Deserialize<'de> for MoveType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let data = <String>::deserialize(deserializer)
            .map_err(|e| D::Error::custom(format_err!("deserialize Move type failed, {}", e)))?;
        data.parse().map_err(D::Error::custom)
    }
}

impl From<&TypeTag> for MoveType {
    fn from(tag: &TypeTag) -> Self {
        match tag {
            TypeTag::Bool => MoveType::Bool,
            TypeTag::U8 => MoveType::U8,
            TypeTag::U16 => MoveType::U16,
            TypeTag::U32 => MoveType::U32,
            TypeTag::U64 => MoveType::U64,
            TypeTag::U128 => MoveType::U128,
            TypeTag::U256 => MoveType::U256,
            TypeTag::Address => MoveType::Address,
            TypeTag::Signer => MoveType::Signer,
            TypeTag::Vector(v) => MoveType::Vector {
                items: Box::new(MoveType::from(v.as_ref())),
            },
            TypeTag::Struct(v) => MoveType::Struct((&**v).into()),
        }
    }
}

impl TryFrom<MoveType> for TypeTag {
    type Error = anyhow::Error;

    fn try_from(tag: MoveType) -> anyhow::Result<Self> {
        let ret = match tag {
            MoveType::Bool => TypeTag::Bool,
            MoveType::U8 => TypeTag::U8,
            MoveType::U16 => TypeTag::U16,
            MoveType::U32 => TypeTag::U32,
            MoveType::U64 => TypeTag::U64,
            MoveType::U128 => TypeTag::U128,
            MoveType::U256 => TypeTag::U256,
            MoveType::Address => TypeTag::Address,
            MoveType::Signer => TypeTag::Signer,
            MoveType::Vector { items } => TypeTag::Vector(Box::new((*items).try_into()?)),
            MoveType::Struct(v) => TypeTag::Struct(Box::new(v.try_into()?)),
            MoveType::GenericTypeParam { index: _ } => TypeTag::Address, // Dummy type, allows for Object<T>
            _ => {
                return Err(anyhow::anyhow!(
                    "Invalid move type for converting into `TypeTag`: {:?}",
                    &tag
                ))
            }
        };
        Ok(ret)
    }
}

/// A Move struct tag for referencing an onchain struct type
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MoveStructTag {
    pub address: Address,
    pub module: IdentifierWrapper,
    pub name: IdentifierWrapper,
    /// Generic type parameters associated with the struct
    pub generic_type_params: Vec<MoveType>,
}

impl FromStr for MoveStructTag {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Self, anyhow::Error> {
        Ok(parse_struct_tag(s)?.into())
    }
}

impl From<StructTag> for MoveStructTag {
    fn from(tag: StructTag) -> Self {
        Self {
            address: tag.address.into(),
            module: tag.module.into(),
            name: tag.name.into(),
            generic_type_params: tag.type_args.into_iter().map(MoveType::from).collect(),
        }
    }
}

impl From<&StructTag> for MoveStructTag {
    fn from(tag: &StructTag) -> Self {
        Self {
            address: tag.address.into(),
            module: IdentifierWrapper::from(&tag.module),
            name: IdentifierWrapper::from(&tag.name),
            generic_type_params: tag.type_args.iter().map(MoveType::from).collect(),
        }
    }
}

impl fmt::Display for MoveStructTag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}::{}::{}", self.address, self.module, self.name)?;
        if let Some(first_ty) = self.generic_type_params.first() {
            write!(f, "<")?;
            write!(f, "{}", first_ty)?;
            for ty in self.generic_type_params.iter().skip(1) {
                write!(f, ", {}", ty)?;
            }
            write!(f, ">")?;
        }
        Ok(())
    }
}

impl Serialize for MoveStructTag {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.to_string().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for MoveStructTag {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let data = <String>::deserialize(deserializer)?;
        data.parse().map_err(D::Error::custom)
    }
}

impl TryFrom<MoveStructTag> for StructTag {
    type Error = anyhow::Error;

    fn try_from(tag: MoveStructTag) -> anyhow::Result<Self> {
        Ok(Self {
            address: tag.address.into(),
            module: tag.module.into(),
            name: tag.name.into(),
            type_args: tag
                .generic_type_params
                .into_iter()
                .map(|p| p.try_into())
                .collect::<anyhow::Result<Vec<TypeTag>>>()?,
        })
    }
}
