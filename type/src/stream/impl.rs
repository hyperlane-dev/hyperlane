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
    /// - `&mut Stream` - A mutable reference to the `Stream` at the given address.
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
    /// - `&Stream` - A reference to the `Stream` instance.
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
    /// - `&mut Stream` - A mutable reference to the `Stream` instance.
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

/// Creates a new `PooledReader` wrapping the given stream and buffer.
impl<'a> PooledReader<'a> {
    /// Creates a new `PooledReader` over the given stream.
    ///
    /// # Arguments
    ///
    /// - `&'a mut TcpStream` - The TCP stream to read from.
    /// - `Vec<u8>` - The read buffer; its full length is used as capacity.
    ///
    /// # Returns
    ///
    /// - `Self` - A reader with an empty valid-data region.
    pub(crate) fn new(stream: &'a mut TcpStream, buffer: Vec<u8>) -> Self {
        Self {
            stream,
            buffer,
            start: 0,
            end: 0,
        }
    }
}

/// Returns the read buffer to the thread-local pool when the reader is dropped.
impl Drop for PooledReader<'_> {
    /// Releases the buffer back to the pool for reuse by later requests.
    fn drop(&mut self) {
        let buffer: Vec<u8> = mem::take(&mut self.buffer);
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
        if this.start < this.end {
            let available: usize = this.end - this.start;
            let amount: usize = available.min(buf.remaining());
            let end: usize = this.start + amount;
            buf.put_slice(&this.buffer[this.start..end]);
            this.start = end;
            return Poll::Ready(Ok(()));
        }
        Pin::new(&mut *this.stream).poll_read(cx, buf)
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
        if this.start >= this.end {
            this.start = 0;
            this.end = 0;
            let mut read_buf: ReadBuf<'_> = ReadBuf::new(&mut this.buffer);
            match Pin::new(&mut *this.stream).poll_read(cx, &mut read_buf) {
                Poll::Ready(Ok(())) => {
                    this.end = read_buf.filled().len();
                }
                Poll::Ready(Err(error)) => return Poll::Ready(Err(error)),
                Poll::Pending => return Poll::Pending,
            }
        }
        Poll::Ready(Ok(&this.buffer[this.start..this.end]))
    }

    /// Marks the given number of bytes as consumed.
    ///
    /// # Arguments
    ///
    /// - `Pin<&mut Self>` - The pinned reader.
    /// - `usize` - The number of bytes to consume.
    fn consume(self: Pin<&mut Self>, amount: usize) {
        let this: &mut Self = self.get_mut();
        this.start = (this.start + amount).min(this.end);
    }
}

