use super::*;

/// Process-wide pool of idle asynchronous HTTP/1.1 keep-alive connections.
pub(crate) static ASYNC_H1_POOL: OnceLock<Mutex<AsyncH1PoolMap>> = OnceLock::new();

/// Process-wide pool of idle synchronous HTTP/1.1 keep-alive connections.
pub(crate) static SYNC_H1_POOL: OnceLock<Mutex<SyncH1PoolMap>> = OnceLock::new();

/// Process-wide pool of multiplexed asynchronous HTTP/2 connection senders.
pub(crate) static ASYNC_H2_POOL: OnceLock<Mutex<AsyncH2PoolMap>> = OnceLock::new();

thread_local! {
    /// Thread-local pool of reusable byte buffers avoiding per-request response allocations.
    pub(crate) static CLIENT_BUFFER_POOL: RefCell<Vec<Vec<u8>>> = const { RefCell::new(Vec::new()) };
}
