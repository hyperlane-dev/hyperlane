use super::*;

/// The configuration the plugin was given, when it was given one.
pub(super) static CONFIG: OnceLock<CompressionConfig> = OnceLock::new();

/// The rules the plugin answers with, when it was given a configuration.
pub(super) static POLICY: OnceLock<CompressionPolicy> = OnceLock::new();
