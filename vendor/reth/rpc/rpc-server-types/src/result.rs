//! Additional helpers for converting errors.

/// Constructs an internal JSON-RPC error.
pub fn internal_rpc_err(msg: impl Into<String>) -> jsonrpsee_types::error::ErrorObject<'static> {
    rpc_err(jsonrpsee_types::error::INTERNAL_ERROR_CODE, msg, None)
}

/// Constructs a JSON-RPC error, consisting of `code`, `message` and optional `data`.
pub fn rpc_err(
    code: i32,
    msg: impl Into<String>,
    data: Option<&[u8]>,
) -> jsonrpsee_types::error::ErrorObject<'static> {
    jsonrpsee_types::error::ErrorObject::owned(
        code,
        msg.into(),
        data.map(|data| {
            jsonrpsee_core::to_json_raw_value(&alloy_primitives::hex::encode_prefixed(data))
                .expect("serializing String can't fail")
        }),
    )
}
