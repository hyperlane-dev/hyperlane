/// The name of the attribute that carries the moment a request arrived.
pub(crate) const ATTRIBUTE_STARTED_AT: &str = "hyperlane-plugin-logger-started-at";

/// The text that stands for a request that carries no path.
pub(crate) const PATH_UNKNOWN: &str = "-";

/// The text that stands for a request that carries no method.
pub(crate) const METHOD_UNKNOWN: &str = "-";

/// Error reported when the logger configuration was already installed.
pub(crate) const ERROR_CONFIG_ALREADY_SET: &str = "the logger configuration was already set";

/// Error reported when the log file slot was already installed.
pub(crate) const ERROR_FILE_ALREADY_SET: &str = "the log file was already set";

/// Error reported when a global `log` implementation was already installed.
pub(crate) const ERROR_LOGGER_ALREADY_INSTALLED: &str = "a logger was already installed";

/// Replacement sequence emitted for carriage returns in log records.
pub(crate) const ESCAPED_CR: &str = "\\r";

/// Replacement sequence emitted for line feeds in log records.
pub(crate) const ESCAPED_LF: &str = "\\n";

/// Replacement sequence emitted for ANSI escape bytes in log records.
pub(crate) const ESCAPED_ESC: &str = "\\x1b";
