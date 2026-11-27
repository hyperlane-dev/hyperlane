use super::*;

/// Global process-wide [`TracingPlugin`] singleton initialized via [`TracingPlugin::init`].
pub(crate) static TRACING_PLUGIN: OnceLock<TracingPlugin> = OnceLock::new();

/// Lock-free monotonic counter feeding non-blocking trace and span ID generation.
pub(crate) static TRACE_SEQ: AtomicU64 = AtomicU64::new(1);

/// Process-unique entropy seed initialized once on first ID generation.
pub(crate) static TRACE_SEED: OnceLock<u64> = OnceLock::new();
