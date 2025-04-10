use alloy_consensus::{
    transaction::{PooledTransaction, RlpEcdsaDecodableTx, RlpEcdsaEncodableTx},
    BlobTransactionSidecar, SignableTransaction, Signed, TxEip1559, TxEip2930, TxEip4844,
    TxEip4844WithSidecar, TxEip7702, TxLegacy, TxType,
};
use alloy_eips::{
    eip2718::{Eip2718Error, Eip2718Result},
    eip2930::AccessList,
    eip7702::SignedAuthorization,
    Decodable2718, Encodable2718, Typed2718,
};
use alloy_primitives::{
    keccak256, Address, Bytes, ChainId, PrimitiveSignature as Signature, TxHash, TxKind, B256, U256,
};
use alloy_rlp::{Decodable, Encodable};
use reth_primitives_traits::{
    crypto::secp256k1::{recover_signer, recover_signer_unchecked},
    size::InMemorySize,
    transaction::signed::{RecoveryError, SignedTransaction},
};
use std::{
    hash::{Hash, Hasher},
    sync::OnceLock,
};

macro_rules! delegate {
    ($self:expr => $tx:ident.$method:ident($($arg:expr),*)) => {
        match $self {
            Transaction::Legacy($tx) => $tx.$method($($arg),*),
            Transaction::Eip2930($tx) => $tx.$method($($arg),*),
            Transaction::Eip1559($tx) => $tx.$method($($arg),*),
            Transaction::Eip4844($tx) => $tx.$method($($arg),*),
            Transaction::Eip7702($tx) => $tx.$method($($arg),*),
        }
    };
}

/// A raw transaction.
///
/// Transaction types were introduced in [EIP-2718](https://eips.ethereum.org/EIPS/eip-2718).
#[derive(Debug, Clone, PartialEq, Eq, Hash, derive_more::From)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
// #[cfg_attr(any(test, feature = "arbitrary"), derive(arbitrary::Arbitrary))]
// #[cfg_attr(any(test, feature = "reth-codec"), reth_codecs::add_arbitrary_tests(compact))]
pub enum Transaction {
    /// Legacy transaction (type `0x0`).
    ///
    /// Traditional Ethereum transactions, containing parameters `nonce`, `gasPrice`, `gasLimit`,
    /// `to`, `value`, `data`, `v`, `r`, and `s`.
    ///
    /// These transactions do not utilize access lists nor do they incorporate EIP-1559 fee market
    /// changes.
    Legacy(TxLegacy),
    /// Transaction with an [`AccessList`] ([EIP-2930](https://eips.ethereum.org/EIPS/eip-2930)), type `0x1`.
    ///
    /// The `accessList` specifies an array of addresses and storage keys that the transaction
    /// plans to access, enabling gas savings on cross-contract calls by pre-declaring the accessed
    /// contract and storage slots.
    Eip2930(TxEip2930),
    /// A transaction with a priority fee ([EIP-1559](https://eips.ethereum.org/EIPS/eip-1559)), type `0x2`.
    ///
    /// Unlike traditional transactions, EIP-1559 transactions use an in-protocol, dynamically
    /// changing base fee per gas, adjusted at each block to manage network congestion.
    ///
    /// - `maxPriorityFeePerGas`, specifying the maximum fee above the base fee the sender is
    ///   willing to pay
    /// - `maxFeePerGas`, setting the maximum total fee the sender is willing to pay.
    ///
    /// The base fee is burned, while the priority fee is paid to the miner who includes the
    /// transaction, incentivizing miners to include transactions with higher priority fees per
    /// gas.
    Eip1559(TxEip1559),
    /// Shard Blob Transactions ([EIP-4844](https://eips.ethereum.org/EIPS/eip-4844)), type `0x3`.
    ///
    /// Shard Blob Transactions introduce a new transaction type called a blob-carrying transaction
    /// to reduce gas costs. These transactions are similar to regular Ethereum transactions but
    /// include additional data called a blob.
    ///
    /// Blobs are larger (~125 kB) and cheaper than the current calldata, providing an immutable
    /// and read-only memory for storing transaction data.
    ///
    /// EIP-4844, also known as proto-danksharding, implements the framework and logic of
    /// danksharding, introducing new transaction formats and verification rules.
    Eip4844(TxEip4844),
    /// EOA Set Code Transactions ([EIP-7702](https://eips.ethereum.org/EIPS/eip-7702)), type `0x4`.
    ///
    /// EOA Set Code Transactions give the ability to set contract code for an EOA in perpetuity
    /// until re-assigned by the same EOA. This allows for adding smart contract functionality to
    /// the EOA.
    Eip7702(TxEip7702),
}

