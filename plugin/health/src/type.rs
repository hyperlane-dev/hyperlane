use super::*;

/// Boxed `Send` future returned by an async [`HealthCheck`].
pub type HealthFuture<'a> = Pin<Box<dyn Future<Output = CheckOutcome> + Send + 'a>>;

/// Thread-safe shared reference to a pluggable [`HealthCheck`].
pub type SharedHealthCheck = Arc<dyn HealthCheck>;
