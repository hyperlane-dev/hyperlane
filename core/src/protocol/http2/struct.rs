use super::*;

/// Frames a response onto an HTTP/2 request stream as the pump produces it.
///
/// The framing code moves a stream half out of the field with `take` before it
/// can write on it, so both stream fields are reached through their mutable
/// accessor rather than a shared one.
#[derive(Data)]
pub(crate) struct H2Framer {
    /// The response half, used only for the head.
    #[get(skip)]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(super) send_response: Option<h2::server::SendResponse<Bytes>>,
    /// The stream half, used for every data frame.
    #[get(skip)]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(super) send_stream: Option<h2::SendStream<Bytes>>,
    /// Whether the end of stream has already been sent.
    #[get(type(copy), pub(crate))]
    #[get_mut(skip)]
    #[set(pub(crate))]
    pub(super) finished: bool,
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
#[derive(Clone, Debug)]
pub(crate) struct H2RequestEnv {
    /// The server whose pipeline serves the request.
    pub(crate) server: Arc<Server>,
    /// The parsing limits and read timeout for the connection.
    pub(crate) request_config: RequestConfig,
    /// The addresses of the connection carrying the request.
    pub(crate) peers: StreamPeers,
}
