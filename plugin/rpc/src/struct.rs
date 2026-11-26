use super::*;

/// JSON-RPC 2.0 error object returned inside [`RpcResponse`].
#[derive(Clone, Data, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct RpcError {
    /// Numeric JSON-RPC 2.0 error code.
    #[get(type(copy))]
    pub(crate) code: i64,
    /// Human-readable error summary.
    pub(crate) message: String,
    /// Optional structured error details.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[get(type(clone))]
    pub(crate) data: Option<Value>,
}

/// JSON-RPC 2.0 incoming request envelope.
#[derive(Clone, Data, Debug, Deserialize, PartialEq, Serialize)]
pub struct RpcRequest {
    /// Protocol version string (must equal `"2.0"`).
    pub(crate) jsonrpc: String,
    /// Target RPC method name.
    pub(crate) method: String,
    /// Optional positional array or named object parameters.
    #[serde(default)]
    #[get(type(clone))]
    pub(crate) params: Option<Value>,
    /// Correlation identifier (`None` marks a JSON-RPC notification expecting no reply).
    #[serde(default)]
    #[get(type(clone))]
    pub(crate) id: Option<RpcId>,
}

/// JSON-RPC 2.0 outgoing response envelope.
#[derive(Clone, Data, Debug, Deserialize, PartialEq, Serialize)]
pub struct RpcResponse {
    /// Protocol version string (`"2.0"`).
    pub(crate) jsonrpc: String,
    /// Success result value (`None` on error).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[get(type(clone))]
    pub(crate) result: Option<Value>,
    /// Structured error object (`None` on success).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[get(type(clone))]
    pub(crate) error: Option<RpcError>,
    /// Echoed correlation identifier (`RpcId::Null` for parse/invalid-request errors).
    pub(crate) id: RpcId,
}

/// Configuration for [`RpcRouter`] dispatch and [`RpcHook`] HTTP endpoint handling.
#[derive(Clone, Data, Debug, Eq, PartialEq)]
pub struct RpcConfig {
    /// HTTP path matched by [`RpcHook`] (default `"/rpc"`).
    pub(crate) endpoint: String,
    /// Maximum incoming JSON body size in bytes.
    #[get(type(copy))]
    pub(crate) max_body_size: usize,
    /// Maximum batch array length allowed per HTTP request.
    #[get(type(copy))]
    pub(crate) max_batch_size: usize,
}

/// Thread-safe registry mapping JSON-RPC method names to async handlers.
#[derive(Clone, Default)]
pub struct RpcRouter {
    /// Registered method handlers keyed by method name.
    pub(crate) methods: HashMap<String, RpcHandler>,
}

/// Process-wide [`RpcPlugin`] registry holding [`RpcConfig`] and an immutable [`RpcRouter`].
#[derive(Clone)]
pub struct RpcPlugin {
    /// Active JSON-RPC server configuration.
    pub(crate) config: RpcConfig,
    /// Registered JSON-RPC method router.
    pub(crate) router: RpcRouter,
}

/// Opt-in [`ServerHook`] dispatching HTTP `POST` requests on the configured JSON-RPC endpoint.
#[derive(Clone, Copy, Debug, Default)]
pub struct RpcHook;
