use super::*;

/// Implementation of `From` trait for converting `usize` address into `&Stream`.
impl From<usize> for &'static Stream {
    /// Converts a memory address into a reference to `Stream`.
    ///
    /// # Arguments
    ///
    /// - `usize` - The memory address of the `Stream` instance.
    ///
    /// # Returns
    ///
    /// - `&'static Stream` - A reference to the `Stream` at the given address.
    ///
    /// # Safety
    ///
    /// - The address is guaranteed to be a valid `Stream` instance
    ///   that was previously converted from a reference and is managed by the runtime.
    #[inline(always)]
    fn from(address: usize) -> &'static Stream {
        unsafe { &*(address as *const Stream) }
    }
}

/// Implementation of `From` trait for converting `usize` address into `&mut Stream`.
impl<'a> From<usize> for &'a mut Stream {
    /// Converts a memory address into a mutable reference to `Stream`.
    ///
    /// # Arguments
    ///
    /// - `usize` - The memory address of the `Stream` instance.
    ///
    /// # Returns
    ///
    /// - `&'a mut Stream` - A mutable reference to the `Stream` at the given address.
    ///
    /// # Safety
    ///
    /// - The address is guaranteed to be a valid `Stream` instance
    ///   that was previously converted from a reference and is managed by the runtime.
    #[inline(always)]
    fn from(address: usize) -> &'a mut Stream {
        unsafe { &mut *(address as *mut Stream) }
    }
}

/// Implementation of `From` trait for converting `&Stream` into `usize` address.
impl From<&Stream> for usize {
    /// Converts a reference to `Stream` into its memory address.
    ///
    /// # Arguments
    ///
    /// - `&Stream` - The reference to the `Stream` instance.
    ///
    /// # Returns
    ///
    /// - `usize` - The memory address of the `Stream` instance.
    #[inline(always)]
    fn from(stream: &Stream) -> Self {
        stream as *const Stream as usize
    }
}

/// Implementation of `From` trait for converting `&mut Stream` into `usize` address.
impl From<&mut Stream> for usize {
    /// Converts a mutable reference to `Stream` into its memory address.
    ///
    /// # Arguments
    ///
    /// - `&mut Stream` - The mutable reference to the `Stream` instance.
    ///
    /// # Returns
    ///
    /// - `usize` - The memory address of the `Stream` instance.
    #[inline(always)]
    fn from(stream: &mut Stream) -> Self {
        stream as *mut Stream as usize
    }
}

/// Implementation of `AsRef` trait for `Stream`.
impl AsRef<Stream> for Stream {
    /// Converts `&Stream` to `&Stream` via memory address conversion.
    ///
    /// # Returns
    ///
    /// - `&Self` - A reference to the `Stream` instance.
    #[inline(always)]
    fn as_ref(&self) -> &Self {
        let address: usize = self.into();
        address.into()
    }
}

/// Implementation of `AsMut` trait for `Stream`.
impl AsMut<Stream> for Stream {
    /// Converts `&mut Stream` to `&mut Stream` via memory address conversion.
    ///
    /// # Returns
    ///
    /// - `&mut Self` - A mutable reference to the `Stream` instance.
    #[inline(always)]
    fn as_mut(&mut self) -> &mut Self {
        let address: usize = self.into();
        address.into()
    }
}

/// Implementation of `Lifetime` trait for `Stream`.
impl Lifetime for Stream {
    /// Converts a reference to the stream into a `'static` reference.
    ///
    /// # Returns
    ///
    /// - `&'static Self` - A reference to the stream with a `'static` lifetime.
    ///
    /// # Safety
    ///
    /// - The address is guaranteed to be a valid `Self` instance
    ///   that was previously converted from a reference and is managed by the runtime.
    #[inline(always)]
    unsafe fn leak(&self) -> &'static Self {
        let address: usize = self.into();
        address.into()
    }

    /// Converts a reference to the stream into a `'static` mutable reference.
    ///
    /// # Returns
    ///
    /// - `&'static mut Self` - A mutable reference to the stream with a `'static` lifetime.
    ///
    /// # Safety
    ///
    /// - The address is guaranteed to be a valid `Self` instance
    ///   that was previously converted from a reference and is managed by the runtime.
    #[inline(always)]
    unsafe fn leak_mut(&self) -> &'static mut Self {
        let address: usize = self.into();
        address.into()
    }
}

