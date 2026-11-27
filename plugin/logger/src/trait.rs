use super::*;

/// Pluggable output sink receiving formatted log records from [`LoggerPlugin`].
pub trait LogSink: Send + Sync + 'static {
    /// Receives one sanitized, formatted log line.
    ///
    /// # Arguments
    ///
    /// - `LevelFilter` - Log severity level of the record.
    /// - `&str` - Formatted single-line log payload.
    fn write_record(&self, level: LevelFilter, line: &str);

    /// Flushes any buffered records held by this sink.
    fn flush(&self) {}
}
