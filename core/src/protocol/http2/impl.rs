use super::*;

/// Inherent implementation of [`H2DirectState`].
impl H2DirectState {
    /// Reinitializes pooled state for a new HTTP/2 request stream while retaining buffer capacity.
    ///
    /// # Arguments
    ///
    /// - `H2SendResponse` - The response half of the new HTTP/2 stream.
    /// - `StreamPeers` - The local and remote addresses of the carrying TCP connection.
    /// - `bool` - Whether the incoming request method was `HEAD`.
    #[inline(always)]
    pub(super) fn reinit(
        &mut self,
        send_response: H2SendResponse,
        peers: StreamPeers,
        is_head: bool,
    ) {
        self.set_send_response(Some(send_response));
        self.set_send_stream(None);
        self.get_mut_pending_head().clear();
        self.get_mut_pending_body().clear();
        self.set_pending_body_offset(0);
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
        let _: Option<H2SendResponse> = self.get_mut_send_response().take();
        let _: Option<H2SendStream> = self.get_mut_send_stream().take();
        if self.get_pending_head().capacity() > MAX_POOLED_BUFFER_CAPACITY {
            self.set_pending_head(Vec::new());
        } else {
            self.get_mut_pending_head().clear();
        }
        if self.get_pending_body().capacity() > MAX_POOLED_BUFFER_CAPACITY {
            self.set_pending_body(Vec::new());
        } else {
            self.get_mut_pending_body().clear();
        }
        self.set_pending_body_offset(0);
    }

    /// Resets the underlying HTTP/2 stream after a framing error or truncation.
    pub(super) fn reset(&mut self) {
        if let Some(mut send_response) = self.get_mut_send_response().take() {
            send_response.send_reset(Reason::INTERNAL_ERROR);
        }
        if let Some(mut send_stream) = self.get_mut_send_stream().take() {
            send_stream.send_reset(Reason::INTERNAL_ERROR);
        }
        self.set_finished(true);
    }

    /// Marks the stream failed, resets it, and builds an I/O error.
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

    /// Drains any buffered trailing body bytes extracted after a split head.
    ///
    /// # Arguments
    ///
    /// - `&mut task::Context<'_>` - The task context for flow-control backpressure.
    ///
    /// # Returns
    ///
    /// - `Poll<io::Result<()>>` - Ready once all buffered body bytes are sent.
    pub(super) fn poll_drain_pending_body(
        &mut self,
        context: &mut task::Context<'_>,
    ) -> Poll<io::Result<()>> {
        while self.get_pending_body_offset() < self.get_pending_body().len() {
            if self.get_suppress_body() {
                self.get_mut_pending_body().clear();
                self.set_pending_body_offset(0);
                return Poll::Ready(Ok(()));
            }
            let remaining: usize = self.get_pending_body().len() - self.get_pending_body_offset();
            let capacity: Poll<Option<Result<usize, H2Error>>> = {
                let Some(send_stream) = self.get_mut_send_stream().as_mut() else {
                    return Poll::Ready(Err(self.fail(MISSING_SEND_CAPACITY)));
                };
                send_stream.reserve_capacity(remaining);
                send_stream.poll_capacity(context)
            };
            match capacity {
                Poll::Ready(Some(Ok(capacity))) if capacity > 0 => {
                    let take: usize = capacity.min(remaining);
                    let start: usize = self.get_pending_body_offset();
                    let end: usize = start + take;
                    let new_len: usize = self.get_body_len() + take;
                    let is_final: bool = self.try_get_announced_len() == Some(new_len);
                    let chunk: Bytes = Bytes::copy_from_slice(&self.get_pending_body()[start..end]);
                    let Some(send_stream) = self.get_mut_send_stream().as_mut() else {
                        return Poll::Ready(Err(self.fail(MISSING_SEND_CAPACITY)));
                    };
                    if let Err(error) = send_stream.send_data(chunk, is_final) {
                        return Poll::Ready(Err(self.fail(&error.to_string())));
                    }
                    self.set_body_len(new_len);
                    self.set_pending_body_offset(end);
                    if is_final {
                        self.set_finished(true);
                        let _: Option<H2SendStream> = self.get_mut_send_stream().take();
                    }
                }
                Poll::Ready(Some(Ok(_))) => continue,
                Poll::Ready(Some(Err(error))) => {
                    return Poll::Ready(Err(self.fail(&error.to_string())));
                }
                Poll::Ready(None) => {
                    return Poll::Ready(Err(self.fail(MISSING_SEND_CAPACITY)));
                }
                Poll::Pending => return Poll::Pending,
            }
        }
        self.get_mut_pending_body().clear();
        self.set_pending_body_offset(0);
        Poll::Ready(Ok(()))
    }

