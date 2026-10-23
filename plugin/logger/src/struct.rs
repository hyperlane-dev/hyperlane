use super::*;

/// The configuration of the logger.
#[derive(Clone, Debug)]
pub struct LoggerConfig {
    /// The highest level that is written.
    pub level: LevelFilter,
    /// The file the log is appended to, when there is one.
    pub file: Option<String>,
}

/// The plugin that writes the log of the server.
#[derive(Clone, Copy, Debug, Default)]
pub struct LoggerPlugin;

/// The middleware that notes when a request arrived.
#[derive(Clone, Copy, Debug, Default)]
#[hyperlane::request_middleware(100)]
pub struct RequestLogger;

/// The middleware that writes the line of an answered request.
#[derive(Clone, Copy, Debug, Default)]
#[hyperlane::response_middleware(100)]
pub struct ResponseLogger;
