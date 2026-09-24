use super::*;

/// Blanket implementation for AsyncReadWrite trait.
///
/// # Generic Parameters
///
/// - `T` - Type implementing AsyncRead + AsyncWrite + Unpin + Send
impl<T: AsyncRead + AsyncWrite + Unpin + Send> AsyncReadWrite for T {}

/// Blanket implementation for ReadWrite trait.
///
/// # Generic Parameters
///
/// - `T` - Type implementing Read + Write
impl<T: Read + Write> ReadWrite for T {}

/// Async request trait implementation for HttpRequest.
///
/// # Associated Types
///
/// - `RequestResult` - The result type of async requests.
///
/// # Returns
///
/// - `Pin<Box<dyn Future<Output = RequestResult> + Send + '_>>` - Future representing the async request.
impl AsyncRequestTrait for HttpRequest {
    type RequestResult = RequestResult;

    /// Sends an asynchronous HTTP request.
    ///
    /// # Returns
    ///
    /// - `Pin<Box<dyn Future<Output = RequestResult> + Send + '_>>` - Future representing the async request.
    fn send(&mut self) -> Pin<Box<dyn Future<Output = Self::RequestResult> + Send + '_>> {
        Box::pin(self.send_async())
    }
}

/// Sync request trait implementation for HttpRequest.
///
/// # Associated Types
///
/// - `RequestResult` - The result type of sync requests.
///
/// # Returns
///
/// - `RequestResult` - Result of the sync request.
impl RequestTrait for HttpRequest {
    type RequestResult = RequestResult;

    /// Sends a synchronous HTTP request.
    ///
    /// # Returns
    ///
    /// - `RequestResult` - Result of the sync request.
    fn send(&mut self) -> Self::RequestResult {
        self.send_sync()
    }
}

/// Default implementation for HttpRequest.
///
/// # Returns
///
/// - `HttpRequest` - Default initialized HttpRequest with:
///   - Empty methods
///   - Empty URL
///   - Empty headers
///   - Default body
///   - Default config
///   - Default tmp storage
///   - Default response
impl Default for HttpRequest {
    #[inline(always)]
    fn default() -> Self {
        Self {
            methods: Arc::new(Method::default()),
            url: Arc::new(String::new()),
            header: Arc::new(hash_map_xx_hash3_64()),
            body: Arc::new(Body::default()),
            config: Arc::new(RwLock::new(Config::default())),
            tmp: Arc::new(RwLock::new(Tmp::default())),
            response: Arc::new(RwLock::new(HttpResponseBinary::default())),
        }
    }
}

/// Implements methods for the `HttpRequest` struct.
///
/// These methods provide functionality for managing HTTP requests, including:
/// - Retrieving or setting HTTP attributes.
/// - Constructing and sending HTTP GET or POST requests.
/// - Parsing responses and handling redirects.
impl HttpRequest {
    /// Gets the protocol from config.
    ///
    /// # Arguments
    ///
    /// - `&Config` - Request configuration.
    ///
    /// # Returns
    ///
    /// - `String` - The HTTP protocol.
    #[inline(always)]
    pub(crate) fn get_protocol(config: &Config) -> String {
        config.url_obj.protocol.to_lowercase()
    }

    /// Gets the HTTP methods.
    ///
    /// # Returns
    ///
    /// - `Method` - The HTTP methods.
    #[inline(always)]
    pub(crate) fn get_methods(&self) -> Method {
        self.methods.as_ref().clone()
    }

    /// Gets the request URL.
    ///
    /// # Returns
    ///
    /// - `String` - The request URL.
    #[inline(always)]
    fn get_url(&self) -> String {
        self.url.as_ref().clone()
    }

    /// Gets the request headers.
    ///
    /// # Returns
    ///
    /// - `RequestHeaders` - The request headers.
    #[inline(always)]
    fn get_header(&self) -> RequestHeaders {
        self.header.as_ref().clone()
    }

    /// Gets the request body.
    ///
    /// # Returns
    ///
    /// - `Body` - The request body.
    #[inline(always)]
    fn get_body(&self) -> Body {
        self.body.as_ref().clone()
    }

    /// Sets the URL for the HTTP request.
    ///
    /// # Arguments
    ///
    /// - `String` - The new URL to set.
    #[inline(always)]
    pub(crate) fn url(&mut self, url: String) {
        self.url = Arc::new(url);
    }

    /// Parses the current URL into a `HttpUrlComponents` object.
    ///
    /// Returns `Ok(HttpUrlComponents)` if the parsing succeeds, or `Err(RequestError::Request(String))` otherwise.
    /// Parses the current URL into a `HttpUrlComponents` object.
    ///
    /// # Returns
    ///
    /// - `Ok(HttpUrlComponents)` if parsing succeeds
    /// - `Err(RequestError::Request(String))` if parsing fails
    pub(crate) fn parse_url(&self) -> Result<HttpUrlComponents, RequestError> {
        match HttpUrlComponents::parse(self.get_url()) {
            Ok(parse_res) => Ok(parse_res),
            Err(error) => Err(RequestError::Request(error.to_string())),
        }
    }

    /// Converts the HTTP headers into a formatted HTTP header string and returns it as a byte vector.
    ///
    /// This method processes the HTTP headers by combining both user-defined and required headers.
    /// Required headers such as `Host`, `Content-Length`, `Accept`, and `User-Agent` are added if
    /// they are missing, with appropriate default values. The headers are then formatted into
    /// the standard HTTP header format and converted into a vector of bytes.
    ///
    /// # Returns
    ///
    /// A `Vec<u8>` containing the formatted HTTP headers as a byte sequence.
    ///
    /// # Notes
    ///
    /// - The `Host` header is derived from the URL's host in the configuration.
    /// - The `Content-Length` header is calculated based on the request method:
    ///   - For `GET` requests, it is set to `0`.
    ///   - For other methods, it is determined by the length of the body.
    /// - If any required header is missing, it is automatically added with its default value.
    /// - Headers are concatenated into a string with each header ending in a line break specified by `HTTP_BR`.
    ///
    /// # Behavior
    ///
    /// This function ensures that all necessary headers are present and correctly formatted
    /// before constructing the HTTP request.
    fn header_contains_key_case_insensitive(header: &RequestHeaders, target_key: &str) -> bool {
        header
            .keys()
            .any(|key| key.eq_ignore_ascii_case(target_key))
    }

    /// Converts HTTP headers into formatted HTTP header bytes.
    ///
    /// # Returns
    ///
    /// - `Vec<u8>` - The formatted HTTP headers as bytes.
    pub(crate) fn get_header_bytes(&self) -> Vec<u8> {
        let mut header: RequestHeaders = self.get_header();
        let body_length: usize = if self.get_methods().is_get() {
            0usize
        } else {
            self.get_body_bytes().len()
        };
        if let Ok(config) = self.config.read() {
            let host_value: String = config.url_obj.host.clone().unwrap_or_default();
            let content_length_value: String = body_length.to_string();
            if !Self::header_contains_key_case_insensitive(&header, HOST) {
                let mut host_deque: VecDeque<String> = VecDeque::new();
                host_deque.push_front(host_value);
                header.insert(HOST.to_owned(), host_deque);
            }
            if !Self::header_contains_key_case_insensitive(&header, CONTENT_LENGTH) {
                let mut content_length_deque: VecDeque<String> = VecDeque::new();
                content_length_deque.push_front(content_length_value);
                header.insert(CONTENT_LENGTH.to_owned(), content_length_deque);
            }
            if !Self::header_contains_key_case_insensitive(&header, ACCEPT) {
                let mut accept_deque: VecDeque<String> = VecDeque::new();
                accept_deque.push_front(ACCEPT_ANY.to_owned());
                header.insert(ACCEPT.to_owned(), accept_deque);
            }
            if !Self::header_contains_key_case_insensitive(&header, USER_AGENT) {
                let mut user_agent_deque: VecDeque<String> = VecDeque::new();
                user_agent_deque.push_front(APP_NAME.to_owned());
                header.insert(USER_AGENT.to_owned(), user_agent_deque);
            }
        }
        let estimated_size: usize = header
            .iter()
            .map(|(k, v)| {
                k.len()
                    + v.front()
                        .map_or(0, |header_value: &String| header_value.len())
                    + 4
            })
            .sum();
        let mut header_bytes: Vec<u8> = Vec::with_capacity(estimated_size);
        for (key, value) in &header {
            header_bytes.extend_from_slice(key.as_bytes());
            header_bytes.extend_from_slice(b": ");
            if let Some(header_value) = value.front() {
                header_bytes.extend_from_slice(header_value.as_bytes());
            }
            header_bytes.extend_from_slice(HTTP_BR_BYTES);
        }
        header_bytes
    }