    /// Parses and emits the response head, optionally forwarding inline body bytes.
    ///
    /// # Arguments
    ///
    /// - `&mut task::Context<'_>` - The task context for flow-control backpressure.
    /// - `&[u8]` - The serialized head bytes up to and including the blank line.
    /// - `&[u8]` - Any body bytes already present in the same write.
    ///
    /// # Returns
    ///
    /// - `io::Result<usize>` - Number of inline body bytes consumed directly.
    pub(super) fn emit_head_and_inline_body(
        &mut self,
        context: &mut task::Context<'_>,
        head_bytes: &[u8],
        body_bytes: &[u8],
    ) -> io::Result<usize> {
        let parsed: ParsedProtocolHead =
            match parse_protocol_response_head(head_bytes, self.get_is_head()) {
                Ok(parsed) => parsed,
                Err(error) => return Err(self.fail(&error.to_string())),
            };
        self.set_announced_len(parsed.announced_len);
        self.set_suppress_body(parsed.suppress_body);
        let Some(mut send_response) = self.get_mut_send_response().take() else {
            return Err(self.fail("response head already sent"));
        };
        if parsed.suppress_body {
            if let Err(error) = send_response.send_response(parsed.response, true) {
                return Err(self.fail(&error.to_string()));
            }
            self.set_head_sent(true);
            self.set_finished(true);
            return Ok(body_bytes.len());
        }
        if check_body_budget(self.try_get_announced_len(), 0, body_bytes.len()) {
            send_response.send_reset(Reason::INTERNAL_ERROR);
            return Err(self.fail(OVERFLOWING_RESPONSE_BODY));
        }
        if parsed.announced_len == Some(0) && body_bytes.is_empty() {
            if let Err(error) = send_response.send_response(parsed.response, true) {
                return Err(self.fail(&error.to_string()));
            }
            self.set_head_sent(true);
            self.set_finished(true);
            return Ok(0);
        }
        let mut send_stream: H2SendStream =
            match send_response.send_response(parsed.response, false) {
                Ok(stream) => stream,
                Err(error) => return Err(self.fail(&error.to_string())),
            };
        self.set_head_sent(true);
        if body_bytes.is_empty() {
            self.set_send_stream(Some(send_stream));
            return Ok(0);
        }
        send_stream.reserve_capacity(body_bytes.len());
        match send_stream.poll_capacity(context) {
            Poll::Ready(Some(Ok(capacity))) if capacity > 0 => {
                let take: usize = capacity.min(body_bytes.len());
                let is_final: bool = self.try_get_announced_len() == Some(take);
                let chunk: Bytes = Bytes::copy_from_slice(&body_bytes[..take]);
                if let Err(error) = send_stream.send_data(chunk, is_final) {
                    return Err(self.fail(&error.to_string()));
                }
                self.set_body_len(take);
                if is_final {
                    self.set_finished(true);
                } else {
                    self.set_send_stream(Some(send_stream));
                }
                Ok(take)
            }
            Poll::Ready(Some(Ok(_))) | Poll::Pending => {
                self.set_send_stream(Some(send_stream));
                Ok(0)
            }
            Poll::Ready(Some(Err(error))) => Err(self.fail(&error.to_string())),
            Poll::Ready(None) => Err(self.fail(MISSING_SEND_CAPACITY)),
        }
    }
}

/// Inherent implementation of [`H2DirectTransport`].
impl H2DirectTransport {
    /// Creates a direct HTTP/2 transport backed by a shared pooled state handle.
    ///
    /// # Arguments
    ///
    /// - `Arc<Mutex<H2DirectState>>` - Shared framing state recycled across requests.
    ///
    /// # Returns
    ///
    /// - `Self` - The direct HTTP/2 transport.
    pub(crate) fn from_shared(state: Arc<Mutex<H2DirectState>>) -> Self {
        Self { state }
    }
}

