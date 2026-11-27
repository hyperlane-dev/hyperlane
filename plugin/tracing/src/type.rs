use super::*;

/// Thread-safe shared reference to a pluggable [`SpanExporter`].
pub type SharedSpanExporter = Arc<dyn SpanExporter>;
