// Copyright © Aptos Foundation
// Parts of the project are originally copyright © Meta Platforms, Inc.
// SPDX-License-Identifier: Apache-2.0

use crate::function_info::FunctionInfo;
use anyhow::{ensure, Error, Result};
use aptos_crypto::traits::{
    CryptoMaterialError, ValidCryptoMaterial, ValidCryptoMaterialStringExt,
};
use aptos_crypto_derive::{CryptoHasher, DeserializeKey, SerializeKey};
use move_core_types::account_address::AccountAddress;
use serde::{Deserialize, Serialize};
use std::{fmt, str::FromStr};

/// Each transaction submitted to the Aptos blockchain contains a `TransactionAuthenticator`. During
/// transaction execution, the executor will check if every `AccountAuthenticator`'s signature on
/// the transaction hash is well-formed and whether the sha3 hash of the
/// `AccountAuthenticator`'s `AuthenticationKeyPreimage` matches the `AuthenticationKey` stored
/// under the participating signer's account address.
#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum TransactionAuthenticator {
    // /// Single Ed25519 signature
    // Ed25519 {
    //     public_key: Ed25519PublicKey,
    //     signature: Ed25519Signature,
    // },
    // /// K-of-N multisignature
    // MultiEd25519 {
    //     public_key: MultiEd25519PublicKey,
    //     signature: MultiEd25519Signature,
    // },
    // /// Multi-agent transaction.
    // MultiAgent {
    //     sender: AccountAuthenticator,
    //     secondary_signer_addresses: Vec<AccountAddress>,
    //     secondary_signers: Vec<AccountAuthenticator>,
    // },
    // /// Optional Multi-agent transaction with a fee payer.
    // FeePayer {
    //     sender: AccountAuthenticator,
    //     secondary_signer_addresses: Vec<AccountAddress>,
    //     secondary_signers: Vec<AccountAuthenticator>,
    //     fee_payer_address: AccountAddress,
    //     fee_payer_signer: AccountAuthenticator,
    // },
    SingleSender { sender: AccountAuthenticator },
}

impl TransactionAuthenticator {
    /// Create a single-sender authenticator
    pub fn single_sender(sender: AccountAuthenticator) -> Self {
        Self::SingleSender { sender }
    }
}

/// An `AccountAuthenticator` is an abstraction of a signature scheme. It must know:
/// (1) How to check its signature against a message and public key
/// (2) How to convert its public key into an `AuthenticationKeyPreimage` structured as
/// (public_key | signature_scheme_id).
/// Each on-chain `Account` must store an `AuthenticationKey` (computed via a sha3 hash of `(public
/// key bytes | scheme as u8)`).
#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum AccountAuthenticator {
    // /// Ed25519 Single signature
    // Ed25519 {
    //     public_key: Ed25519PublicKey,
    //     signature: Ed25519Signature,
    // },
    // /// Ed25519 K-of-N multisignature
    // MultiEd25519 {
    //     public_key: MultiEd25519PublicKey,
    //     signature: MultiEd25519Signature,
    // },
    // SingleKey {
    //     authenticator: SingleKeyAuthenticator,
    // },
    // MultiKey {
    //     authenticator: MultiKeyAuthenticator,
    // },
    // NoAccountAuthenticator,
    Abstraction {
        function_info: FunctionInfo,
        auth_data: AbstractionAuthData,
    }, // ... add more schemes here
}

impl AccountAuthenticator {
    // /// Unique identifier for the signature scheme
    // pub fn scheme(&self) -> Scheme {
    //     match self {
    //         Self::Ed25519 { .. } => Scheme::Ed25519,
    //         Self::MultiEd25519 { .. } => Scheme::MultiEd25519,
    //         Self::SingleKey { .. } => Scheme::SingleKey,
    //         Self::MultiKey { .. } => Scheme::MultiKey,
    //         Self::NoAccountAuthenticator => Scheme::NoScheme,
    //         Self::Abstraction { .. } => Scheme::Abstraction,
    //     }
    // }

    // /// Create a single-signature ed25519 authenticator
    // pub fn ed25519(public_key: Ed25519PublicKey, signature: Ed25519Signature) -> Self {
    //     Self::Ed25519 {
    //         public_key,
    //         signature,
    //     }
    // }

    // /// Create a multisignature ed25519 authenticator
    // pub fn multi_ed25519(
    //     public_key: MultiEd25519PublicKey,
    //     signature: MultiEd25519Signature,
    // ) -> Self {
    //     Self::MultiEd25519 {
    //         public_key,
    //         signature,
    //     }
    // }

