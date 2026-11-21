use super::*;

/// Wraps a negotiated TLS session driven through [`UnbufferedServerConnection`]
/// with worker-local pooled record buffers and zero-allocation in-place writes.
pub(crate) struct TlsTransport<T = TcpStream> {
    /// Underlying accepted stream transport (Tokio `TcpStream` or Linux `io_uring` transport).
    pub(super) tcp: T,
    /// Unbuffered rustls server state machine.
    pub(super) conn: UnbufferedServerConnection,
    /// Incoming encrypted TLS record buffer (`0..in_len` valid).
    pub(super) in_buf: Vec<u8>,
    /// Number of unread ciphertext bytes stored in `in_buf`.
    pub(super) in_len: usize,
    /// Decrypted plaintext bytes awaiting consumption by `poll_read`.
    pub(super) plain_buf: Vec<u8>,
    /// Cursor of the first unread byte in `plain_buf`.
    pub(super) plain_pos: usize,
    /// Outgoing encrypted TLS bytes awaiting transmission (`out_pos..out_len`).
    pub(super) out_buf: Vec<u8>,
    /// Cursor of the first untransmitted byte in `out_buf`.
    pub(super) out_pos: usize,
    /// Total valid encrypted bytes in `out_buf`.
    pub(super) out_len: usize,
    /// Whether the peer has sent `close_notify`.
    pub(super) peer_closed: bool,
    /// Whether this side has already queued `close_notify`.
    pub(super) write_closed: bool,
}

/// Shared immutable server state owned once per listener and cloned via a single `Arc`.
pub(crate) struct ConnectionShared {
    /// The server whose pipeline serves each request.
    pub(crate) server: Server,
    /// Pre-compiled inline hook dispatch table built once at server startup.
    pub(crate) pipeline: CompiledPipeline,
    /// The parsing limits and read timeouts for the connection.
    pub(crate) request_config: RequestConfig,
    /// The protocol served on this connection.
    pub(crate) protocol: ServerProtocol,
    /// Optional validated rustls server configuration for stream listeners.
    pub(crate) tls_config: Option<Arc<TlsServerConfig>>,
}

/// The server, compiled pipeline, limits, and protocol owned by an accepted connection.
#[derive(Clone)]
pub(crate) struct ConnectionEnv {
    /// Shared immutable listener state.
    pub(crate) shared: Arc<ConnectionShared>,
}

/// The real socket addresses of the connection carrying a request stream.
#[derive(Clone, Copy, Debug)]
pub(crate) struct StreamPeers {
    /// The local address of the carrying connection.
    pub(crate) local: SocketAddr,
    /// The remote address of the carrying connection.
    pub(crate) remote: SocketAddr,
}
