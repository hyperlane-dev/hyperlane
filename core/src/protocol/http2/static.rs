use super::*;

thread_local! {
    /// Worker-local pool of boxed `Stream` wrappers paired with their reusable HTTP/2 state handles.
    pub(crate) static H2_STREAM_POOL: RefCell<Vec<PooledH2Stream>> =
        const { RefCell::new(Vec::new()) };
}
