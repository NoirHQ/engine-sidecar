// Copyright © Aptos Foundation
// Parts of the project are originally copyright © Meta Platforms, Inc.
// SPDX-License-Identifier: Apache-2.0

use anyhow::Result;
use serde::{de::DeserializeOwned, Serialize};
use thiserror::Error;

/// An error type for key and signature validation issues, see [`ValidCryptoMaterial`].
///
/// This enum reflects there are two interesting causes of validation
/// failure for the ingestion of key or signature material: deserialization errors
/// (often, due to mangled material or curve equation failure for ECC) and
/// validation errors (material recognizable but unacceptable for use,
/// e.g. unsafe).
#[derive(Clone, Debug, PartialEq, Eq, Error)]
#[error("{:?}", self)]
pub enum CryptoMaterialError {
    /// Struct to be signed does not serialize correctly.
    SerializationError,
    /// Key or signature material does not deserialize correctly.
    DeserializationError,
    /// Key or signature material deserializes, but is otherwise not valid.
    ValidationError,
    /// Key, threshold or signature material does not have the expected size.
    WrongLengthError,
    /// Part of the signature or key is not canonical resulting to malleability issues.
    CanonicalRepresentationError,
    /// A curve point (i.e., a public key) lies on a small group.
    SmallSubgroupError,
    /// A curve point (i.e., a public key) does not satisfy the curve equation.
    PointNotOnCurveError,
    /// BitVec errors in accountable multi-sig schemes.
    BitVecError(String),
}

/// Key or more generally crypto material with a notion of byte validation.
///
/// A type family for material that knows how to serialize and
/// deserialize, as well as validate byte-encoded material. The
/// validation must be implemented as a [`TryFrom`] which
/// classifies its failures against the above
/// [`CryptoMaterialError`].
///
/// This provides an implementation for a validation that relies on a
/// round-trip to bytes and corresponding [`TryFrom`].
pub trait ValidCryptoMaterial:
    // The for<'a> exactly matches the assumption "deserializable from any lifetime".
    for<'a> TryFrom<&'a [u8], Error=CryptoMaterialError> + Serialize + DeserializeOwned
{
    /// Prefix for AIP-80 e.g. ed25519-priv
    const AIP_80_PREFIX: &'static str;

    /// Convert the valid crypto material to bytes.
    fn to_bytes(&self) -> Vec<u8>;
}

/// An extension to/from Strings for [`ValidCryptoMaterial`].
///
/// Relies on [`hex`] for string encoding / decoding.
/// No required fields, provides a default implementation.
pub trait ValidCryptoMaterialStringExt: ValidCryptoMaterial {
    /// When trying to convert from bytes, we simply decode the string into
    /// bytes before checking if we can convert.
    fn from_encoded_string(encoded_str: &str) -> std::result::Result<Self, CryptoMaterialError> {
        let mut str = encoded_str;
        // First strip the AIP-80 prefix
        str = str.strip_prefix(Self::AIP_80_PREFIX).unwrap_or(str);

        // Strip 0x at beginning if there is one
        str = str.strip_prefix("0x").unwrap_or(str);

        let bytes_out = ::hex::decode(str);
        // We defer to `try_from` to make sure we only produce valid crypto materials.
        bytes_out
            // We reinterpret a failure to serialize: key is mangled someway.
            .or(Err(CryptoMaterialError::DeserializationError))
            .and_then(|ref bytes| Self::try_from(bytes))
    }

    /// A function to encode into hex-string after serializing.
    fn to_encoded_string(&self) -> Result<String> {
        Ok(format!("0x{}", ::hex::encode(self.to_bytes())))
    }

    /// Creates an AIP-80 formatted string for the crypto material
    fn to_aip_80_string(&self) -> Result<String> {
        let bytes = self.to_encoded_string()?;
        Ok(format!("{}{}", Self::AIP_80_PREFIX, bytes))
    }
}

// There's nothing required in this extension, so let's just derive it
// for anybody that has a ValidCryptoMaterial.
impl<T: ValidCryptoMaterial> ValidCryptoMaterialStringExt for T {}