    /// Converts the HTTP body into a URL-encoded byte vector (`Vec<u8>`).
    ///
    /// This method processes the body of the HTTP request based on the `Content-Type` header.
    /// If the `Content-Type` is valid and supports conversion, the body is transformed into a
    /// URL-encoded string and returned as a vector of bytes.
    ///
    /// # Returns
    ///
    /// A `Vec<u8>` containing the URL-encoded representation of the HTTP body.
    /// If the `Content-Type` is not recognized or if the body cannot be converted,
    /// an empty byte vector is returned.
    ///
    /// # Notes
    ///
    /// The `Content-Type` header is matched case-insensitively. If no matching `Content-Type`
    /// is found or the parsing fails, the method defaults to returning an empty byte vector.
    /// The body processing relies on the implementation of the `ContentType` parsing logic.
    /// Converts the HTTP body into URL-encoded bytes.
    ///
    /// # Returns
    ///
    /// - `Vec<u8>` - The URL-encoded body bytes.
    pub(crate) fn get_body_bytes(&self) -> Vec<u8> {
        let header: RequestHeaders = self.get_header();
        let body: Body = self.get_body();
        if let Some(content_type_value) = header.get(CONTENT_TYPE)
            && let Some(first_value) = content_type_value.front()
        {
            let res: String = first_value
                .to_lowercase()
                .parse::<ContentType>()
                .unwrap_or_default()
                .get_body_string(&body);
            return res.into_bytes();
        }
        for (key, value) in &header {
            if key.eq_ignore_ascii_case(CONTENT_TYPE)
                && let Some(first_value) = value.front()
            {
                let res: String = first_value
                    .to_lowercase()
                    .parse::<ContentType>()
                    .unwrap_or_default()
                    .get_body_string(&body);
                return res.into_bytes();
            }
        }
        String::new().into_bytes()
    }

    /// Retrieves the full path of the HTTP request, including the query string if present.
    ///
    /// This function constructs and returns the complete path of the HTTP request, which
    /// is composed of the path and, if available, the query string. The method checks if
    /// the `url_obj` contains a query string, and if it does, appends it to the path using
    /// the appropriate query separator (`?`). If no query string is present, only the
    /// path is returned.
    ///
    /// The function defaults to a predefined path (`DEFAULT_HTTP_PATH`) if the path is
    /// not set in the `url_obj` configuration. If the query string is empty, the function
    /// simply returns the path.
    ///
    /// # Returns
    ///
    /// - `String` - The full path, including the query string if available, or just the
    ///   path if no query string is present.
    ///
    /// Gets the full request path including query string.
    ///
    /// # Returns
    ///
    /// - `String` - The full path with query string if present.
    pub(crate) fn get_path(&self) -> String {
        let path: String = self.config.read().map_or(String::new(), |config| {
            let query: String = config.url_obj.query.clone().unwrap_or_default();
            if query.is_empty() {
                config
                    .url_obj
                    .path
                    .clone()
                    .unwrap_or(DEFAULT_HTTP_PATH.to_string())
            } else {
                format!(
                    "{}{}{}",
                    config.url_obj.path.clone().unwrap_or_default(),
                    QUERY,
                    query
                )
            }
        });
        path
    }

    /// Sends a GET request over the provided stream and returns the HTTP response.
    ///
    /// This method constructs and sends an HTTP GET request to the server. It formats the URL path
    /// and query parameters based on the current configuration and sends the request to the server
    /// via the provided `stream`. After sending the request, it waits for the response and reads
    /// the result.
    ///
    /// # Arguments
    /// - `stream`: A mutable reference to a `Box<dyn ReadWrite>`, representing the stream used
    ///   for sending and receiving data.
    ///
    /// # Returns
    ///
    /// - `Result<BoxResponseTrait, RequestError>` -
    /// - `Ok(BoxResponseTrait)` contains the HTTP response received from the server.
    /// - `Err(RequestError)` indicates that an error occurred while sending the request or reading the response.
    fn send_get_request(
        &mut self,
        stream: &mut Box<dyn ReadWrite>,
    ) -> Result<BoxResponseTrait, RequestError> {
        let path: String = self.get_path();
        let header_bytes: Vec<u8> = self.get_header_bytes();
        let http_version_str: String =
            self.config.read().map_or("HTTP/1.1".to_string(), |config| {
                config.http_version.to_string()
            });

        let request: Vec<u8> =
            SharedRequestBuilder::build_get_request(path, header_bytes, http_version_str);

        stream
            .write_all(&request)
            .and_then(|_| stream.flush())
            .map_err(|error: std::io::Error| RequestError::Request(error.to_string()))?;
        self.read_response(stream)
    }

    /// Sends a POST request over the provided stream and returns the HTTP response.
    ///
    /// This method constructs and sends an HTTP POST request to the server. It formats the URL path
    /// and sends the body content along with the headers to the server via the provided `stream`. After
    /// sending the request, it waits for the response and reads the result.
    ///
    /// # Arguments
    /// - `stream`: A mutable reference to a `Box<dyn ReadWrite>`, representing the stream used
    ///   for sending and receiving data.
    ///
    /// # Returns
    ///
    /// - `Result<BoxResponseTrait, RequestError>` -
    /// - `Ok(BoxResponseTrait)` contains the HTTP response received from the server.
    /// - `Err(RequestError)` indicates that an error occurred while sending the request or reading the response.
    fn send_post_request(
        &mut self,
        stream: &mut Box<dyn ReadWrite>,
    ) -> Result<BoxResponseTrait, RequestError> {
        let path: String = self.get_path();
        let header_bytes: Vec<u8> = self.get_header_bytes();
        let body_bytes: Vec<u8> = self.get_body_bytes();
        let http_version_str: String =
            self.config.read().map_or("HTTP/1.1".to_string(), |config| {
                config.http_version.to_string()
            });

        let request: Vec<u8> = SharedRequestBuilder::build_post_request(
            path,
            header_bytes,
            body_bytes,
            http_version_str,
        );

        stream
            .write_all(&request)
            .and_then(|_| stream.flush())
            .map_err(|error: std::io::Error| RequestError::Request(error.to_string()))?;
        self.read_response(stream)
    }