    // /// Create a single-signature authenticator
    // pub fn single_key(authenticator: SingleKeyAuthenticator) -> Self {
    //     Self::SingleKey { authenticator }
    // }

    // /// Create a multi-signature authenticator
    // pub fn multi_key(authenticator: MultiKeyAuthenticator) -> Self {
    //     Self::MultiKey { authenticator }
    // }

    /// Create a abstracted authenticator
    pub fn abstraction(
        function_info: FunctionInfo,
        signing_message_digest: Vec<u8>,
        authenticator: Vec<u8>,
    ) -> Self {
        Self::Abstraction {
            function_info,
            auth_data: AbstractionAuthData::V1 {
                signing_message_digest,
                authenticator,
            },
        }
    }

    // /// Create a domain abstracted authenticator
    // pub fn derivable_abstraction(
    //     function_info: FunctionInfo,
    //     signing_message_digest: Vec<u8>,
    //     abstract_signature: Vec<u8>,
    //     abstract_public_key: Vec<u8>,
    // ) -> Self {
    //     Self::Abstraction {
    //         function_info,
    //         auth_data: AbstractionAuthData::DerivableV1 {
    //             signing_message_digest,
    //             abstract_signature,
    //             abstract_public_key,
    //         },
    //     }
    // }

    // pub fn is_abstracted(&self) -> bool {
    //     matches!(self, Self::Abstraction { .. })
    // }

    // /// Return Ok if the authenticator's public key matches its signature, Err otherwise
    // pub fn verify<T: Serialize + CryptoHash>(&self, message: &T) -> Result<()> {
    //     match self {
    //         Self::Ed25519 {
    //             public_key,
    //             signature,
    //         } => signature.verify(message, public_key),
    //         Self::MultiEd25519 {
    //             public_key,
    //             signature,
    //         } => signature.verify(message, public_key),
    //         Self::SingleKey { authenticator } => authenticator.verify(message),
    //         Self::MultiKey { authenticator } => authenticator.verify(message),
    //         Self::NoAccountAuthenticator => bail!("No signature to verify."),
    //         // Abstraction delayed the authentication after prologue.
    //         Self::Abstraction { auth_data, .. } => {
    //             ensure!(auth_data.signing_message_digest() == &HashValue::sha3_256_of(signing_message(message)?.as_slice()).to_vec(), "The signing message digest provided in Abstraction Authenticator is not expected");
    //             Ok(())
    //         },
    //     }
    // }

    /// Return the raw bytes of `self.public_key`
    pub fn public_key_bytes(&self) -> Vec<u8> {
        match self {
            // Self::Ed25519 { public_key, .. } => public_key.to_bytes().to_vec(),
            // Self::MultiEd25519 { public_key, .. } => public_key.to_bytes().to_vec(),
            // Self::SingleKey { authenticator } => authenticator.public_key_bytes(),
            // Self::MultiKey { authenticator } => authenticator.public_key_bytes(),
            // Self::NoAccountAuthenticator => vec![],
            Self::Abstraction { .. } => vec![],
        }
    }

    /// Return the raw bytes of `self.signature`
    pub fn signature_bytes(&self) -> Vec<u8> {
        match self {
            // Self::Ed25519 { signature, .. } => signature.to_bytes().to_vec(),
            // Self::MultiEd25519 { signature, .. } => signature.to_bytes().to_vec(),
            // Self::SingleKey { authenticator } => authenticator.signature_bytes(),
            // Self::MultiKey { authenticator } => authenticator.signature_bytes(),
            // Self::NoAccountAuthenticator => vec![],
            Self::Abstraction { .. } => vec![],
        }
    }

    // /// Return an authentication proof derived from `self`'s public key and scheme id
    // pub fn authentication_proof(&self) -> AuthenticationProof {
    //     match self {
    //         Self::NoAccountAuthenticator => AuthenticationProof::None,
    //         Self::Abstraction {
    //             function_info,
    //             auth_data,
    //         } => AuthenticationProof::Abstraction {
    //             function_info: function_info.clone(),
    //             auth_data: auth_data.clone(),
    //         },
    //         Self::Ed25519 { .. }
    //         | Self::MultiEd25519 { .. }
    //         | Self::SingleKey { .. }
    //         | Self::MultiKey { .. } => AuthenticationProof::Key(
    //             AuthenticationKey::from_preimage(self.public_key_bytes(), self.scheme()).to_vec(),
    //         ),
    //     }
    // }

