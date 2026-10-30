use super::*;

/// Implements the `std::error::Error` trait for `RequestError`.
impl Error for RequestError {}

/// Provides a default value for `RequestError`.
impl Default for RequestError {
    /// Provides a default value for `RequestError`.
    ///
    /// Returns a `RequestError::Unknown` with `HttpStatus::InternalServerError`.
    #[inline(always)]
    fn default() -> Self {
        RequestError::Unknown(HttpStatus::InternalServerError)
    }
}

/// Converts an I/O error to a `RequestError`.
///
/// Maps connection reset and aborted errors to `ClientDisconnected`,
/// all other I/O errors are mapped to `ReadConnection`.
impl From<io::Error> for RequestError {
    /// Converts an I/O error to a `RequestError`.
    ///
    /// # Arguments
    ///
    /// - `io::Error` - The I/O error to convert.
    ///
    /// # Returns
    ///
    /// - `RequestError` - The corresponding request error.
    #[inline(always)]
    fn from(error: io::Error) -> Self {
        let kind: ErrorKind = error.kind();
        if kind == ErrorKind::ConnectionReset || kind == ErrorKind::ConnectionAborted {
            return RequestError::ClientDisconnected(HttpStatus::BadRequest);
        }
        RequestError::ReadConnection(HttpStatus::BadRequest)
    }
}

/// Converts a timeout elapsed error to a `RequestError`.
///
/// Maps timeout errors to `ReadTimeout` with `HttpStatus::RequestTimeout`.
impl From<Elapsed> for RequestError {
    /// Converts a timeout elapsed error to a `RequestError`.
    ///
    /// # Arguments
    ///
    /// - `Elapsed` - The elapsed error to convert.
    ///
    /// # Returns
    ///
    /// - `RequestError` - The corresponding request error as `ReadTimeout`.
    #[inline(always)]
    fn from(_: Elapsed) -> Self {
        RequestError::ReadTimeout(HttpStatus::RequestTimeout)
    }
}

/// Converts a parse int error to a `RequestError`.
///
/// Maps parse int errors to `InvalidContentLength` with `HttpStatus::BadRequest`.
impl From<ParseIntError> for RequestError {
    /// Converts a parse int error to a `RequestError`.
    ///
    /// # Arguments
    ///
    /// - `ParseIntError` - The parse error to convert.
    ///
    /// # Returns
    ///
    /// - `RequestError` - The corresponding request error as `InvalidContentLength`.
    #[inline(always)]
    fn from(_: ParseIntError) -> Self {
        RequestError::InvalidContentLength(HttpStatus::BadRequest)
    }
}

/// Converts a response error to a `RequestError`.
///
/// Maps response errors to `WriteTimeout` with `HttpStatus::InternalServerError`.
impl From<ResponseError> for RequestError {
    /// Converts a response error to a `RequestError`.
    ///
    /// # Arguments
    ///
    /// - `ResponseError` - The response error to convert.
    ///
    /// # Returns
    ///
    /// - `RequestError` - The corresponding request error as `WriteTimeout`.
    #[inline(always)]
    fn from(_: ResponseError) -> Self {
        RequestError::WriteTimeout(HttpStatus::InternalServerError)
    }
}

/// Inherent implementation of [`RequestError`].
impl RequestError {
    /// Gets the HTTP status associated with this error.
    ///
    /// Returns the HttpStatus enum variant that corresponds to this error.
    ///
    /// # Returns
    ///
    /// - `HttpStatus` - The HTTP status associated with this error.
    pub fn get_http_status(&self) -> HttpStatus {
        match self {
            Self::HttpRead(status) => *status,
            Self::GetTcpStream(status) => *status,
            Self::GetTlsStream(status) => *status,
            Self::ReadConnection(status) => *status,
            Self::RequestAborted(status) => *status,
            Self::TlsStreamConnect(status) => *status,
            Self::NeedOpenRedirect(status) => *status,
            Self::MaxRedirectTimes(status) => *status,
            Self::MethodsNotSupport(status) => *status,
            Self::RedirectInvalidUrl(status) => *status,
            Self::ClientDisconnected(status) => *status,
            Self::RedirectUrlDeadLoop(status) => *status,
            Self::ClientClosedConnection(status) => *status,
            Self::ServerClosedConnection(status) => *status,
            Self::IncompleteWebSocketFrame(status) => *status,
            Self::RequestTooLong(status) => *status,
            Self::PathTooLong(status) => *status,
            Self::QueryTooLong(status) => *status,
            Self::HeaderLineTooLong(status) => *status,
            Self::TooManyHeaders(status) => *status,
            Self::HeaderKeyTooLong(status) => *status,
            Self::HeaderValueTooLong(status) => *status,
            Self::ContentLengthTooLarge(status) => *status,
            Self::InvalidContentLength(status) => *status,
            Self::InvalidUrlScheme(status) => *status,
            Self::InvalidUrlHost(status) => *status,
            Self::InvalidUrlPort(status) => *status,
            Self::InvalidUrlPath(status) => *status,
            Self::InvalidUrlQuery(status) => *status,
            Self::InvalidUrlFragment(status) => *status,
            Self::ReadTimeout(status) => *status,
            Self::WriteTimeout(status) => *status,
            Self::TcpConnectionFailed(status) => *status,
            Self::TlsHandshakeFailed(status) => *status,
            Self::TlsCertificateInvalid(status) => *status,
            Self::WebSocketFrameTooLarge(status) => *status,
            Self::WebSocketOpcodeUnsupported(status) => *status,
            Self::WebSocketMaskMissing(status) => *status,
            Self::WebSocketPayloadCorrupted(status) => *status,
            Self::WebSocketInvalidUtf8(status) => *status,
            Self::WebSocketInvalidCloseCode(status) => *status,
            Self::WebSocketInvalidExtension(status) => *status,
            Self::HttpRequestPartsInsufficient(status) => *status,
            Self::TcpStreamConnect(status) => *status,
            Self::TlsConnectorBuild(status) => *status,
            Self::InvalidUrl(status) => *status,
            Self::ConfigReadError(status) => *status,
            Self::TcpStreamConnectString(status) => *status,
            Self::TlsConnectorBuildString(status) => *status,
            Self::Request(_) => HttpStatus::BadRequest,
            Self::Unknown(status) => *status,
        }
    }