/// Implementation of `Transport` trait for the plain TCP socket.
///
/// The socket is the reference transport: it already carries a socket
/// identity, so its addresses are reported directly.
impl Transport for TcpStream {
    /// Reports the address of the remote end of the socket.
    ///
    /// # Returns
    ///
    /// - `Result<SocketAddr, io::Error>` - The peer address of the socket.
    fn peer_addr(&self) -> Result<SocketAddr, io::Error> {
        TcpStream::peer_addr(self)
    }

    /// Reports the address of the local end of the socket.
    ///
    /// # Returns
    ///
    /// - `Result<SocketAddr, io::Error>` - The local address of the socket.
    fn local_addr(&self) -> Result<SocketAddr, io::Error> {
        TcpStream::local_addr(self)
    }
}

/// Implements [`Drop`] for [`PooledReader`].
impl Drop for PooledReader<'_> {
    /// Releases the buffer back to the pool for reuse by later requests.
    fn drop(&mut self) {
        let buffer: Vec<u8> = mem::take(self.get_mut_buffer());
        return_read_buffer(buffer);
    }
}

/// Implements non-blocking buffered reads for `PooledReader`.
///
/// Buffered bytes are served first; once drained, reads are delegated
/// directly to the underlying stream.
impl AsyncRead for PooledReader<'_> {
    /// Polls to read data into the provided buffer.
    ///
    /// # Arguments
    ///
    /// - `Pin<&mut Self>` - The pinned reader.
    /// - `&mut Context<'_>` - The task context.
    /// - `&mut ReadBuf<'_>` - The destination buffer.
    ///
    /// # Returns
    ///
    /// - `Poll<io::Result<()>>` - Ready when data was read or an error occurred.
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> Poll<io::Result<()>> {
        let this: &mut Self = self.get_mut();
        if this.get_start() < this.get_end() {
            let available: usize = this.get_end() - this.get_start();
            let amount: usize = available.min(buf.remaining());
            let end: usize = this.get_start() + amount;
            buf.put_slice(&this.get_buffer()[this.get_start()..end]);
            this.set_start(end);
            return Poll::Ready(Ok(()));
        }
        return_read_buffer(mem::take(this.get_mut_buffer()));
        Pin::new(&mut **this.get_mut_stream()).poll_read(cx, buf)
    }
}

/// Implements buffered-read support for `PooledReader`.
///
/// The internal buffer is refilled from the stream only when fully
/// consumed, so a single socket read serves multiple line parses.
impl AsyncBufRead for PooledReader<'_> {
    /// Polls to fill the internal buffer and returns the available data.
    ///
    /// # Arguments
    ///
    /// - `Pin<&mut Self>` - The pinned reader.
    /// - `&mut Context<'_>` - The task context.
    ///
    /// # Returns
    ///
    /// - `Poll<io::Result<&[u8]>>` - Ready with the unconsumed bytes, or an error.
    fn poll_fill_buf(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<&[u8]>> {
        let this: &mut Self = self.get_mut();
        let PooledReader {
            stream,
            buffer,
            capacity,
            start,
            end,
        } = this;
        if *start >= *end {
            *start = 0;
            *end = 0;
            if buffer.is_empty() {
                *buffer = take_read_buffer(*capacity);
            }
            let mut read_buf: ReadBuf<'_> = ReadBuf::new(buffer);
            match Pin::new(&mut **stream).poll_read(cx, &mut read_buf) {
                Poll::Ready(Ok(())) => {
                    *end = read_buf.filled().len();
                }
                Poll::Ready(Err(error)) => return Poll::Ready(Err(error)),
                Poll::Pending => {
                    return_read_buffer(mem::take(buffer));
                    return Poll::Pending;
                }
            }
        }
        Poll::Ready(Ok(&buffer[*start..*end]))
    }

    /// Marks the given number of bytes as consumed.
    ///
    /// # Arguments
    ///
    /// - `Pin<&mut Self>` - The pinned reader.
    /// - `usize` - The number of bytes to consume.
    fn consume(self: Pin<&mut Self>, amount: usize) {
        let this: &mut Self = self.get_mut();
        let new_start: usize = (this.get_start() + amount).min(this.get_end());
        this.set_start(new_start);
    }
}

/// Default construction for [`WebSocketReadState`].
impl Default for WebSocketReadState {
    /// Initializes WebSocket parsing state with an active heartbeat window.
    ///
    /// # Returns
    ///
    /// - `Self` - The initial WebSocket read state.
    #[inline(always)]
    fn default() -> Self {
        Self {
            buffer: Vec::new(),
            cursor: 0,
            full_frame: Vec::new(),
            fragment_opcode: None,
            is_client_response: true,
        }
    }
}

