use super::*;

pub(crate) struct TestTransport {
    stream: DuplexStream,
}

impl TestTransport {
    pub(crate) fn new(stream: DuplexStream) -> Self {
        Self { stream }
    }
}

impl Transport for TestTransport {
    fn peer_addr(&self) -> Result<SocketAddr, io::Error> {
        Err(ErrorKind::Unsupported.into())
    }

    fn local_addr(&self) -> Result<SocketAddr, io::Error> {
        Err(ErrorKind::Unsupported.into())
    }
}

impl AsyncRead for TestTransport {
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> Poll<io::Result<()>> {
        Pin::new(&mut self.get_mut().stream).poll_read(cx, buf)
    }
}

impl AsyncWrite for TestTransport {
    fn poll_write(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<io::Result<usize>> {
        Pin::new(&mut self.get_mut().stream).poll_write(cx, buf)
    }

    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        Pin::new(&mut self.get_mut().stream).poll_flush(cx)
    }

    fn poll_shutdown(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        Pin::new(&mut self.get_mut().stream).poll_shutdown(cx)
    }
}

impl Debug for TestTransport {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str("TestTransport")
    }
}