    /// Reads the HTTP response from the provided stream.
    ///
    /// This method reads the response from the server after sending an HTTP request. It processes the
    /// headers, checks for redirects, and retrieves the response body based on the content length.
    /// If a redirect is detected, it follows the redirection URL. The method ensures that the entire
    /// response is read before returning.
    ///
    /// # Arguments
    /// - `stream`: A mutable reference to a `Box<dyn ReadWrite>`, representing the stream used
    ///   for receiving the response.
    ///
    /// # Returns
    ///
    /// - `Result<BoxResponseTrait, RequestError>` -
    /// - `Ok(BoxResponseTrait)` contains the complete HTTP response after processing headers and body.
    /// - `Err(RequestError)` indicates that an error occurred while reading the response.
    fn read_response(
        &mut self,
        stream: &mut Box<dyn ReadWrite>,
    ) -> Result<BoxResponseTrait, RequestError> {
        let buffer_size: usize = self
            .config
            .read()
            .map_or(DEFAULT_BUFFER_SIZE, |config| config.buffer);
        let mut buffer: Vec<u8> = vec![0; buffer_size];
        let initial_capacity: usize = buffer_size.max(8192);
        let mut response_bytes: Vec<u8> = Vec::with_capacity(initial_capacity);
        let mut headers_done: bool = false;
        let mut content_length: usize = 0;
        let mut redirect_url: Option<Vec<u8>> = None;
        let mut headers_end_pos: usize = 0;
        let mut is_chunked: bool = false;
        let http_version: String = self
            .config
            .read()
            .map_or(HttpVersion::default().to_string(), |config| {
                config.http_version.to_string()
            });
        let http_version_bytes: Vec<u8> = http_version.to_lowercase().into_bytes();
        let location_sign_key: Vec<u8> = format!("{}:", LOCATION.to_lowercase()).into_bytes();
        'read_loop: while let Ok(n) = stream.read(&mut buffer) {
            if n == 0 {
                break;
            }
            let new_capacity: usize = SharedResponseHandler::calculate_buffer_capacity(
                &response_bytes,
                n,
                response_bytes.capacity(),
            );
            if new_capacity > 0 {
                response_bytes.reserve(new_capacity - response_bytes.capacity());
            }
            let old_len: usize = response_bytes.len();
            response_bytes.extend_from_slice(&buffer[..n]);
            if !headers_done {
                let search_start: usize = old_len.saturating_sub(3);
                if let Some(pos) =
                    SharedResponseHandler::find_double_crlf(&response_bytes, search_start)
                {
                    headers_done = true;
                    headers_end_pos = pos + 4;
                    SharedResponseHandler::parse_response_headers(
                        &response_bytes[..headers_end_pos],
                        &http_version_bytes,
                        &location_sign_key,
                        &mut content_length,
                        &mut redirect_url,
                        &mut is_chunked,
                    )?;
                }
            }
            if headers_done {
                if is_chunked {
                    if Self::is_chunked_response_complete(&response_bytes[headers_end_pos..]) {
                        break 'read_loop;
                    }
                } else {
                    let total_expected_length: usize = headers_end_pos + content_length;
                    if response_bytes.len() >= total_expected_length {
                        response_bytes.truncate(total_expected_length);
                        break 'read_loop;
                    }
                }
            }
        }
        if is_chunked {
            let body_bytes: Vec<u8> = response_bytes[headers_end_pos..].to_vec();
            let decoded_body: Vec<u8> = SharedResponseHandler::parse_chunked_body(&body_bytes);
            response_bytes.truncate(headers_end_pos);
            response_bytes.extend_from_slice(&decoded_body);
        }
        self.response = Arc::new(RwLock::new(<HttpResponseBinary as ResponseTrait>::from(
            &response_bytes,
        )));
        if let Ok(config) = self.config.read()
            && (!config.redirect || redirect_url.is_none())
        {
            if config.decode
                && let Ok(mut response) = self.response.write()
            {
                *response = response.decode(config.buffer);
            }
            let response: BoxResponseTrait = Box::new(self.response.read().map_or(
                HttpResponseBinary::default(),
                |response: RwLockReadGuard<'_, HttpResponseBinary>| response.clone(),
            ));
            return Ok(response);
        }
        let url: String = String::from_utf8(redirect_url.unwrap())
            .map_err(|error: FromUtf8Error| RequestError::Request(error.to_string()))?;
        self.handle_redirect(url)
    }

    /// Handles HTTP redirects by following the redirection URL.
    ///
    /// # Arguments
    ///
    /// - `url`: The redirection URL to follow.
    ///
    /// Returns `Ok(BoxResponseTrait)` if the redirection is successful, or `Err(RequestError)` otherwise.
    fn handle_redirect(&mut self, url: String) -> Result<BoxResponseTrait, RequestError> {
        if let Ok(mut config) = self.config.write() {
            if !config.redirect {
                return Err(RequestError::Request("Redirect Not Enabled".to_string()));
            }
            if let Ok(mut tmp) = self.tmp.clone().write() {
                if tmp.visit_url.contains(&url) {
                    return Err(RequestError::Request("Redirect URL Dead Loop".to_string()));
                }
                tmp.visit_url.insert(url.clone());
                if config.redirect_times >= config.max_redirect_times {
                    return Err(RequestError::Request(
                        "Max Redirect Times Exceeded".to_string(),
                    ));
                }
                config.redirect_times += 1;
            }
        }
        self.url(url.clone());
        self.send_sync()
    }

    /// Checks if a chunked response is complete.
    ///
    /// A chunked response is complete when it contains a chunk with size 0
    /// (the terminating chunk).
    ///
    /// # Arguments
    ///
    /// - `&[u8]` - The raw bytes of the chunked body.
    ///
    /// # Returns
    ///
    /// - `bool` - True if the chunked response is complete, false otherwise.
    fn is_chunked_response_complete(body_bytes: &[u8]) -> bool {
        let mut pos: usize = 0;
        while pos < body_bytes.len() {
            let chunk_size_end: usize = match body_bytes[pos..]
                .windows(2)
                .position(|window: &[u8]| window == b"\r\n")
            {
                Some(p) => pos + p,
                None => return false,
            };
            let chunk_size_str: &[u8] = &body_bytes[pos..chunk_size_end];
            let chunk_size_str: &[u8] = match chunk_size_str.iter().position(|&b| b == b';') {
                Some(p) => &chunk_size_str[..p],
                None => chunk_size_str,
            };
            let chunk_size: usize = match std::str::from_utf8(chunk_size_str) {
                Ok(s) => match usize::from_str_radix(s.trim(), 16) {
                    Ok(n) => n,
                    Err(_) => return false,
                },
                Err(_) => return false,
            };
            if chunk_size == 0 {
                return true;
            }
            let chunk_data_start: usize = chunk_size_end + 2;
            let chunk_data_end: usize = chunk_data_start + chunk_size;
            if chunk_data_end + 2 > body_bytes.len() {
                return false;
            }
            pos = chunk_data_end + 2;
        }
        false
    }

    /// Determines the appropriate port for the HTTP request.
    ///
    /// # Arguments
    ///
    /// - `port`: The default port (if any).
    /// - `config`: Configuration for HTTP requests
    ///
    /// Returns the resolved port.
    pub(crate) fn get_port(&self, port: u16, config: &Config) -> u16 {
        if port != 0 {
            return port;
        }
        let protocol: String = Self::get_protocol(config);
        Protocol::get_port(&protocol)
    }

    /// Establishes a connection stream to the specified host and port.
    ///
    /// This method attempts to create a connection stream based on the protocol type
    /// (HTTP or HTTPS) defined by the current configuration. It supports both plain
    /// TCP connections and TLS-secured connections. If the protocol is HTTPS, it will
    /// use the `TlsConnector` to establish a secure TLS connection. For both cases,
    /// it ensures a read timeout is set on the stream.
    ///
    /// # Arguments
    ///
    /// - `host`: The hostname or IP address to connect to.
    /// - `port`: The port number to connect to.
    ///
    /// # Returns
    ///
    /// - `Ok(Box<dyn ReadWrite>)`: A boxed stream that implements the `ReadWrite` trait,
    ///   representing the established connection.
    /// - `Err(RequestError)`: An error indicating what went wrong during the connection process.
    fn get_connection_stream(
        &self,
        host: String,
        port: u16,
    ) -> Result<Box<dyn ReadWrite>, RequestError> {
        let config: Config = self
            .config
            .read()
            .map_or(Config::default(), |config| config.clone());
        if let Some(proxy_config) = &config.proxy {
            return self.get_proxy_connection_stream(host, port, proxy_config);
        }
        let host_port: (String, u16) = (host.clone(), port);
        let timeout: Duration = Duration::from_millis(config.timeout);
        let tcp_stream: TcpStream = TcpStream::connect(host_port.clone())
            .map_err(|error: std::io::Error| RequestError::Request(error.to_string()))?;
        tcp_stream
            .set_read_timeout(Some(timeout))
            .map_err(|error: std::io::Error| RequestError::Request(error.to_string()))?;
        tcp_stream
            .set_write_timeout(Some(timeout))
            .map_err(|error: std::io::Error| RequestError::Request(error.to_string()))?;
        let stream: Result<Box<dyn ReadWrite>, RequestError> =
            if Self::get_protocol(&config) == HTTPS_LOWERCASE {
                match self.tmp.clone().read() {
                    Ok(tmp) => {
                        let roots: RootCertStore = tmp.root_cert.clone();
                        let tls_config: ClientConfig = ClientConfig::builder()
                            .with_root_certificates(roots)
                            .with_no_client_auth();
                        let client_config: Arc<ClientConfig> = Arc::new(tls_config);
                        let dns_name: ServerName<'_> = ServerName::try_from(host.clone()).map_err(
                            |error: InvalidDnsNameError| RequestError::Request(error.to_string()),
                        )?;
                        let session: ClientConnection =
                            ClientConnection::new(Arc::clone(&client_config), dns_name).map_err(
                                |error: rustls::Error| RequestError::Request(error.to_string()),
                            )?;
                        let tls_stream: StreamOwned<ClientConnection, TcpStream> =
                            StreamOwned::new(session, tcp_stream);
                        return Ok(Box::new(tls_stream));
                    }
                    Err(error) => Err(RequestError::Request(error.to_string())),
                }
            } else {
                Ok(Box::new(tcp_stream))
            };
        stream
    }

    /// Establishes a proxy connection stream to the specified host and port.
    fn get_proxy_connection_stream(
        &self,
        target_host: String,
        target_port: u16,
        proxy_config: &ProxyConfig,
    ) -> Result<Box<dyn ReadWrite>, RequestError> {
        let timeout: Duration = Duration::from_millis(self.config.read().map_or(
            DEFAULT_HIGH_SECURITY_READ_TIMEOUT_MS,
            |config: RwLockReadGuard<'_, Config>| config.timeout,
        ));
        match proxy_config.proxy_type {
            ProxyType::Http | ProxyType::Https => {
                self.get_http_proxy_connection(target_host, target_port, proxy_config, timeout)
            }
            ProxyType::Socks5 => {
                self.get_socks5_proxy_connection(target_host, target_port, proxy_config, timeout)
            }
        }
    }

    /// Establishes an HTTP/HTTPS proxy connection.
    fn get_http_proxy_connection(
        &self,
        target_host: String,
        target_port: u16,
        proxy_config: &ProxyConfig,
        timeout: Duration,
    ) -> Result<Box<dyn ReadWrite>, RequestError> {
        let proxy_host_port: (String, u16) = (proxy_config.host.clone(), proxy_config.port);
        let tcp_stream: TcpStream = TcpStream::connect(proxy_host_port)
            .map_err(|error: std::io::Error| RequestError::Request(error.to_string()))?;
        tcp_stream
            .set_read_timeout(Some(timeout))
            .map_err(|error: std::io::Error| RequestError::Request(error.to_string()))?;
        tcp_stream
            .set_write_timeout(Some(timeout))
            .map_err(|error: std::io::Error| RequestError::Request(error.to_string()))?;
        let mut proxy_stream: Box<dyn ReadWrite> = if proxy_config.proxy_type == ProxyType::Https {
            match self.tmp.clone().read() {
                Ok(tmp) => {
                    let roots: RootCertStore = tmp.root_cert.clone();
                    let tls_config: ClientConfig = ClientConfig::builder()
                        .with_root_certificates(roots)
                        .with_no_client_auth();
                    let client_config: Arc<ClientConfig> = Arc::new(tls_config);
                    let dns_name: ServerName<'_> = ServerName::try_from(proxy_config.host.clone())
                        .map_err(|error: InvalidDnsNameError| {
                            RequestError::Request(error.to_string())
                        })?;
                    let session: ClientConnection =
                        ClientConnection::new(Arc::clone(&client_config), dns_name).map_err(
                            |error: rustls::Error| RequestError::Request(error.to_string()),
                        )?;
                    let tls_stream: StreamOwned<ClientConnection, TcpStream> =
                        StreamOwned::new(session, tcp_stream);
                    Box::new(tls_stream)
                }
                Err(error) => {
                    return Err(RequestError::Request(error.to_string()));
                }
            }
        } else {
            Box::new(tcp_stream)
        };
        let connect_request: String = if let (Some(username), Some(password)) =
            (&proxy_config.username, &proxy_config.password)
        {
            let auth: String = format!("{username}:{password}");
            let auth_encoded: String = base64_encode(auth.as_bytes());
            format!(
                "CONNECT {target_host}:{target_port} HTTP/1.1\r\nHost: {target_host}:{target_port}\r\nProxy-Authorization: Basic {auth_encoded}\r\n\r\n"
            )
        } else {
            format!(
                "CONNECT {target_host}:{target_port} HTTP/1.1\r\nHost: {target_host}:{target_port}\r\n\r\n"
            )
        };
        proxy_stream
            .write_all(connect_request.as_bytes())
            .map_err(|error: std::io::Error| RequestError::Request(error.to_string()))?;
        proxy_stream
            .flush()
            .map_err(|error: std::io::Error| RequestError::Request(error.to_string()))?;
        let mut response_buffer = [0u8; 1024];
        let bytes_read: usize = proxy_stream
            .read(&mut response_buffer)
            .map_err(|error: std::io::Error| RequestError::Request(error.to_string()))?;
        let response_str: &str = std::str::from_utf8(&response_buffer[..bytes_read]).unwrap_or("");
        let headers_end_pos: Option<usize> = response_str.find("\r\n\r\n");
        let pre_read_data: Vec<u8> = if let Some(pos) = headers_end_pos {
            let header_part: &str = &response_str[..pos];
            if !header_part.starts_with("HTTP/1.1 200") && !header_part.starts_with("HTTP/1.0 200")
            {
                return Err(RequestError::Request("Internal Server Error".to_string()));
            }
            response_buffer[pos + 4..bytes_read].to_vec()
        } else {
            if !response_str.starts_with("HTTP/1.1 200")
                && !response_str.starts_with("HTTP/1.0 200")
            {
                return Err(RequestError::Request("Internal Server Error".to_string()));
            }
            vec![]
        };
        let config: Config = self
            .config
            .read()
            .map_or(Config::default(), |config| config.clone());
        if Self::get_protocol(&config) == HTTPS_LOWERCASE {
            match self.tmp.clone().read() {
                Ok(tmp) => {
                    let roots: RootCertStore = tmp.root_cert.clone();
                    let tls_config: ClientConfig = ClientConfig::builder()
                        .with_root_certificates(roots)
                        .with_no_client_auth();
                    let client_config: Arc<ClientConfig> = Arc::new(tls_config);
                    let dns_name: ServerName<'_> = ServerName::try_from(target_host.clone())
                        .map_err(|error: InvalidDnsNameError| {
                            RequestError::Request(error.to_string())
                        })?;
                    let session: ClientConnection =
                        ClientConnection::new(Arc::clone(&client_config), dns_name).map_err(
                            |error: rustls::Error| RequestError::Request(error.to_string()),
                        )?;
                    let tunnel_stream = SyncProxyTunnelStream::new(proxy_stream, pre_read_data);
                    let tls_stream: StreamOwned<ClientConnection, SyncProxyTunnelStream> =
                        StreamOwned::new(session, tunnel_stream);
                    return Ok(Box::new(tls_stream));
                }
                Err(error) => {
                    return Err(RequestError::Request(error.to_string()));
                }
            }
        }
        let tunnel_stream: SyncProxyTunnelStream =
            SyncProxyTunnelStream::new(proxy_stream, pre_read_data);
        Ok(Box::new(tunnel_stream))
    }

    /// Establishes a SOCKS5 proxy connection.
    fn get_socks5_proxy_connection(
        &self,
        target_host: String,
        target_port: u16,
        proxy_config: &ProxyConfig,
        timeout: Duration,
    ) -> Result<Box<dyn ReadWrite>, RequestError> {
        let proxy_host_port: (String, u16) = (proxy_config.host.clone(), proxy_config.port);
        let mut tcp_stream: TcpStream = TcpStream::connect(proxy_host_port)
            .map_err(|error: std::io::Error| RequestError::Request(error.to_string()))?;
        tcp_stream
            .set_read_timeout(Some(timeout))
            .map_err(|error: std::io::Error| RequestError::Request(error.to_string()))?;
        tcp_stream
            .set_write_timeout(Some(timeout))
            .map_err(|error: std::io::Error| RequestError::Request(error.to_string()))?;
        let auth_methods: Vec<u8> =
            if proxy_config.username.is_some() && proxy_config.password.is_some() {
                vec![0x05, 0x02, 0x00, 0x02]
            } else {
                vec![0x05, 0x01, 0x00]
            };
        tcp_stream
            .write_all(&auth_methods)
            .map_err(|error: std::io::Error| RequestError::Request(error.to_string()))?;
        let mut response = [0u8; 2];
        tcp_stream
            .read_exact(&mut response)
            .map_err(|error: std::io::Error| RequestError::Request(error.to_string()))?;
        if response[0] != 0x05 {
            return Err(RequestError::Request("Internal Server Error".to_string()));
        }
        match response[1] {
            0x00 => {}
            0x02 => {
                if let (Some(username), Some(password)) =
                    (&proxy_config.username, &proxy_config.password)
                {
                    let mut auth_request: Vec<u8> = vec![0x01];
                    auth_request.push(username.len() as u8);
                    auth_request.extend_from_slice(username.as_bytes());
                    auth_request.push(password.len() as u8);
                    auth_request.extend_from_slice(password.as_bytes());
                    tcp_stream
                        .write_all(&auth_request)
                        .map_err(|error: std::io::Error| {
                            RequestError::Request(error.to_string())
                        })?;
                    let mut auth_response: [u8; 2] = [0u8; 2];
                    tcp_stream.read_exact(&mut auth_response).map_err(
                        |error: std::io::Error| RequestError::Request(error.to_string()),
                    )?;
                    if auth_response[1] != 0x00 {
                        return Err(RequestError::Request("Internal Server Error".to_string()));
                    }
                } else {
                    return Err(RequestError::Request("Internal Server Error".to_string()));
                }
            }
            0xFF => {
                return Err(RequestError::Request("Internal Server Error".to_string()));
            }
            _ => {
                return Err(RequestError::Request("Internal Server Error".to_string()));
            }
        }
        let mut connect_request: Vec<u8> = vec![0x05, 0x01, 0x00];
        if target_host.parse::<Ipv4Addr>().is_ok() {
            connect_request.push(0x01);
            let ip: Ipv4Addr = target_host.parse().unwrap();
            connect_request.extend_from_slice(&ip.octets());
        } else if target_host.parse::<Ipv6Addr>().is_ok() {
            connect_request.push(0x04);
            let ip: Ipv6Addr = target_host.parse().unwrap();
            connect_request.extend_from_slice(&ip.octets());
        } else {
            connect_request.push(0x03);
            connect_request.push(target_host.len() as u8);
            connect_request.extend_from_slice(target_host.as_bytes());
        }
        connect_request.extend_from_slice(&target_port.to_be_bytes());
        tcp_stream
            .write_all(&connect_request)
            .map_err(|error: std::io::Error| RequestError::Request(error.to_string()))?;
        let mut connect_response: [u8; 4] = [0u8; 4];
        tcp_stream
            .read_exact(&mut connect_response)
            .map_err(|error: std::io::Error| RequestError::Request(error.to_string()))?;
        if connect_response[0] != 0x05 || connect_response[1] != 0x00 {
            return Err(RequestError::Request("Internal Server Error".to_string()));
        }
        match connect_response[3] {
            0x01 => {
                let mut skip = [0u8; 6];
                tcp_stream
                    .read_exact(&mut skip)
                    .map_err(|error: std::io::Error| RequestError::Request(error.to_string()))?;
            }
            0x03 => {
                let mut len = [0u8; 1];
                tcp_stream
                    .read_exact(&mut len)
                    .map_err(|error: std::io::Error| RequestError::Request(error.to_string()))?;
                let mut skip = vec![0u8; len[0] as usize + 2];
                tcp_stream
                    .read_exact(&mut skip)
                    .map_err(|error: std::io::Error| RequestError::Request(error.to_string()))?;
            }
            0x04 => {
                let mut skip = [0u8; 18];
                tcp_stream
                    .read_exact(&mut skip)
                    .map_err(|error: std::io::Error| RequestError::Request(error.to_string()))?;
            }
            _ => {
                return Err(RequestError::Request("Internal Server Error".to_string()));
            }
        }
        let proxy_stream: Box<dyn ReadWrite> = Box::new(tcp_stream);
        let config: Config = self
            .config
            .read()
            .map_or(Config::default(), |config| config.clone());
        if Self::get_protocol(&config) == HTTPS_LOWERCASE {
            match self.tmp.clone().read() {
                Ok(tmp) => {
                    let roots: RootCertStore = tmp.root_cert.clone();
                    let tls_config: ClientConfig = ClientConfig::builder()
                        .with_root_certificates(roots)
                        .with_no_client_auth();
                    let client_config: Arc<ClientConfig> = Arc::new(tls_config);
                    let dns_name: ServerName<'_> = ServerName::try_from(target_host.clone())
                        .map_err(|error: InvalidDnsNameError| {
                            RequestError::Request(error.to_string())
                        })?;
                    let session: ClientConnection =
                        ClientConnection::new(Arc::clone(&client_config), dns_name).map_err(
                            |error: rustls::Error| RequestError::Request(error.to_string()),
                        )?;
                    let tunnel_stream: SyncProxyTunnelStream =
                        SyncProxyTunnelStream::new(proxy_stream, vec![]);
                    let tls_stream: StreamOwned<ClientConnection, SyncProxyTunnelStream> =
                        StreamOwned::new(session, tunnel_stream);
                    return Ok(Box::new(tls_stream));
                }
                Err(error) => {
                    return Err(RequestError::Request(error.to_string()));
                }
            }
        }
        Ok(proxy_stream)
    }
}

