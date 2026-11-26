use super::*;

/// Inherent implementation of [`H3DirectState`].
impl H3DirectState {
    /// Reinitializes pooled state for a new QUIC send stream while retaining `pending_wire` capacity.
    ///
    /// # Arguments
    ///
    /// - `QuicSendStream` - The QUIC send half of the new request stream.
    /// - `StreamPeers` - The local and remote addresses of the carrying QUIC connection.
    /// - `bool` - Whether the incoming request method was `HEAD`.
    #[inline(always)]
    pub(super) fn reinit(
        &mut self,
        send_stream: QuicSendStream,
        peers: StreamPeers,
        is_head: bool,
    ) {
        self.set_send_stream(Some(send_stream));
        self.get_mut_pending_wire().clear();
        self.set_pending_wire_offset(0);
        self.get_mut_pending_head().clear();
        self.set_head_sent(false);
        self.set_suppress_body(is_head);
        self.set_is_head(is_head);
        self.set_announced_len(None);
        self.set_body_len(0);
        self.set_finished(false);
        self.set_failed(false);
        self.set_peers(peers);
    }

    /// Cleans up stream handles and trims oversized buffers before returning state to the pool.
    #[inline(always)]
    pub(super) fn release(&mut self) {
        if !self.get_finished() {
            self.reset();
        }
        let _: Option<QuicSendStream> = self.get_mut_send_stream().take();
        if self.get_pending_wire().capacity() > MAX_POOLED_BUFFER_CAPACITY {
            self.set_pending_wire(Vec::with_capacity(H3_READ_CHUNK_SIZE));
        } else {
            self.get_mut_pending_wire().clear();
        }
        if self.get_pending_head().capacity() > MAX_POOLED_BUFFER_CAPACITY {
            self.set_pending_head(Vec::new());
        } else {
            self.get_mut_pending_head().clear();
        }
        self.set_pending_wire_offset(0);
    }

    /// Resets the underlying QUIC send stream with `H3_INTERNAL_ERROR`.
    pub(super) fn reset(&mut self) {
        if let Some(mut stream) = self.get_mut_send_stream().take() {
            let _: Result<(), _> = stream.reset(VarInt::from_u32(H3_INTERNAL_ERROR));
        }
        self.set_finished(true);
    }

    /// Marks the stream failed, resets it, and constructs an I/O error.
    ///
    /// # Arguments
    ///
    /// - `&str` - The error message to report.
    ///
    /// # Returns
    ///
    /// - `io::Error` - The wrapped framing error.
    pub(super) fn fail(&mut self, message: &str) -> io::Error {
        self.set_failed(true);
        self.reset();
        io::Error::other(message.to_owned())
    }

    /// Attempts to write all remaining bytes in `pending_wire` onto the QUIC send stream.
    ///
    /// # Arguments
    ///
    /// - `&mut task::Context<'_>` - The task context.
    ///
    /// # Returns
    ///
    /// - `Poll<io::Result<()>>` - `Ready(Ok(()))` when fully flushed, or `Pending` under backpressure.
    #[inline(always)]
    pub(super) fn poll_drain_pending_wire(
        &mut self,
        context: &mut task::Context<'_>,
    ) -> Poll<io::Result<()>> {
        while self.get_pending_wire_offset() < self.get_pending_wire().len() {
            let start: usize = self.get_pending_wire_offset();
            let Self {
                send_stream,
                pending_wire,
                pending_wire_offset,
                ..
            } = self;
            let Some(stream) = send_stream.as_mut() else {
                return Poll::Ready(Err(self.fail(MISSING_SEND_CAPACITY)));
            };
            match Pin::new(stream).poll_write(context, &pending_wire[start..]) {
                Poll::Ready(Ok(0)) => {
                    return Poll::Ready(Err(self.fail(MISSING_SEND_CAPACITY)));
                }
                Poll::Ready(Ok(written)) => {
                    *pending_wire_offset += written;
                }
                Poll::Ready(Err(error)) => {
                    return Poll::Ready(Err(self.fail(&error.to_string())));
                }
                Poll::Pending => return Poll::Pending,
            }
        }
        self.get_mut_pending_wire().clear();
        self.set_pending_wire_offset(0);
        Poll::Ready(Ok(()))
    }

