use super::*;

/// Implements [`Transport`] for [`DuplexTransport`].
impl Transport for DuplexTransport {
    /// Reports the address of the remote end of the carrying connection.
    ///
    /// # Arguments
    ///
    /// - `&Self` - The `value` argument.
    ///
    /// # Returns
    ///
    /// - `Result<SocketAddr, io::Error>` - The connection's remote address.
    fn peer_addr(&self) -> Result<SocketAddr, io::Error> {
        self.get_remote()
            .ok_or_else(|| io::Error::new(io::ErrorKind::Unsupported, NO_ADDRESS))
    }

    /// Reports the address of the local end of the carrying connection.
    ///
    /// # Arguments
    ///
    /// - `&Self` - The `value` argument.
    ///
    /// # Returns
    ///
    /// - `Result<SocketAddr, io::Error>` - The connection's local address.
    fn local_addr(&self) -> Result<SocketAddr, io::Error> {
        self.get_local()
            .ok_or_else(|| io::Error::new(io::ErrorKind::Unsupported, NO_ADDRESS))
    }
}

/// Implements [`AsyncRead`] for [`DuplexTransport`].
impl AsyncRead for DuplexTransport {
    /// Reads from the half the framing pump writes nothing into.
    ///
    /// The lifecycle never reads a response back, so the read half is drained to
    /// `EOF` rather than blocking the pipeline on a write that will never come.
    ///
    /// # Arguments
    ///
    /// - `Pin<&mut Self>` - The pinned reader.
    /// - `&mut task::Context<'_>` - The task context.
    /// - `&mut ReadBuf<'_>` - The buffer to read into.
    ///
    /// # Returns
    ///
    /// - `Poll<io::Result<()>>` - Ready with zero bytes once drained.
    fn poll_read(
        self: Pin<&mut Self>,
        _context: &mut task::Context<'_>,
        buffer: &mut ReadBuf<'_>,
    ) -> Poll<io::Result<()>> {
        buffer.advance(0);
        Poll::Ready(Ok(()))
    }
}

/// Implements [`AsyncWrite`] for [`DuplexTransport`].
impl AsyncWrite for DuplexTransport {
    /// Writes response bytes into the pipe the framing pump reads.
    ///
    /// # Arguments
    ///
    /// - `Pin<&mut Self>` - The pinned writer.
    /// - `&mut task::Context<'_>` - The task context.
    /// - `&[u8]` - The bytes to write.
    ///
    /// # Returns
    ///
    /// - `Poll<io::Result<usize>>` - Ready with the number of bytes written.
    fn poll_write(
        self: Pin<&mut Self>,
        context: &mut task::Context<'_>,
        bytes: &[u8],
    ) -> Poll<io::Result<usize>> {
        let this: &mut Self = self.get_mut();
        Pin::new(&mut this.get_mut_pipe()).poll_write(context, bytes)
    }

    /// Flushes the pipe, which the duplex pair propagates to the pump.
    ///
    /// # Arguments
    ///
    /// - `Pin<&mut Self>` - The pinned writer.
    /// - `&mut task::Context<'_>` - The task context.
    ///
    /// # Returns
    ///
    /// - `Poll<io::Result<()>>` - Ready once the pipe is flushed.
    fn poll_flush(self: Pin<&mut Self>, context: &mut task::Context<'_>) -> Poll<io::Result<()>> {
        let this: &mut Self = self.get_mut();
        Pin::new(&mut this.get_mut_pipe()).poll_flush(context)
    }

    /// Shuts the write half down, which the pump observes as end of stream.
    ///
    /// # Arguments
    ///
    /// - `Pin<&mut Self>` - The pinned writer.
    /// - `&mut task::Context<'_>` - The task context.
    ///
    /// # Returns
    ///
    /// - `Poll<io::Result<()>>` - Ready once the pipe is shut down.
    fn poll_shutdown(
        self: Pin<&mut Self>,
        context: &mut task::Context<'_>,
    ) -> Poll<io::Result<()>> {
        let this: &mut Self = self.get_mut();
        Pin::new(&mut this.get_mut_pipe()).poll_shutdown(context)
    }
}

/// Inherent implementation of [`TlsTransport`].
impl TlsTransport {
    /// Wraps a negotiated TLS session.
    ///
    /// # Arguments
    ///
    /// - `tokio_rustls::server::TlsStream<TcpStream>` - The negotiated session.
    ///
    pub(crate) fn new(session: tokio_rustls::server::TlsStream<TcpStream>) -> Self {
        Self { session }
    }
}