    /// Gets the numeric HTTP status code associated with this error.
    ///
    /// Returns the numeric status code (e.g., 400, 404, 500) that corresponds to this error.
    ///
    /// # Returns
    ///
    /// - `ResponseStatusCode` - The numeric HTTP status code.
    pub fn get_http_status_code(&self) -> ResponseStatusCode {
        self.get_http_status().code()
    }
}

/// Implementation of `Default` trait for `RequestConfig`.
impl Default for RequestConfig {
    /// Creates a new `RequestConfig` with default secure settings.
    ///
    /// This constructor initializes the configuration with standard security limits
    /// suitable for most HTTP request parsing scenarios.
    ///
    /// # Returns
    ///
    /// - `Self` - A new `RequestConfig` instance with default settings.
    #[inline(always)]
    fn default() -> Self {
        Self {
            buffer_size: DEFAULT_BUFFER_SIZE,
            max_path_size: DEFAULT_MAX_PATH_SIZE,
            max_header_count: DEFAULT_MAX_HEADER_COUNT,
            max_header_key_size: DEFAULT_MAX_HEADER_KEY_SIZE,
            max_header_value_size: DEFAULT_MAX_HEADER_VALUE_SIZE,
            max_body_size: DEFAULT_MAX_BODY_SIZE,
            read_timeout_ms: DEFAULT_READ_TIMEOUT_MS,
        }
    }
}

/// Inherent implementation of [`RequestConfig`].
impl RequestConfig {
    /// Creates a new `RequestConfig` from a JSON string.
    ///
    /// # Arguments
    ///
    /// - `C` - The configuration, which requires the `AsRef<str>` bound.
    ///
    /// # Returns
    ///
    /// - `Result<RequestConfig, serde_json::Error>` - The parsed `RequestConfig` or an error.
    pub fn from_json<C>(json: C) -> Result<RequestConfig, serde_json::Error>
    where
        C: AsRef<str>,
    {
        serde_json::from_str(json.as_ref())
    }

    /// Creates a new `RequestConfig` with low-security settings.
    ///
    /// This constructor initializes the configuration with less restrictive limits
    /// for environments where higher limits are needed.
    ///
    /// # Returns
    ///
    /// - `Self` - A new `RequestConfig` instance with low-security settings.
    #[inline(always)]
    pub fn low_security() -> Self {
        Self {
            buffer_size: DEFAULT_LOW_SECURITY_BUFFER_SIZE,
            max_path_size: DEFAULT_LOW_SECURITY_MAX_PATH_SIZE,
            max_header_count: DEFAULT_LOW_SECURITY_MAX_HEADER_COUNT,
            max_header_key_size: DEFAULT_LOW_SECURITY_MAX_HEADER_KEY_SIZE,
            max_header_value_size: DEFAULT_LOW_SECURITY_MAX_HEADER_VALUE_SIZE,
            max_body_size: DEFAULT_LOW_SECURITY_MAX_BODY_SIZE,
            read_timeout_ms: DEFAULT_LOW_SECURITY_READ_TIMEOUT_MS,
        }
    }

    /// Creates a new `RequestConfig` with high-security settings.
    ///
    /// This constructor initializes the configuration with more restrictive limits
    /// to provide maximum protection against various attacks in high-risk environments.
    ///
    /// # Returns
    ///
    /// - `Self` - A new `RequestConfig` instance with high-security settings.
    #[inline(always)]
    pub fn high_security() -> Self {
        Self {
            buffer_size: DEFAULT_HIGH_SECURITY_BUFFER_SIZE,
            max_path_size: DEFAULT_HIGH_SECURITY_MAX_PATH_SIZE,
            max_header_count: DEFAULT_HIGH_SECURITY_MAX_HEADER_COUNT,
            max_header_key_size: DEFAULT_HIGH_SECURITY_MAX_HEADER_KEY_SIZE,
            max_header_value_size: DEFAULT_HIGH_SECURITY_MAX_HEADER_VALUE_SIZE,
            max_body_size: DEFAULT_HIGH_SECURITY_MAX_BODY_SIZE,
            read_timeout_ms: DEFAULT_HIGH_SECURITY_READ_TIMEOUT_MS,
        }
    }
}

/// Provides a default value for `Request`.
///
/// Returns a new `Request` instance with all fields initialized to their default values.
impl Default for Request {
    /// Builds the default `Request` with all fields set to their defaults.
    #[inline(always)]
    fn default() -> Self {
        Self {
            method: Method::default(),
            host: String::new(),
            version: HttpVersion::default(),
            path: String::new(),
            querys: hash_map_xx_hash3_64(),
            headers: hash_map_xx_hash3_64(),
            body: Vec::new(),
        }
    }
}

/// Inherent implementation of [`Request`].
impl Request {
    /// Resets the request to its default state while retaining allocated capacity.
    ///
    /// This keeps the header map, query map, body, and string allocations so
    /// persistent (keep-alive) connections avoid repeated allocation per request.
    ///
    /// # Returns
    ///
    /// - `&mut Self` - A mutable reference to self for chaining.
    pub fn reset(&mut self) -> &mut Self {
        self.set_method(Method::default());
        self.get_mut_host().clear();
        self.set_version(HttpVersion::default());
        self.get_mut_path().clear();
        self.get_mut_querys().clear();
        self.recycle_headers();
        self.get_mut_body().clear();
        self
    }

    /// Hands the header entries to the pool and empties the header map.
    ///
    /// The map ends up empty, exactly as clearing it would, but the name and
    /// the value buffers of every header survive in the pool. The next
    /// request on the same connection takes them back and overwrites them,
    /// so a connection that keeps its headers stops allocating for them.
    ///
    /// # Returns
    ///
    /// - `()` - The headers have been recycled.
    fn recycle_headers(&mut self) {
        let headers: &mut RequestHeaders = self.get_mut_headers();
        if headers.is_empty() {
            return;
        }
        for (key, values) in headers.drain() {
            return_header_entry((key, values));
        }
    }

