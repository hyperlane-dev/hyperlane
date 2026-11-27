use super::*;

/// Global process-wide [`SessionPlugin`] singleton initialized via [`SessionPlugin::init`].
pub(crate) static SESSION_PLUGIN: OnceLock<SessionPlugin> = OnceLock::new();

/// Lock-free monotonic counter feeding high-entropy session ID generation.
pub(crate) static SESSION_SEQ: AtomicU64 = AtomicU64::new(1);

/// Process-unique seed initialized once on first session ID generation.
pub(crate) static SESSION_SEED: OnceLock<u64> = OnceLock::new();
