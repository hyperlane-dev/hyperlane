use super::*;

/// The configuration the plugin was given, when it was given one.
pub(super) static CONFIG: OnceLock<StaticFilesConfig> = OnceLock::new();
