use super::*;

/// Pluggable asynchronous cleanup hook executed in LIFO order during graceful shutdown.
pub trait ShutdownCallback: Send + Sync + 'static {
    /// Returns a human-readable identifier for this cleanup callback.
    ///
    /// # Returns
    ///
    /// - `&str` - Callback name recorded in [`ShutdownReport`].
    fn name(&self) -> &str;

    /// Executes cleanup work asynchronously during graceful shutdown.
    ///
    /// # Returns
    ///
    /// - `ShutdownFuture<'_>` - Future completing once cleanup finishes.
    fn on_shutdown(&self) -> ShutdownFuture<'_>;
}