    /// Parses the first line of HTTP request into method, path, and version components.
    ///
    /// # Arguments
    ///
    /// - `&str` - The first line string of HTTP request to parse.
    ///
    /// # Returns
    ///
    /// - `Result<(RequestMethod, &str, RequestVersion), RequestError>` - A tuple containing:
    ///   - The parsed HTTP method
    ///   - The full path string
    ///   - The parsed HTTP version
    ///   - Or an error if parsing fails
    #[inline(always)]
    pub(crate) fn get_http_first_line(
        line: &str,
    ) -> Result<(RequestMethod, &str, RequestVersion), RequestError> {
        let mut parts: SplitWhitespace<'_> = line.split_whitespace();
        let method_str: &str = parts
            .next()
            .ok_or(RequestError::HttpRequestPartsInsufficient(
                HttpStatus::BadRequest,
            ))?;
        let full_path: &str = parts
            .next()
            .ok_or(RequestError::HttpRequestPartsInsufficient(
                HttpStatus::BadRequest,
            ))?;
        let version_str: &str = parts
            .next()
            .ok_or(RequestError::HttpRequestPartsInsufficient(
                HttpStatus::BadRequest,
            ))?;
        if parts.next().is_some() {
            return Err(RequestError::HttpRequestPartsInsufficient(
                HttpStatus::BadRequest,
            ));
        }
        let method: RequestMethod = method_str
            .parse::<RequestMethod>()
            .unwrap_or(Method::Unknown(method_str.to_string()));
        let version: RequestVersion = match version_str.parse::<RequestVersion>() {
            Ok(RequestVersion::Unknown(_)) | Err(_) => {
                return Err(RequestError::HttpRequestPartsInsufficient(
                    HttpStatus::BadRequest,
                ));
            }
            Ok(version) => version,
        };
        Ok((method, full_path, version))
    }

    /// Validates the path length against the maximum allowed size.
    ///
    /// # Arguments
    ///
    /// - `&str` - The path string to check.
    /// - `usize` - The maximum allowed path size.
    ///
    /// # Returns
    ///
    /// - `Result<(), RequestError>` - Ok if valid, or an error if the path is too long.
    #[inline(always)]
    pub(crate) fn check_http_path_size(path: &str, max_size: usize) -> Result<(), RequestError> {
        if path.len() > max_size && max_size != DEFAULT_LOW_SECURITY_MAX_PATH_SIZE {
            return Err(RequestError::PathTooLong(HttpStatus::URITooLong));
        }
        Ok(())
    }

    /// Parses the query string from the full path.
    ///
    /// Handles both query parameters (after `?`) and hash fragments (after `#`),
    /// ensuring proper parsing when both are present.
    ///
    /// # Arguments
    ///
    /// - `&str` - The full path string containing the query.
    /// - `Option<usize>` - The index of the query separator (`?`), if present.
    /// - `Option<usize>` - The index of the hash separator (`#`), if present.
    ///
    /// # Returns
    ///
    /// - `&str` - The parsed query string slice, or empty string if no query.
    #[inline(always)]
    pub(crate) fn get_http_query(
        path: &str,
        query_index: Option<usize>,
        hash_index: Option<usize>,
    ) -> &str {
        query_index.map_or(EMPTY_STR, |query_index: usize| {
            let temp: &str = &path[query_index + 1..];
            match hash_index {
                None => temp,
                Some(hash_index) if hash_index <= query_index => temp,
                Some(hash_index) => &temp[..hash_index - query_index - 1],
            }
        })
    }

    /// Parses the request path without query string or hash fragment.
    ///
    /// # Arguments
    ///
    /// - `&str` - The full path string.
    /// - `Option<usize>` - The index of the query separator (`?`), if present.
    /// - `Option<usize>` - The index of the hash separator (`#`), if present.
    ///
    /// # Returns
    ///
    /// - `&str` - The request path slice without query or hash.
    #[inline(always)]
    pub(crate) fn get_http_path(
        path: &str,
        query_index: Option<usize>,
        hash_index: Option<usize>,
    ) -> &str {
        match query_index.or(hash_index) {
            Some(separator_index) => &path[..separator_index],
            None => path,
        }
    }

    /// Parses a query string as_ref key-value pairs into the given map.
    ///
    /// Expects format "key1=value1&key2=value2". Empty values are allowed, and a
    /// key that repeats keeps every value instead of letting the last one win.
    /// The target map is expected to be empty; entries are inserted without
    /// reallocating the map when it already has sufficient capacity.
    ///
    /// # Arguments
    ///
    /// - `&str` - The query string to parse.
    /// - `&mut RequestQuerys` - The map to insert parsed parameters into.
    #[inline(always)]
    pub(crate) fn fill_http_querys(query: &str, querys: &mut RequestQuerys) {
        if query.is_empty() {
            return;
        }
        for pair in query.split(AND) {
            if let Some((key, value)) = pair.split_once(EQUAL) {
                if !key.is_empty() {
                    querys
                        .entry(key.to_string())
                        .or_default()
                        .push_back(value.to_string());
                }
            } else if !pair.is_empty() {
                querys.entry(pair.to_string()).or_default();
            }
        }
    }

    /// Checks if the header count exceeds the maximum allowed.
    ///
    /// # Arguments
    ///
    /// - `usize` - The current number of headers parsed.
    /// - `usize` - The maximum allowed number of headers.
    ///
    /// # Returns
    ///
    /// - `Result<(), RequestError>` - Returns an error if the limit is exceeded and not in low security mode.
    #[inline(always)]
    pub(crate) fn check_http_header_count(
        count: usize,
        max_count: usize,
    ) -> Result<(), RequestError> {
        if count > max_count && max_count != DEFAULT_LOW_SECURITY_MAX_HEADER_COUNT {
            return Err(RequestError::TooManyHeaders(
                HttpStatus::RequestHeaderFieldsTooLarge,
            ));
        }
        Ok(())
    }

    /// Checks if a header key exceeds the maximum allowed length.
    ///
    /// # Arguments
    ///
    /// - `&str` - The header key to check.
    /// - `usize` - The maximum allowed length for a header key.
    ///
    /// # Returns
    ///
    /// - `Result<(), RequestError>` - Returns an error if the limit is exceeded and not in low security mode.
    #[inline(always)]
    pub(crate) fn check_http_header_key_size(
        key: &str,
        max_size: usize,
    ) -> Result<(), RequestError> {
        if key.len() > max_size && max_size != DEFAULT_LOW_SECURITY_MAX_HEADER_KEY_SIZE {
            return Err(RequestError::HeaderKeyTooLong(
                HttpStatus::RequestHeaderFieldsTooLarge,
            ));
        }
        Ok(())
    }

