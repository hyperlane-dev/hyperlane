use super::*;

/// Internal state of a direct `quinn`-native HTTP/3 response transport.
#[derive(Data)]
pub(crate) struct H3DirectState {
    /// The underlying QUIC send stream owned directly without intermediate `h3` wrappers.
    #[get(skip)]
    #[get_mut(pub(super))]
    #[set(pub(super))]
    pub(super) send_stream: Option<QuicSendStream>,
    /// Reusable wire buffer holding any backpressured encoded HTTP/3 frame bytes.
    #[get(pub(super))]
    #[get_mut(pub(super))]
    #[set(pub(super))]
    pub(super) pending_wire: Vec<u8>,
    /// Offset of the first unwritten byte in `pending_wire`.
    #[get(pub(super), type(copy))]
    #[get_mut(pub(super))]
    #[set(pub(super))]
    pub(super) pending_wire_offset: usize,
    /// Buffered head bytes accumulated only when a response head spans multiple writes.
    #[get(pub(super))]
    #[get_mut(pub(super))]
    #[set(pub(super))]
    pub(super) pending_head: Vec<u8>,
    /// Whether the response HEADERS frame has already been encoded and queued.
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
    /// Total body bytes queued so far.
    #[get(pub(super), type(copy))]
    #[get_mut(pub(super))]
    #[set(pub(super))]
    pub(super) body_len: usize,
    /// Whether the QUIC send stream has been finished or reset.
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

/// Direct `Transport` implementation that encodes HTTP/1.1 bytes straight into
/// HTTP/3 `HEADERS` and `DATA` frames on a `quinn::SendStream` with zero async boxing.
#[derive(Data)]
pub(crate) struct H3DirectTransport {
    /// Exclusively owned framing state wrapped in a mutex to satisfy `Transport: Sync`.
    #[get(skip)]
    #[get_mut(pub(super))]
    #[set(pub(super))]
    pub(super) state: Mutex<H3DirectState>,
    /// The local and remote addresses of the carrying QUIC connection.
    #[get(pub(super), type(copy))]
    #[get_mut(pub(super))]
    #[set(pub(super))]
    pub(super) peers: StreamPeers,
}

/// Shared per-connection atomic flags tracking peer-initiated HTTP/3 critical streams.
#[derive(Default)]
pub(crate) struct H3ControlState {
    /// Set immediately upon recognizing the peer's unidirectional control stream (`0x00`).
    pub(super) control_seen: AtomicBool,
    /// Set immediately upon recognizing the peer's QPACK encoder stream (`0x02`).
    pub(super) qpack_enc_seen: AtomicBool,
    /// Set immediately upon recognizing the peer's QPACK decoder stream (`0x03`).
    pub(super) qpack_dec_seen: AtomicBool,
}

/// The environment one HTTP/3 request stream is served in.
#[derive(Clone)]
pub(crate) struct H3RequestEnv {
    /// Shared listener state containing the server and compiled inline pipeline.
    pub(crate) shared: Arc<ConnectionShared>,
    /// The addresses of the connection carrying the request.
    pub(crate) peers: StreamPeers,
}
