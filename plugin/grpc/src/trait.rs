use super::*;

/// Pluggable interceptor invoked before each gRPC unary service dispatch.
pub trait GrpcInterceptor: Send + Sync + 'static {
    /// Inspects or rejects an incoming gRPC call before the service handler runs.
    ///
    /// # Arguments
    ///
    /// - `&str` - Full gRPC method path (`/package.Service/Method`).
    ///
    /// # Returns
    ///
    /// - `Result<(), GrpcError>` - `Ok(())` to continue dispatch, or `Err(GrpcError)` to short-circuit.
    fn before_call(&self, path: &str) -> Result<(), GrpcError>;
}