    // /// Return the number of signatures included in this account authenticator.
    // pub fn number_of_signatures(&self) -> usize {
    //     match self {
    //         Self::Ed25519 { .. } => 1,
    //         Self::MultiEd25519 { signature, .. } => signature.signatures().len(),
    //         Self::SingleKey { .. } => 1,
    //         Self::MultiKey { authenticator } => authenticator.signatures.len(),
    //         Self::NoAccountAuthenticator => 0,
    //         Self::Abstraction { .. } => 0,
    //     }
    // }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize, Hash)]
pub enum AbstractionAuthData {
    V1 {
        #[serde(with = "serde_bytes")]
        signing_message_digest: Vec<u8>,
        #[serde(with = "serde_bytes")]
        authenticator: Vec<u8>,
    },
    DerivableV1 {
        #[serde(with = "serde_bytes")]
        signing_message_digest: Vec<u8>,
        #[serde(with = "serde_bytes")]
        abstract_signature: Vec<u8>,
        #[serde(with = "serde_bytes")]
        abstract_public_key: Vec<u8>,
    },
}

impl AbstractionAuthData {
    pub fn signing_message_digest(&self) -> &Vec<u8> {
        match self {
            Self::V1 {
                signing_message_digest,
                ..
            }
            | Self::DerivableV1 {
                signing_message_digest,
                ..
            } => signing_message_digest,
        }
    }
}

/// A struct that represents an account authentication key. An account's address is the last 32
/// bytes of authentication key used to create it
#[derive(
    Clone,
    Copy,
    CryptoHasher,
    Debug,
    DeserializeKey,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    SerializeKey,
)]
// #[cfg_attr(any(test, feature = "fuzzing"), derive(Arbitrary))]
pub struct AuthenticationKey([u8; AuthenticationKey::LENGTH]);

impl AuthenticationKey {
    /// The number of bytes in an authentication key.
    pub const LENGTH: usize = AccountAddress::LENGTH;

    /// Create an authentication key from `bytes`
    pub const fn new(bytes: [u8; Self::LENGTH]) -> Self {
        Self(bytes)
    }

    /// Return an authentication key that is impossible (in expectation) to sign for--useful for
    /// intentionally relinquishing control of an account.
    pub const fn zero() -> Self {
        Self([0; 32])
    }

    // /// Create an authentication key from a preimage by taking its sha3 hash
    // pub fn from_preimage(mut public_key_bytes: Vec<u8>, scheme: Scheme) -> AuthenticationKey {
    //     public_key_bytes.push(scheme as u8);
    //     AuthenticationKey::new(*HashValue::sha3_256_of(&public_key_bytes).as_ref())
    // }

    // /// Construct a preimage from a transaction-derived AUID as (txn_hash || auid_scheme_id)
    // pub fn auid(mut txn_hash: Vec<u8>, auid_counter: u64) -> Self {
    //     txn_hash.extend(auid_counter.to_le_bytes().to_vec());
    //     Self::from_preimage(txn_hash, Scheme::DeriveAuid)
    // }

    // pub fn object_address_from_object(
    //     source: &AccountAddress,
    //     derive_from: &AccountAddress,
    // ) -> AuthenticationKey {
    //     let mut bytes = source.to_vec();
    //     bytes.append(&mut derive_from.to_vec());
    //     Self::from_preimage(bytes, Scheme::DeriveObjectAddressFromObject)
    // }

    // pub fn domain_abstraction_address(
    //     func_info_bcs_bytes: Vec<u8>,
    //     account_identity: &[u8],
    // ) -> AuthenticationKey {
    //     let mut bytes = func_info_bcs_bytes;
    //     bytes.append(&mut bcs::to_bytes(account_identity).expect("must serialize byte array"));
    //     Self::from_preimage(bytes, Scheme::DeriveDomainAbstraction)
    // }

    // /// Create an authentication key from an Ed25519 public key
    // pub fn ed25519(public_key: &Ed25519PublicKey) -> AuthenticationKey {
    //     Self::from_preimage(public_key.to_bytes().to_vec(), Scheme::Ed25519)
    // }

    // /// Create an authentication key from a MultiEd25519 public key
    // pub fn multi_ed25519(public_key: &MultiEd25519PublicKey) -> Self {
    //     Self::from_preimage(public_key.to_bytes(), Scheme::MultiEd25519)
    // }