    /// Checks if a header value exceeds the maximum allowed length.
    ///
    /// # Arguments
    ///
    /// - `&str` - The header value to check.
    /// - `usize` - The maximum allowed length for a header value.
    ///
    /// # Returns
    ///
    /// - `Result<(), RequestError>` - Returns an error if the limit is exceeded and not in low security mode.
    #[inline(always)]
    pub(crate) fn check_http_header_value_size(
        value: &str,
        max_size: usize,
    ) -> Result<(), RequestError> {
        if value.len() > max_size && max_size != DEFAULT_LOW_SECURITY_MAX_HEADER_VALUE_SIZE {
            return Err(RequestError::HeaderValueTooLong(
                HttpStatus::RequestHeaderFieldsTooLarge,
            ));
        }
        Ok(())
    }

    /// Parses the Content-Length header value and checks it against max body size.
    ///
    /// # Arguments
    ///
    /// - `&str` - The Content-Length header value string.
    /// - `usize` - The maximum allowed body size.
    ///
    /// # Returns
    ///
    /// - `Result<usize, RequestError>` - The parsed content length or an error.
    #[inline(always)]
    pub(crate) fn check_http_body_size(
        value: &str,
        max_size: usize,
    ) -> Result<usize, RequestError> {
        if value.is_empty() || !value.bytes().all(|byte: u8| byte.is_ascii_digit()) {
            return Err(RequestError::InvalidContentLength(HttpStatus::BadRequest));
        }
        let length: usize = value.parse::<usize>()?;
        if length > max_size && max_size != DEFAULT_LOW_SECURITY_MAX_BODY_SIZE {
            return Err(RequestError::ContentLengthTooLarge(
                HttpStatus::PayloadTooLarge,
            ));
        }
        Ok(length)
    }

    /// Parses HTTP headers from a buffered reader into this request.
    ///
    /// This method reads header lines from the provided buffered reader until an empty line
    /// is encountered, which indicates the end of headers. It checks header count, length,
    /// and content according to the provided configuration. The request's headers map and
    /// host string are expected to be empty; they are filled without reallocating when they
    /// already have sufficient capacity.
    ///
    /// The line buffer belongs to the caller so that a keep-alive connection
    /// reads every header line of every request into the same allocation.
    ///
    /// # Arguments
    ///
    /// - `&mut R` - A mutable reference to a buffered reader, which requires the
    ///   `AsyncBufReadExt + Unpin` bound.
    /// - `&RequestConfig` - Configuration for security limits and buffer settings.
    /// - `&mut String` - The buffer the header lines are read into.
    ///
    /// # Returns
    ///
    /// - `Result<usize, RequestError>` - The content length parsed from the
    ///   Content-Length header, or an error if parsing fails.
    pub(crate) async fn get_http_headers<R>(
        &mut self,
        reader: &mut R,
        config: &RequestConfig,
        header_line_buffer: &mut String,
    ) -> Result<usize, RequestError>
    where
        R: AsyncBufReadExt + Unpin,
    {
        let Request {
            headers,
            host,
            version,
            ..
        } = self;
        let max_header_count: usize = config.get_max_header_count();
        let max_header_key_size: usize = config.get_max_header_key_size();
        let max_header_value_size: usize = config.get_max_header_value_size();
        let max_body_size: usize = config.get_max_body_size();
        let mut content_size: usize = 0;
        let mut header_count: usize = 0;
        let mut content_length_seen: bool = false;
        let mut host_seen: bool = false;
        loop {
            header_line_buffer.clear();
            AsyncBufReadExt::read_line(reader, header_line_buffer).await?;
            let header_line: &str = header_line_buffer
                .strip_suffix(HTTP_HEADER_LINE_FEED)
                .ok_or(RequestError::HttpRead(HttpStatus::BadRequest))?;
            let header_line: &str = header_line
                .strip_suffix(HTTP_HEADER_CARRIAGE_RETURN)
                .unwrap_or(header_line);
            if header_line.is_empty() {
                break;
            }
            if header_line.starts_with(HTTP_HEADER_OPTIONAL_WHITESPACE) {
                return Err(RequestError::HttpRead(HttpStatus::BadRequest));
            }
            header_count += 1;
            Self::check_http_header_count(header_count, max_header_count)?;
            let (key, value_part): (&str, &str) = header_line
                .split_once(COLON)
                .ok_or(RequestError::HttpRead(HttpStatus::BadRequest))?;
            let valid_key: bool = key.bytes().all(|byte: u8| {
                byte.is_ascii_alphanumeric() || HTTP_HEADER_TOKEN_PUNCTUATION.contains(&byte)
            });
            if key.is_empty() || !valid_key {
                return Err(RequestError::HttpRead(HttpStatus::BadRequest));
            }
            Self::check_http_header_key_size(key, max_header_key_size)?;
            if value_part.bytes().any(|byte: u8| {
                (byte < HTTP_HEADER_CONTROL_LIMIT && byte != HTTP_HEADER_TAB)
                    || byte == HTTP_HEADER_DELETE
            }) {
                return Err(RequestError::HttpRead(HttpStatus::BadRequest));
            }
            let value: &str = value_part.trim_matches(HTTP_HEADER_OPTIONAL_WHITESPACE);
            Self::check_http_header_value_size(value, max_header_value_size)?;
            match key {
                key if key.eq_ignore_ascii_case(HTTP_HEADER_TRANSFER_ENCODING) => {
                    return Err(RequestError::HttpRead(HttpStatus::BadRequest));
                }
                key if key.eq_ignore_ascii_case(HOST) => {
                    if host_seen {
                        return Err(RequestError::HttpRead(HttpStatus::BadRequest));
                    }
                    host_seen = true;
                    host.clear();
                    host.push_str(value);
                }
                key if key.eq_ignore_ascii_case(CONTENT_LENGTH) => {
                    if content_length_seen {
                        return Err(RequestError::InvalidContentLength(HttpStatus::BadRequest));
                    }
                    content_length_seen = true;
                    content_size = Self::check_http_body_size(value, max_body_size)?;
                }
                _ => {}
            }
            Self::insert_http_header(headers, key, value);
        }
        if *version == HTTP_HOST_REQUIRED_VERSION && !host_seen {
            return Err(RequestError::HttpRead(HttpStatus::BadRequest));
        }
        Ok(content_size)
    }