impl HttpRequest {
    /// Sends the HTTP request synchronously.
    pub(crate) fn send_sync(&mut self) -> RequestResult {
        let methods: Method = self.get_methods();
        let mut host: String = String::new();
        let mut port: u16 = u16::default();
        if let Ok(mut config) = self.config.write() {
            config.url_obj = self
                .parse_url()
                .map_err(|error: RequestError| RequestError::Request(error.to_string()))?;
            host = config.url_obj.host.clone().unwrap_or_default();
            port = self.get_port(config.url_obj.port.unwrap_or_default(), &config);
        }
        let mut stream: BoxReadWrite = self.get_connection_stream(host, port)?;
        let res: Result<BoxResponseTrait, RequestError> = match methods {
            m if m.is_get() => self.send_get_request(&mut stream),
            m if m.is_post() => self.send_post_request(&mut stream),
            _err => Err(RequestError::Request("Method Not Allowed".to_string())),
        };
        res
    }
}

/// Async implementation for HttpRequest
impl HttpRequest {
    /// Sends an async GET request.
    ///
    /// # Arguments
    ///
    /// - `&mut BoxAsyncReadWrite` - The async stream to write to.
    ///
    /// # Returns
    ///
    /// - `Result<BoxResponseTrait, RequestError>` - Result containing the response or error.
    async fn send_get_request_async(
        &mut self,
        stream: &mut BoxAsyncReadWrite,
    ) -> Result<BoxResponseTrait, RequestError> {
        let path: String = self.get_path();
        let header_bytes: Vec<u8> = self.get_header_bytes();
        let http_version_str: String =
            self.config.read().map_or("HTTP/1.1".to_string(), |config| {
                config.http_version.to_string()
            });
        let request: Vec<u8> =
            SharedRequestBuilder::build_get_request(path, header_bytes, http_version_str);
        stream
            .write_all(&request)
            .await
            .map_err(|error: std::io::Error| RequestError::Request(error.to_string()))?;
        stream
            .flush()
            .await
            .map_err(|error: std::io::Error| RequestError::Request(error.to_string()))?;
        self.read_response_async(stream).await
    }

