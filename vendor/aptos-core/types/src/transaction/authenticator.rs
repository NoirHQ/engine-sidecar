// Copyright © Aptos Foundation
// Parts of the project are originally copyright © Meta Platforms, Inc.
// SPDX-License-Identifier: Apache-2.0

use crate::function_info::FunctionInfo;
use serde::{Deserialize, Serialize};

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