impl Transaction {
    /// Returns [`TxType`] of the transaction.
    pub const fn tx_type(&self) -> TxType {
        match self {
            Self::Legacy(_) => TxType::Legacy,
            Self::Eip2930(_) => TxType::Eip2930,
            Self::Eip1559(_) => TxType::Eip1559,
            Self::Eip4844(_) => TxType::Eip4844,
            Self::Eip7702(_) => TxType::Eip7702,
        }
    }

    /// This sets the transaction's nonce.
    pub fn set_nonce(&mut self, nonce: u64) {
        match self {
            Self::Legacy(tx) => tx.nonce = nonce,
            Self::Eip2930(tx) => tx.nonce = nonce,
            Self::Eip1559(tx) => tx.nonce = nonce,
            Self::Eip4844(tx) => tx.nonce = nonce,
            Self::Eip7702(tx) => tx.nonce = nonce,
        }
    }
}

impl Typed2718 for Transaction {
    fn ty(&self) -> u8 {
        delegate!(self => tx.ty())
    }
}

impl alloy_consensus::Transaction for Transaction {
    fn chain_id(&self) -> Option<ChainId> {
        delegate!(self => tx.chain_id())
    }

    fn nonce(&self) -> u64 {
        delegate!(self => tx.nonce())
    }

    fn gas_limit(&self) -> u64 {
        delegate!(self => tx.gas_limit())
    }

    fn gas_price(&self) -> Option<u128> {
        delegate!(self => tx.gas_price())
    }

    fn max_fee_per_gas(&self) -> u128 {
        delegate!(self => tx.max_fee_per_gas())
    }

    fn max_priority_fee_per_gas(&self) -> Option<u128> {
        delegate!(self => tx.max_priority_fee_per_gas())
    }

    fn max_fee_per_blob_gas(&self) -> Option<u128> {
        delegate!(self => tx.max_fee_per_blob_gas())
    }

    fn priority_fee_or_price(&self) -> u128 {
        delegate!(self => tx.priority_fee_or_price())
    }

    fn effective_gas_price(&self, base_fee: Option<u64>) -> u128 {
        delegate!(self => tx.effective_gas_price(base_fee))
    }

    fn is_dynamic_fee(&self) -> bool {
        delegate!(self => tx.is_dynamic_fee())
    }

    fn kind(&self) -> alloy_primitives::TxKind {
        delegate!(self => tx.kind())
    }

    fn is_create(&self) -> bool {
        delegate!(self => tx.is_create())
    }

    fn value(&self) -> alloy_primitives::U256 {
        delegate!(self => tx.value())
    }

    fn input(&self) -> &alloy_primitives::Bytes {
        delegate!(self => tx.input())
    }

    fn access_list(&self) -> Option<&alloy_eips::eip2930::AccessList> {
        delegate!(self => tx.access_list())
    }

    fn blob_versioned_hashes(&self) -> Option<&[B256]> {
        delegate!(self => tx.blob_versioned_hashes())
    }

    fn authorization_list(&self) -> Option<&[alloy_eips::eip7702::SignedAuthorization]> {
        delegate!(self => tx.authorization_list())
    }
}

impl SignableTransaction<Signature> for Transaction {
    fn set_chain_id(&mut self, chain_id: alloy_primitives::ChainId) {
        delegate!(self => tx.set_chain_id(chain_id))
    }

    fn encode_for_signing(&self, out: &mut dyn alloy_rlp::BufMut) {
        delegate!(self => tx.encode_for_signing(out))
    }

    fn payload_len_for_signature(&self) -> usize {
        delegate!(self => tx.payload_len_for_signature())
    }

    fn into_signed(self, signature: Signature) -> Signed<Self> {
        let tx_hash = delegate!(&self => tx.tx_hash(&signature));
        Signed::new_unchecked(self, signature, tx_hash)
    }
}

impl InMemorySize for Transaction {
    fn size(&self) -> usize {
        delegate!(self => tx.size())
    }
}

/// Signed Ethereum transaction.
#[derive(Debug, Clone, Eq, derive_more::AsRef, derive_more::Deref)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
// #[cfg_attr(any(test, feature = "reth-codec"), reth_codecs::add_arbitrary_tests(rlp))]
#[cfg_attr(feature = "test-utils", derive(derive_more::DerefMut))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct TransactionSigned {
    /// Transaction hash
    #[cfg_attr(feature = "serde", serde(skip))]
    hash: OnceLock<TxHash>,
    /// The transaction signature values
    signature: Signature,
    /// Raw transaction info
    #[deref]
    #[as_ref]
    #[cfg_attr(feature = "test-utils", deref_mut)]
    transaction: Transaction,
}

