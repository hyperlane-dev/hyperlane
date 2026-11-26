use super::*;

/// Boxed `Send` future returned by an async unary gRPC service handler.
pub type GrpcFuture = Pin<Box<dyn Future<Output = Result<Vec<u8>, GrpcError>> + Send>>;

/// Thread-safe async unary gRPC handler taking decoded request payload bytes.
pub type GrpcHandler = Arc<dyn Fn(Vec<u8>) -> GrpcFuture + Send + Sync>;
