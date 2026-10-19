use super::*;

/// A bidirectional byte pipe that a request/response cycle can run over.
///
/// The transport is the only part of the server that knows what is carrying
/// the bytes. Everything above this trait is protocol agnostic: a plain TCP
/// socket, a TLS session and a per-request duplex stream are the same thing to
/// the parsing pipeline once they are type erased behind `Transport`.
///
/// Implementations report their own socket metadata, so a transport without a
/// socket identity (a duplex pair, an in-memory pipe) answers with an error
/// instead of pretending to have an address.
pub trait Transport: AsyncRead + AsyncWrite + Debug + Unpin + Send + Sync {
    /// Reports the address of the remote end of the transport.
    ///
    /// # Returns
    ///
    /// - `Result<SocketAddr, io::Error>` - The remote address, or an error when the
    ///   transport has no socket identity.
    fn peer_addr(&self) -> Result<SocketAddr, io::Error>;

    /// Reports the address of the local end of the transport.
    ///
    /// # Returns
    ///
    /// - `Result<SocketAddr, io::Error>` - The local address, or an error when the
    ///   transport has no socket identity.
    fn local_addr(&self) -> Result<SocketAddr, io::Error>;
}
