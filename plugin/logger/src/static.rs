use super::*;

/// The configuration the plugin was given, when it was given one.
pub(super) static CONFIG: OnceLock<LoggerConfig> = OnceLock::new();

/// The file the log is appended to, when the configuration named one.
pub(super) static FILE: OnceLock<Mutex<Option<File>>> = OnceLock::new();