    /// Sends an async POST request.
    ///
    /// # Arguments
    ///
    /// - `&mut BoxAsyncReadWrite` - The async stream to write to.
    ///
    /// # Returns
    ///
    /// - `Result<BoxResponseTrait, RequestError>` - Result containing the response or error.
    async fn send_post_request_async(
        &mut self,
        stream: &mut BoxAsyncReadWrite,
    ) -> Result<BoxResponseTrait, RequestError> {
        let path: String = self.get_path();
        let header_bytes: Vec<u8> = self.get_header_bytes();
        let body_bytes: Vec<u8> = self.get_body_bytes();
        let http_version_str: String =
            self.config.read().map_or("HTTP/1.1".to_string(), |config| {
                config.http_version.to_string()
            });
        let request: Vec<u8> = SharedRequestBuilder::build_post_request(
            path,
            header_bytes,
            body_bytes,
            http_version_str,
        );
        stream
            .write_all(&request)
            .await
            .map_err(|error: std::io::Error| RequestError::Request(error.to_string()))?;
        stream
            .flush()
            .await
            .map_err(|error: std::io::Error| RequestError::Request(error.to_string()))?;
        self.read_response_async(stream).await
    }

    /// Reads an async HTTP response.
    ///
    /// # Arguments
    ///
    /// - `&mut BoxAsyncReadWrite` - The async stream to read from.
    ///
    /// # Returns
    ///
    /// - `Result<BoxResponseTrait, RequestError>` - Result containing the response or error.
    async fn read_response_async(
        &mut self,
        stream: &mut BoxAsyncReadWrite,
    ) -> Result<BoxResponseTrait, RequestError> {
        let buffer_size: usize = self
            .config
            .read()
            .map_or(DEFAULT_BUFFER_SIZE, |config| config.buffer);
        let mut buffer: Vec<u8> = vec![0; buffer_size];
        let initial_capacity: usize = buffer_size.max(8192);
        let mut response_bytes: Vec<u8> = Vec::with_capacity(initial_capacity);
        let mut headers_done: bool = false;
        let mut content_length: usize = 0;
        let mut redirect_url: Option<Vec<u8>> = None;
        let mut headers_end_pos: usize = 0;
        let mut is_chunked: bool = false;
        let http_version: String = self
            .config
            .read()
            .map_or(HttpVersion::default().to_string(), |config| {
                config.http_version.to_string()
            });
        let http_version_bytes: Vec<u8> = http_version.to_lowercase().into_bytes();
        let location_sign_key: Vec<u8> = format!("{}:", LOCATION.to_lowercase()).into_bytes();
        'read_loop: loop {
            let bytes_read: usize = stream
                .read(&mut buffer)
                .await
                .map_err(|error: std::io::Error| RequestError::Request(error.to_string()))?;
            if bytes_read == 0 {
                break;
            }
            let new_capacity: usize = SharedResponseHandler::calculate_buffer_capacity(
                &response_bytes,
                bytes_read,
                response_bytes.capacity(),
            );
            if new_capacity > 0 {
                response_bytes.reserve(new_capacity - response_bytes.capacity());
            }
            let old_len: usize = response_bytes.len();
            response_bytes.extend_from_slice(&buffer[..bytes_read]);
            if !headers_done {
                let search_start: usize = old_len.saturating_sub(3);
                if let Some(pos) =
                    SharedResponseHandler::find_double_crlf(&response_bytes, search_start)
                {
                    headers_done = true;
                    headers_end_pos = pos + 4;
                    SharedResponseHandler::parse_response_headers(
                        &response_bytes[..headers_end_pos],
                        &http_version_bytes,
                        &location_sign_key,
                        &mut content_length,
                        &mut redirect_url,
                        &mut is_chunked,
                    )?;
                }
            }
            if headers_done {
                if is_chunked {
                    if Self::is_chunked_response_complete(&response_bytes[headers_end_pos..]) {
                        break 'read_loop;
                    }
                } else {
                    let total_expected_length: usize = headers_end_pos + content_length;
                    if response_bytes.len() >= total_expected_length {
                        response_bytes.truncate(total_expected_length);
                        break 'read_loop;
                    }
                }
            }
        }
        if is_chunked {
            let body_bytes: Vec<u8> = response_bytes[headers_end_pos..].to_vec();
            let decoded_body: Vec<u8> = SharedResponseHandler::parse_chunked_body(&body_bytes);
            response_bytes.truncate(headers_end_pos);
            response_bytes.extend_from_slice(&decoded_body);
        }
        self.response = Arc::new(RwLock::new(<HttpResponseBinary as ResponseTrait>::from(
            &response_bytes,
        )));
        let (should_redirect, should_decode, buffer_size) = {
            if let Ok(config) = self.config.read() {
                (config.redirect, config.decode, config.buffer)
            } else {
                (false, false, DEFAULT_BUFFER_SIZE)
            }
        };
        if !should_redirect || redirect_url.is_none() {
            if should_decode && let Ok(mut response) = self.response.write() {
                *response = response.decode(buffer_size);
            }
            let response: BoxResponseTrait = Box::new(self.response.read().map_or(
                HttpResponseBinary::default(),
                |response: RwLockReadGuard<'_, HttpResponseBinary>| response.clone(),
            ));
            return Ok(response);
        }
        let url: String = String::from_utf8(redirect_url.unwrap())
            .map_err(|error: FromUtf8Error| RequestError::Request(error.to_string()))?;
        self.handle_redirect_async(url).await
    }

    /// Handles async HTTP redirects.
    ///
    /// # Arguments
    ///
    /// - `String` - The redirect URL.
    ///
    /// # Returns
    ///
    /// - `Pin<Box<dyn Future<Output = Result<BoxResponseTrait, RequestError>> + Send + '_>>` - Future representing the redirect handling.
    fn handle_redirect_async(
        &mut self,
        url: String,
    ) -> Pin<Box<dyn Future<Output = Result<BoxResponseTrait, RequestError>> + Send + '_>> {
        Box::pin(async move {
            {
                if let Ok(mut config) = self.config.write() {
                    if !config.redirect {
                        return Err(RequestError::Request("Redirect Not Enabled".to_string()));
                    }
                    if let Ok(mut tmp) = self.tmp.clone().write() {
                        if tmp.visit_url.contains(&url) {
                            return Err(RequestError::Request(
                                "Redirect URL Dead Loop".to_string(),
                            ));
                        }
                        tmp.visit_url.insert(url.clone());
                        if config.redirect_times >= config.max_redirect_times {
                            return Err(RequestError::Request(
                                "Max Redirect Times Exceeded".to_string(),
                            ));
                        }
                        config.redirect_times += 1;
                    }
                }
            }
            self.url(url.clone());
            self.send_async().await
        })
    }

    /// Establishes an async connection stream.
    ///
    /// # Arguments
    ///
    /// - `String` - The host to connect to.
    /// - `u16` - The port to connect to.
    ///
    /// # Returns
    ///
    /// - `Result<BoxAsyncReadWrite, RequestError>` - Result containing the stream or error.
    async fn get_connection_stream_async(
        &self,
        host: String,
        port: u16,
    ) -> Result<BoxAsyncReadWrite, RequestError> {
        let config: Config = self
            .config
            .read()
            .map_or(Config::default(), |config| config.clone());
        if let Some(proxy_config) = &config.proxy {
            return self
                .get_proxy_connection_stream_async(host, port, proxy_config)
                .await;
        }
        let host_port: (String, u16) = (host.clone(), port);
        let tcp_stream: http_type::tokio::net::TcpStream =
            http_type::tokio::net::TcpStream::connect(host_port.clone())
                .await
                .map_err(|error: std::io::Error| RequestError::Request(error.to_string()))?;
        if Self::get_protocol(&config) == HTTPS_LOWERCASE {
            let roots: RootCertStore = {
                match self.tmp.clone().read() {
                    Ok(tmp) => tmp.root_cert.clone(),
                    Err(error) => {
                        return Err(RequestError::Request(error.to_string()));
                    }
                }
            };
            let tls_config: ClientConfig = ClientConfig::builder()
                .with_root_certificates(roots)
                .with_no_client_auth();
            let connector: TlsConnector = TlsConnector::from(Arc::new(tls_config));
            let dns_name: ServerName<'_> = ServerName::try_from(host.clone())
                .map_err(|error: InvalidDnsNameError| RequestError::Request(error.to_string()))?;
            let tls_stream: TlsStream<http_type::tokio::net::TcpStream> = connector
                .connect(dns_name, tcp_stream)
                .await
                .map_err(|error: std::io::Error| RequestError::Request(error.to_string()))?;
            Ok(Box::new(tls_stream))
        } else {
            Ok(Box::new(tcp_stream))
        }
    }

    /// Establishes an async proxy connection stream.
    ///
    /// # Arguments
    ///
    /// - `String` - The target host.
    /// - `u16` - The target port.
    /// - `&ProxyConfig` - The proxy configuration.
    ///
    /// # Returns
    ///
    /// - `Result<BoxAsyncReadWrite, RequestError>` - Result containing the stream or error.
    async fn get_proxy_connection_stream_async(
        &self,
        target_host: String,
        target_port: u16,
        proxy_config: &ProxyConfig,
    ) -> Result<BoxAsyncReadWrite, RequestError> {
        match proxy_config.proxy_type {
            ProxyType::Http | ProxyType::Https => {
                self.get_http_proxy_connection_async(target_host, target_port, proxy_config)
                    .await
            }
            ProxyType::Socks5 => {
                self.get_socks5_proxy_connection_async(target_host, target_port, proxy_config)
                    .await
            }
        }
    }

    /// Establishes an async HTTP/HTTPS proxy connection.
    ///
    /// # Arguments
    ///
    /// - `String` - The target host.
    /// - `u16` - The target port.
    /// - `&ProxyConfig` - The proxy configuration.
    ///
    /// # Returns
    ///
    /// - `Result<BoxAsyncReadWrite, RequestError>` - Result containing the stream or error.
    async fn get_http_proxy_connection_async(
        &self,
        target_host: String,
        target_port: u16,
        proxy_config: &ProxyConfig,
    ) -> Result<BoxAsyncReadWrite, RequestError> {
        let proxy_host_port: (String, u16) = (proxy_config.host.clone(), proxy_config.port);
        let tcp_stream: http_type::tokio::net::TcpStream =
            http_type::tokio::net::TcpStream::connect(proxy_host_port)
                .await
                .map_err(|error: std::io::Error| RequestError::Request(error.to_string()))?;
        let mut proxy_stream: BoxAsyncReadWrite = if proxy_config.proxy_type == ProxyType::Https {
            let roots: RootCertStore = {
                match self.tmp.clone().read() {
                    Ok(tmp) => tmp.root_cert.clone(),
                    Err(error) => {
                        return Err(RequestError::Request(error.to_string()));
                    }
                }
            };
            let tls_config: ClientConfig = ClientConfig::builder()
                .with_root_certificates(roots)
                .with_no_client_auth();
            let connector: TlsConnector = TlsConnector::from(Arc::new(tls_config));
            let dns_name: ServerName<'_> = ServerName::try_from(proxy_config.host.clone())
                .map_err(|error: InvalidDnsNameError| RequestError::Request(error.to_string()))?;
            let tls_stream: TlsStream<http_type::tokio::net::TcpStream> = connector
                .connect(dns_name, tcp_stream)
                .await
                .map_err(|error: std::io::Error| RequestError::Request(error.to_string()))?;
            Box::new(tls_stream)
        } else {
            Box::new(tcp_stream)
        };
        let connect_request: String = if let (Some(username), Some(password)) =
            (&proxy_config.username, &proxy_config.password)
        {
            let auth: String = format!("{username}:{password}");
            let auth_encoded: String = base64_encode(auth.as_bytes());
            format!(
                "CONNECT {target_host}:{target_port} HTTP/1.1\r\nHost: {target_host}:{target_port}\r\nProxy-Authorization: Basic {auth_encoded}\r\n\r\n"
            )
        } else {
            format!(
                "CONNECT {target_host}:{target_port} HTTP/1.1\r\nHost: {target_host}:{target_port}\r\n\r\n"
            )
        };
        proxy_stream
            .write_all(connect_request.as_bytes())
            .await
            .map_err(|error: std::io::Error| RequestError::Request(error.to_string()))?;
        proxy_stream
            .flush()
            .await
            .map_err(|error: std::io::Error| RequestError::Request(error.to_string()))?;
        let mut response_buffer: [u8; 1024] = [0u8; 1024];
        let bytes_read: usize = proxy_stream
            .read(&mut response_buffer)
            .await
            .map_err(|error: std::io::Error| RequestError::Request(error.to_string()))?;
        let response_str: &str = std::str::from_utf8(&response_buffer[..bytes_read]).unwrap_or("");
        let headers_end_pos: Option<usize> = response_str.find("\r\n\r\n");
        let pre_read_data: Vec<u8> = if let Some(pos) = headers_end_pos {
            let header_part: &str = &response_str[..pos];
            if !header_part.starts_with("HTTP/1.1 200") && !header_part.starts_with("HTTP/1.0 200")
            {
                return Err(RequestError::Request("Internal Server Error".to_string()));
            }
            response_buffer[pos + 4..bytes_read].to_vec()
        } else {
            if !response_str.starts_with("HTTP/1.1 200")
                && !response_str.starts_with("HTTP/1.0 200")
            {
                return Err(RequestError::Request("Internal Server Error".to_string()));
            }
            vec![]
        };
        let config: Config = self
            .config
            .read()
            .map_or(Config::default(), |config| config.clone());
        if Self::get_protocol(&config) == HTTPS_LOWERCASE {
            let roots: RootCertStore = {
                match self.tmp.clone().read() {
                    Ok(tmp) => tmp.root_cert.clone(),
                    Err(error) => {
                        return Err(RequestError::Request(error.to_string()));
                    }
                }
            };
            let tls_config: ClientConfig = ClientConfig::builder()
                .with_root_certificates(roots)
                .with_no_client_auth();
            let connector: TlsConnector = TlsConnector::from(Arc::new(tls_config));
            let dns_name: ServerName<'_> = ServerName::try_from(target_host.clone())
                .map_err(|error: InvalidDnsNameError| RequestError::Request(error.to_string()))?;
            let tunnel_stream: ProxyTunnelStream =
                ProxyTunnelStream::new(proxy_stream, pre_read_data);
            let tls_stream: TlsStream<ProxyTunnelStream> = connector
                .connect(dns_name, tunnel_stream)
                .await
                .map_err(|error: std::io::Error| RequestError::Request(error.to_string()))?;
            return Ok(Box::new(tls_stream) as BoxAsyncReadWrite);
        }
        let tunnel_stream: ProxyTunnelStream = ProxyTunnelStream::new(proxy_stream, pre_read_data);
        Ok(Box::new(tunnel_stream) as BoxAsyncReadWrite)
    }

    /// Establishes an async SOCKS5 proxy connection.
    ///
    /// # Arguments
    ///
    /// - `String` - The target host.
    /// - `u16` - The target port.
    /// - `&ProxyConfig` - The proxy configuration.
    ///
    /// # Returns
    ///
    /// - `Result<BoxAsyncReadWrite, RequestError>` - Result containing the stream or error.
    async fn get_socks5_proxy_connection_async(
        &self,
        target_host: String,
        target_port: u16,
        proxy_config: &ProxyConfig,
    ) -> Result<BoxAsyncReadWrite, RequestError> {
        let proxy_host_port: (String, u16) = (proxy_config.host.clone(), proxy_config.port);
        let mut tcp_stream: http_type::tokio::net::TcpStream =
            http_type::tokio::net::TcpStream::connect(proxy_host_port)
                .await
                .map_err(|error: std::io::Error| RequestError::Request(error.to_string()))?;
        let auth_methods: Vec<u8> =
            if proxy_config.username.is_some() && proxy_config.password.is_some() {
                vec![0x05, 0x02, 0x00, 0x02]
            } else {
                vec![0x05, 0x01, 0x00]
            };
        tcp_stream
            .write_all(&auth_methods)
            .await
            .map_err(|error: std::io::Error| RequestError::Request(error.to_string()))?;
        let mut response: [u8; 2] = [0u8; 2];
        tcp_stream
            .read_exact(&mut response)
            .await
            .map_err(|error: std::io::Error| RequestError::Request(error.to_string()))?;
        if response[0] != 0x05 {
            return Err(RequestError::Request("Internal Server Error".to_string()));
        }
        match response[1] {
            0x00 => {}
            0x02 => {
                if let (Some(username), Some(password)) =
                    (&proxy_config.username, &proxy_config.password)
                {
                    let mut auth_request = vec![0x01];
                    auth_request.push(username.len() as u8);
                    auth_request.extend_from_slice(username.as_bytes());
                    auth_request.push(password.len() as u8);
                    auth_request.extend_from_slice(password.as_bytes());

                    tcp_stream.write_all(&auth_request).await.map_err(
                        |error: std::io::Error| RequestError::Request(error.to_string()),
                    )?;

                    let mut auth_response = [0u8; 2];
                    tcp_stream.read_exact(&mut auth_response).await.map_err(
                        |error: std::io::Error| RequestError::Request(error.to_string()),
                    )?;

                    if auth_response[1] != 0x00 {
                        return Err(RequestError::Request("Internal Server Error".to_string()));
                    }
                } else {
                    return Err(RequestError::Request("Internal Server Error".to_string()));
                }
            }
            0xFF => {
                return Err(RequestError::Request("Internal Server Error".to_string()));
            }
            _ => {
                return Err(RequestError::Request("Internal Server Error".to_string()));
            }
        }
        let mut connect_request: Vec<u8> = vec![0x05, 0x01, 0x00];
        if target_host.parse::<Ipv4Addr>().is_ok() {
            connect_request.push(0x01);
            let ip: Ipv4Addr = target_host.parse().unwrap();
            connect_request.extend_from_slice(&ip.octets());
        } else if target_host.parse::<Ipv6Addr>().is_ok() {
            connect_request.push(0x04);
            let ip: Ipv6Addr = target_host.parse().unwrap();
            connect_request.extend_from_slice(&ip.octets());
        } else {
            connect_request.push(0x03);
            connect_request.push(target_host.len() as u8);
            connect_request.extend_from_slice(target_host.as_bytes());
        }
        connect_request.extend_from_slice(&target_port.to_be_bytes());
        tcp_stream
            .write_all(&connect_request)
            .await
            .map_err(|error: std::io::Error| RequestError::Request(error.to_string()))?;

        let mut connect_response: [u8; 4] = [0u8; 4];
        tcp_stream
            .read_exact(&mut connect_response)
            .await
            .map_err(|error: std::io::Error| RequestError::Request(error.to_string()))?;

        if connect_response[0] != 0x05 || connect_response[1] != 0x00 {
            return Err(RequestError::Request("Internal Server Error".to_string()));
        }
        match connect_response[3] {
            0x01 => {
                let mut skip: [u8; 6] = [0u8; 6];
                tcp_stream
                    .read_exact(&mut skip)
                    .await
                    .map_err(|error: std::io::Error| RequestError::Request(error.to_string()))?;
            }
            0x03 => {
                let mut len: [u8; 1] = [0u8; 1];
                tcp_stream
                    .read_exact(&mut len)
                    .await
                    .map_err(|error: std::io::Error| RequestError::Request(error.to_string()))?;
                let mut skip: Vec<u8> = vec![0u8; len[0] as usize + 2];
                tcp_stream
                    .read_exact(&mut skip)
                    .await
                    .map_err(|error: std::io::Error| RequestError::Request(error.to_string()))?;
            }
            0x04 => {
                let mut skip: [u8; 18] = [0u8; 18];
                tcp_stream
                    .read_exact(&mut skip)
                    .await
                    .map_err(|error: std::io::Error| RequestError::Request(error.to_string()))?;
            }
            _ => {
                return Err(RequestError::Request("Internal Server Error".to_string()));
            }
        }
        let proxy_stream: BoxAsyncReadWrite = Box::new(tcp_stream);
        let config: Config = self
            .config
            .read()
            .map_or(Config::default(), |config| config.clone());
        if Self::get_protocol(&config) == HTTPS_LOWERCASE {
            let roots: RootCertStore = {
                match self.tmp.clone().read() {
                    Ok(tmp) => tmp.root_cert.clone(),
                    Err(error) => {
                        return Err(RequestError::Request(error.to_string()));
                    }
                }
            };
            let tls_config: ClientConfig = ClientConfig::builder()
                .with_root_certificates(roots)
                .with_no_client_auth();
            let connector: TlsConnector = TlsConnector::from(Arc::new(tls_config));
            let dns_name: ServerName<'_> = ServerName::try_from(target_host.clone())
                .map_err(|error: InvalidDnsNameError| RequestError::Request(error.to_string()))?;
            let tunnel_stream: ProxyTunnelStream = ProxyTunnelStream::new(proxy_stream, Vec::new());
            let tls_stream: TlsStream<ProxyTunnelStream> = connector
                .connect(dns_name, tunnel_stream)
                .await
                .map_err(|error: std::io::Error| RequestError::Request(error.to_string()))?;
            return Ok(Box::new(tls_stream) as BoxAsyncReadWrite);
        }
        Ok(proxy_stream)
    }

    /// Sends the HTTP request asynchronously.
    ///
    /// # Returns
    ///
    /// - `RequestResult` - Result of the async request.
    pub(crate) async fn send_async(&mut self) -> RequestResult {
        let methods: Method = self.get_methods();
        let (host, port) = {
            if let Ok(mut config) = self.config.write() {
                config.url_obj = self
                    .parse_url()
                    .map_err(|error: RequestError| RequestError::Request(error.to_string()))?;
                let host: String = config.url_obj.host.clone().unwrap_or_default();
                let port = self.get_port(config.url_obj.port.unwrap_or_default(), &config);
                (host, port)
            } else {
                (String::new(), 0u16)
            }
        };
        let mut stream: BoxAsyncReadWrite = self.get_connection_stream_async(host, port).await?;
        let res: Result<BoxResponseTrait, RequestError> = match methods {
            m if m.is_get() => self.send_get_request_async(&mut stream).await,
            m if m.is_post() => self.send_post_request_async(&mut stream).await,
            _err => Err(RequestError::Request("Method Not Allowed".to_string())),
        };
        res
    }
}