    /// Encodes the HTTP/3 `HEADERS` frame plus optional coalesced `DATA` frame and writes once.
    ///
    /// # Arguments
    ///
    /// - `&mut task::Context<'_>` - The task context.
    /// - `&[u8]` - Serialized HTTP/1.1 head bytes up to and including `\r\n\r\n`.
    /// - `&[u8]` - Optional trailing body bytes already present in the same write.
    /// - `usize` - Caller byte count corresponding to `head_bytes` + `body_bytes`.
    ///
    /// # Returns
    ///
    /// - `Poll<io::Result<usize>>` - Number of caller bytes accepted, or framing error.
    pub(super) fn poll_emit_head(
        &mut self,
        context: &mut task::Context<'_>,
        head_bytes: &[u8],
        body_bytes: &[u8],
        caller_len: usize,
    ) -> Poll<io::Result<usize>> {
        let is_head: bool = self.get_is_head();
        let wire: &mut Vec<u8> = self.get_mut_pending_wire();
        wire.clear();
        let meta: H3EncodedResponseMeta = match encode_h3_response_head(head_bytes, is_head, wire) {
            Ok(meta) => meta,
            Err(error) => return Poll::Ready(Err(self.fail(&error.to_string()))),
        };
        self.set_announced_len(meta.announced_len);
        self.set_suppress_body(meta.suppress_body);
        self.set_head_sent(true);
        let head_body: &[u8] = if meta.suppress_body { &[] } else { body_bytes };
        let accepted_body: usize = head_body.len().min(MAX_POOLED_BUFFER_CAPACITY);
        if accepted_body > 0 && check_body_budget(self.try_get_announced_len(), 0, accepted_body) {
            return Poll::Ready(Err(self.fail(OVERFLOWING_RESPONSE_BODY)));
        }
        let consumed: usize = if meta.suppress_body {
            caller_len
        } else {
            caller_len - body_bytes.len() + accepted_body
        };
        if accepted_body > 0 {
            let wire: &mut Vec<u8> = self.get_mut_pending_wire();
            wire.push(H3_FRAME_DATA as u8);
            encode_varint(accepted_body as u64, wire);
            wire.extend_from_slice(&head_body[..accepted_body]);
            self.set_body_len(accepted_body);
        }
        self.set_pending_wire_offset(0);
        let is_complete: bool =
            meta.suppress_body || self.try_get_announced_len() == Some(accepted_body);
        match self.poll_drain_pending_wire(context) {
            Poll::Ready(Err(error)) => Poll::Ready(Err(error)),
            Poll::Ready(Ok(())) => {
                if is_complete && !self.get_finished() {
                    self.set_finished(true);
                    if let Some(mut stream) = self.get_mut_send_stream().take() {
                        let _: Result<(), _> = stream.finish();
                    }
                }
                Poll::Ready(Ok(consumed))
            }
            Poll::Pending => Poll::Ready(Ok(consumed)),
        }
    }
}

/// Inherent implementation of [`H3DirectTransport`].
impl H3DirectTransport {
    /// Creates a direct HTTP/3 transport backed by a shared pooled state handle.
    ///
    /// # Arguments
    ///
    /// - `Arc<Mutex<H3DirectState>>` - Shared framing state recycled across requests.
    ///
    /// # Returns
    ///
    /// - `Self` - The direct HTTP/3 transport.
    pub(crate) fn from_shared(state: Arc<Mutex<H3DirectState>>) -> Self {
        Self { state }
    }
}

/// Inherent implementation of [`H3StreamGuard`].
impl H3StreamGuard {
    /// Takes a pooled boxed HTTP/3 `Stream` (or allocates one on first use) and binds it to the send stream.
    ///
    /// # Arguments
    ///
    /// - `QuicSendStream` - The QUIC send half of the bidirectional request stream.
    /// - `StreamPeers` - The local and remote addresses of the carrying QUIC connection.
    /// - `RequestConfig` - The active request limits for the stream.
    /// - `bool` - Whether the incoming request method was `HEAD`.
    ///
    /// # Returns
    ///
    /// - `Self` - Guard owning the ready `Stream` until dropped.
    #[inline(always)]
    pub(crate) fn take(
        send_stream: QuicSendStream,
        peers: StreamPeers,
        request_config: RequestConfig,
        is_head: bool,
    ) -> Self {
        let pooled: Option<PooledH3Stream> =
            H3_STREAM_POOL.with(|pool: &RefCell<Vec<PooledH3Stream>>| pool.borrow_mut().pop());
        let pair: PooledH3Stream = match pooled {
            Some((mut stream, state)) if *stream.get_request_config() == request_config => {
                if let Ok(mut guard) = state.lock() {
                    guard.reinit(send_stream, peers, is_head);
                }
                stream.set_closed(false);
                (stream, state)
            }
            _ => {
                let state: Arc<Mutex<H3DirectState>> = Arc::new(Mutex::new(H3DirectState {
                    send_stream: Some(send_stream),
                    pending_wire: Vec::with_capacity(H3_READ_CHUNK_SIZE),
                    pending_wire_offset: 0,
                    pending_head: Vec::new(),
                    head_sent: false,
                    suppress_body: is_head,
                    is_head,
                    announced_len: None,
                    body_len: 0,
                    finished: false,
                    failed: false,
                    peers,
                }));
                let stream: Stream = Stream::from_transport(
                    H3DirectTransport::from_shared(Arc::clone(&state)),
                    request_config,
                    false,
                );
                (stream, state)
            }
        };
        Self { inner: Some(pair) }
    }

