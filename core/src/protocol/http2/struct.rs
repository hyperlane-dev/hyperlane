use super::*;

/// Internal state of a direct HTTP/2 response transport.
#[derive(Data, Debug)]
pub(crate) struct H2DirectState {
    /// The response half, used once to emit the HEADERS frame.
    #[get(skip)]
    #[get_mut(pub(super))]
    #[set(pub(super))]
    pub(super) send_response: Option<H2SendResponse>,
    /// The data stream half, used to emit DATA frames and EOS.
    #[get(skip)]
    #[get_mut(pub(super))]
    #[set(pub(super))]
    pub(super) send_stream: Option<H2SendStream>,
    /// Buffered bytes accumulated only when a response head spans multiple writes.
    #[get(pub(super))]
    #[get_mut(pub(super))]
    #[set(pub(super))]
    pub(super) pending_head: Vec<u8>,
    /// Buffered trailing body bytes extracted after a split head completes.
    #[get(pub(super))]
    #[get_mut(pub(super))]
    #[set(pub(super))]
    pub(super) pending_body: Vec<u8>,
    /// Read cursor inside `pending_body`.
    #[get(pub(super), type(copy))]
    #[get_mut(pub(super))]
    #[set(pub(super))]
    pub(super) pending_body_offset: usize,
    /// Whether the response head has already been emitted.
    #[get(pub(super), type(copy))]
    #[get_mut(pub(super))]
    #[set(pub(super))]
    pub(super) head_sent: bool,
    /// Whether body bytes must be suppressed (e.g. HEAD, 204, 304, 1xx).
    #[get(pub(super), type(copy))]
    #[get_mut(pub(super))]
    #[set(pub(super))]
    pub(super) suppress_body: bool,
    /// Whether the incoming request method was `HEAD`.
    #[get(pub(super), type(copy))]
    #[get_mut(pub(super))]
    #[set(pub(super))]
    pub(super) is_head: bool,
    /// The validated content length announced in the head, if any.
    #[get(pub(super), type(copy))]
    #[get_mut(pub(super))]
    #[set(pub(super))]
    pub(super) announced_len: Option<usize>,
    /// Total body bytes forwarded so far.
    #[get(pub(super), type(copy))]
    #[get_mut(pub(super))]
    #[set(pub(super))]
    pub(super) body_len: usize,
    /// Whether the end-of-stream flag has already been emitted.
    #[get(pub(super), type(copy))]
    #[get_mut(pub(super))]
    #[set(pub(super))]
    pub(super) finished: bool,
    /// Whether framing encountered an unrecoverable protocol error.
    #[get(pub(super), type(copy))]
    #[get_mut(pub(super))]
    #[set(pub(super))]
    pub(super) failed: bool,
    /// The local and remote addresses of the carrying TCP connection.
    #[get(pub(super), type(copy))]
    #[get_mut(pub(super))]
    #[set(pub(super))]
    pub(super) peers: StreamPeers,
}

/// Direct `Transport` implementation that frames written HTTP/1.1 bytes straight
/// onto an HTTP/2 stream without a duplex pipe or background pump task.
#[derive(Data, Debug)]
pub(crate) struct H2DirectTransport {
    /// Shared framing state recycled across requests via [`H2_STREAM_POOL`].
    #[get(pub(super))]
    #[get_mut(pub(super))]
    #[set(pub(super))]
    pub(super) state: H2DirectStateHandle,
}

/// RAII guard borrowing a pooled boxed HTTP/2 `Stream` and returning it on drop.
pub(crate) struct H2StreamGuard {
    /// Pooled `(Stream, state)` pair released back to [`H2_STREAM_POOL`] on drop.
    pub(super) inner: Option<PooledH2Stream>,
}
