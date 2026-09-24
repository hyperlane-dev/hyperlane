use super::*;

/// Asynchronous proxy tunnel stream wrapper.
///
/// Provides unified interface for different proxy implementations.
pub struct ProxyTunnelStream {
    /// The underlying asynchronous read/write stream.
    pub(super) inner: BoxAsyncReadWrite,
    /// Pre-read data from the stream.
    pub(super) pre_read_data: Vec<u8>,
}

/// Synchronous proxy tunnel stream wrapper.
///
/// Provides unified interface for different proxy implementations.
pub struct SyncProxyTunnelStream {
    /// The underlying synchronous read/write stream.
    pub(super) inner: BoxReadWrite,
    /// Pre-read data from the stream.
    pub(super) pre_read_data: Vec<u8>,
}
