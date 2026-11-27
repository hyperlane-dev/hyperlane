use super::*;

/// Pluggable exporter receiving completed HTTP request spans.
pub trait SpanExporter: Send + Sync + 'static {
    /// Exports a completed [`SpanRecord`] synchronously (implementations should buffer or ship non-blockingly).
    ///
    /// # Arguments
    ///
    /// - `&SpanRecord` - Completed request span details.
    fn export(&self, span: &SpanRecord);
}
