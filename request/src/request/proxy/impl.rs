use super::*;

impl ProxyTunnelStream {
    /// Creates an asynchronous proxy tunnel stream.
    ///
    /// # Arguments
    ///
    /// - `BoxAsyncReadWrite` - The boxed inner stream carrying the tunneled bytes.
    /// - `Vec<u8>` - The data already read from the proxy during the handshake.
    pub(crate) fn new(stream: BoxAsyncReadWrite, pre_read_data: Vec<u8>) -> Self {
        Self {
            inner: stream,
            pre_read_data,
        }
    }
}

impl AsyncRead for ProxyTunnelStream {
    /// Reads the pre-read handshake data before delegating to the inner stream.
    ///
    /// # Arguments
    ///
    /// - `Pin<&mut Self>` - A pinned mutable reference to the tunnel stream.
    /// - `&mut Context<'_>` - The task context used to register the read interest.
    /// - `&mut ReadBuf<'_>` - The buffer that receives the read bytes.
    ///
    /// # Returns
    ///
    /// - `Poll<io::Result<()>>` - The poll outcome carrying any read error.
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> Poll<io::Result<()>> {
        if !self.get_pre_read_data().is_empty() {
            let len: usize = cmp::min(self.get_pre_read_data().len(), buf.remaining());
            buf.put_slice(&self.get_pre_read_data()[..len]);
            self.get_mut_pre_read_data().drain(..len);
            return Poll::Ready(Ok(()));
        }
        Pin::new(self.get_mut_inner()).poll_read(cx, buf)
    }
}

impl AsyncWrite for ProxyTunnelStream {
    /// Writes bytes to the inner stream through the proxy tunnel.
    ///
    /// # Arguments
    ///
    /// - `Pin<&mut Self>` - A pinned mutable reference to the tunnel stream.
    /// - `&mut Context<'_>` - The task context used to register the write interest.
    /// - `&[u8]` - The buffer of bytes to write.
    ///
    /// # Returns
    ///
    /// - `Poll<Result<usize, io::Error>>` - The poll outcome carrying the written byte count or any error.
    fn poll_write(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<Result<usize, io::Error>> {
        Pin::new(self.get_mut_inner()).poll_write(cx, buf)
    }

    /// Flushes the inner stream through the proxy tunnel.
    ///
    /// # Arguments
    ///
    /// - `Pin<&mut Self>` - A pinned mutable reference to the tunnel stream.
    /// - `&mut Context<'_>` - The task context used to register the flush interest.
    ///
    /// # Returns
    ///
    /// - `Poll<Result<(), io::Error>>` - The poll outcome carrying any flush error.
    fn poll_flush(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), io::Error>> {
        Pin::new(self.get_mut_inner()).poll_flush(cx)
    }

    /// Shuts down the inner stream through the proxy tunnel.
    ///
    /// # Arguments
    ///
    /// - `Pin<&mut Self>` - A pinned mutable reference to the tunnel stream.
    /// - `&mut Context<'_>` - The task context used to register the shutdown interest.
    ///
    /// # Returns
    ///
    /// - `Poll<Result<(), io::Error>>` - The poll outcome carrying any shutdown error.
    fn poll_shutdown(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Result<(), io::Error>> {
        Pin::new(self.get_mut_inner()).poll_shutdown(cx)
    }
}

impl Unpin for ProxyTunnelStream {}

impl SyncProxyTunnelStream {
    /// Creates a synchronous proxy tunnel stream.
    ///
    /// # Arguments
    ///
    /// - `BoxReadWrite` - The boxed inner stream carrying the tunneled bytes.
    /// - `Vec<u8>` - The data already read from the proxy during the handshake.
    pub(crate) fn new(stream: BoxReadWrite, pre_read_data: Vec<u8>) -> Self {
        Self {
            inner: stream,
            pre_read_data,
        }
    }
}

impl Read for SyncProxyTunnelStream {
    /// Reads the pre-read handshake data before delegating to the inner stream.
    ///
    /// # Arguments
    ///
    /// - `&mut [u8]` - The buffer that receives the read bytes.
    ///
    /// # Returns
    ///
    /// - `io::Result<usize>` - The read outcome carrying the number of bytes read or any error.
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        if !self.get_pre_read_data().is_empty() {
            let len: usize = cmp::min(self.get_pre_read_data().len(), buf.len());
            buf[..len].copy_from_slice(&self.get_pre_read_data()[..len]);
            self.get_mut_pre_read_data().drain(..len);
            return Ok(len);
        }
        self.get_mut_inner().read(buf)
    }
}

impl Write for SyncProxyTunnelStream {
    /// Writes bytes to the inner stream through the proxy tunnel.
    ///
    /// # Arguments
    ///
    /// - `&[u8]` - The buffer of bytes to write.
    ///
    /// # Returns
    ///
    /// - `io::Result<usize>` - The write outcome carrying the number of bytes written or any error.
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.get_mut_inner().write(buf)
    }

    /// Flushes the inner stream through the proxy tunnel.
    ///
    /// # Returns
    ///
    /// - `io::Result<()>` - The flush outcome carrying any flush error.
    fn flush(&mut self) -> io::Result<()> {
        self.get_mut_inner().flush()
    }
}

impl Proxy {
    /// Creates a plain HTTP proxy.
    ///
    /// # Arguments
    ///
    /// - `H` - The proxy host, converted to a string slice.
    /// - `u16` - The proxy port.
    pub fn http<H: AsRef<str>>(host: H, port: u16) -> Self {
        Self::new(ProxyType::Http, host, port)
    }

    /// Creates an HTTPS (TLS-wrapped) proxy.
    ///
    /// # Arguments
    ///
    /// - `H` - The proxy host, converted to a string slice.
    /// - `u16` - The proxy port.
    pub fn https<H: AsRef<str>>(host: H, port: u16) -> Self {
        Self::new(ProxyType::Https, host, port)
    }

    /// Creates a SOCKS5 proxy.
    ///
    /// # Arguments
    ///
    /// - `H` - The proxy host, converted to a string slice.
    /// - `u16` - The proxy port.
    pub fn socks5<H: AsRef<str>>(host: H, port: u16) -> Self {
        Self::new(ProxyType::Socks5, host, port)
    }

    /// Attaches a username and password to this proxy.
    ///
    /// # Arguments
    ///
    /// - `U` - The proxy username, converted to a string slice.
    /// - `P` - The proxy password, converted to a string slice.
    pub fn auth<U: AsRef<str>, P: AsRef<str>>(mut self, username: U, password: P) -> Self {
        self.set_username(Some(username.as_ref().to_owned()));
        self.set_password(Some(password.as_ref().to_owned()));
        self
    }

    /// Creates a proxy of the given type for a host and port.
    ///
    /// # Arguments
    ///
    /// - `ProxyType` - The protocol used to reach the proxy.
    /// - `H` - The proxy host, converted to a string slice.
    /// - `u16` - The proxy port.
    fn new<H: AsRef<str>>(proxy_type: ProxyType, host: H, port: u16) -> Self {
        Self {
            proxy_type,
            host: host.as_ref().to_owned(),
            port,
            username: None,
            password: None,
        }
    }
}