    /// Stores one header, reusing a pooled name and value where possible.
    ///
    /// The name is lowercased into the buffer of a recycled entry and the
    /// value is written into that entry's value buffer, so a header that was
    /// already seen on this connection costs no allocation at all.
    ///
    /// # Arguments
    ///
    /// - `&mut RequestHeaders` - The header map to store the header in.
    /// - `&str` - The header name, as it arrived.
    /// - `&str` - The header value.
    fn insert_http_header(headers: &mut RequestHeaders, key: &str, value: &str) {
        let (mut name, mut values): (String, VecDeque<String>) =
            take_header_entry().unwrap_or_else(|| (String::new(), VecDeque::new()));
        name.clear();
        name.push_str(key);
        name.make_ascii_lowercase();
        match headers.entry(name) {
            Entry::Occupied(mut occupied) => {
                // The header repeats: it keeps every value it was given.
                occupied.get_mut().push_back(value.to_string());
                return_header_entry((String::new(), values));
            }
            Entry::Vacant(vacant) => {
                // A recycled entry may carry the values of a header that was
                // sent more than once. Those values belong to that header, so
                // only the first one is kept and the rest are dropped.
                while values.len() > 1 {
                    values.pop_back();
                }
                if values.is_empty() {
                    values.push_back(String::new());
                }
                if let Some(slot) = values.front_mut() {
                    slot.clear();
                    slot.push_str(value);
                }
                vacant.insert(values);
            }
        }
    }

    /// Reads the request body from the buffered reader into the given buffer.
    ///
    /// The target buffer is cleared and resized to the expected content size,
    /// retaining its allocation when it already has sufficient capacity.
    ///
    /// # Arguments
    ///
    /// - `&mut R` - The buffered reader to read from, which requires the `AsyncRead + Unpin` bound.
    /// - `&mut RequestBody` - The buffer to read the body bytes into.
    /// - `usize` - The expected content size.
    ///
    /// # Returns
    ///
    /// - `Result<(), RequestError>` - Ok on success, or an error if reading fails.
    #[inline(always)]
    pub(crate) async fn fill_http_body<R>(
        reader: &mut R,
        body: &mut RequestBody,
        content_size: usize,
    ) -> Result<(), RequestError>
    where
        R: AsyncRead + Unpin,
    {
        body.clear();
        if content_size > 0 {
            body.resize(content_size, 0);
            AsyncReadExt::read_exact(reader, body).await?;
        }
        Ok(())
    }

    /// Tries to get a query parameter by key.
    ///
    /// The key type must implement AsRef<str> conversion. A key that repeats in
    /// the query carries every one of its values.
    ///
    /// # Arguments
    ///
    /// - `K` - The query parameter key, which implements `AsRef<str>`.
    ///
    /// # Returns
    ///
    /// - `Option<RequestQuerysValue>` - Every value of the parameter if it exists.
    #[inline(always)]
    pub fn try_get_query<K>(&self, key: K) -> Option<RequestQuerysValue>
    where
        K: AsRef<str>,
    {
        self.get_querys().get(key.as_ref()).cloned()
    }

    /// Tries to get the first query parameter value by key.
    ///
    /// Use this when the parameter is expected to appear once; use
    /// `try_get_query` when every value matters.
    ///
    /// # Arguments
    ///
    /// - `K` - The query parameter key, which implements `AsRef<str>`.
    ///
    /// # Returns
    ///
    /// - `Option<RequestQuerysValueItem>` - The first parameter value if it exists.
    #[inline(always)]
    pub fn try_get_query_front<K>(&self, key: K) -> Option<RequestQuerysValueItem>
    where
        K: AsRef<str>,
    {
        self.get_querys()
            .get(key.as_ref())
            .and_then(|values: &VecDeque<String>| values.front().cloned())
    }

    /// Tries to get the last query parameter value by key.
    ///
    /// # Arguments
    ///
    /// - `K` - The query parameter key, which implements `AsRef<str>`.
    ///
    /// # Returns
    ///
    /// - `Option<RequestQuerysValueItem>` - The last parameter value if it exists.
    #[inline(always)]
    pub fn try_get_query_back<K>(&self, key: K) -> Option<RequestQuerysValueItem>
    where
        K: AsRef<str>,
    {
        self.get_querys()
            .get(key.as_ref())
            .and_then(|values: &VecDeque<String>| values.back().cloned())
    }

    /// Tries to get the number of values of a query parameter.
    ///
    /// # Arguments
    ///
    /// - `K` - The query parameter key, which implements `AsRef<str>`.
    ///
    /// # Returns
    ///
    /// - `Option<usize>` - The number of values if the parameter exists.
    #[inline(always)]
    pub fn try_get_query_size<K>(&self, key: K) -> Option<usize>
    where
        K: AsRef<str>,
    {
        self.get_querys()
            .get(key.as_ref())
            .map(|values: &VecDeque<String>| values.len())
    }

    /// Checks whether a query parameter exists.
    ///
    /// # Arguments
    ///
    /// - `K` - The query parameter key, which implements `AsRef<str>`.
    ///
    /// # Returns
    ///
    /// - `bool` - Whether the parameter exists.
    #[inline(always)]
    pub fn has_query<K>(&self, key: K) -> bool
    where
        K: AsRef<str>,
    {
        self.get_querys().contains_key(key.as_ref())
    }

    /// Gets a query parameter by key.
    ///
    /// # Arguments
    ///
    /// - `K` - The query parameter key, which implements `AsRef<str>`.
    ///
    /// # Returns
    ///
    /// - `RequestQuerysValue` - Every value of the parameter.
    ///
    /// # Panics
    ///
    /// This function will panic if the query parameter key is not found.
    #[inline(always)]
    pub fn get_query<K>(&self, key: K) -> RequestQuerysValue
    where
        K: AsRef<str>,
    {
        self.try_get_query(key).unwrap()
    }

