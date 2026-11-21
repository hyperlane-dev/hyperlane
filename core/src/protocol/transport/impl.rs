use super::*;

/// Inherent implementation of [`TlsTransport`].
impl<T> TlsTransport<T>
where
    T: AsyncRead + AsyncWrite + Unpin,
{
    /// Returns the ALPN protocol negotiated during the TLS handshake, if any.
    ///
    /// # Returns
    ///
    /// - `Option<&[u8]>` - The negotiated ALPN protocol bytes.
    #[inline(always)]
    pub(crate) fn alpn_protocol(&self) -> Option<&[u8]> {
        let Self { conn, .. } = self;
        conn.alpn_protocol()
    }

    /// Discards `count` consumed bytes from the front of `in_buf`.
    ///
    /// # Arguments
    ///
    /// - `usize` - Number of leading bytes to drop.
    #[inline(always)]
    fn discard_incoming(&mut self, count: usize) {
        if count == 0 {
            return;
        }
        let Self { in_buf, in_len, .. } = self;
        if count >= *in_len {
            *in_len = 0;
        } else {
            in_buf.copy_within(count..*in_len, 0);
            *in_len -= count;
        }
    }

    /// Flushes any pending encrypted bytes in `out_buf[out_pos..out_len]` to the TCP socket.
    ///
    /// # Arguments
    ///
    /// - `&mut task::Context<'_>` - The current task context.
    ///
    /// # Returns
    ///
    /// - `Poll<io::Result<()>>` - `Ready(Ok(()))` once `out_buf` is fully written.
    #[inline(always)]
    fn poll_flush_out(&mut self, context: &mut task::Context<'_>) -> Poll<io::Result<()>> {
        let Self {
            tcp,
            out_buf,
            out_pos,
            out_len,
            ..
        } = self;
        while *out_pos < *out_len {
            match Pin::new(&mut *tcp).poll_write(context, &out_buf[*out_pos..*out_len]) {
                Poll::Ready(Ok(0)) => {
                    return Poll::Ready(Err(io::Error::new(
                        io::ErrorKind::WriteZero,
                        "tcp write zero during tls flush",
                    )));
                }
                Poll::Ready(Ok(written)) => {
                    *out_pos += written;
                }
                Poll::Ready(Err(error)) => return Poll::Ready(Err(error)),
                Poll::Pending => return Poll::Pending,
            }
        }
        *out_pos = 0;
        *out_len = 0;
        Poll::Ready(Ok(()))
    }

    /// Encrypts one plaintext slice into `out_buf` and immediately writes it to `tcp`.
    ///
    /// # Arguments
    ///
    /// - `&mut task::Context<'_>` - The current task context.
    /// - `&[u8]` - Plaintext bytes (up to `TLS_MAX_PLAINTEXT_CHUNK`).
    /// - `bool` - Whether to emit close notification instead of application data.
    ///
    /// # Returns
    ///
    /// - `Poll<io::Result<usize>>` - Number of plaintext bytes accepted.
    #[inline(always)]
    fn poll_encrypt_and_write(
        &mut self,
        context: &mut task::Context<'_>,
        plaintext: &[u8],
        close_notify: bool,
    ) -> Poll<io::Result<usize>> {
        let Self { write_closed, .. } = self;
        if *write_closed {
            return Poll::Ready(Err(io::Error::new(
                io::ErrorKind::BrokenPipe,
                "tls write side closed",
            )));
        }
        match self.poll_flush_out(context) {
            Poll::Ready(Ok(())) => {}
            Poll::Ready(Err(error)) => return Poll::Ready(Err(error)),
            Poll::Pending => return Poll::Pending,
        }
        if plaintext.is_empty() && !close_notify {
            return Poll::Ready(Ok(0));
        }
        let chunk: &[u8] = &plaintext[..plaintext.len().min(TLS_MAX_PLAINTEXT_CHUNK)];
        loop {
            let Self {
                tcp,
                conn,
                plain_buf,
                out_buf,
                out_pos,
                out_len,
                peer_closed,
                write_closed,
                ..
            } = self;
            let UnbufferedStatus { state, .. } = conn.process_tls_records(&mut []);
            match state {
                Ok(ConnectionState::WriteTraffic(mut wt)) => {
                    let encrypted_len: usize = loop {
                        let encoded: Result<usize, EncryptError> = if close_notify {
                            wt.queue_close_notify(&mut out_buf[*out_len..])
                        } else {
                            wt.encrypt(chunk, &mut out_buf[*out_len..])
                        };
                        match encoded {
                            Ok(n) => break n,
                            Err(EncryptError::InsufficientSize(need)) => {
                                out_buf.resize(*out_len + need.required_size, 0);
                            }
                            Err(error) => {
                                return Poll::Ready(Err(io::Error::other(format!("{error:?}"))));
                            }
                        }
                    };
                    *out_len += encrypted_len;
                    *write_closed = close_notify;
                    match Pin::new(&mut *tcp).poll_write(context, &out_buf[*out_pos..*out_len]) {
                        Poll::Ready(Ok(0)) => {
                            return Poll::Ready(Err(io::Error::new(
                                io::ErrorKind::WriteZero,
                                TLS_TRANSMIT_WRITE_ZERO,
                            )));
                        }
                        Poll::Ready(Ok(written)) => {
                            *out_pos += written;
                            if *out_pos >= *out_len {
                                *out_pos = 0;
                                *out_len = 0;
                            }
                            return Poll::Ready(Ok(chunk.len()));
                        }
                        Poll::Pending => return Poll::Ready(Ok(chunk.len())),
                        Poll::Ready(Err(error)) => return Poll::Ready(Err(error)),
                    }
                }
                Ok(ConnectionState::EncodeTlsData(mut enc)) => loop {
                    match enc.encode(&mut out_buf[*out_len..]) {
                        Ok(n) => {
                            *out_len += n;
                            break;
                        }
                        Err(EncodeError::InsufficientSize(need)) => {
                            out_buf.resize(*out_len + need.required_size, 0);
                        }
                        Err(error) => {
                            return Poll::Ready(Err(io::Error::other(format!("{error:?}"))));
                        }
                    }
                },
                Ok(ConnectionState::TransmitTlsData(transmit)) => {
                    while *out_pos < *out_len {
                        match Pin::new(&mut *tcp).poll_write(context, &out_buf[*out_pos..*out_len])
                        {
                            Poll::Ready(Ok(0)) => {
                                transmit.done();
                                return Poll::Ready(Err(io::Error::new(
                                    io::ErrorKind::WriteZero,
                                    TLS_TRANSMIT_WRITE_ZERO,
                                )));
                            }
                            Poll::Ready(Ok(written)) => {
                                *out_pos += written;
                            }
                            Poll::Ready(Err(error)) => {
                                transmit.done();
                                return Poll::Ready(Err(error));
                            }
                            Poll::Pending => return Poll::Pending,
                        }
                    }
                    *out_pos = 0;
                    *out_len = 0;
                    transmit.done();
                }
                Ok(ConnectionState::ReadTraffic(mut rt)) => {
                    while let Some(rec) = rt.next_record() {
                        match rec {
                            Ok(app) => plain_buf.extend_from_slice(app.payload),
                            Err(error) => return Poll::Ready(Err(io::Error::other(error))),
                        }
                    }
                }
                Ok(ConnectionState::PeerClosed) => {
                    *peer_closed = true;
                }
                Ok(ConnectionState::Closed) if close_notify => return Poll::Ready(Ok(0)),
                Ok(ConnectionState::Closed) => {
                    return Poll::Ready(Err(io::Error::new(
                        io::ErrorKind::BrokenPipe,
                        "tls connection closed",
                    )));
                }
                Err(error) => return Poll::Ready(Err(io::Error::other(error))),
                Ok(_) => {
                    return Poll::Ready(Err(io::Error::other("unexpected tls state on write")));
                }
            }
        }
    }
}

