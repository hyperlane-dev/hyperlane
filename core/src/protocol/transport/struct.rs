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