    /// Gets the first query parameter value by key.
    ///
    /// # Arguments
    ///
    /// - `K` - The query parameter key, which implements `AsRef<str>`.
    ///
    /// # Returns
    ///
    /// - `RequestQuerysValueItem` - The first parameter value.
    ///
    /// # Panics
    ///
    /// This function will panic if the query parameter key is not found.
    #[inline(always)]
    pub fn get_query_front<K>(&self, key: K) -> RequestQuerysValueItem
    where
        K: AsRef<str>,
    {
        self.try_get_query_front(key).unwrap()
    }

    /// Retrieves the number of query parameters.
    ///
    /// # Returns
    ///
    /// - `usize` - The count of distinct query parameter keys.
    #[inline(always)]
    pub fn get_querys_size(&self) -> usize {
        self.get_querys().len()
    }

    /// Tries to retrieve the value of a request header by its key.
    ///
    /// The key is matched case-insensitively, because the parser stores every
    /// header name lowercased and an HTTP header name is case-insensitive by
    /// specification. Looking a header up with `Content-Type` and with
    /// `content-type` therefore find the same value.
    ///
    /// # Arguments
    ///
    /// - `K` - The header's key, which must implement `AsRef<str>`.
    ///
    /// # Returns
    ///
    /// - `Option<RequestHeadersValue>` - The optional header values.
    #[inline(always)]
    pub fn try_get_header<K>(&self, key: K) -> Option<RequestHeadersValue>
    where
        K: AsRef<str>,
    {
        self.get_headers()
            .get(&key.as_ref().to_ascii_lowercase())
            .cloned()
    }

    /// Retrieves the value of a request header by its key.
    ///
    /// # Arguments
    ///
    /// - `K` - The header's key, which must implement `AsRef<str>`.
    ///
    /// # Returns
    ///
    /// - `RequestHeadersValue` - The optional header values.
    ///
    /// # Panics
    ///
    /// This function will panic if the header key is not found.
    #[inline(always)]
    pub fn get_header<K>(&self, key: K) -> RequestHeadersValue
    where
        K: AsRef<str>,
    {
        self.try_get_header(key).unwrap()
    }

    /// Tries to retrieve the first value of a request header by its key.
    ///
    /// # Arguments
    ///
    /// - `K` - The header's key, which must implement `AsRef<str>`.
    ///
    /// # Returns
    ///
    /// - `Option<RequestHeadersValueItem>` - The first header value if exists.
    #[inline(always)]
    pub fn try_get_header_front<K>(&self, key: K) -> Option<RequestHeadersValueItem>
    where
        K: AsRef<str>,
    {
        self.get_headers()
            .get(&key.as_ref().to_ascii_lowercase())
            .and_then(|header_values: &VecDeque<String>| header_values.front().cloned())
    }

    /// Retrieves the first value of a request header by its key.
    ///
    /// # Arguments
    ///
    /// - `K` - The header's key, which must implement `AsRef<str>`.
    ///
    /// # Returns
    ///
    /// - `RequestHeadersValueItem` - The first header value if exists.
    ///
    /// # Panics
    ///
    /// This function will panic if the header key is not found.
    #[inline(always)]
    pub fn get_header_front<K>(&self, key: K) -> RequestHeadersValueItem
    where
        K: AsRef<str>,
    {
        self.try_get_header_front(key).unwrap()
    }

    /// Tries to retrieve the last value of a request header by its key.
    ///
    /// # Arguments
    ///
    /// - `K` - The header's key, which must implement `AsRef<str>`.
    ///
    /// # Returns
    ///
    /// - `Option<RequestHeadersValueItem>` - The last header value if exists.
    #[inline(always)]
    pub fn try_get_header_back<K>(&self, key: K) -> Option<RequestHeadersValueItem>
    where
        K: AsRef<str>,
    {
        self.get_headers()
            .get(&key.as_ref().to_ascii_lowercase())
            .and_then(|header_values: &VecDeque<String>| header_values.back().cloned())
    }

    /// Retrieves the last value of a request header by its key.
    ///
    /// # Arguments
    ///
    /// - `K` - The header's key, which must implement `AsRef<str>`.
    ///
    /// # Returns
    ///
    /// - `RequestHeadersValueItem` - The last header value if exists.
    ///
    /// # Panics
    ///
    /// This function will panic if the header key is not found.
    #[inline(always)]
    pub fn get_header_back<K>(&self, key: K) -> RequestHeadersValueItem
    where
        K: AsRef<str>,
    {
        self.try_get_header_back(key).unwrap()
    }

    /// Tries to retrieve the number of values for a specific header.
    ///
    /// # Arguments
    ///
    /// - `K` - The header's key, which must implement `AsRef<str>`.
    ///
    /// # Returns
    ///
    /// - `Option<usize>` - The count of values for the header if exists.
    #[inline(always)]
    pub fn try_get_header_size<K>(&self, key: K) -> Option<usize>
    where
        K: AsRef<str>,
    {
        self.get_headers()
            .get(&key.as_ref().to_ascii_lowercase())
            .map(|header_values: &VecDeque<String>| header_values.len())
    }

    /// Retrieves the number of values for a specific header.
    ///
    /// # Arguments
    ///
    /// - `K` - The header's key, which must implement `AsRef<str>`.
    ///
    /// # Returns
    ///
    /// - `usize` - The count of values for the header.
    ///
    /// # Panics
    ///
    /// This function will panic if the header key is not found.
    #[inline(always)]
    pub fn get_header_size<K>(&self, key: K) -> usize
    where
        K: AsRef<str>,
    {
        self.try_get_header_size(key).unwrap()
    }

    /// Retrieves the total number of header values across all headers.
    ///
    /// # Returns
    ///
    /// - `usize` - The total count of all header values.
    #[inline(always)]
    pub fn get_headers_values_size(&self) -> usize {
        self.get_headers()
            .values()
            .map(|header_values: &VecDeque<String>| header_values.len())
            .sum()
    }

    /// Retrieves the number of unique headers.
    ///
    /// # Returns
    ///
    /// - `usize` - The count of unique header keys.
    #[inline(always)]
    pub fn get_headers_size(&self) -> usize {
        self.get_headers().len()
    }

    /// Checks if a specific header exists.
    ///
    /// # Arguments
    ///
    /// - `K` - The header key to check, which must implement `AsRef<str>`.
    ///
    /// # Returns
    ///
    /// - `bool` - Whether the header exists.
    #[inline(always)]
    pub fn has_header<K>(&self, key: K) -> bool
    where
        K: AsRef<str>,
    {
        self.get_headers()
            .contains_key(&key.as_ref().to_ascii_lowercase())
    }