impl Stream {
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
        let buffer: Vec<u8> = take_read_buffer(buffer_size);
        let mut reader: PooledReader<'_> = PooledReader::new(self.get_mut_stream(), buffer);
        let mut line: String = String::with_capacity(REQUEST_LINE_BUFFER_CAPACITY);
        AsyncBufReadExt::read_line(&mut reader, &mut line).await?;
        let (method, path, version): (RequestMethod, &str, RequestVersion) =
            Request::get_http_first_line(&line)?;
        Request::check_http_path_size(path, max_path_size)?;
        let hash_index: Option<usize> = path.find(HASH);
        let query_index: Option<usize> = path.find(QUERY);
        let query: &str = Request::get_http_query(path, query_index, hash_index);
        Request::fill_http_querys(query, &mut request.querys);
        let path_slice: &str = Request::get_http_path(path, query_index, hash_index);
        request.path.push_str(path_slice);
        let content_size: usize = Request::get_http_headers(
            &mut reader,
            &config,
            &mut request.headers,
            &mut request.host,
        )
        .await?;
        request.method = method;
        request.version = version;
        Request::fill_http_body(&mut reader, &mut request.body, content_size).await?;
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
        timeout(duration, self.fill_http_from_stream(request)).await?
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
    /// - `Result<Request, RequestError>` - The parsed WebSocket request or an error.
    pub async fn try_get_websocket_request(&mut self) -> Result<RequestBody, RequestError> {
        if self.get_closed() {
            return Err(RequestError::ServerClosedConnection(HttpStatus::BadRequest));
        }
        let config: RequestConfig = *self.get_request_config();
        let buffer_size: usize = config.get_buffer_size();
        let read_timeout_ms: u64 = config.get_read_timeout_ms();
        let mut dynamic_buffer: Vec<u8> = Vec::with_capacity(buffer_size);
        let mut temp_buffer: Vec<u8> = vec![0; buffer_size];
        let mut full_frame: Vec<u8> = Vec::new();
        let mut is_client_response: bool = false;
        let duration_opt: Option<Duration> =
            if read_timeout_ms == DEFAULT_LOW_SECURITY_READ_TIMEOUT_MS {
                None
            } else {
                let adjusted_timeout_ms: u64 = (read_timeout_ms >> 1) + (read_timeout_ms & 1);
                Some(Duration::from_millis(adjusted_timeout_ms))
            };
        loop {
            let len: usize = match self
                .get_websocket_from_stream(&mut temp_buffer, duration_opt, &mut is_client_response)
                .await
            {
                Ok(Some(len)) => len,
                Ok(None) => continue,
                Err(error) => return Err(error),
            };
            if len == 0 {
                return Err(RequestError::IncompleteWebSocketFrame(
                    HttpStatus::BadRequest,
                ));
            }
            dynamic_buffer.extend_from_slice(&temp_buffer[..len]);
            while let Some((frame, consumed)) = WebSocketFrame::decode_ws_frame(&dynamic_buffer) {
                is_client_response = true;
                dynamic_buffer.drain(0..consumed);
                match frame.get_opcode() {
                    WebSocketOpcode::Close => {
                        return Err(RequestError::ClientClosedConnection(HttpStatus::BadRequest));
                    }
                    WebSocketOpcode::Ping | WebSocketOpcode::Pong => continue,
                    WebSocketOpcode::Text | WebSocketOpcode::Binary => {
                        match frame.build_full_frame(&mut full_frame) {
                            Ok(Some(result)) => return Ok(result),
                            Ok(None) => continue,
                            Err(error) => return Err(error),
                        }
                    }
                    _ => {
                        return Err(RequestError::WebSocketOpcodeUnsupported(
                            HttpStatus::NotImplemented,
                        ));
                    }
                }
            }
        }
    }

    /// Reads data from the stream with optional timeout handling.
    ///
    /// # Arguments
    ///
    /// - `&mut [u8]` - The buffer to read data into.
    /// - `Option<Duration>` - The optional timeout duration. If Some, timeout is applied; if None, no timeout.
    /// - `&mut bool` - Mutable reference to track if we got a client response.
    ///
    /// # Returns
    ///
    /// - `Result<Option<usize>, RequestError>` - The number of bytes read, None for timeout/ping, or an error.
    pub(crate) async fn get_websocket_from_stream(
        &mut self,
        buffer: &mut [u8],
        duration_opt: Option<Duration>,
        is_client_response: &mut bool,
    ) -> Result<Option<usize>, RequestError> {
        let stream: &mut TcpStream = self.get_mut_stream();
        if let Some(duration) = duration_opt {
            return match timeout(duration, stream.read(buffer)).await {
                Ok(result) => match result {
                    Ok(len) => Ok(Some(len)),
                    Err(error) => Err(error.into()),
                },
                Err(error) => {
                    if !*is_client_response {
                        return Err(error.into());
                    }
                    *is_client_response = false;
                    self.try_send(&PING_FRAME).await?;
                    Ok(None)
                }
            };
        }
        match stream.read(buffer).await {
            Ok(len) => Ok(Some(len)),
            Err(error) => Err(error.into()),
        }
    }

    /// Sends data over the stream.
    ///
    /// # Arguments
    ///
    /// - `AsRef<[u8]>` - The data to send (must implement AsRef<[u8]>).
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
    /// - `AsRef<[u8]>` - The data to send (must implement AsRef<[u8]>).
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
    /// - `IntoIterator<Item = AsRef<[u8]>>` - The data list to send.
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
        let stream: &mut TcpStream = self.get_mut_stream();
        for data in data_iter {
            stream.write_all(data.as_ref()).await?;
        }
        Ok(())
    }

    /// Sends multiple data.
    ///
    /// # Arguments
    ///
    /// - `IntoIterator<Item = AsRef<[u8]>>` - The data list to send.
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
