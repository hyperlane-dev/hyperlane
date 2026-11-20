use super::*;

thread_local! {
    /// Worker-local pool of reusable record buffers for [`TlsTransport`].
    pub(crate) static TLS_BUFFER_POOL: RefCell<Vec<Vec<u8>>> = const { RefCell::new(Vec::new()) };
}