    /// Returns a mutable reference to the pooled `Stream`.
    ///
    /// # Returns
    ///
    /// - `Option<&mut Stream>` - The active HTTP/3 stream wrapper, or `None` once finalized.
    #[inline(always)]
    pub(crate) fn stream_mut(&mut self) -> Option<&mut Stream> {
        let Self { inner } = self;
        inner.as_mut().map(|pair: &mut PooledH3Stream| &mut pair.0)
    }

    /// Finalizes the QUIC send stream if not yet finished, then recycles the pooled `Stream`.
    #[inline(always)]
    pub(crate) async fn finish(mut self) {
        let Self { inner } = &mut self;
        let Some((mut stream, state)) = inner.take() else {
            return;
        };
        let already_finished: bool = match state.lock() {
            Ok(mut guard) if guard.get_finished() && !guard.get_failed() => {
                guard.release();
                true
            }
            Ok(_) => false,
            Err(_) => return,
        };
        if !already_finished {
            let _: Result<(), io::Error> = stream.get_mut_stream().shutdown().await;
            if let Ok(mut guard) = state.lock() {
                guard.release();
            } else {
                return;
            }
        }
        stream.set_closed(false);
        H3_STREAM_POOL.with(|pool: &RefCell<Vec<PooledH3Stream>>| {
            let mut pool: RefMut<'_, Vec<PooledH3Stream>> = pool.borrow_mut();
            if pool.len() < MAX_POOLED_CONTEXTS {
                pool.push((stream, state));
            }
        });
    }
}

/// Returns the pooled HTTP/3 `Stream` to [`H3_STREAM_POOL`] after releasing stream handles.
impl Drop for H3StreamGuard {
    /// Resets any unfinished stream and recycles the boxed `Stream` wrapper.
    fn drop(&mut self) {
        let Self { inner } = self;
        let Some((mut stream, state)) = inner.take() else {
            return;
        };
        if let Ok(mut guard) = state.lock() {
            guard.release();
        } else {
            return;
        }
        stream.set_closed(false);
        H3_STREAM_POOL.with(|pool: &RefCell<Vec<PooledH3Stream>>| {
            let mut pool: RefMut<'_, Vec<PooledH3Stream>> = pool.borrow_mut();
            if pool.len() < MAX_POOLED_CONTEXTS {
                pool.push((stream, state));
            }
        });
    }
}

/// Implements [`fmt::Debug`] for [`H3DirectTransport`].
impl fmt::Debug for H3DirectTransport {
    /// Formats the direct HTTP/3 transport for diagnostic output.
    ///
    /// # Arguments
    ///
    /// - `&mut fmt::Formatter<'_>` - The formatter to write into.
    ///
    /// # Returns
    ///
    /// - `fmt::Result` - The formatting result.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.debug_struct("H3DirectTransport").finish()
    }
}

/// Ensures an unfinished QUIC send stream is reset and pooled wire storage is returned on drop.
impl Drop for H3DirectTransport {
    /// Resets the stream when dropped before finishing cleanly and recycles its wire buffer.
    fn drop(&mut self) {
        if let Ok(mut state) = self.get_state().lock() {
            if !state.finished {
                state.reset();
            }
            let wire: Vec<u8> = mem::take(state.get_mut_pending_wire());
            return_h3_buffer(wire);
        }
    }
}

