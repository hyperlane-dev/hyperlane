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
}

/// Direct `Transport` implementation that frames written HTTP/1.1 bytes straight
/// onto an HTTP/2 stream without a duplex pipe or background pump task.
#[derive(Data, Debug)]
pub(crate) struct H2DirectTransport {
    /// Exclusively owned framing state wrapped in a mutex to satisfy `Transport: Sync`.
    #[get(skip)]
    #[get_mut(pub(super))]
    #[set(pub(super))]
    pub(super) state: Mutex<H2DirectState>,
    /// The local and remote addresses of the carrying TCP connection.
    #[get(pub(super), type(copy))]
    #[get_mut(pub(super))]
    #[set(pub(super))]
    pub(super) peers: StreamPeers,
}

/// The environment one HTTP/2 request stream is served in.
///
/// A request stream needs the server that serves it, the limits and read timeout
/// its head and body are bounded by, and the addresses the duplex transport
/// reports to the framework. Carried together they stop being the same four
/// parameters repeated in the same order at every layer of the HTTP/2 path.
///
/// Every field but the server handle is `Copy`. The handle is an `Arc` the
/// connection loop already cloned, so passing the environment by value moves that
/// handle instead of cloning it again.
#[derive(Clone)]
pub(crate) struct H2RequestEnv {
    /// Shared listener state containing the server and compiled inline pipeline.
    pub(crate) shared: Arc<ConnectionShared>,
    /// The addresses of the connection carrying the request.
    pub(crate) peers: StreamPeers,
}