/// Implements [`fmt::Debug`] for [`TlsTransport`].
impl<T> fmt::Debug for TlsTransport<T> {
    /// Formats the unbuffered TLS transport for diagnostics.
    ///
    /// # Arguments
    ///
    /// - `&mut fmt::Formatter<'_>` - The formatter to write into.
    ///
    /// # Returns
    ///
    /// - `fmt::Result` - The formatting result.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self {
            in_len,
            out_len,
            peer_closed,
            ..
        } = self;
        formatter
            .debug_struct("TlsTransport")
            .field("in_len", in_len)
            .field("out_len", out_len)
            .field("peer_closed", peer_closed)
            .finish()
    }
}

/// Returns pooled record buffers to the worker-local pool on drop.
impl<T> Drop for TlsTransport<T> {
    /// Recycles the incoming and outgoing TLS record buffers.
    fn drop(&mut self) {
        let Self {
            in_buf, out_buf, ..
        } = self;
        return_tls_buffer(mem::take(in_buf));
        return_tls_buffer(mem::take(out_buf));
    }
}

/// Implements [`Transport`] for [`TlsTransport`].
impl<T> Transport for TlsTransport<T>
where
    T: AsyncRead + AsyncWrite + Transport + Unpin + Send + Sync + 'static,
{
    /// Reports the actual peer address.
    ///
    /// # Returns
    ///
    /// - `Result<SocketAddr, io::Error>` - The peer address.
    fn peer_addr(&self) -> Result<SocketAddr, io::Error> {
        let Self { tcp, .. } = self;
        tcp.peer_addr()
    }

    /// Reports the actual local address.
    ///
    /// # Returns
    ///
    /// - `Result<SocketAddr, io::Error>` - The local address.
    fn local_addr(&self) -> Result<SocketAddr, io::Error> {
        let Self { tcp, .. } = self;
        tcp.local_addr()
    }
}