    /// Checks if a header contains a specific value.
    ///
    /// # Arguments
    ///
    /// - `K` - The header key to check, which must implement `AsRef<str>`.
    /// - `V` - The value to search for, which must implement `AsRef<str>`.
    ///
    /// # Returns
    ///
    /// - `bool` - Whether the header contains the value.
    #[inline(always)]
    pub fn has_header_value<K, V>(&self, key: K, value: V) -> bool
    where
        K: AsRef<str>,
        V: AsRef<str>,
    {
        if let Some(values) = self.get_headers().get(&key.as_ref().to_ascii_lowercase()) {
            values.iter().any(|data: &String| data == value.as_ref())
        } else {
            false
        }
    }

    /// Tries to parse cookies from the `Cookie` header.
    ///
    /// This method retrieves the `Cookie` header value and parses it into
    /// a collection of key-value pairs representing the cookies.
    ///
    /// # Returns
    ///
    /// - `Option<Cookies>` - The parsed cookies if the header exists, otherwise `None`.
    #[inline(always)]
    pub fn try_get_cookies(&self) -> Option<Cookies> {
        self.try_get_header_back(COOKIE)
            .map(|cookie_header: String| Cookie::parse(cookie_header))
    }

    /// Parses cookies from the `Cookie` header.
    ///
    /// This method retrieves the `Cookie` header value and parses it into
    /// a collection of key-value pairs representing the cookies.
    ///
    /// # Returns
    ///
    /// - `Cookies` - The parsed cookies.
    ///
    /// # Panics
    ///
    /// This function will panic if the `Cookie` header is not found.
    #[inline(always)]
    pub fn get_cookies(&self) -> Cookies {
        self.try_get_cookies().unwrap()
    }

    /// Tries to get a cookie value by its key.
    ///
    /// This method first parses the cookies from the `Cookie` header,
    /// then attempts to retrieve the value for the specified key.
    ///
    /// # Arguments
    ///
    /// - `K` - The cookie key, which implements `AsRef<str>`.
    ///
    /// # Returns
    ///
    /// - `Option<CookieValue>` - The cookie value if exists.
    #[inline(always)]
    pub fn try_get_cookie<K>(&self, key: K) -> Option<CookieValue>
    where
        K: AsRef<str>,
    {
        self.try_get_cookies()
            .and_then(|cookies: Cookies| cookies.get(key.as_ref()).cloned())
    }

    /// Gets a cookie value by its key.
    ///
    /// This method first parses the cookies from the `Cookie` header,
    /// then retrieves the value for the specified key.
    ///
    /// # Arguments
    ///
    /// - `K` - The cookie key, which implements `AsRef<str>`.
    ///
    /// # Returns
    ///
    /// - `CookieValue` - The cookie value.
    ///
    /// # Panics
    ///
    /// This function will panic if the `Cookie` header is not found
    /// or the cookie key does not exist.
    #[inline(always)]
    pub fn get_cookie<K>(&self, key: K) -> CookieValue
    where
        K: AsRef<str>,
    {
        self.try_get_cookie(key).unwrap()
    }

    /// Retrieves the upgrade type from the request headers.
    ///
    /// This method looks for the `UPGRADE` header and attempts to parse its value
    /// as_ref an `UpgradeType`. If the header is missing or the value is invalid,
    /// it returns the default `UpgradeType`.
    ///
    /// # Returns
    ///
    /// - `UpgradeType` - The parsed upgrade type.
    #[inline(always)]
    pub fn get_upgrade_type(&self) -> UpgradeType {
        self.try_get_header_back(UPGRADE)
            .and_then(|data: String| data.parse::<UpgradeType>().ok())
            .unwrap_or_default()
    }

    /// Retrieves the body content of the request as a UTF-8 encoded string.
    ///
    /// This method uses `String::from_utf8_lossy` to convert the byte slice returned by `self.get_body()` as a string.
    /// If the byte slice contains invalid UTF-8 sequences, they will be replaced with the Unicode replacement character ().
    ///
    /// # Returns
    ///
    /// - `String` - The body content as a string.
    #[inline(always)]
    pub fn get_body_string(&self) -> String {
        String::from_utf8_lossy(self.get_body()).into_owned()
    }

    /// Deserializes the body content of the request as_ref a specified type `T`.
    ///
    /// This method first retrieves the body content as a byte slice using `self.get_body()`.
    /// It then attempts to deserialize the byte slice as_ref the specified type `T` using `json_from_slice`.
    ///
    /// The target type `T` is a type parameter of this method and must implement `DeserializeOwned`.
    ///
    /// # Returns
    ///
    /// - `Result<T, serde_json::Error>` - The deserialization result.
    #[inline(always)]
    pub fn try_get_body_json<T>(&self) -> Result<T, serde_json::Error>
    where
        T: DeserializeOwned,
    {
        serde_json::from_slice(self.get_body())
    }

    /// Deserializes the body content of the request as_ref a specified type `T`.
    ///
    /// This method first retrieves the body content as a byte slice using `self.get_body()`.
    /// It then attempts to deserialize the byte slice as_ref the specified type `T` using `json_from_slice`.
    ///
    /// The target type `T` is a type parameter of this method and must implement `DeserializeOwned`.
    ///
    /// # Returns
    ///
    /// - `T` - The deserialized body content.
    ///
    /// # Panics
    ///
    /// This function will panic if the deserialization fails.
    #[inline(always)]
    pub fn get_body_json<T>(&self) -> T
    where
        T: DeserializeOwned,
    {
        self.try_get_body_json().unwrap()
    }

    /// Checks whether the WebSocket upgrade is enabled for this request.
    ///
    /// This method determines if the `UPGRADE` header indicates a WebSocket connection.
    ///
    /// # Returns
    ///
    /// - `bool` - Whether WebSocket upgrade is enabled.
    #[inline(always)]
    pub fn is_ws_upgrade_type(&self) -> bool {
        self.get_upgrade_type().is_ws()
    }

    /// Checks whether the request declares a `multipart/form-data` body.
    ///
    /// # Returns
    ///
    /// - `bool` - Whether the request content type is `multipart/form-data`.
    #[inline(always)]
    pub fn is_multipart_form_data(&self) -> bool {
        self.try_get_content_type()
            .map(|content_type: String| MultipartFormData::is_form_data(content_type))
            .unwrap_or(false)
    }

