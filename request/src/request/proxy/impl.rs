use super::*;

/// Inherent implementation of [`ProxyTunnelStream`].
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

/// Implements [`AsyncRead`] for [`ProxyTunnelStream`].
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

/// Implements [`AsyncWrite`] for [`ProxyTunnelStream`].
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

/// Implements [`Unpin`] for [`ProxyTunnelStream`].
impl Unpin for ProxyTunnelStream {}

/// Inherent implementation of [`SyncProxyTunnelStream`].
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

/// Implements [`Read`] for [`SyncProxyTunnelStream`].
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

/// Implements [`Write`] for [`SyncProxyTunnelStream`].
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

/// Inherent implementation of [`Proxy`].
impl Proxy {
    /// Creates a plain HTTP proxy.
    ///
    /// # Arguments
    ///
    /// - `H` - The proxy host, converted to a string slice.
    /// - `u16` - The proxy port.
    ///
    /// # Returns
    ///
    /// - `Self` - The configured HTTP proxy.
    pub fn http<H>(host: H, port: u16) -> Self
    where
        H: AsRef<str>,
    {
        Self::new(ProxyType::Http, host, port)
    }

    /// Creates an HTTPS (TLS-wrapped) proxy.
    ///
    /// # Arguments
    ///
    /// - `H` - The proxy host, converted to a string slice.
    /// - `u16` - The proxy port.
    ///
    /// # Returns
    ///
    /// - `Self` - The configured HTTPS proxy.
    pub fn https<H>(host: H, port: u16) -> Self
    where
        H: AsRef<str>,
    {
        Self::new(ProxyType::Https, host, port)
    }

    /// Creates a SOCKS5 proxy.
    ///
    /// # Arguments
    ///
    /// - `H` - The proxy host, converted to a string slice.
    /// - `u16` - The proxy port.
    ///
    /// # Returns
    ///
    /// - `Self` - The configured SOCKS5 proxy.
    pub fn socks5<H>(host: H, port: u16) -> Self
    where
        H: AsRef<str>,
    {
        Self::new(ProxyType::Socks5, host, port)
    }

    /// Attaches a username and password to this proxy.
    ///
    /// # Arguments
    ///
    /// - `U` - The proxy username, converted to a string slice.
    /// - `P` - The proxy password, converted to a string slice.
    ///
    /// # Returns
    ///
    /// - `Self` - The proxy updated with credentials.
    pub fn auth<U, P>(mut self, username: U, password: P) -> Self
    where
        U: AsRef<str>,
        P: AsRef<str>,
    {
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
    ///
    /// # Returns
    ///
    /// - `Self` - The newly constructed proxy.
    fn new<H>(proxy_type: ProxyType, host: H, port: u16) -> Self
    where
        H: AsRef<str>,
    {
        Self {
            proxy_type,
            host: host.as_ref().to_owned(),
            port,
            username: None,
            password: None,
        }
    }
}
