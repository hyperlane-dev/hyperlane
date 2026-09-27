use super::*;

#[derive(CustomDebug, Data, DisplayDebug, New)]
pub struct Stream {
    #[get_mut(pub(super))]
    #[set(pub(super))]
    pub(super) stream: TcpStream,
    #[get_mut(pub(super))]
    #[set(pub(super))]
    pub(super) request_config: RequestConfig,
    #[get(type(copy))]
    #[get_mut(pub(super))]
    pub(super) closed: bool,
}

/// A buffered reader over a `TcpStream` with a reusable read buffer.
///
/// This reader batches socket reads into an internal buffer to reduce
/// syscall count, and returns its buffer to a thread-local pool on drop
/// so keep-alive requests avoid repeated buffer allocation.
#[derive(Data, New)]
pub(crate) struct PooledReader<'a> {
    /// The underlying TCP stream being read.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(super) stream: &'a mut TcpStream,
    /// The reusable read buffer holding unconsumed bytes in `start..end`.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(super) buffer: Vec<u8>,
    /// The index of the first unconsumed byte in the buffer.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    #[new(skip)]
    pub(super) start: usize,
    /// The index one past the last valid byte in the buffer.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    #[new(skip)]
    pub(super) end: usize,
}
