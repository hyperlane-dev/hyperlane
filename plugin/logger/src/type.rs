use super::*;

/// Thread-safe shared reference to a custom [`LogSink`].
pub type SharedLogSink = Arc<dyn LogSink>;