/// Implements [`AsyncRead`] for [`TlsTransport`].
impl<T> AsyncRead for TlsTransport<T>
where
    T: AsyncRead + AsyncWrite + Unpin,
{
    /// Reads decrypted TLS application data into `buffer`.
    ///
    /// # Arguments
    ///
    /// - `Pin<&mut Self>` - The pinned transport.
    /// - `&mut task::Context<'_>` - The current task context.
    /// - `&mut ReadBuf<'_>` - The destination buffer.
    ///
    /// # Returns
    ///
    /// - `Poll<io::Result<()>>` - The read result.
    fn poll_read(
        self: Pin<&mut Self>,
        context: &mut task::Context<'_>,
        buffer: &mut ReadBuf<'_>,
    ) -> Poll<io::Result<()>> {
        let this: &mut Self = self.get_mut();
        match this.poll_flush_out(context) {
            Poll::Ready(Ok(())) => {}
            Poll::Ready(Err(error)) => return Poll::Ready(Err(error)),
            Poll::Pending => return Poll::Pending,
        }
        {
            let Self {
                plain_buf,
                plain_pos,
                peer_closed,
                ..
            } = this;
            if *plain_pos < plain_buf.len() {
                let avail: usize = plain_buf.len() - *plain_pos;
                let take: usize = avail.min(buffer.remaining());
                buffer.put_slice(&plain_buf[*plain_pos..*plain_pos + take]);
                *plain_pos += take;
                if *plain_pos >= plain_buf.len() {
                    plain_buf.clear();
                    *plain_pos = 0;
                }
                return Poll::Ready(Ok(()));
            }
            if *peer_closed {
                return Poll::Ready(Ok(()));
            }
        }
        if buffer.remaining() == 0 {
            return Poll::Ready(Ok(()));
        }
        let initial_filled: usize = buffer.filled().len();
        loop {
            {
                let (discard, action): (usize, Result<u8, io::Error>) = {
                    let Self {
                        conn,
                        in_buf,
                        in_len,
                        plain_buf,
                        out_buf,
                        out_len,
                        peer_closed,
                        ..
                    } = this;
                    let UnbufferedStatus { discard, state } =
                        conn.process_tls_records(&mut in_buf[..*in_len]);
                    let step: Result<u8, io::Error> = match state {
                        Ok(ConnectionState::ReadTraffic(mut rt)) => {
                            let mut err: Option<io::Error> = None;
                            while let Some(rec) = rt.next_record() {
                                match rec {
                                    Ok(app) => {
                                        let payload: &[u8] = app.payload;
                                        if plain_buf.is_empty() && buffer.remaining() > 0 {
                                            let direct: usize =
                                                payload.len().min(buffer.remaining());
                                            buffer.put_slice(&payload[..direct]);
                                            if direct < payload.len() {
                                                plain_buf.extend_from_slice(&payload[direct..]);
                                            }
                                        } else {
                                            plain_buf.extend_from_slice(payload);
                                        }
                                    }
                                    Err(error) => {
                                        err = Some(io::Error::other(error));
                                        break;
                                    }
                                }
                            }
                            match err {
                                Some(error) => Err(error),
                                None => Ok(1),
                            }
                        }
                        Ok(ConnectionState::EncodeTlsData(mut enc)) => loop {
                            match enc.encode(&mut out_buf[*out_len..]) {
                                Ok(n) => {
                                    *out_len += n;
                                    break Ok(2);
                                }
                                Err(EncodeError::InsufficientSize(need)) => {
                                    out_buf.resize(*out_len + need.required_size, 0);
                                }
                                Err(error) => {
                                    break Err(io::Error::other(format!("{error:?}")));
                                }
                            }
                        },
                        Ok(ConnectionState::TransmitTlsData(transmit)) => {
                            if *out_len == 0 {
                                transmit.done();
                                Ok(2)
                            } else {
                                Ok(3)
                            }
                        }
                        Ok(ConnectionState::PeerClosed | ConnectionState::Closed) => {
                            *peer_closed = true;
                            Ok(4)
                        }
                        Ok(ConnectionState::WriteTraffic(_)) => Ok(0),
                        Ok(ConnectionState::ReadEarlyData(mut red)) => {
                            while red.next_record().is_some() {}
                            Ok(2)
                        }
                        Ok(_) => Ok(0),
                        Err(error) => Err(io::Error::other(error)),
                    };
                    (discard, step)
                };
                this.discard_incoming(discard);
                match action {
                    Ok(1) => {
                        if buffer.filled().len() > initial_filled {
                            return Poll::Ready(Ok(()));
                        }
                        continue;
                    }
                    Ok(2) => continue,
                    Ok(3) => match this.poll_flush_out(context) {
                        Poll::Ready(Ok(())) => continue,
                        Poll::Ready(Err(error)) => return Poll::Ready(Err(error)),
                        Poll::Pending => return Poll::Pending,
                    },
                    Ok(4) => return Poll::Ready(Ok(())),
                    Ok(_) => {}
                    Err(error) => return Poll::Ready(Err(error)),
                }
            }
            let Self {
                tcp,
                in_buf,
                in_len,
                ..
            } = this;
            if *in_len == in_buf.len() {
                if in_buf.len() >= TLS_MAX_INCOMING_BUFFER {
                    return Poll::Ready(Err(io::Error::other("tls incoming record exceeds limit")));
                }
                let next_len: usize = (in_buf.len() * 2).min(TLS_MAX_INCOMING_BUFFER);
                in_buf.resize(next_len, 0);
            }
            let mut tcp_buf: ReadBuf<'_> = ReadBuf::new(&mut in_buf[*in_len..]);
            match Pin::new(&mut *tcp).poll_read(context, &mut tcp_buf) {
                Poll::Pending => return Poll::Pending,
                Poll::Ready(Err(error)) => return Poll::Ready(Err(error)),
                Poll::Ready(Ok(())) => {
                    let read_len: usize = tcp_buf.filled().len();
                    if read_len == 0 {
                        return Poll::Ready(Err(io::Error::new(
                            io::ErrorKind::UnexpectedEof,
                            "tls peer closed connection without close_notify",
                        )));
                    }
                    *in_len += read_len;
                }
            }
        }
    }
}