impl Default for TransactionSigned {
    fn default() -> Self {
        Self::new_unhashed(
            Transaction::Legacy(Default::default()),
            Signature::test_signature(),
        )
    }
}

impl TransactionSigned {
    fn recalculate_hash(&self) -> B256 {
        keccak256(self.encoded_2718())
    }

    /// Returns the signature of the transaction
    pub fn signature(&self) -> &Signature {
        &self.signature
    }
}

impl Hash for TransactionSigned {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.signature.hash(state);
        self.transaction.hash(state);
    }
}

impl PartialEq for TransactionSigned {
    fn eq(&self, other: &Self) -> bool {
        self.signature == other.signature
            && self.transaction == other.transaction
            && self.tx_hash() == other.tx_hash()
    }
}

impl TransactionSigned {
    /// Creates a new signed transaction from the given transaction, signature and hash.
    pub fn new(transaction: Transaction, signature: Signature, hash: B256) -> Self {
        Self {
            hash: hash.into(),
            signature,
            transaction,
        }
    }

    /// Consumes the type and returns the transaction.
    #[inline]
    pub fn into_transaction(self) -> Transaction {
        self.transaction
    }

    /// Returns the transaction.
    #[inline]
    pub const fn transaction(&self) -> &Transaction {
        &self.transaction
    }

    /// Returns the transaction hash.
    #[inline]
    pub fn hash(&self) -> &B256 {
        self.hash.get_or_init(|| self.recalculate_hash())
    }

    /// Creates a new signed transaction from the given transaction and signature without the hash.
    ///
    /// Note: this only calculates the hash on the first [`TransactionSigned::hash`] call.
    pub fn new_unhashed(transaction: Transaction, signature: Signature) -> Self {
        Self {
            hash: Default::default(),
            signature,
            transaction,
        }
    }

    /// Splits the `TransactionSigned` into its transaction and signature.
    pub fn split(self) -> (Transaction, Signature) {
        (self.transaction, self.signature)
    }

    /// Converts from an EIP-4844 transaction to a [`PooledTransaction`] with the given sidecar.
    ///
    /// Returns an `Err` containing the original `TransactionSigned` if the transaction is not
    /// EIP-4844.
    pub fn try_into_pooled_eip4844(
        self,
        sidecar: BlobTransactionSidecar,
    ) -> Result<PooledTransaction, Self> {
        let hash = *self.tx_hash();
        Ok(match self {
            // If the transaction is an EIP-4844 transaction...
            Self {
                transaction: Transaction::Eip4844(tx),
                signature,
                ..
            } => {
                // Construct a pooled eip488 tx with the provided sidecar.
                PooledTransaction::Eip4844(Signed::new_unchecked(
                    TxEip4844WithSidecar { tx, sidecar },
                    signature,
                    hash,
                ))
            }
            // If the transaction is not EIP-4844, return an error with the original
            // transaction.
            _ => return Err(self),
        })
    }

    /// Returns the [`TxEip4844`] if the transaction is an EIP-4844 transaction.
    pub const fn as_eip4844(&self) -> Option<&TxEip4844> {
        match &self.transaction {
            Transaction::Eip4844(tx) => Some(tx),
            _ => None,
        }
    }

    /// Provides mutable access to the transaction.
    #[cfg(feature = "test-utils")]
    pub fn transaction_mut(&mut self) -> &mut Transaction {
        &mut self.transaction
    }

    /// Splits the transaction into parts.
    pub fn into_parts(self) -> (Transaction, Signature, B256) {
        let hash = *self.hash.get_or_init(|| self.recalculate_hash());
        (self.transaction, self.signature, hash)
    }
}

impl Typed2718 for TransactionSigned {
    fn ty(&self) -> u8 {
        self.transaction.ty()
    }
}

impl alloy_consensus::Transaction for TransactionSigned {
    fn chain_id(&self) -> Option<ChainId> {
        self.transaction.chain_id()
    }

    fn nonce(&self) -> u64 {
        self.transaction.nonce()
    }

    fn gas_limit(&self) -> u64 {
        self.transaction.gas_limit()
    }

    fn gas_price(&self) -> Option<u128> {
        self.transaction.gas_price()
    }

    fn max_fee_per_gas(&self) -> u128 {
        self.transaction.max_fee_per_gas()
    }

    fn max_priority_fee_per_gas(&self) -> Option<u128> {
        self.transaction.max_priority_fee_per_gas()
    }

    fn max_fee_per_blob_gas(&self) -> Option<u128> {
        self.transaction.max_fee_per_blob_gas()
    }

