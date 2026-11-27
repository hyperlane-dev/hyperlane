use super::*;

/// Boxed `Send` future returned by an async [`ShutdownCallback`].
pub type ShutdownFuture<'a> = Pin<Box<dyn Future<Output = ()> + Send + 'a>>;

/// Thread-safe shared reference to a pluggable [`ShutdownCallback`].
pub type SharedShutdownCallback = Arc<dyn ShutdownCallback>;