/// Implements [`AsyncWrite`] for [`TlsTransport`].
impl<T> AsyncWrite for TlsTransport<T>
where
    T: AsyncRead + AsyncWrite + Unpin,
{
    /// Encrypts `bytes` in place into the pooled record buffer and writes directly to TCP.
    ///
    /// # Arguments
    ///
    /// - `Pin<&mut Self>` - The pinned transport.
    /// - `&mut task::Context<'_>` - The current task context.
    /// - `&[u8]` - The plaintext bytes to encrypt and write.
    ///
    /// # Returns
    ///
    /// - `Poll<io::Result<usize>>` - Number of plaintext bytes accepted.
    fn poll_write(
        self: Pin<&mut Self>,
        context: &mut task::Context<'_>,
        bytes: &[u8],
    ) -> Poll<io::Result<usize>> {
        self.get_mut().poll_encrypt_and_write(context, bytes, false)
    }

    /// Coalesces multiple `IoSlice` buffers (such as HTTP/2 HEADERS + DATA frames)
    /// into a single TLS record and single TCP write when they fit within one record.
    ///
    /// # Arguments
    ///
    /// - `Pin<&mut Self>` - The pinned transport.
    /// - `&mut task::Context<'_>` - The current task context.
    /// - `&[IoSlice<'_>]` - Ordered plaintext slices to encrypt.
    ///
    /// # Returns
    ///
    /// - `Poll<io::Result<usize>>` - Total plaintext bytes accepted.
    fn poll_write_vectored(
        self: Pin<&mut Self>,
        context: &mut task::Context<'_>,
        buffers: &[IoSlice<'_>],
    ) -> Poll<io::Result<usize>> {
        let this: &mut Self = self.get_mut();
        let mut first_non_empty: Option<&[u8]> = None;
        let mut non_empty_count: usize = 0;
        let mut total_len: usize = 0;
        for slice in buffers {
            let bytes: &[u8] = slice.as_ref();
            if !bytes.is_empty() {
                if first_non_empty.is_none() {
                    first_non_empty = Some(bytes);
                }
                non_empty_count += 1;
                total_len = total_len.saturating_add(bytes.len());
            }
        }
        let Some(first) = first_non_empty else {
            return Poll::Ready(Ok(0));
        };
        if non_empty_count == 1 || first.len() >= TLS_MAX_PLAINTEXT_CHUNK {
            return this.poll_encrypt_and_write(context, first, false);
        }
        let cap: usize = total_len.min(TLS_MAX_PLAINTEXT_CHUNK);
        let mut scratch: [MaybeUninit<u8>; TLS_MAX_PLAINTEXT_CHUNK] =
            [const { MaybeUninit::uninit() }; TLS_MAX_PLAINTEXT_CHUNK];
        let mut filled: usize = 0;
        for slice_item in buffers {
            let bytes: &[u8] = slice_item.as_ref();
            if bytes.is_empty() {
                continue;
            }
            let take: usize = bytes.len().min(cap - filled);
            if take == 0 {
                break;
            }
            // SAFETY: `filled + take <= cap <= TLS_MAX_PLAINTEXT_CHUNK` and `MaybeUninit<u8>`
            // has the exact layout of `u8`.
            unsafe {
                ptr::copy_nonoverlapping(
                    bytes.as_ptr(),
                    scratch.as_mut_ptr().add(filled).cast::<u8>(),
                    take,
                );
            }
            filled += take;
            if filled == cap {
                break;
            }
        }
        // SAFETY: Exactly `0..filled` bytes were initialized in `scratch` above.
        let coalesced: &[u8] =
            unsafe { slice::from_raw_parts(scratch.as_ptr().cast::<u8>(), filled) };
        this.poll_encrypt_and_write(context, coalesced, false)
    }

    /// Advertises vectored write coalescing support so HTTP/2 batches HEADERS + DATA.
    ///
    /// # Returns
    ///
    /// - `bool` - Always `true`.
    fn is_write_vectored(&self) -> bool {
        true
    }

    /// Flushes any pending encrypted TLS bytes and flushes the underlying TCP socket.
    ///
    /// # Arguments
    ///
    /// - `Pin<&mut Self>` - The pinned transport.
    /// - `&mut task::Context<'_>` - The current task context.
    ///
    /// # Returns
    ///
    /// - `Poll<io::Result<()>>` - The flush result.
    fn poll_flush(self: Pin<&mut Self>, context: &mut task::Context<'_>) -> Poll<io::Result<()>> {
        let this: &mut Self = self.get_mut();
        match this.poll_flush_out(context) {
            Poll::Ready(Ok(())) => {
                let Self { tcp, .. } = this;
                Pin::new(&mut *tcp).poll_flush(context)
            }
            Poll::Ready(Err(error)) => Poll::Ready(Err(error)),
            Poll::Pending => Poll::Pending,
        }
    }

    /// Sends the TLS `close_notify` alert and shuts down the underlying TCP socket.
    ///
    /// # Arguments
    ///
    /// - `Pin<&mut Self>` - The pinned transport.
    /// - `&mut task::Context<'_>` - The current task context.
    ///
    /// # Returns
    ///
    /// - `Poll<io::Result<()>>` - The shutdown result.
    fn poll_shutdown(
        self: Pin<&mut Self>,
        context: &mut task::Context<'_>,
    ) -> Poll<io::Result<()>> {
        let this: &mut Self = self.get_mut();
        match this.poll_flush_out(context) {
            Poll::Ready(Ok(())) => {}
            Poll::Ready(Err(error)) => return Poll::Ready(Err(error)),
            Poll::Pending => return Poll::Pending,
        }
        let needs_close: bool = {
            let Self { write_closed, .. } = this;
            !*write_closed
        };
        if needs_close {
            match this.poll_encrypt_and_write(context, &[], true) {
                Poll::Ready(Ok(_)) => {}
                Poll::Ready(Err(error)) => return Poll::Ready(Err(error)),
                Poll::Pending => return Poll::Pending,
            }
        }
        match this.poll_flush_out(context) {
            Poll::Ready(Ok(())) => {
                let Self { tcp, .. } = this;
                Pin::new(&mut *tcp).poll_shutdown(context)
            }
            Poll::Ready(Err(error)) => Poll::Ready(Err(error)),
            Poll::Pending => Poll::Pending,
        }
    }
}
