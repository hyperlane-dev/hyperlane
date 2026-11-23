use super::*;

/// The configuration the compress middleware works from, when one was given.
pub(super) static CONFIG: OnceLock<CompressConfig> = OnceLock::new();