/// Inherent implementation of [`TlsTransport`].
impl TlsTransport {
    /// Borrows the negotiated session.
    ///
    /// The generated accessors cannot express this read: the transport only
    /// ever lends the session out, so it is handed back as a shared reference
    /// rather than by value, and a `&self` method has no mutable accessor to
    /// take it through.
    ///
    /// # Arguments
    ///
    /// - `&Self` - The `value` argument.
    ///
    /// # Returns
    ///
    /// - `&tokio_rustls::server::TlsStream<TcpStream>` - The negotiated session.
    pub(crate) fn get_session_ref(&self) -> &tokio_rustls::server::TlsStream<TcpStream> {
        &self.session
    }
}

/// Implements [`Transport`] for [`TlsTransport`].
impl Transport for TlsTransport {
    /// Reports the address of the remote end of the session.
    ///
    /// # Arguments
    ///
    /// - `&Self` - The `value` argument.
    ///
    /// # Returns
    ///
    /// - `Result<SocketAddr, io::Error>` - The peer address of the socket.
    fn peer_addr(&self) -> Result<SocketAddr, io::Error> {
        TcpStream::peer_addr(self.get_session_ref().get_ref().0)
    }

    /// Reports the address of the local end of the session.
    ///
    /// # Arguments
    ///
    /// - `&Self` - The `value` argument.
    ///
    /// # Returns
    ///
    /// - `Result<SocketAddr, io::Error>` - The local address of the socket.
    fn local_addr(&self) -> Result<SocketAddr, io::Error> {
        TcpStream::local_addr(self.get_session_ref().get_ref().0)
    }
}

/// Implements [`AsyncRead`] for [`TlsTransport`].
impl AsyncRead for TlsTransport {
    /// Reads decrypted bytes from the negotiated session.
    ///
    /// # Arguments
    ///
    /// - `Pin<&mut Self>` - The pinned reader.
    /// - `&mut task::Context<'_>` - The task context.
    /// - `&mut ReadBuf<'_>` - The buffer to read into.
    ///
    /// # Returns
    ///
    /// - `Poll<io::Result<()>>` - Ready with the bytes read.
    fn poll_read(
        self: Pin<&mut Self>,
        context: &mut task::Context<'_>,
        buffer: &mut ReadBuf<'_>,
    ) -> Poll<io::Result<()>> {
        let this: &mut Self = self.get_mut();
        Pin::new(&mut this.get_mut_session()).poll_read(context, buffer)
    }
}

/// Implements [`AsyncWrite`] for [`TlsTransport`].
impl AsyncWrite for TlsTransport {
    /// Writes encrypted bytes onto the negotiated session.
    ///
    /// # Arguments
    ///
    /// - `Pin<&mut Self>` - The pinned writer.
    /// - `&mut task::Context<'_>` - The task context.
    /// - `&[u8]` - The bytes to write.
    ///
    /// # Returns
    ///
    /// - `Poll<io::Result<usize>>` - Ready with the number of bytes written.
    fn poll_write(
        self: Pin<&mut Self>,
        context: &mut task::Context<'_>,
        bytes: &[u8],
    ) -> Poll<io::Result<usize>> {
        let this: &mut Self = self.get_mut();
        Pin::new(&mut this.get_mut_session()).poll_write(context, bytes)
    }

    /// Flushes the negotiated session.
    ///
    /// # Arguments
    ///
    /// - `Pin<&mut Self>` - The pinned writer.
    /// - `&mut task::Context<'_>` - The task context.
    ///
    /// # Returns
    ///
    /// - `Poll<io::Result<()>>` - Ready once the session is flushed.
    fn poll_flush(self: Pin<&mut Self>, context: &mut task::Context<'_>) -> Poll<io::Result<()>> {
        let this: &mut Self = self.get_mut();
        Pin::new(&mut this.get_mut_session()).poll_flush(context)
    }

    /// Shuts the negotiated session down.
    ///
    /// # Arguments
    ///
    /// - `Pin<&mut Self>` - The pinned writer.
    /// - `&mut task::Context<'_>` - The task context.
    ///
    /// # Returns
    ///
    /// - `Poll<io::Result<()>>` - Ready once the session is shut down.
    fn poll_shutdown(
        self: Pin<&mut Self>,
        context: &mut task::Context<'_>,
    ) -> Poll<io::Result<()>> {
        let this: &mut Self = self.get_mut();
        Pin::new(&mut this.get_mut_session()).poll_shutdown(context)
    }
}
