use super::*;

/// Shared mutable handle to an HTTP/2 direct framing state.
pub(crate) type H2DirectStateHandle = Arc<Mutex<H2DirectState>>;

/// One pooled boxed `Stream` paired with its direct HTTP/2 state handle.
pub(crate) type PooledH2Stream = (Stream, H2DirectStateHandle);
