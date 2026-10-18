use super::*;

/// A transport that carries an HTTP/2 or HTTP/3 request stream to the pipeline.
///
/// HTTP/2 and HTTP/3 deliver a request as a bidirectional stream, and the
/// framework pipeline writes HTTP/1.1 shaped bytes onto it: a status line, a
/// header block, then body bytes. A duplex pipe joins the two halves: the
/// lifecycle writes into one end exactly as it would write to a socket, and a
/// concurrent pump reads the other end, frames the head once it is complete and
/// forwards every following chunk as a data frame. A handler that flushes
/// progressively therefore reaches the client before it returns, which a sink
/// that collected everything and framed it afterwards could not do.
///
/// The transport itself performs no framing. It only exposes the pipe to the
/// framework and reports the addresses of the connection carrying it.
#[derive(Data, Debug)]
pub(crate) struct DuplexTransport {
    /// The half the framework pipeline writes onto.
    #[get(skip)]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(super) pipe: DuplexStream,
    /// The local address of the connection carrying this stream.
    #[get(type(copy), pub(crate))]
    #[get_mut(skip)]
    #[set(pub(crate))]
    pub(super) local: Option<SocketAddr>,
    /// The remote address of the connection carrying this stream.
    #[get(type(copy), pub(crate))]
    #[get_mut(skip)]
    #[set(pub(crate))]
    pub(super) remote: Option<SocketAddr>,
}

/// Wraps a TLS session so it satisfies the framework transport contract.
///
/// An HTTP/1.1 request over TLS must present the same `Stream` surface as a
/// plaintext one, so the negotiated session is carried with the address
/// information the framework reads from it. The session is the real socket,
/// so keep alive, upgrades and raw writes behave exactly as they do in
/// plaintext.
#[derive(Data, Debug)]
pub(crate) struct TlsTransport {
    /// The negotiated TLS session over the accepted socket.
    #[get(skip)]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(super) session: tokio_rustls::server::TlsStream<TcpStream>,
}

/// The environment one accepted connection is served in.
///
/// Every protocol path needs the same three things: the server whose pipeline
/// runs each request, the parsing limits and read timeouts those requests are
/// bounded by, and the protocol this connection is served as. Carried as one
/// value they stop being the same three parameters repeated in the same order at
/// every layer.
///
/// The server handle is held in an `Arc` exactly as before: one clone per spawned
/// connection, and one per request task, which is the count the previous
/// signature already forced. `Clone` copies the handle and adds one to the number
/// of existing clones of that `Arc`. The other two fields are `Copy`, so they are
/// carried by value.
#[derive(Clone, Debug)]
pub(crate) struct ConnectionEnv {
    /// The server whose pipeline serves each request.
    pub(crate) server: Arc<Server>,
    /// The parsing limits and read timeouts for the connection.
    pub(crate) request_config: RequestConfig,
    /// The protocol served on this connection.
    pub(crate) protocol: ServerProtocol,
}

/// The addresses of the connection carrying one request stream.
///
/// The duplex transport reports these to the framework, so a handler that reads
/// them sees the real connection rather than a fabricated one. They are the same
/// two values on every protocol path and the duplex transport is always built
/// from both, so they travel together.
#[derive(Clone, Copy, Debug)]
pub(crate) struct StreamPeers {
    /// The local address of the carrying connection.
    pub(crate) local: SocketAddr,
    /// The remote address of the carrying connection.
    pub(crate) remote: SocketAddr,
}