/// Inherent implementation of [`Stream`].
impl Stream {
    /// Boxes a transport into a `Stream` for the request pipeline.
    ///
    /// This is the single construction path: a plain TCP socket, a TLS session
    /// and a per-request duplex stream all enter here, so nothing above the
    /// transport line has to know which one it is serving. The transport is
    /// type erased immediately, which keeps `Stream` a fixed size regardless of
    /// the concrete protocol adapter behind it.
    ///
    /// # Arguments
    ///
    /// - `T` - The transport type to serve the request cycle over.
    /// - `RequestConfig` - The parsing limits and read timeout for the cycle.
    /// - `bool` - Whether the connection starts already closed.
    ///
    /// # Returns
    ///
    /// - `Self` - A stream serving bytes through the boxed transport.
    pub fn from_transport<T>(transport: T, request_config: RequestConfig, closed: bool) -> Self
    where
        T: Transport + 'static,
    {
        Self {
            stream: Box::new(transport),
            request_config,
            closed,
            leftover: Vec::new(),
            leftover_start: 0,
            leftover_end: 0,
            line_buffer: String::new(),
            header_line_buffer: String::new(),
            websocket_state: None,
            read_timer: None,
        }
    }

    /// Boxes a TCP socket into a `Stream` for the request pipeline.
    ///
    /// Kept as the socket-facing constructor so the accept loop keeps its
    /// existing call shape; it delegates to `from_transport`, so there is one
    /// construction path rather than two.
    ///
    /// # Arguments
    ///
    /// - `TcpStream` - The accepted socket to serve the request cycle over.
    /// - `RequestConfig` - The parsing limits and read timeout for the cycle.
    /// - `bool` - Whether the connection starts already closed.
    ///
    /// # Returns
    ///
    /// - `Self` - A stream serving bytes through the boxed socket.
    pub fn new(stream: TcpStream, request_config: RequestConfig, closed: bool) -> Self {
        Self::from_transport(stream, request_config, closed)
    }

    /// Reports the address of the remote end of the transport.
    ///
    /// Delegates to the transport itself, so a transport without a socket
    /// identity answers with an error rather than a fabricated address.
    ///
    /// # Returns
    ///
    /// - `Result<SocketAddr, io::Error>` - The peer address reported by the transport.
    #[inline(always)]
    pub fn get_peer_addr(&self) -> Result<SocketAddr, io::Error> {
        self.get_stream().peer_addr()
    }

    /// Reports the address of the local end of the transport.
    ///
    /// Delegates to the transport itself, so a transport without a socket
    /// identity answers with an error rather than a fabricated address.
    ///
    /// # Returns
    ///
    /// - `Result<SocketAddr, io::Error>` - The local address reported by the transport.
    #[inline(always)]
    pub fn get_local_addr(&self) -> Result<SocketAddr, io::Error> {
        self.get_stream().local_addr()
    }

    /// Shuts the transport down.
    ///
    /// The transport itself is crate-internal, so a caller that needs to
    /// close the connection reaches it through this method rather than
    /// through the field.
    ///
    /// # Returns
    ///
    /// - `Result<(), io::Error>` - Whether the transport shut down.
    #[inline(always)]
    pub async fn shutdown(&mut self) -> Result<(), io::Error> {
        self.get_mut_stream().shutdown().await
    }

    /// Checks if the connection should be kept alive.
    ///
    /// This method evaluates whether the connection should remain open based on
    /// the closed state and the keep_alive parameter.
    ///
    /// # Arguments
    ///
    /// - `bool` - Whether keep-alive is enabled for the request.
    ///
    /// # Returns
    ///
    /// - `bool` - True if the connection should be kept alive, otherwise false.
    #[inline(always)]
    pub fn is_keep_alive(&self, keep_alive: bool) -> bool {
        !self.get_closed() && keep_alive
    }

