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
            line_buffer: String::with_capacity(REQUEST_LINE_BUFFER_CAPACITY),
            header_line_buffer: String::with_capacity(HEADER_LINE_BUFFER_CAPACITY),
            websocket_state: None,
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
        AsyncBufReadExt::read_line(&mut reader, line_buffer).await?;
        let (method, path, version): (RequestMethod, &str, RequestVersion) =
            Request::get_http_first_line(line_buffer)?;
        Request::check_http_path_size(path, max_path_size)?;
        let hash_index: Option<usize> = path.find(HASH);
        let query_index: Option<usize> = path.find(QUERY);
        let query: &str = Request::get_http_query(path, query_index, hash_index);
        Request::fill_http_querys(query, request.get_mut_querys());
        let path_slice: &str = Request::get_http_path(path, query_index, hash_index);
        request.get_mut_path().push_str(path_slice);
        request.set_version(version);
        let content_size: usize = request
            .get_http_headers(&mut reader, &config, header_line_buffer)
            .await?;
        request.set_method(method);
        Request::fill_http_body(&mut reader, request.get_mut_body(), content_size).await?;
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
        let mut reading: Pin<&mut _> = pin!(self.fill_http_from_stream(request));
        // A request whose bytes have already arrived is read on the first
        // poll, so the ordinary case never arms a timer at all. Arming one
        // costs a timer registration and a cancellation per request, and that
        // churn is the most expensive thing a request path can do once the
        // concurrency is high enough for the timer driver to become a
        // bottleneck. The timer is armed only when the read really has to
        // wait, which is exactly when the timeout can still fire.
        if let Some(result) = poll_once(reading.as_mut()).await {
            return result;
        }
        timeout(duration, reading).await?
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
            let parsed: Result<Option<RequestBody>, RequestError> =
                self.try_parse_websocket_request();
            if let Some(body) = self.reset_websocket_state_on_error(parsed)? {
                return Ok(body);
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
    /// # Returns
    ///
    /// - `Result<Option<RequestBody>, RequestError>` - A complete message, pending input, or a protocol error.
    fn try_parse_websocket_request(&mut self) -> Result<Option<RequestBody>, RequestError> {
        let state: &mut WebSocketReadState = self.get_mut_websocket_state().as_deref_mut().ok_or(
            RequestError::IncompleteWebSocketFrame(HttpStatus::BadRequest),
        )?;
        let WebSocketReadState {
            buffer,
            full_frame,
            is_client_response,
        }: &mut WebSocketReadState = state;
        while let Some((frame, consumed)) = WebSocketFrame::decode_ws_frame(&buffer[..]) {
            *is_client_response = true;
            buffer.drain(..consumed);
            if buffer.is_empty() {
                let mut recycled: Vec<u8> = mem::take(buffer);
                recycled.resize(recycled.capacity().min(MAX_POOLED_READ_BUFFER_SIZE), 0);
                return_read_buffer(recycled);
            }
            match frame.get_opcode() {
                WebSocketOpcode::Close => {
                    return Err(RequestError::ClientClosedConnection(HttpStatus::BadRequest));
                }
                WebSocketOpcode::Ping | WebSocketOpcode::Pong => continue,
                WebSocketOpcode::Text | WebSocketOpcode::Binary => {
                    if frame.get_fin() && full_frame.is_empty() {
                        let WebSocketFrame { payload_data, .. }: WebSocketFrame = frame;
                        return Ok(Some(payload_data));
                    }
                    if let Some(body) = frame.build_full_frame(full_frame)? {
                        return Ok(Some(body));
                    }
                }
                _ => {
                    return Err(RequestError::WebSocketOpcodeUnsupported(
                        HttpStatus::NotImplemented,
                    ));
                }
            }
        }
        Ok(None)
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
