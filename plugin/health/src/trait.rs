use super::*;

/// Pluggable asynchronous health check probe (database, Redis, disk, upstream, etc.).
pub trait HealthCheck: Send + Sync + 'static {
    /// Returns the stable unique name of this dependency probe.
    ///
    /// # Returns
    ///
    /// - `&str` - Probe name included in the JSON report.
    fn name(&self) -> &str;

    /// Executes the health probe asynchronously.
    ///
    /// # Returns
    ///
    /// - `HealthFuture<'_>` - Future resolving to a [`CheckOutcome`].
    fn check(&self) -> HealthFuture<'_>;
}
