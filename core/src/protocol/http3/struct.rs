use super::*;

/// Frames a response onto an HTTP/3 request stream as the pump produces it.
///
/// The framing code moves the QUIC stream out of the field with `take` before
/// it can write on it, so the stream field is reached through its mutable
/// accessor rather than a shared one.
#[derive(Data)]
pub(crate) struct H3Framer {
    /// The request stream the frames are written to.
    #[get(skip)]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(super) request_stream: Option<RequestH3Stream>,
    /// Whether the end of stream has already been sent.
    #[get(type(copy), pub(crate))]
    #[get_mut(skip)]
    #[set(pub(crate))]
    pub(super) finished: bool,
}

/// The environment one HTTP/3 request stream is served in.
///
/// A request stream needs the server that serves it, the limits and read timeout
/// its head and body are bounded by, and the addresses the duplex transport
/// reports to the framework. Carried together they stop being the same four
/// parameters repeated in the same order at every layer of the HTTP/3 path.
///
/// Every field but the server handle is `Copy`. The handle is an `Arc` the
/// connection loop already cloned, so passing the environment by value moves that
/// handle instead of cloning it again.
#[derive(Clone, Debug)]
pub(crate) struct H3RequestEnv {
    /// The server whose pipeline serves the request.
    pub(crate) server: Arc<Server>,
    /// The parsing limits and read timeout for the connection.
    pub(crate) request_config: RequestConfig,
    /// The addresses of the connection carrying the request.
    pub(crate) peers: StreamPeers,
}
