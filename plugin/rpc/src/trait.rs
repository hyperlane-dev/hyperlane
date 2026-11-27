use super::*;

/// Pluggable interceptor invoked before each JSON-RPC 2.0 method dispatch.
pub trait RpcInterceptor: Send + Sync + 'static {
    /// Inspects or rejects an incoming JSON-RPC call before its method handler runs.
    ///
    /// # Arguments
    ///
    /// - `&str` - Target JSON-RPC method name.
    /// - `&Option<Value>` - Optional method parameters.
    ///
    /// # Returns
    ///
    /// - `Result<(), RpcError>` - `Ok(())` to continue dispatch, or `Err(RpcError)` to short-circuit.
    fn before_call(&self, method: &str, params: &Option<Value>) -> Result<(), RpcError>;
}
