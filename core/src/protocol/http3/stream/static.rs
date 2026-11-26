use super::*;

thread_local! {
    /// Worker-local pool of reusable byte buffers for HTTP/3 request reading and frame encoding.
    pub(crate) static H3_BUFFER_POOL: RefCell<Vec<Vec<u8>>> = const { RefCell::new(Vec::new()) };
    /// Worker-local pool of boxed `Stream` wrappers paired with their reusable HTTP/3 state handles.
    pub(crate) static H3_STREAM_POOL: RefCell<Vec<PooledH3Stream>> =
        const { RefCell::new(Vec::new()) };
}
