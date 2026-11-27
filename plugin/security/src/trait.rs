use super::*;

/// Extensible request inspection rule evaluated before routing.
pub trait SecurityRule: Send + Sync + 'static {
    /// Inspects `ctx` and returns `Err(SecurityViolation)` to block the request immediately.
    ///
    /// # Arguments
    ///
    /// - `&Context` - Incoming request context.
    ///
    /// # Returns
    ///
    /// - `Result<(), SecurityViolation>` - `Ok(())` to permit, or violation details to reject.
    fn inspect(&self, ctx: &Context) -> Result<(), SecurityViolation>;
}