    /// Tries to get the request content type with its parameters stripped.
    ///
    /// A `Content-Type` header such as `text/html; charset=utf-8` yields the
    /// media type alone, so a caller comparing against a media type does not
    /// have to know that parameters may follow it.
    ///
    /// # Returns
    ///
    /// - `Option<RequestContentType>` - The media type, or `None` when the header is absent.
    #[inline(always)]
    pub fn try_get_content_type(&self) -> Option<RequestContentType> {
        self.try_get_header_front(CONTENT_TYPE)
            .map(|value: String| {
                value
                    .split(SEMICOLON)
                    .next()
                    .unwrap_or(EMPTY_STR)
                    .trim()
                    .to_ascii_lowercase()
            })
    }

    /// Tries to get one parameter of the request `Content-Type` header.
    ///
    /// The name is matched case-insensitively and a quoted value is unquoted,
    /// so both `boundary=abc` and `boundary="abc"` yield `abc`.
    ///
    /// # Arguments
    ///
    /// - `K` - The parameter name, which requires the `AsRef<str>` bound.
    ///
    /// # Returns
    ///
    /// - `Option<RequestContentTypeParameter>` - The parameter value, or `None` when absent.
    pub fn try_get_content_type_parameter<K>(&self, name: K) -> Option<RequestContentTypeParameter>
    where
        K: AsRef<str>,
    {
        let content_type: String = self.try_get_header_front(CONTENT_TYPE)?;
        let name: &str = name.as_ref();
        for parameter in content_type.split(SEMICOLON).skip(1) {
            let (key, value) = parameter.trim().split_once(EQUAL)?;
            if !key.trim().eq_ignore_ascii_case(name) {
                continue;
            }
            let value: &str = value.trim();
            let value: &str = value
                .strip_prefix(MULTIPART_PARAMETER_QUOTE)
                .and_then(|value: &str| value.strip_suffix(MULTIPART_PARAMETER_QUOTE))
                .unwrap_or(value);
            if !value.is_empty() {
                return Some(value.to_string());
            }
        }
        None
    }

    /// Tries to decode a `multipart/form-data` body into its parts.
    ///
    /// The boundary is read from the request `Content-Type` header rather than
    /// guessed, so a body whose content type declares no boundary, a truncated
    /// body, and a part without a name all yield `None`.
    ///
    /// # Returns
    ///
    /// - `Option<MultipartFormDataParts>` - The decoded parts, or `None` when unavailable.
    pub fn try_get_body_form_data(&self) -> Option<MultipartFormDataParts> {
        let boundary: MultipartFormDataBoundary =
            self.try_get_content_type_parameter(MULTIPART_BOUNDARY_PARAMETER)?;
        MultipartFormData::parse(self.get_body(), &boundary)
    }

    /// Decodes an `application/x-www-form-urlencoded` body into its fields.
    ///
    /// # Returns
    ///
    /// - `BodyForm` - The decoded form fields.
    #[inline(always)]
    pub fn get_body_form(&self) -> BodyForm {
        MultipartFormData::parse_url_encoded(self.get_body())
    }

    /// Tries to get a plain text field of a `multipart/form-data` body.
    ///
    /// A part that uploads a file is skipped, so a file field never silently
    /// decodes as text.
    ///
    /// # Arguments
    ///
    /// - `K` - The form field name, which requires the `AsRef<str>` bound.
    ///
    /// # Returns
    ///
    /// - `Option<BodyFormValue>` - The field value, or `None` when absent or a file.
    pub fn try_get_body_form_data_text<K>(&self, name: K) -> Option<BodyFormValue>
    where
        K: AsRef<str>,
    {
        let parts: MultipartFormDataParts = self.try_get_body_form_data()?;
        MultipartFormData::try_get_part_text(&parts, name)
    }

    /// Checks if the current upgrade type is HTTP/2 cleartext (h2c).
    ///
    /// # Returns
    ///
    /// - `bool` - Whether the upgrade type is h2c.
    #[inline(always)]
    pub fn is_h2c_upgrade_type(&self) -> bool {
        self.get_upgrade_type().is_h2c()
    }

    /// Checks if the current upgrade type is TLS (any version).
    ///
    /// # Returns
    ///
    /// - `bool` - Whether the upgrade type is TLS.
    #[inline(always)]
    pub fn is_tls_upgrade_type(&self) -> bool {
        self.get_upgrade_type().is_tls()
    }

    /// Checks whether the upgrade type is unknown.
    ///
    /// # Returns
    ///
    /// - `bool` - Whether the upgrade type is unknown.
    #[inline(always)]
    pub fn is_unknown_upgrade_type(&self) -> bool {
        self.get_upgrade_type().is_unknown()
    }

    /// Determines if a keep-alive connection should be enabled for this request.
    ///
    /// This function checks the `Connection` header and the HTTP version to determine
    /// if keep-alive should be enabled. The logic is as follows:
    ///
    /// 1. If the `Connection` header exists:
    ///    - Returns `true` if the header value is "keep-alive" (case-insensitive).
    ///    - Returns `false` if the header value is "close" (case-insensitive).
    /// 2. If no `Connection` header is present:
    ///    - Returns `true` if the HTTP version is 1.1 or higher.
    ///    - Returns `false` otherwise.
    ///
    /// # Returns
    ///
    /// - `bool` - Whether keep-alive should be enabled.
    #[inline(always)]
    pub fn is_enable_keep_alive(&self) -> bool {
        if let Some(connection_value) = self.try_get_header_back(CONNECTION) {
            if connection_value.eq_ignore_ascii_case(KEEP_ALIVE) {
                return true;
            } else if connection_value.eq_ignore_ascii_case(CLOSE) {
                return self.is_ws_upgrade_type();
            }
        }
        self.get_version().is_http1_1_or_higher() || self.is_ws_upgrade_type()
    }

    /// Determines if keep-alive should be disabled for this request.
    ///
    /// # Returns
    ///
    /// - `bool` - Whether keep-alive should be disabled.
    #[inline(always)]
    pub fn is_disable_keep_alive(&self) -> bool {
        !self.is_enable_keep_alive()
    }
}
