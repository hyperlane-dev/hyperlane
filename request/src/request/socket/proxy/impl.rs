use super::*;

impl WebSocketProxyTunnelStream {
    pub(crate) fn new(stream: BoxAsyncReadWrite) -> Self {
        Self { inner: stream }
    }
}

impl AsyncRead for WebSocketProxyTunnelStream {
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> Poll<std::io::Result<()>> {
        Pin::new(&mut self.inner).poll_read(cx, buf)
    }
}

impl AsyncWrite for WebSocketProxyTunnelStream {
    fn poll_write(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<Result<usize, std::io::Error>> {
        Pin::new(&mut self.inner).poll_write(cx, buf)
    }

    fn poll_flush(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Result<(), std::io::Error>> {
        Pin::new(&mut self.inner).poll_flush(cx)
    }

    fn poll_shutdown(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Result<(), std::io::Error>> {
        Pin::new(&mut self.inner).poll_shutdown(cx)
    }
}

impl Unpin for WebSocketProxyTunnelStream {}

impl Debug for WebSocketProxyTunnelStream {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WebSocketProxyTunnelStream")
            .field("inner", &"<BoxAsyncReadWrite>")
            .finish()
    }
}

impl SyncWebSocketProxyTunnelStream {
    #[allow(dead_code)]
    pub(crate) fn new(stream: BoxReadWrite) -> Self {
        Self { inner: stream }
    }
}

impl Read for SyncWebSocketProxyTunnelStream {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.inner.read(buf)
    }
}

impl Write for SyncWebSocketProxyTunnelStream {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.inner.write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.inner.flush()
    }
}
