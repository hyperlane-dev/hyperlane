use super::*;

#[derive(CustomDebug, Data, DisplayDebug)]
pub struct Stream {
    /// The bidirectional byte pipe the request/response cycle runs over.
    ///
    /// The concrete type is erased behind `Transport`, so a plain TCP socket, a
    /// TLS session and a per-request duplex stream are interchangeable here.
    /// Address metadata comes from the transport itself, so a transport without
    /// a socket identity reports an error instead of a fabricated address.
    pub stream: Box<dyn Transport>,
    #[get_mut(pub(super))]
    #[set(pub(super))]
    pub(super) request_config: RequestConfig,
    #[get(type(copy))]
    #[get_mut(pub(super))]
    pub(super) closed: bool,
    /// Bytes read ahead of the current request, already consumed from the
    /// transport but not yet parsed into a request.
    ///
    /// A single read can deliver bytes belonging to the next pipelined
    /// request. Those bytes are carried here between requests so
    /// HTTP pipelining is not silently dropped.
    #[get_mut(pub(super))]
    #[set(pub(super))]
    pub(super) leftover: Vec<u8>,
    /// Offset of the first unconsumed byte in `leftover`.
    #[get(type(copy))]
    #[get_mut(pub(super))]
    #[set(pub(super))]
    pub(super) leftover_start: usize,
    /// Offset one past the last valid byte in `leftover`.
    #[get(type(copy))]
    #[get_mut(pub(super))]
    #[set(pub(super))]
    pub(super) leftover_end: usize,
}

/// A buffered reader over a transport with a reusable read buffer.
///
/// This reader batches reads into an internal buffer to reduce syscall count,
/// and returns its buffer to a thread-local pool on drop so keep-alive
/// requests avoid repeated buffer allocation.
#[derive(Data, New)]
pub(crate) struct PooledReader<'a> {
    /// The underlying transport being read.
    #[get(pub(super))]
    #[get_mut(pub(super))]
    #[set(pub(super))]
    pub(super) stream: &'a mut Box<dyn Transport>,
    /// The reusable read buffer holding unconsumed bytes in `start..end`.
    #[get(pub(super))]
    #[get_mut(pub(super))]
    #[set(pub(super))]
    pub(super) buffer: Vec<u8>,
    /// The index of the first unconsumed byte in the buffer.
    #[get(pub(super), type(copy))]
    #[get_mut(pub(super))]
    #[set(pub(super))]
    #[new(skip)]
    pub(super) start: usize,
    /// The index one past the last valid byte in the buffer.
    #[get(pub(super), type(copy))]
    #[get_mut(pub(super))]
    #[set(pub(super))]
    #[new(skip)]
    pub(super) end: usize,
}
