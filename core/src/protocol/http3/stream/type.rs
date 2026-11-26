use super::*;

/// Shared mutable handle to an HTTP/3 direct framing state.
pub(crate) type H3DirectStateHandle = Arc<Mutex<H3DirectState>>;

/// One pooled boxed `Stream` paired with its direct HTTP/3 state handle.
pub(crate) type PooledH3Stream = (Stream, H3DirectStateHandle);
