use super::*;

/// Reuses the configured cryptographic entropy source without rebuilding a provider per CID.
pub(crate) static SHARD_SECURE_RANDOM: OnceLock<&'static dyn SecureRandom> = OnceLock::new();