    fn priority_fee_or_price(&self) -> u128 {
        self.transaction.priority_fee_or_price()
    }

    fn effective_gas_price(&self, base_fee: Option<u64>) -> u128 {
        self.transaction.effective_gas_price(base_fee)
    }

    fn is_dynamic_fee(&self) -> bool {
        self.transaction.is_dynamic_fee()
    }

    fn kind(&self) -> TxKind {
        self.transaction.kind()
    }

    fn is_create(&self) -> bool {
        self.transaction.is_create()
    }

    fn value(&self) -> U256 {
        self.transaction.value()
    }

    fn input(&self) -> &Bytes {
        self.transaction.input()
    }

    fn access_list(&self) -> Option<&AccessList> {
        self.transaction.access_list()
    }

    fn blob_versioned_hashes(&self) -> Option<&[B256]> {
        self.transaction.blob_versioned_hashes()
    }

    fn authorization_list(&self) -> Option<&[SignedAuthorization]> {
        self.transaction.authorization_list()
    }
}

impl InMemorySize for TransactionSigned {
    fn size(&self) -> usize {
        let Self {
            hash: _,
            signature,
            transaction,
        } = self;
        self.tx_hash().size() + signature.size() + transaction.size()
    }
}

impl Encodable2718 for TransactionSigned {
    fn type_flag(&self) -> Option<u8> {
        (!self.transaction.is_legacy()).then(|| self.ty())
    }

    fn encode_2718_len(&self) -> usize {
        delegate!(&self.transaction => tx.eip2718_encoded_length(&self.signature))
    }

    fn encode_2718(&self, out: &mut dyn alloy_rlp::BufMut) {
        delegate!(&self.transaction => tx.eip2718_encode(&self.signature, out))
    }

    fn trie_hash(&self) -> B256 {
        *self.tx_hash()
    }
}

impl Decodable2718 for TransactionSigned {
    fn typed_decode(ty: u8, buf: &mut &[u8]) -> Eip2718Result<Self> {
        match ty
            .try_into()
            .map_err(|_| Eip2718Error::UnexpectedType(ty))?
        {
            TxType::Legacy => Err(Eip2718Error::UnexpectedType(0)),
            TxType::Eip2930 => {
                let (tx, signature) = TxEip2930::rlp_decode_with_signature(buf)?;
                Ok(Self {
                    transaction: Transaction::Eip2930(tx),
                    signature,
                    hash: Default::default(),
                })
            }
            TxType::Eip1559 => {
                let (tx, signature) = TxEip1559::rlp_decode_with_signature(buf)?;
                Ok(Self {
                    transaction: Transaction::Eip1559(tx),
                    signature,
                    hash: Default::default(),
                })
            }
            TxType::Eip4844 => {
                let (tx, signature) = TxEip4844::rlp_decode_with_signature(buf)?;
                Ok(Self {
                    transaction: Transaction::Eip4844(tx),
                    signature,
                    hash: Default::default(),
                })
            }
            TxType::Eip7702 => {
                let (tx, signature) = TxEip7702::rlp_decode_with_signature(buf)?;
                Ok(Self {
                    transaction: Transaction::Eip7702(tx),
                    signature,
                    hash: Default::default(),
                })
            }
        }
    }

    fn fallback_decode(buf: &mut &[u8]) -> Eip2718Result<Self> {
        let (tx, signature) = TxLegacy::rlp_decode_with_signature(buf)?;
        Ok(Self {
            transaction: Transaction::Legacy(tx),
            signature,
            hash: Default::default(),
        })
    }
}

impl Encodable for TransactionSigned {
    fn encode(&self, out: &mut dyn alloy_rlp::BufMut) {
        self.network_encode(out);
    }

    fn length(&self) -> usize {
        self.network_len()
    }
}

impl Decodable for TransactionSigned {
    fn decode(buf: &mut &[u8]) -> alloy_rlp::Result<Self> {
        Self::network_decode(buf).map_err(Into::into)
    }
}

impl SignedTransaction for TransactionSigned {
    fn tx_hash(&self) -> &TxHash {
        self.hash.get_or_init(|| self.recalculate_hash())
    }

    fn recover_signer(&self) -> Result<Address, RecoveryError> {
        let signature_hash = self.signature_hash();
        recover_signer(&self.signature, signature_hash)
    }

    fn recover_signer_unchecked_with_buf(
        &self,
        buf: &mut Vec<u8>,
    ) -> Result<Address, RecoveryError> {
        self.encode_for_signing(buf);
        let signature_hash = keccak256(buf);
        recover_signer_unchecked(&self.signature, signature_hash)
    }
}