/// Inherent implementation of [`H2StreamGuard`].
impl H2StreamGuard {
    /// Takes a pooled boxed HTTP/2 `Stream` (or allocates one on first use) and binds it to the request stream.
    ///
    /// # Arguments
    ///
    /// - `H2SendResponse` - The response half of the HTTP/2 stream.
    /// - `StreamPeers` - The local and remote addresses of the carrying TCP connection.
    /// - `RequestConfig` - The active request limits for the stream.
    /// - `bool` - Whether the incoming request method was `HEAD`.
    ///
    /// # Returns
    ///
    /// - `Self` - Guard owning the ready `Stream` until dropped.
    #[inline(always)]
    pub(crate) fn take(
        send_response: H2SendResponse,
        peers: StreamPeers,
        request_config: RequestConfig,
        is_head: bool,
    ) -> Self {
        let pooled: Option<PooledH2Stream> =
            H2_STREAM_POOL.with(|pool: &RefCell<Vec<PooledH2Stream>>| pool.borrow_mut().pop());
        let pair: PooledH2Stream = match pooled {
            Some((mut stream, state)) if *stream.get_request_config() == request_config => {
                if let Ok(mut guard) = state.lock() {
                    guard.reinit(send_response, peers, is_head);
                }
                stream.set_closed(false);
                (stream, state)
            }
            _ => {
                let state: Arc<Mutex<H2DirectState>> = Arc::new(Mutex::new(H2DirectState {
                    send_response: Some(send_response),
                    send_stream: None,
                    pending_head: Vec::new(),
                    pending_body: Vec::new(),
                    pending_body_offset: 0,
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
                    H2DirectTransport::from_shared(Arc::clone(&state)),
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
    /// - `Option<&mut Stream>` - The active HTTP/2 stream wrapper, or `None` once finalized.
    #[inline(always)]
    pub(crate) fn stream_mut(&mut self) -> Option<&mut Stream> {
        let Self { inner } = self;
        inner.as_mut().map(|pair: &mut PooledH2Stream| &mut pair.0)
    }

    /// Finalizes the HTTP/2 stream if end-of-stream was not yet emitted, then recycles it.
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
        H2_STREAM_POOL.with(|pool: &RefCell<Vec<PooledH2Stream>>| {
            let mut pool: RefMut<'_, Vec<PooledH2Stream>> = pool.borrow_mut();
            if pool.len() < MAX_POOLED_CONTEXTS {
                pool.push((stream, state));
            }
        });
    }
}

/// Returns the pooled HTTP/2 `Stream` to [`H2_STREAM_POOL`] after releasing stream handles.
impl Drop for H2StreamGuard {
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
        H2_STREAM_POOL.with(|pool: &RefCell<Vec<PooledH2Stream>>| {
            let mut pool: RefMut<'_, Vec<PooledH2Stream>> = pool.borrow_mut();
            if pool.len() < MAX_POOLED_CONTEXTS {
                pool.push((stream, state));
            }
        });
    }
}

/// Ensures an unfinished HTTP/2 stream is reset if dropped before shutdown.
impl Drop for H2DirectTransport {
    /// Resets the stream when dropped before end-of-stream was emitted.
    fn drop(&mut self) {
        if let Ok(mut state) = self.get_state().lock()
            && !state.finished
        {
            state.reset();
        }
    }
}

/// Implements [`Transport`] for [`H2DirectTransport`].
impl Transport for H2DirectTransport {
    /// Reports the remote address of the carrying TCP connection.
    ///
    /// # Returns
    ///
    /// - `Result<SocketAddr, io::Error>` - The peer socket address.
    fn peer_addr(&self) -> Result<SocketAddr, io::Error> {
        self.get_state()
            .lock()
            .map(|state: MutexGuard<'_, H2DirectState>| state.get_peers().remote)
            .map_err(|_| io::Error::other(MISSING_SEND_CAPACITY))
    }

    /// Reports the local address of the carrying TCP connection.
    ///
    /// # Returns
    ///
    /// - `Result<SocketAddr, io::Error>` - The local socket address.
    fn local_addr(&self) -> Result<SocketAddr, io::Error> {
        self.get_state()
            .lock()
            .map(|state: MutexGuard<'_, H2DirectState>| state.get_peers().local)
            .map_err(|_| io::Error::other(MISSING_SEND_CAPACITY))
    }
}

/// Implements [`AsyncRead`] for [`H2DirectTransport`].
impl AsyncRead for H2DirectTransport {
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

/// Implements [`AsyncWrite`] for [`H2DirectTransport`].
impl AsyncWrite for H2DirectTransport {
    /// Parses and frames written HTTP/1.1 bytes directly onto the HTTP/2 stream.
    ///
    /// # Arguments
    ///
    /// - `Pin<&mut Self>` - The pinned transport.
    /// - `&mut task::Context<'_>` - The task context.
    /// - `&[u8]` - The bytes written by the handler or middleware.
    ///
    /// # Returns
    ///
    /// - `Poll<io::Result<usize>>` - The number of bytes consumed.
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
        if state.pending_body_offset < state.pending_body.len() {
            match state.poll_drain_pending_body(context) {
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
                    let consumed_body: usize = match state.emit_head_and_inline_body(
                        context,
                        &bytes[..body_start],
                        &bytes[body_start..],
                    ) {
                        Ok(consumed) => consumed,
                        Err(error) => return Poll::Ready(Err(error)),
                    };
                    return Poll::Ready(Ok(body_start + consumed_body));
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
            let consumed_body: usize = match state.emit_head_and_inline_body(
                context,
                &pending_head[..body_start],
                &pending_head[body_start..],
            ) {
                Ok(consumed) => consumed,
                Err(error) => return Poll::Ready(Err(error)),
            };
            let trailing_start: usize = body_start + consumed_body;
            if !state.suppress_body && trailing_start < pending_head.len() {
                state
                    .pending_body
                    .extend_from_slice(&pending_head[trailing_start..]);
            }
            return Poll::Ready(Ok(accepted));
        }
        if state.suppress_body {
            return Poll::Ready(Ok(bytes.len()));
        }
        if check_body_budget(state.announced_len, state.body_len, bytes.len()) {
            return Poll::Ready(Err(state.fail(OVERFLOWING_RESPONSE_BODY)));
        }
        let current_body_len: usize = state.body_len;
        let announced_len: Option<usize> = state.announced_len;
        loop {
            let capacity: Poll<Option<Result<usize, H2Error>>> = {
                let Some(send_stream) = state.send_stream.as_mut() else {
                    return Poll::Ready(Err(state.fail(OVERFLOWING_RESPONSE_BODY)));
                };
                send_stream.reserve_capacity(bytes.len());
                send_stream.poll_capacity(context)
            };
            match capacity {
                Poll::Ready(Some(Ok(capacity))) if capacity > 0 => {
                    let take: usize = capacity.min(bytes.len());
                    let new_len: usize = current_body_len + take;
                    let is_final: bool = announced_len == Some(new_len);
                    let chunk: Bytes = Bytes::copy_from_slice(&bytes[..take]);
                    let Some(send_stream) = state.send_stream.as_mut() else {
                        return Poll::Ready(Err(state.fail(OVERFLOWING_RESPONSE_BODY)));
                    };
                    if let Err(error) = send_stream.send_data(chunk, is_final) {
                        return Poll::Ready(Err(state.fail(&error.to_string())));
                    }
                    state.body_len = new_len;
                    if is_final {
                        state.finished = true;
                        let _: Option<H2SendStream> = state.send_stream.take();
                    }
                    return Poll::Ready(Ok(take));
                }
                Poll::Ready(Some(Ok(_))) => continue,
                Poll::Pending => return Poll::Pending,
                Poll::Ready(Some(Err(error))) => {
                    return Poll::Ready(Err(state.fail(&error.to_string())));
                }
                Poll::Ready(None) => {
                    return Poll::Ready(Err(state.fail(MISSING_SEND_CAPACITY)));
                }
            }
        }
    }

    /// Flushes any buffered trailing body bytes onto the HTTP/2 stream.
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
        state.poll_drain_pending_body(context)
    }

    /// Validates the completed response and emits the final EOS flag if needed.
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
        match state.poll_drain_pending_body(context) {
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
            if let Some(mut send_stream) = state.send_stream.take()
                && let Err(error) = send_stream.send_data(Bytes::new(), true)
            {
                return Poll::Ready(Err(state.fail(&error.to_string())));
            }
        }
        Poll::Ready(Ok(()))
    }
}
