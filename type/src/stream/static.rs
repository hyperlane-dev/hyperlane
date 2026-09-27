use super::*;

thread_local! {
    /// Thread-local pool of reusable read buffers for `PooledReader`.
    pub(crate) static READ_BUFFER_POOL: RefCell<Vec<Vec<u8>>> = const { RefCell::new(Vec::new()) };
}
