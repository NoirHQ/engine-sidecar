//! Commonly used code snippets

use crate::error::{EthApiError, EthResult};
use alloy_consensus::transaction::Recovered;
use reth_primitives_traits::transaction::signed::SignedTransaction;

/// Recovers a [`SignedTransaction`] from an enveloped encoded byte stream.
///
/// This is a helper function that returns the appropriate RPC-specific error if the input data is
/// malformed.
///
/// See [`alloy_eips::eip2718::Decodable2718::decode_2718`]
pub fn recover_raw_transaction<T: SignedTransaction>(mut data: &[u8]) -> EthResult<Recovered<T>> {
    if data.is_empty() {
        return Err(EthApiError::EmptyRawTransactionData);
    }

    let transaction =
        T::decode_2718(&mut data).map_err(|_| EthApiError::FailedToDecodeSignedTransaction)?;

    transaction
        .try_into_recovered()
        .or(Err(EthApiError::InvalidTransactionSignature))
}
