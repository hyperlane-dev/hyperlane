/// JSON-RPC 2.0 protocol version literal.
pub const JSONRPC_VERSION: &str = "2.0";

/// Default HTTP endpoint path handled by [`crate::RpcHook`].
pub const DEFAULT_RPC_ENDPOINT: &str = "/rpc";

/// Default maximum request body size accepted by [`crate::RpcHook`] (1 MiB).
pub const DEFAULT_RPC_MAX_BODY_SIZE: usize = 1024 * 1024;

/// Default maximum number of items permitted in one JSON-RPC batch array.
pub const DEFAULT_RPC_MAX_BATCH_SIZE: usize = 64;

/// JSON-RPC 2.0 Parse Error code.
pub const RPC_PARSE_ERROR: i64 = -32700;

/// JSON-RPC 2.0 Invalid Request code.
pub const RPC_INVALID_REQUEST: i64 = -32600;

/// JSON-RPC 2.0 Method Not Found code.
pub const RPC_METHOD_NOT_FOUND: i64 = -32601;

/// JSON-RPC 2.0 Invalid Params code.
pub const RPC_INVALID_PARAMS: i64 = -32602;

/// JSON-RPC 2.0 Internal Error code.
pub const RPC_INTERNAL_ERROR: i64 = -32603;

/// Standard Parse Error message.
pub(crate) const MSG_PARSE_ERROR: &str = "Parse error";

/// Standard Invalid Request message.
pub(crate) const MSG_INVALID_REQUEST: &str = "Invalid Request";

/// Standard Method Not Found message.
pub(crate) const MSG_METHOD_NOT_FOUND: &str = "Method not found";

/// Standard Invalid Params message.
pub(crate) const MSG_INVALID_PARAMS: &str = "Invalid params";

/// Standard Internal Error message.
pub(crate) const MSG_INTERNAL_ERROR: &str = "Internal error";

/// Error message when a batch exceeds `max_batch_size`.
pub(crate) const MSG_BATCH_TOO_LARGE: &str = "Batch size exceeds configured limit";