    /// Parses the HTTP request content from the stream into the given request.
    ///
    /// The request is reset first, then filled in place so its existing
    /// allocations are reused across keep-alive requests.
    ///
    /// # Arguments
    ///
    /// - `&mut Request` - The request object to fill.
    ///
    /// # Returns
    ///
    /// - `Result<(), RequestError>` - Ok on success, or an error if parsing fails.
    async fn fill_http_from_stream(&mut self, request: &mut Request) -> Result<(), RequestError> {
        request.reset();
        let config: RequestConfig = *self.get_request_config();
        let buffer_size: usize = config.get_buffer_size();
        let max_path_size: usize = config.get_max_path_size();
        // Move read-ahead storage and its valid window into the reader instead
        // of allocating and copying the remaining pipeline for every request.
        let start: usize = self.get_leftover_start();
        let end: usize = self.get_leftover_end();
        let buffer: Vec<u8> = mem::take(self.get_mut_leftover());
        self.set_leftover_start(0);
        self.set_leftover_end(0);
        // Split the borrow of the stream: the reader takes the transport, the
        // line buffers are separate fields, so both can be held at once.
        let Stream {
            stream,
            line_buffer,
            header_line_buffer,
            ..
        } = self;
        let mut reader: PooledReader<'_> = PooledReader::new(stream, buffer, buffer_size);
        reader.set_start(start);
        reader.set_end(end);
        line_buffer.clear();
        let filled_len: usize = future::poll_fn(|cx: &mut Context<'_>| {
            Pin::new(&mut reader).poll_fill_buf(cx).map_ok(<[u8]>::len)
        })
        .await?;
        if filled_len == 0 {
            return Err(RequestError::ClientClosedConnection(HttpStatus::BadRequest));
        }
        let fast_framing: Option<RequestFraming> = {
            let slice: &[u8] = &reader.get_buffer()[reader.get_start()..reader.get_end()];
            match request.parse_http_head_from_slice(slice, &config, line_buffer)? {
                Some((framing, consumed)) => {
                    Pin::new(&mut reader).consume(consumed);
                    Some(framing)
                }
                None => None,
            }
        };
        let framing: RequestFraming = match fast_framing {
            Some(framing) => framing,
            None => {
                let max_request_line: usize = if max_path_size == DEFAULT_LOW_SECURITY_MAX_PATH_SIZE
                {
                    usize::MAX
                } else {
                    max_path_size.saturating_add(HTTP_REQUEST_LINE_OVERHEAD)
                };
                let line_len: usize = read_http_line_bounded(
                    &mut reader,
                    line_buffer,
                    max_request_line,
                    RequestError::RequestTooLong(HttpStatus::URITooLong),
                )
                .await?;
                if line_len == 0 {
                    return Err(RequestError::ClientClosedConnection(HttpStatus::BadRequest));
                }
                let (method, path, version): (RequestMethod, &str, RequestVersion) =
                    Request::get_http_first_line(line_buffer)?;
                Request::check_http_path_size(path, max_path_size)?;
                let (query_index, hash_index): (Option<usize>, Option<usize>) =
                    Request::find_path_separators(path);
                let query: &str = Request::get_http_query(path, query_index, hash_index);
                let max_query_count: usize = if max_path_size == DEFAULT_LOW_SECURITY_MAX_PATH_SIZE
                {
                    usize::MAX
                } else {
                    HTTP_MAX_QUERY_PARAM_COUNT
                };
                Request::fill_http_querys(query, request.get_mut_querys(), max_query_count)?;
                let path_slice: &str = Request::get_http_path(path, query_index, hash_index);
                request.get_mut_path().push_str(path_slice);
                request.set_version(version);
                let framing: RequestFraming = request
                    .get_http_headers(&mut reader, &config, header_line_buffer)
                    .await?;
                request.set_method(method);
                framing
            }
        };
        match framing {
            RequestFraming::Fixed(0) => {}
            RequestFraming::Fixed(content_size) => {
                let buffered: usize = reader.get_end() - reader.get_start();
                if content_size <= buffered {
                    let body: &mut RequestBody = request.get_mut_body();
                    body.clear();
                    let body_start: usize = reader.get_start();
                    body.extend_from_slice(
                        &reader.get_buffer()[body_start..body_start + content_size],
                    );
                    reader.set_start(body_start + content_size);
                } else {
                    if expects_http_continue(request) {
                        let transport: &mut Box<dyn Transport> = reader.get_mut_stream();
                        transport.write_all(HTTP_CONTINUE_RESPONSE).await?;
                        transport.flush().await?;
                    }
                    Request::fill_http_body(&mut reader, request.get_mut_body(), content_size)
                        .await?;
                }
            }
            RequestFraming::Chunked => {
                Box::pin(fill_chunked_request(&mut reader, request, &config)).await?;
            }
        }
        // Preserve bytes read ahead for the next pipelined request: they have
        // already left the socket and cannot be read again.
        let reader_start: usize = reader.get_start();
        let reader_end: usize = reader.get_end();
        let pending: Vec<u8> = if reader_start < reader_end {
            mem::take(reader.get_mut_buffer())
        } else {
            Vec::new()
        };
        drop(reader);
        let pending_start: usize = if pending.is_empty() { 0 } else { reader_start };
        let pending_end: usize = if pending.is_empty() { 0 } else { reader_end };
        self.set_leftover(pending);
        self.set_leftover_start(pending_start);
        self.set_leftover_end(pending_end);
        Ok(())
    }

    /// Parses an HTTP request from a TCP stream into the given request.
    ///
    /// The request is reset and filled in place, reusing its allocations.
    /// An HTTP/1.1 continue expectation can write an interim response after
    /// header validation and before waiting for the body. A chunked body is
    /// decoded, with trailers discarded and its content length normalized.
    /// If the timeout is DEFAULT_LOW_SECURITY_READ_TIMEOUT_MS, no timeout is applied.
    ///
    /// # Arguments
    ///
    /// - `&mut Request` - The request object to reset and fill.
    ///
    /// # Returns
    ///
    /// - `Result<(), RequestError>` - Ok on success, or an error if parsing fails.
    pub async fn try_fill_http_request(
        &mut self,
        request: &mut Request,
    ) -> Result<(), RequestError> {
        if self.get_closed() {
            return Err(RequestError::ServerClosedConnection(HttpStatus::BadRequest));
        }
        let timeout_ms: u64 = self.get_request_config().get_read_timeout_ms();
        if timeout_ms == DEFAULT_LOW_SECURITY_READ_TIMEOUT_MS {
            return self.fill_http_from_stream(request).await;
        }
        let duration: Duration = Duration::from_millis(timeout_ms);
        let mut timer_slot: Option<Pin<Box<Sleep>>> = {
            let Stream { read_timer, .. } = self;
            read_timer.take()
        };
        let outcome: Option<Result<(), RequestError>> = {
            let mut reading: Pin<&mut _> = pin!(self.fill_http_from_stream(request));
            poll_with_timeout(reading.as_mut(), &mut timer_slot, duration).await
        };
        let Stream { read_timer, .. } = self;
        *read_timer = timer_slot;
        match outcome {
            Some(result) => result,
            None => Err(RequestError::ReadTimeout(HttpStatus::RequestTimeout)),
        }
    }

    /// Parses an HTTP request from a TCP stream.
    ///
    /// Wraps the stream in a buffered reader and delegates to `http_from_reader`.
    /// If the timeout is DEFAULT_LOW_SECURITY_READ_TIMEOUT_MS, no timeout is applied.
    ///
    /// # Returns
    ///
    /// - `Result<Request, RequestError>` - The parsed request or an error.
    pub async fn try_get_http_request(&mut self) -> Result<Request, RequestError> {
        let mut request: Request = Request::default();
        self.try_fill_http_request(&mut request).await?;
        Ok(request)
    }

    /// Parses a WebSocket request from a TCP stream.
    ///
    /// Wraps the stream in a buffered reader and delegates to `ws_from_reader`.
    /// If the timeout is DEFAULT_LOW_SECURITY_READ_TIMEOUT_MS, no timeout is applied.
    ///
    /// # Returns
    ///
    /// - `Result<RequestBody, RequestError>` - The parsed WebSocket request or an error.
    pub async fn try_get_websocket_request(&mut self) -> Result<RequestBody, RequestError> {
        if self.get_closed() {
            return self.reset_websocket_state_on_error(Err(RequestError::ServerClosedConnection(
                HttpStatus::BadRequest,
            )));
        }
        let config: RequestConfig = *self.get_request_config();
        let read_timeout_ms: u64 = config.get_read_timeout_ms();
        if self.get_mut_websocket_state().is_none() {
            let mut buffer: Vec<u8> = mem::take(self.get_mut_leftover());
            buffer.truncate(self.get_leftover_end());
            buffer.drain(..self.get_leftover_start());
            self.set_leftover_start(0);
            self.set_leftover_end(0);
            self.set_websocket_state(Some(Box::new(WebSocketReadState {
                buffer,
                ..WebSocketReadState::default()
            })));
        }
        let duration_opt: Option<Duration> =
            if read_timeout_ms == DEFAULT_LOW_SECURITY_READ_TIMEOUT_MS {
                None
            } else {
                let adjusted_timeout_ms: u64 = (read_timeout_ms >> 1) + (read_timeout_ms & 1);
                Some(Duration::from_millis(adjusted_timeout_ms))
            };
        loop {
            let parsed: Result<WebSocketParseStep, RequestError> =
                self.try_parse_websocket_request();
            match self.reset_websocket_state_on_error(parsed)? {
                WebSocketParseStep::Message(body) => return Ok(body),
                WebSocketParseStep::Ping(payload) => {
                    let mut pong: Vec<u8> = Vec::with_capacity(2 + payload.len());
                    pong.push(0x8A);
                    pong.push(payload.len() as u8);
                    pong.extend_from_slice(&payload);
                    let send_result: Result<(), RequestError> =
                        self.try_send(&pong).await.map_err(Into::into);
                    self.reset_websocket_state_on_error(send_result)?;
                    let flush_result: Result<(), RequestError> =
                        self.try_flush().await.map_err(Into::into);
                    self.reset_websocket_state_on_error(flush_result)?;
                    continue;
                }
                WebSocketParseStep::NeedMore => {}
            }
            let read: Result<Option<usize>, RequestError> =
                self.get_websocket_from_stream(duration_opt).await;
            if matches!(self.reset_websocket_state_on_error(read)?, Some(0)) {
                return self.reset_websocket_state_on_error(Err(
                    RequestError::IncompleteWebSocketFrame(HttpStatus::BadRequest),
                ));
            }
        }
    }

    /// Discards cached parsing state only after an operation completes with an error.
    ///
    /// # Arguments
    ///
    /// - `Result<T, RequestError>` - The completed parsing or reading operation.
    ///
    /// # Returns
    ///
    /// - `Result<T, RequestError>` - The unchanged result after releasing failed parsing state.
    #[inline(always)]
    fn reset_websocket_state_on_error<T>(
        &mut self,
        result: Result<T, RequestError>,
    ) -> Result<T, RequestError> {
        if result.is_err() {
            drop(self.get_mut_websocket_state().take());
        }
        result
    }

    /// Parses buffered frames without discarding incomplete or subsequent input.
    ///
    /// Enforces RFC 6455 control frame constraints, continuation sequencing, UTF-8 validity
    /// for text messages, close-code validity, and the configured `max_body_size` cap.
    ///
    /// # Returns
    ///
    /// - `Result<WebSocketParseStep, RequestError>` - A complete message, a ping requiring pong, pending input, or a protocol error.
    fn try_parse_websocket_request(&mut self) -> Result<WebSocketParseStep, RequestError> {
        let max_body_size: usize = self.get_request_config().get_max_body_size();
        let state: &mut WebSocketReadState = self.get_mut_websocket_state().as_deref_mut().ok_or(
            RequestError::IncompleteWebSocketFrame(HttpStatus::BadRequest),
        )?;
        let WebSocketReadState {
            buffer,
            cursor,
            full_frame,
            fragment_opcode,
            is_client_response,
        }: &mut WebSocketReadState = state;
        while let Some((frame, consumed)) =
            WebSocketFrame::decode_ws_frame_checked(&buffer[*cursor..], max_body_size)?
        {
            *is_client_response = true;
            *cursor += consumed;
            if *cursor >= buffer.len() {
                *cursor = 0;
                let mut recycled: Vec<u8> = mem::take(buffer);
                recycled.resize(recycled.capacity().min(MAX_POOLED_READ_BUFFER_SIZE), 0);
                return_read_buffer(recycled);
            } else if *cursor >= MAX_POOLED_READ_BUFFER_SIZE {
                buffer.drain(..*cursor);
                *cursor = 0;
            }
            match frame.get_opcode() {
                WebSocketOpcode::Close => {
                    let payload: &[u8] = frame.get_payload_data();
                    if payload.len() == 1 {
                        return Err(RequestError::WebSocketInvalidCloseCode(
                            HttpStatus::BadRequest,
                        ));
                    }
                    if payload.len() >= 2 {
                        let code: u16 = u16::from_be_bytes([payload[0], payload[1]]);
                        let is_std_close: bool =
                            (1000..=1003).contains(&code) || (1007..=1011).contains(&code);
                        let valid_code: bool = is_std_close || (3000..=4999).contains(&code);
                        if !valid_code {
                            return Err(RequestError::WebSocketInvalidCloseCode(
                                HttpStatus::BadRequest,
                            ));
                        }
                        if from_utf8(&payload[2..]).is_err() {
                            return Err(RequestError::WebSocketInvalidUtf8(HttpStatus::BadRequest));
                        }
                    }
                    return Err(RequestError::ClientClosedConnection(HttpStatus::BadRequest));
                }
                WebSocketOpcode::Ping => {
                    let WebSocketFrame { payload_data, .. }: WebSocketFrame = frame;
                    return Ok(WebSocketParseStep::Ping(payload_data));
                }
                WebSocketOpcode::Pong => continue,
                WebSocketOpcode::Text | WebSocketOpcode::Binary => {
                    if fragment_opcode.is_some() {
                        return Err(RequestError::WebSocketPayloadCorrupted(
                            HttpStatus::BadRequest,
                        ));
                    }
                    let is_text: bool = frame.is_text_opcode();
                    if frame.get_fin() {
                        let WebSocketFrame { payload_data, .. }: WebSocketFrame = frame;
                        if is_text && from_utf8(&payload_data).is_err() {
                            return Err(RequestError::WebSocketInvalidUtf8(HttpStatus::BadRequest));
                        }
                        return Ok(WebSocketParseStep::Message(payload_data));
                    }
                    let opcode: WebSocketOpcode = *frame.get_opcode();
                    let payload: &[u8] = frame.get_payload_data();
                    if max_body_size != DEFAULT_LOW_SECURITY_MAX_BODY_SIZE
                        && payload.len() > max_body_size
                    {
                        return Err(RequestError::WebSocketFrameTooLarge(
                            HttpStatus::PayloadTooLarge,
                        ));
                    }
                    full_frame.clear();
                    full_frame.extend_from_slice(payload);
                    *fragment_opcode = Some(opcode);
                }
                WebSocketOpcode::Continuation => {
                    let Some(active_opcode) = fragment_opcode.as_ref() else {
                        return Err(RequestError::WebSocketPayloadCorrupted(
                            HttpStatus::BadRequest,
                        ));
                    };
                    let is_text: bool = active_opcode.is_text();
                    let payload: &[u8] = frame.get_payload_data();
                    if max_body_size != DEFAULT_LOW_SECURITY_MAX_BODY_SIZE
                        && payload.len() > max_body_size.saturating_sub(full_frame.len())
                    {
                        return Err(RequestError::WebSocketFrameTooLarge(
                            HttpStatus::PayloadTooLarge,
                        ));
                    }
                    full_frame.extend_from_slice(payload);
                    if frame.get_fin() {
                        *fragment_opcode = None;
                        let body: RequestBody = mem::take(full_frame);
                        if is_text && from_utf8(&body).is_err() {
                            return Err(RequestError::WebSocketInvalidUtf8(HttpStatus::BadRequest));
                        }
                        return Ok(WebSocketParseStep::Message(body));
                    }
                }
                WebSocketOpcode::Reserved(_) => {
                    return Err(RequestError::WebSocketOpcodeUnsupported(
                        HttpStatus::NotImplemented,
                    ));
                }
            }
        }
        Ok(WebSocketParseStep::NeedMore)
    }

    /// Polls a WebSocket read using a buffer retained only while data is ready.
    ///
    /// # Arguments
    ///
    /// - `&mut Context<'_>` - The registered task context.
    ///
    /// # Returns
    ///
    /// - `Poll<io::Result<usize>>` - The bytes appended to persistent parsing state.
    fn poll_websocket_read(&mut self, cx: &mut Context<'_>) -> Poll<io::Result<usize>> {
        let capacity: usize = self.get_request_config().get_buffer_size();
        let max_body_size: usize = self.get_request_config().get_max_body_size();
        if let Some(state) = self.try_get_websocket_state().as_deref() {
            let unread: usize = state.get_buffer().len().saturating_sub(state.get_cursor());
            if max_body_size != DEFAULT_LOW_SECURITY_MAX_BODY_SIZE
                && unread > max_body_size.saturating_add(14)
            {
                return Poll::Ready(Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    WEBSOCKET_FRAME_TOO_LARGE_ERROR,
                )));
            }
        }
        let mut temporary: Vec<u8> = take_read_buffer(capacity);
        let mut read_buffer: ReadBuf<'_> = ReadBuf::new(&mut temporary[..capacity]);
        match Pin::new(&mut **self.get_mut_stream()).poll_read(cx, &mut read_buffer) {
            Poll::Ready(Ok(())) => {
                let len: usize = read_buffer.filled().len();
                temporary.truncate(len);
                let state: &mut WebSocketReadState =
                    match self.get_mut_websocket_state().as_deref_mut() {
                        Some(state) => state,
                        None => {
                            return_read_buffer(temporary);
                            return Poll::Ready(Err(io::Error::new(
                                io::ErrorKind::InvalidData,
                                WEBSOCKET_STATE_ERROR,
                            )));
                        }
                    };
                let cursor: usize = state.get_cursor();
                if cursor >= state.get_buffer().len() {
                    state.set_cursor(0);
                    state.get_mut_buffer().clear();
                } else if cursor > 0 {
                    state.get_mut_buffer().drain(..cursor);
                    state.set_cursor(0);
                }
                if state.get_buffer().is_empty() {
                    mem::swap(state.get_mut_buffer(), &mut temporary);
                } else {
                    state.get_mut_buffer().extend_from_slice(&temporary);
                }
                return_read_buffer(temporary);
                Poll::Ready(Ok(len))
            }
            Poll::Ready(Err(error)) => {
                return_read_buffer(temporary);
                Poll::Ready(Err(error))
            }
            Poll::Pending => {
                return_read_buffer(temporary);
                Poll::Pending
            }
        }
    }

    /// Reads data from the stream with optional timeout handling.
    ///
    /// # Arguments
    ///
    /// - `Option<Duration>` - The timeout interval, or no timeout.
    ///
    /// # Returns
    ///
    /// - `Result<Option<usize>, RequestError>` - Bytes read, a heartbeat, or an error.
    pub(crate) async fn get_websocket_from_stream(
        &mut self,
        duration_opt: Option<Duration>,
    ) -> Result<Option<usize>, RequestError> {
        if let Some(duration) = duration_opt {
            return match timeout(
                duration,
                future::poll_fn(|cx: &mut Context<'_>| self.poll_websocket_read(cx)),
            )
            .await
            {
                Ok(result) => result.map(Some).map_err(Into::into),
                Err(error) => {
                    let state: &mut WebSocketReadState =
                        self.get_mut_websocket_state().as_deref_mut().ok_or(
                            RequestError::IncompleteWebSocketFrame(HttpStatus::BadRequest),
                        )?;
                    if !state.get_is_client_response() {
                        return Err(error.into());
                    }
                    state.set_is_client_response(false);
                    self.try_send(&PING_FRAME).await?;
                    Ok(None)
                }
            };
        }
        future::poll_fn(|cx: &mut Context<'_>| self.poll_websocket_read(cx))
            .await
            .map(Some)
            .map_err(Into::into)
    }

    /// Sends data over the stream.
    ///
    /// # Arguments
    ///
    /// - `D` - The data to send, which must implement `AsRef<[u8]>`.
    ///
    /// # Returns
    ///
    /// - `Result<(), ResponseError>` - Result indicating success or failure.
    pub async fn try_send<D>(&mut self, data: D) -> Result<(), ResponseError>
    where
        D: AsRef<[u8]>,
    {
        if self.get_closed() {
            return Err(ResponseError::ConnectionClosed);
        }
        Ok(self.get_mut_stream().write_all(data.as_ref()).await?)
    }

    /// Sends data over the stream.
    ///
    /// # Arguments
    ///
    /// - `D` - The data to send, which must implement `AsRef<[u8]>`.
    ///
    /// # Panics
    ///
    /// Panics if the write operation fails.
    pub async fn send<D>(&mut self, data: D)
    where
        D: AsRef<[u8]>,
    {
        self.try_send(data).await.unwrap();
    }

    /// Sends multiple data.
    ///
    /// # Arguments
    ///
    /// - `I` - The data list to send, which must implement `IntoIterator<Item = D>` with
    ///   `D` requiring the `AsRef<[u8]>` bound.
    ///
    /// # Returns
    ///
    /// - `Result<(), ResponseError>` - Result indicating success or failure.
    pub async fn try_send_list<I, D>(&mut self, data_iter: I) -> Result<(), ResponseError>
    where
        I: IntoIterator<Item = D>,
        D: AsRef<[u8]>,
    {
        if self.get_closed() {
            return Err(ResponseError::ConnectionClosed);
        }
        let stream: &mut Box<dyn Transport> = self.get_mut_stream();
        for data in data_iter {
            stream.write_all(data.as_ref()).await?;
        }
        Ok(())
    }

    /// Sends multiple data.
    ///
    /// # Arguments
    ///
    /// - `I` - The data list to send, which must implement `IntoIterator<Item = D>` with
    ///   `D` requiring the `AsRef<[u8]>` bound.
    ///
    /// # Panics
    ///
    /// Panics if any write operation fails.
    pub async fn send_list<I, D>(&mut self, data_iter: I)
    where
        I: IntoIterator<Item = D>,
        D: AsRef<[u8]>,
    {
        self.try_send_list(data_iter).await.unwrap();
    }

    /// Flushes all buffered data to the stream.
    ///
    /// # Returns
    ///
    /// - `Result<(), ResponseError>` - Result indicating success or failure.
    pub async fn try_flush(&mut self) -> Result<(), ResponseError> {
        if self.get_closed() {
            return Err(ResponseError::ConnectionClosed);
        }
        Ok(self.get_mut_stream().flush().await?)
    }

    /// Flushes all buffered data to the stream.
    ///
    /// # Panics
    ///
    /// Panics if the flush operation fails.
    pub async fn flush(&mut self) {
        self.try_flush().await.unwrap();
    }
}
