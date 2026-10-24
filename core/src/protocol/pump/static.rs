use super::*;

thread_local! {
    /// Thread-local pool of reusable body buffers.
    ///
    /// The body buffer is the largest buffer a response cycle allocates, and
    /// the multiplexed protocols allocate one per response. A pooled buffer
    /// keeps its capacity, so the second response on the same worker does not
    /// allocate one at all.
    pub(crate) static BODY_BUFFER_POOL: RefCell<Vec<Vec<u8>>> = const { RefCell::new(Vec::new()) };
}
