/// Default bounded non-blocking log channel capacity for background file writes.
pub const DEFAULT_LOGGER_CHANNEL_CAPACITY: usize = 16_384;

/// Default buffer size in bytes for the background buffered file writer.
pub(crate) const LOGGER_FILE_BUFFER_BYTES: usize = 64 * 1024;

/// Background writer thread name.
pub(crate) const LOGGER_WRITER_THREAD_NAME: &str = "hyperlane-logger";

/// Debug struct label for [`crate::LoggerConfig`].
pub(crate) const DEBUG_LOGGER_CONFIG: &str = "LoggerConfig";

/// Debug field name for log level.
pub(crate) const DEBUG_FIELD_LEVEL: &str = "level";

/// Debug field name for output file path.
pub(crate) const DEBUG_FIELD_FILE: &str = "file";

/// Debug field name for console flag.
pub(crate) const DEBUG_FIELD_CONSOLE: &str = "console";

/// Debug field name for output format.
pub(crate) const DEBUG_FIELD_FORMAT: &str = "format";

/// Debug field name for channel capacity.
pub(crate) const DEBUG_FIELD_CHANNEL_CAPACITY: &str = "channel_capacity";

/// Debug field name indicating custom sink presence.
pub(crate) const DEBUG_FIELD_HAS_SINK: &str = "has_sink";

/// The name of the attribute that carries the moment a request arrived.
pub(crate) const ATTRIBUTE_STARTED_AT: &str = "hyperlane-plugin-logger-started-at";

/// The text that stands for a request that carries no path.
pub(crate) const PATH_UNKNOWN: &str = "-";

/// The text that stands for a request that carries no method.
pub(crate) const METHOD_UNKNOWN: &str = "-";

/// Error reported when the logger configuration was already installed.
pub(crate) const ERROR_CONFIG_ALREADY_SET: &str = "the logger configuration was already set";

/// Error reported when the log writer was already installed.
pub(crate) const ERROR_FILE_ALREADY_SET: &str = "the log file was already set";

/// Error reported when a global `log` implementation was already installed.
pub(crate) const ERROR_LOGGER_ALREADY_INSTALLED: &str = "a logger was already installed";

/// Replacement sequence emitted for carriage returns in log records.
pub(crate) const ESCAPED_CR: &str = "\\r";

/// Replacement sequence emitted for line feeds in log records.
pub(crate) const ESCAPED_LF: &str = "\\n";

/// Replacement sequence emitted for ANSI escape bytes in log records.
pub(crate) const ESCAPED_ESC: &str = "\\x1b";

/// Replacement sequence emitted for double quotes in JSON log records.
pub(crate) const ESCAPED_QUOTE: &str = "\\\"";

/// Replacement sequence emitted for backslashes in JSON log records.
pub(crate) const ESCAPED_BACKSLASH: &str = "\\\\";

/// Replacement sequence emitted for tabs in JSON log records.
pub(crate) const ESCAPED_TAB: &str = "\\t";

/// JSON record prefix before the log level field.
pub(crate) const JSON_PREFIX_LEVEL: &str = "{\"level\":\"";

/// JSON record separator between level and target fields.
pub(crate) const JSON_SEP_TARGET: &str = "\",\"target\":\"";

/// JSON record separator between target and message fields.
pub(crate) const JSON_SEP_MESSAGE: &str = "\",\"message\":\"";

/// JSON record closing suffix.
pub(crate) const JSON_SUFFIX: &str = "\"}";
