use super::*;

/// Global initialized [`SsePlugin`] instance.
pub(crate) static SSE_PLUGIN: OnceLock<SsePlugin> = OnceLock::new();
