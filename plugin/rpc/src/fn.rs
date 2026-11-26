use super::*;

/// Encodes one [`RpcResponse`] into JSON bytes, falling back to an empty object on serialization error.
///
/// # Arguments
///
/// - `&RpcResponse` - Response envelope to serialize.
///
/// # Returns
///
/// - `Vec<u8>` - Serialized JSON payload bytes.
pub(crate) fn encode_rpc_response(response: &RpcResponse) -> Vec<u8> {
    to_vec(response).unwrap_or_default()
}