    // /// Create an authentication key from an AnyPublicKey
    // pub fn any_key(public_key: AnyPublicKey) -> AuthenticationKey {
    //     Self::from_preimage(public_key.to_bytes(), Scheme::SingleKey)
    // }

    // /// Create an authentication key from multiple AnyPublicKeys
    // pub fn multi_key(public_keys: MultiKey) -> AuthenticationKey {
    //     Self::from_preimage(public_keys.to_bytes(), Scheme::MultiKey)
    // }

    // /// Return the authentication key as an account address
    // pub fn account_address(&self) -> AccountAddress {
    //     AccountAddress::new(self.0)
    // }

    /// Construct a vector from this authentication key
    pub fn to_vec(&self) -> Vec<u8> {
        self.0.to_vec()
    }

    // /// Create a random authentication key. For testing only
    // pub fn random() -> Self {
    //     let mut rng = OsRng;
    //     let buf: [u8; Self::LENGTH] = rng.gen();
    //     AuthenticationKey::new(buf)
    // }
}

#[derive(Debug)]
#[repr(u8)]
pub enum Scheme {
    Ed25519 = 0,
    MultiEd25519 = 1,
    SingleKey = 2,
    MultiKey = 3,
    Abstraction = 4,
    DeriveDomainAbstraction = 5,
    NoScheme = 250,
    /// Scheme identifier used to derive addresses (not the authentication key) of objects and
    /// resources accounts. This application serves to domain separate hashes. Without such
    /// separation, an adversary could create (and get a signer for) a these accounts
    /// when a their address matches matches an existing address of a MultiEd25519 wallet.
    /// Add new derived schemes below.
    DeriveAuid = 251,
    DeriveObjectAddressFromObject = 252,
    DeriveObjectAddressFromGuid = 253,
    DeriveObjectAddressFromSeed = 254,
    DeriveResourceAccountAddress = 255,
}

impl ValidCryptoMaterial for AuthenticationKey {
    const AIP_80_PREFIX: &'static str = "";

    fn to_bytes(&self) -> Vec<u8> {
        self.to_vec()
    }
}

impl TryFrom<&[u8]> for AuthenticationKey {
    type Error = CryptoMaterialError;

    fn try_from(bytes: &[u8]) -> std::result::Result<AuthenticationKey, CryptoMaterialError> {
        if bytes.len() != Self::LENGTH {
            return Err(CryptoMaterialError::WrongLengthError);
        }
        let mut addr = [0u8; Self::LENGTH];
        addr.copy_from_slice(bytes);
        Ok(AuthenticationKey(addr))
    }
}

impl TryFrom<Vec<u8>> for AuthenticationKey {
    type Error = CryptoMaterialError;

    fn try_from(bytes: Vec<u8>) -> std::result::Result<AuthenticationKey, CryptoMaterialError> {
        AuthenticationKey::try_from(&bytes[..])
    }
}

impl FromStr for AuthenticationKey {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        ensure!(
            !s.is_empty(),
            "authentication key string should not be empty.",
        );
        let bytes_out = ::hex::decode(s)?;
        let key = AuthenticationKey::try_from(bytes_out.as_slice())?;
        Ok(key)
    }
}

impl AsRef<[u8]> for AuthenticationKey {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl fmt::LowerHex for AuthenticationKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", hex::encode(self.0))
    }
}

impl fmt::Display for AuthenticationKey {
    fn fmt(&self, f: &mut fmt::Formatter) -> std::fmt::Result {
        // Forward to the LowerHex impl with a "0x" prepended (the # flag).
        write!(f, "{:#x}", self)
    }
}

impl AccountAuthenticator {
    /// Unique identifier for the signature scheme
    pub fn scheme(&self) -> Scheme {
        match self {
            // Self::Ed25519 { .. } => Scheme::Ed25519,
            // Self::MultiEd25519 { .. } => Scheme::MultiEd25519,
            // Self::SingleKey { .. } => Scheme::SingleKey,
            // Self::MultiKey { .. } => Scheme::MultiKey,
            // Self::NoAccountAuthenticator => Scheme::NoScheme,
            Self::Abstraction { .. } => Scheme::Abstraction,
        }
    }
}

impl fmt::Display for AccountAuthenticator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "AccountAuthenticator[scheme id: {:?}, public key: {}, signature: {}]",
            self.scheme(),
            hex::encode(self.public_key_bytes()),
            hex::encode(self.signature_bytes())
        )
    }
}
