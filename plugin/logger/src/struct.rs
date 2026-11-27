use super::*;

/// The configuration of the logger.
#[derive(Clone)]
pub struct LoggerConfig {
    /// The highest level that is written.
    pub(crate) level: LevelFilter,
    /// The file the log is appended to, when there is one.
    pub(crate) file: Option<String>,
    /// Whether formatted lines are also echoed to standard output.
    pub(crate) console: bool,
    /// Output line format (plain text or structured JSON).
    pub(crate) format: LogFormat,
    /// Bounded non-blocking channel capacity for background file writes.
    pub(crate) channel_capacity: usize,
    /// Optional custom extensible output sink invoked alongside built-in outputs.
    pub(crate) sink: Option<SharedLogSink>,
}

/// The plugin that writes the log of the server.
#[derive(Clone, Copy, Debug, Default)]
pub struct LoggerPlugin;

/// The middleware that notes when a request arrived.
#[derive(Clone, Copy, Debug, Default)]
#[hyperlane::request_middleware(-120)]
pub struct RequestLogger;

/// The middleware that writes the line of an answered request.
#[derive(Clone, Copy, Debug, Default)]
#[hyperlane::response_middleware(100)]
pub struct ResponseLogger;