/// Implements [`Transport`] for [`H3DirectTransport`].
impl Transport for H3DirectTransport {
    /// Reports the remote address of the carrying QUIC connection.
    ///
    /// # Returns
    ///
    /// - `Result<SocketAddr, io::Error>` - The peer socket address.
    fn peer_addr(&self) -> Result<SocketAddr, io::Error> {
        self.get_state()
            .lock()
            .map(|state: MutexGuard<'_, H3DirectState>| state.get_peers().remote)
            .map_err(|_| io::Error::other(MISSING_SEND_CAPACITY))
    }

    /// Reports the local address of the carrying QUIC connection.
    ///
    /// # Returns
    ///
    /// - `Result<SocketAddr, io::Error>` - The local socket address.
    fn local_addr(&self) -> Result<SocketAddr, io::Error> {
        self.get_state()
            .lock()
            .map(|state: MutexGuard<'_, H3DirectState>| state.get_peers().local)
            .map_err(|_| io::Error::other(MISSING_SEND_CAPACITY))
    }
}

/// Implements [`AsyncRead`] for [`H3DirectTransport`].
impl AsyncRead for H3DirectTransport {
    /// Immediately signals EOF because request bodies are collected beforehand.
    ///
    /// # Arguments
    ///
    /// - `Pin<&mut Self>` - The pinned transport.
    /// - `&mut task::Context<'_>` - The task context.
    /// - `&mut ReadBuf<'_>` - The destination read buffer.
    ///
    /// # Returns
    ///
    /// - `Poll<io::Result<()>>` - Ready with zero bytes read.
    fn poll_read(
        self: Pin<&mut Self>,
        _context: &mut task::Context<'_>,
        buffer: &mut ReadBuf<'_>,
    ) -> Poll<io::Result<()>> {
        buffer.advance(0);
        Poll::Ready(Ok(()))
    }
}

/// Implements [`AsyncWrite`] for [`H3DirectTransport`].
impl AsyncWrite for H3DirectTransport {
    /// Encodes written HTTP/1.1 bytes directly into HTTP/3 frames on the QUIC send stream.
    ///
    /// # Arguments
    ///
    /// - `Pin<&mut Self>` - The pinned transport.
    /// - `&mut task::Context<'_>` - The task context.
    /// - `&[u8]` - The bytes written by the handler or middleware.
    ///
    /// # Returns
    ///
    /// - `Poll<io::Result<usize>>` - The number of bytes accepted.
    fn poll_write(
        self: Pin<&mut Self>,
        context: &mut task::Context<'_>,
        bytes: &[u8],
    ) -> Poll<io::Result<usize>> {
        let Ok(mut state) = self.get_mut().get_state().lock() else {
            return Poll::Ready(Err(io::Error::other(MISSING_SEND_CAPACITY)));
        };
        if state.failed {
            return Poll::Ready(Err(io::Error::other(MISSING_SEND_CAPACITY)));
        }
        if state.pending_wire_offset < state.pending_wire.len() {
            match state.poll_drain_pending_wire(context) {
                Poll::Ready(Ok(())) => {}
                Poll::Ready(Err(error)) => return Poll::Ready(Err(error)),
                Poll::Pending => return Poll::Pending,
            }
        }
        if bytes.is_empty() {
            return Poll::Ready(Ok(0));
        }
        if !state.head_sent {
            if state.pending_head.is_empty() {
                if let Some(body_start) = find_head_end(bytes) {
                    if body_start > RESPONSE_HEAD_MAX {
                        return Poll::Ready(Err(state.fail(RESPONSE_HEAD_TOO_LONG)));
                    }
                    return state.poll_emit_head(
                        context,
                        &bytes[..body_start],
                        &bytes[body_start..],
                        bytes.len(),
                    );
                }
                if bytes.len() > RESPONSE_HEAD_MAX {
                    return Poll::Ready(Err(state.fail(RESPONSE_HEAD_TOO_LONG)));
                }
                state.pending_head.extend_from_slice(bytes);
                return Poll::Ready(Ok(bytes.len()));
            }
            let accepted: usize = bytes
                .len()
                .min(RESPONSE_HEAD_MAX - state.pending_head.len());
            if accepted == 0 {
                return Poll::Ready(Err(state.fail(RESPONSE_HEAD_TOO_LONG)));
            }
            state.pending_head.extend_from_slice(&bytes[..accepted]);
            let Some(body_start) = find_head_end(&state.pending_head) else {
                if state.pending_head.len() > RESPONSE_HEAD_MAX {
                    return Poll::Ready(Err(state.fail(RESPONSE_HEAD_TOO_LONG)));
                }
                return Poll::Ready(Ok(accepted));
            };
            if body_start > RESPONSE_HEAD_MAX {
                return Poll::Ready(Err(state.fail(RESPONSE_HEAD_TOO_LONG)));
            }
            let pending_head: Vec<u8> = mem::take(&mut state.pending_head);
            return state.poll_emit_head(
                context,
                &pending_head[..body_start],
                &pending_head[body_start..],
                accepted,
            );
        }
        if state.suppress_body {
            return Poll::Ready(Ok(bytes.len()));
        }
        let accepted: usize = bytes.len().min(MAX_POOLED_BUFFER_CAPACITY);
        if check_body_budget(state.announced_len, state.body_len, accepted) {
            return Poll::Ready(Err(state.fail(OVERFLOWING_RESPONSE_BODY)));
        }
        state.body_len += accepted;
        let wire: &mut Vec<u8> = state.get_mut_pending_wire();
        wire.clear();
        wire.push(H3_FRAME_DATA as u8);
        encode_varint(accepted as u64, wire);
        wire.extend_from_slice(&bytes[..accepted]);
        state.pending_wire_offset = 0;
        let is_complete: bool = state.announced_len == Some(state.body_len);
        match state.poll_drain_pending_wire(context) {
            Poll::Ready(Err(error)) => Poll::Ready(Err(error)),
            Poll::Ready(Ok(())) => {
                if is_complete && !state.finished {
                    state.finished = true;
                    if let Some(mut stream) = state.send_stream.take() {
                        let _: Result<(), _> = stream.finish();
                    }
                }
                Poll::Ready(Ok(accepted))
            }
            Poll::Pending => Poll::Ready(Ok(accepted)),
        }
    }

    /// Flushes any backpressured HTTP/3 frame bytes onto the QUIC send stream.
    ///
    /// # Arguments
    ///
    /// - `Pin<&mut Self>` - The pinned transport.
    /// - `&mut task::Context<'_>` - The task context.
    ///
    /// # Returns
    ///
    /// - `Poll<io::Result<()>>` - Ready once flushed.
    fn poll_flush(self: Pin<&mut Self>, context: &mut task::Context<'_>) -> Poll<io::Result<()>> {
        let Ok(mut state) = self.get_mut().get_state().lock() else {
            return Poll::Ready(Err(io::Error::other(MISSING_SEND_CAPACITY)));
        };
        if state.failed {
            return Poll::Ready(Err(io::Error::other(MISSING_SEND_CAPACITY)));
        }
        state.poll_drain_pending_wire(context)
    }

    /// Validates the completed response and synchronously finishes the QUIC send stream.
    ///
    /// # Arguments
    ///
    /// - `Pin<&mut Self>` - The pinned transport.
    /// - `&mut task::Context<'_>` - The task context.
    ///
    /// # Returns
    ///
    /// - `Poll<io::Result<()>>` - Ready once finalized or reset on protocol violation.
    fn poll_shutdown(
        self: Pin<&mut Self>,
        context: &mut task::Context<'_>,
    ) -> Poll<io::Result<()>> {
        let Ok(mut state) = self.get_mut().get_state().lock() else {
            return Poll::Ready(Err(io::Error::other(MISSING_SEND_CAPACITY)));
        };
        if state.failed {
            state.reset();
            return Poll::Ready(Err(io::Error::other(MISSING_SEND_CAPACITY)));
        }
        match state.poll_drain_pending_wire(context) {
            Poll::Ready(Ok(())) => {}
            Poll::Ready(Err(error)) => return Poll::Ready(Err(error)),
            Poll::Pending => return Poll::Pending,
        }
        if !state.head_sent {
            return Poll::Ready(Err(state.fail(MISSING_RESPONSE_HEAD)));
        }
        if !state.suppress_body
            && let Some(announced) = state.announced_len
            && state.body_len != announced
        {
            return Poll::Ready(Err(state.fail(TRUNCATED_RESPONSE_BODY)));
        }
        if !state.finished {
            state.finished = true;
            if let Some(mut stream) = state.send_stream.take() {
                let _: Result<(), _> = stream.finish();
            }
        }
        Poll::Ready(Ok(()))
    }
}
