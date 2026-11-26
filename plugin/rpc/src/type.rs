use super::*;

/// Boxed `Send` future returned by an async JSON-RPC method handler.
pub type RpcFuture = Pin<Box<dyn Future<Output = Result<Value, RpcError>> + Send>>;

/// Thread-safe async JSON-RPC method callback.
pub type RpcHandler = Arc<dyn Fn(Option<Value>) -> RpcFuture + Send + Sync>;
