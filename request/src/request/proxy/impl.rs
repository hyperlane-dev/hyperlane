use super::*;

impl ProxyTunnelStream {
    pub(crate) fn new(stream: BoxAsyncReadWrite, pre_read_data: Vec<u8>) -> Self {
        Self {
            inner: stream,
            pre_read_data,
        }
    }
}

impl AsyncRead for ProxyTunnelStream {
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> Poll<std::io::Result<()>> {
        if !self.pre_read_data.is_empty() {
            let len: usize = std::cmp::min(self.pre_read_data.len(), buf.remaining());
            buf.put_slice(&self.pre_read_data[..len]);
            self.pre_read_data.drain(..len);
            return Poll::Ready(Ok(()));
        }
        Pin::new(&mut self.inner).poll_read(cx, buf)
    }
}

impl AsyncWrite for ProxyTunnelStream {
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

impl Unpin for ProxyTunnelStream {}

impl SyncProxyTunnelStream {
    pub(crate) fn new(stream: BoxReadWrite, pre_read_data: Vec<u8>) -> Self {
        Self {
            inner: stream,
            pre_read_data,
        }
    }
}

impl Read for SyncProxyTunnelStream {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        if !self.pre_read_data.is_empty() {
            let len: usize = std::cmp::min(self.pre_read_data.len(), buf.len());
            buf[..len].copy_from_slice(&self.pre_read_data[..len]);
            self.pre_read_data.drain(..len);
            return Ok(len);
        }
        self.inner.read(buf)
    }
}

impl Write for SyncProxyTunnelStream {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.inner.write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.inner.flush()
    }
}
