use super::*;

/// Boxed `Send` future returned by an async JSON-RPC 2.0 method handler.
pub type RpcFuture = Pin<Box<dyn Future<Output = Result<Value, RpcError>> + Send>>;

/// Thread-safe async JSON-RPC 2.0 method handler callback.
pub type RpcHandler = Arc<dyn Fn(Option<Value>) -> RpcFuture + Send + Sync>;

/// Thread-safe shared reference to a custom [`RpcInterceptor`].
pub type SharedRpcInterceptor = Arc<dyn RpcInterceptor>;

/// Pre-validated JSON-RPC call ready for async execution or immediate short-circuit reply.
pub(crate) type PreparedRpcCall =
    Result<(RpcHandler, Option<Value>, Option<RpcId>), Option<Box<RpcResponse>>>;
