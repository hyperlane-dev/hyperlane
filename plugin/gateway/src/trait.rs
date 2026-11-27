use super::*;

/// Pluggable interceptor invoked around upstream reverse-proxy forwarding.
pub trait GatewayInterceptor: Send + Sync + 'static {
    /// Runs before forwarding to `target_url`.
    ///
    /// Returning `false` short-circuits upstream dispatch so the interceptor may write its own response.
    ///
    /// # Arguments
    ///
    /// - `&mut Context` - Incoming request and outgoing response context.
    /// - `&str` - Resolved upstream URL about to be called.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` to continue forwarding, or `false` to short-circuit.
    fn before_forward(&self, _ctx: &mut Context, _target_url: &str) -> bool {
        true
    }

    /// Runs after the upstream response has been written into `ctx`.
    ///
    /// # Arguments
    ///
    /// - `&mut Context` - Request and populated response context.
    /// - `&str` - Upstream base URL that served the request.
    fn after_forward(&self, _ctx: &mut Context, _upstream_url: &str) {}
}
