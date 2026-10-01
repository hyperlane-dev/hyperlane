use super::*;

impl<T: AsyncRead + AsyncWrite + Unpin + Send> AsyncReadWrite for T {}
impl<T: Read + Write> ReadWrite for T {}

impl HttpRequest {
    /// Send the HTTP request synchronously, returning the parsed response.
    ///
    /// # Returns
    ///
    /// - `RequestResult` - The parsed HTTP response, or a `RequestError` on failure.
    pub fn send(&mut self) -> RequestResult {
        self.send_sync()
    }

    /// Send the HTTP request asynchronously, returning the parsed response.
    ///
    /// # Returns
    ///
    /// - `RequestResult` - The parsed HTTP response, or a `RequestError` on failure.
    pub async fn send_async(&mut self) -> RequestResult {
        self.send_async_impl().await
    }

    /// Parse the configured URL into a [`HttpUrlComponents`].
    ///
    /// # Returns
    ///
    /// - `Result<HttpUrlComponents, RequestError>` - The parsed URL components, or a `RequestError` when the URL is malformed.
    pub(crate) fn parse_url(&self) -> Result<HttpUrlComponents, RequestError> {
        HttpUrlComponents::parse(self.get_url_ref())
            .map_err(|e: ::http_type::HttpUrlError| RequestError::Request(e.to_string()))
    }

    /// `Host` + path (including query string) for the request line.
    ///
    /// # Returns
    ///
    /// - `String` - The request path, with the query string appended when present.
    pub(crate) fn full_path(&self) -> String {
        let url_obj = self.parse_url().unwrap_or_default();
        let query = url_obj.query.unwrap_or_default();
        let path = url_obj.path.unwrap_or_default();
        if query.is_empty() {
            path
        } else {
            format!("{}{}{}", path, QUERY, query)
        }
    }

    /// Lower-case the protocol ("http" / "https").
    ///
    /// # Arguments
    ///
    /// - `&RequestConfig` - The configuration whose HTTP version is lower-cased.
    ///
    /// # Returns
    ///
    /// - `String` - The lower-cased protocol name.
    pub(crate) fn protocol_lower(config: &RequestConfig) -> String {
        config.http_version.to_string().to_ascii_lowercase()
    }

    /// Build the wire-format header bytes, with `Host`, `Content-Length`,
    /// `Accept`, `User-Agent` auto-filled if missing.
    ///
    /// # Arguments
    ///
    /// - `usize` - The length of the encoded request body.
    ///
    /// # Returns
    ///
    /// - `Vec<u8>` - The wire-format request header bytes.
    pub(crate) fn header_bytes(&self, body_length: usize) -> Vec<u8> {
        let mut header: HashMap<String, String> = self.get_headers();
        let host_value: String = self
            .parse_url()
            .ok()
            .and_then(|u: ::http_type::HttpUrlComponents| u.host.clone())
            .unwrap_or_default();
        let content_length_value: String = body_length.to_string();
        if !Self::header_has_key(&header, HOST) {
            header.insert(HOST.to_ascii_lowercase(), host_value);
        }
        if !Self::header_has_key(&header, CONTENT_LENGTH) {
            header.insert(CONTENT_LENGTH.to_ascii_lowercase(), content_length_value);
        }
        if !Self::header_has_key(&header, ACCEPT) {
            header.insert(ACCEPT.to_ascii_lowercase(), ACCEPT_ANY.to_owned());
        }
        if !Self::header_has_key(&header, USER_AGENT) {
            header.insert(USER_AGENT.to_ascii_lowercase(), APP_NAME.to_owned());
        }
        let estimated_size: usize = header
            .iter()
            .map(|(k, v): (&String, &String)| k.len() + v.len() + 4)
            .sum();
        let mut out: Vec<u8> = Vec::with_capacity(estimated_size);
        for (key, value) in &header {
            out.extend_from_slice(key.as_bytes());
            out.extend_from_slice(b": ");
            out.extend_from_slice(value.as_bytes());
            out.extend_from_slice(HTTP_BR_BYTES);
        }
        out
    }

    /// Check whether a header map already contains a key, comparing case-insensitively.
    ///
    /// # Arguments
    ///
    /// - `&HashMap<String, String>` - The header map to search.
    /// - `&str` - The header key to look for.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` when the map contains the key, `false` otherwise.
    fn header_has_key(header: &HashMap<String, String>, target_key: &str) -> bool {
        let target = target_key.to_ascii_lowercase();
        header.keys().any(|k: &String| k == &target)
    }

    /// Encode `self.body` according to the `Content-Type` header.
    /// Returns empty bytes if no recognised content type.
    ///
    /// # Returns
    ///
    /// - `Vec<u8>` - The encoded request body, empty when the content type is unrecognised.
    pub(crate) fn body_bytes(&self) -> Vec<u8> {
        let ct = self
            .headers
            .iter()
            .find(|(k, _): &(&String, &String)| k.eq_ignore_ascii_case(CONTENT_TYPE))
            .map(|(_, v): (&String, &String)| v.clone());
        match ct {
            Some(value) => value
                .to_lowercase()
                .parse::<ContentType>()
                .unwrap_or_default()
                .get_body_string(&String::from_utf8_lossy(self.get_body_ref().as_slice()))
                .into_bytes(),
            None => Vec::new(),
        }
    }

    /// Synchronous send.
    ///
    /// # Returns
    ///
    /// - `RequestResult` - The parsed HTTP response, or a `RequestError` on failure.
    pub(crate) fn send_sync(&mut self) -> RequestResult {
        let url_obj = self.parse_url()?;
        let host: String = url_obj.host.clone().unwrap_or_default();
        let port: u16 = self.resolve_port(url_obj.port.unwrap_or_default());
        let mut stream: BoxReadWrite = self.open_sync_stream(host, port)?;
        let method = self.get_method();
        if method.is_get() {
            self.send_get_request_sync(&mut stream)
        } else if method.is_post() {
            self.send_post_request_sync(&mut stream)
        } else {
            Err(RequestError::Request("Method Not Allowed".to_string()))
        }
    }

    /// Asynchronous send.
    ///
    /// # Returns
    ///
    /// - `RequestResult` - The parsed HTTP response, or a `RequestError` on failure.
    async fn send_async_impl(&mut self) -> RequestResult {
        let url_obj = self.parse_url()?;
        let host: String = url_obj.host.clone().unwrap_or_default();
        let port: u16 = self.resolve_port(url_obj.port.unwrap_or_default());
        let mut stream: BoxAsyncReadWrite = self.open_async_stream(host, port).await?;
        let method = self.get_method();
        if method.is_get() {
            self.send_get_request_async(&mut stream).await
        } else if method.is_post() {
            self.send_post_request_async(&mut stream).await
        } else {
            Err(RequestError::Request("Method Not Allowed".to_string()))
        }
    }

    /// Resolve the effective TCP port, falling back to the protocol default.
    ///
    /// # Arguments
    ///
    /// - `u16` - The port parsed from the URL, `0` when the URL omits one.
    ///
    /// # Returns
    ///
    /// - `u16` - The port from the URL, or the protocol default when it is `0`.
    fn resolve_port(&self, port: u16) -> u16 {
        if port != 0 {
            return port;
        }
        let protocol = Self::protocol_lower(self.get_config_ref());
        Protocol::get_port(&protocol)
    }

    /// Check whether the request uses the HTTPS protocol.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` when the configured protocol is `https`, `false` otherwise.
    fn is_https(&self) -> bool {
        Self::protocol_lower(self.get_config_ref()) == HTTPS_LOWERCASE
    }

    // ---------- sync stream plumbing ----------

    /// Open a synchronous stream to the target host, directly or through a proxy.
    ///
    /// # Arguments
    ///
    /// - `String` - The target host name or IP address.
    /// - `u16` - The target TCP port.
    ///
    /// # Returns
    ///
    /// - `Result<BoxReadWrite, RequestError>` - The connected stream, TLS-wrapped for HTTPS, or a `RequestError` on failure.
    fn open_sync_stream(&self, host: String, port: u16) -> Result<BoxReadWrite, RequestError> {
        if let Some(proxy) = &self.get_config_ref().proxy {
            return self.open_sync_proxy_stream(host, port, proxy);
        }
        let timeout = Duration::from_millis(self.get_config_ref().timeout);
        let tcp = TcpStream::connect((host.clone(), port))
            .map_err(|e: std::io::Error| RequestError::Request(e.to_string()))?;
        tcp.set_read_timeout(Some(timeout))
            .map_err(|e: std::io::Error| RequestError::Request(e.to_string()))?;
        tcp.set_write_timeout(Some(timeout))
            .map_err(|e: std::io::Error| RequestError::Request(e.to_string()))?;
        if self.is_https() {
            let roots: RootCertStore = self.get_tmp_ref().get_root_cert().clone();
            let tls_cfg = ClientConfig::builder()
                .with_root_certificates(roots)
                .with_no_client_auth();
            let dns = ServerName::try_from(host.clone())
                .map_err(|e: InvalidDnsNameError| RequestError::Request(e.to_string()))?;
            let session = ClientConnection::new(Arc::new(tls_cfg), dns)
                .map_err(|e: rustls::Error| RequestError::Request(e.to_string()))?;
            Ok(Box::new(StreamOwned::new(session, tcp)))
        } else {
            Ok(Box::new(tcp))
        }
    }

    /// Write a GET request to the stream and read the response.
    ///
    /// # Arguments
    ///
    /// - `&mut BoxReadWrite` - The connected stream used to write the request and read the response.
    ///
    /// # Returns
    ///
    /// - `Result<HttpResponse, RequestError>` - The parsed response, or a `RequestError` on I/O or parse failure.
    fn send_get_request_sync(
        &mut self,
        stream: &mut BoxReadWrite,
    ) -> Result<HttpResponse, RequestError> {
        let path = self.full_path();
        let header_bytes = self.header_bytes(0);
        let version = self.get_config_ref().http_version.to_string();
        let request = build_http_request("GET", path, header_bytes, None, version);
        stream
            .write_all(&request)
            .and_then(|_: ()| stream.flush())
            .map_err(|e: std::io::Error| RequestError::Request(e.to_string()))?;
        self.read_response_sync(stream)
    }

    /// Write a POST request with the encoded body to the stream and read the response.
    ///
    /// # Arguments
    ///
    /// - `&mut BoxReadWrite` - The connected stream used to write the request and read the response.
    ///
    /// # Returns
    ///
    /// - `Result<HttpResponse, RequestError>` - The parsed response, or a `RequestError` on I/O or parse failure.
    fn send_post_request_sync(
        &mut self,
        stream: &mut BoxReadWrite,
    ) -> Result<HttpResponse, RequestError> {
        let body_bytes = self.body_bytes();
        let path = self.full_path();
        let header_bytes = self.header_bytes(body_bytes.len());
        let version = self.get_config_ref().http_version.to_string();
        let request = build_http_request("POST", path, header_bytes, Some(body_bytes), version);
        stream
            .write_all(&request)
            .and_then(|_: ()| stream.flush())
            .map_err(|e: std::io::Error| RequestError::Request(e.to_string()))?;
        self.read_response_sync(stream)
    }

    /// Read a full response from the stream, handling content-length and chunked bodies.
    ///
    /// # Arguments
    ///
    /// - `&mut BoxReadWrite` - The connected stream to read the response from.
    ///
    /// # Returns
    ///
    /// - `Result<HttpResponse, RequestError>` - The parsed response, or a `RequestError` on I/O or parse failure.
    fn read_response_sync(
        &mut self,
        stream: &mut BoxReadWrite,
    ) -> Result<HttpResponse, RequestError> {
        let buffer_size = self.get_config_ref().buffer_size;
        let mut buffer = vec![0u8; buffer_size];
        let mut response_bytes: Vec<u8> = Vec::with_capacity(buffer_size.max(8192));
        let mut headers_done = false;
        let mut content_length = 0usize;
        let mut redirect_url: Option<Vec<u8>> = None;
        let mut headers_end_pos = 0usize;
        let mut is_chunked = false;
        let version_bytes = self
            .config
            .http_version
            .to_string()
            .to_ascii_lowercase()
            .into_bytes();
        let location_key = format!("{}:", LOCATION.to_ascii_lowercase()).into_bytes();
        loop {
            let n = stream
                .read(&mut buffer)
                .map_err(|e: std::io::Error| RequestError::Request(e.to_string()))?;
            if n == 0 {
                break;
            }
            let new_cap = calculate_buffer_capacity(&response_bytes, n, response_bytes.capacity());
            if new_cap > 0 {
                response_bytes.reserve(new_cap - response_bytes.capacity());
            }
            let old_len = response_bytes.len();
            response_bytes.extend_from_slice(&buffer[..n]);
            if !headers_done {
                let search_start = old_len.saturating_sub(3);
                if let Some(pos) = find_double_crlf(&response_bytes, search_start) {
                    headers_done = true;
                    headers_end_pos = pos + 4;
                    parse_response_headers(
                        &response_bytes[..headers_end_pos],
                        &version_bytes,
                        &location_key,
                        &mut content_length,
                        &mut redirect_url,
                        &mut is_chunked,
                    )?;
                }
            }
            if headers_done {
                if is_chunked {
                    if Self::is_chunked_response_complete(&response_bytes[headers_end_pos..]) {
                        break;
                    }
                } else if response_bytes.len() >= headers_end_pos + content_length {
                    response_bytes.truncate(headers_end_pos + content_length);
                    break;
                }
            }
        }
        if is_chunked {
            let body_bytes = response_bytes[headers_end_pos..].to_vec();
            let decoded = parse_chunked_body(&body_bytes);
            response_bytes.truncate(headers_end_pos);
            response_bytes.extend_from_slice(&decoded);
        }
        let mut response = HttpResponse::from_bytes(&response_bytes);
        if !self.get_config_ref().redirect || redirect_url.is_none() {
            if self.get_config_ref().decode {
                response = response.decode(self.get_config_ref().buffer_size);
            }
            return Ok(response);
        }
        let url_bytes: Vec<u8> = redirect_url
            .take()
            .ok_or_else(|| RequestError::Request("Missing Redirect URL".to_string()))?;
        let url = String::from_utf8(url_bytes)
            .map_err(|e: FromUtf8Error| RequestError::Request(e.to_string()))?;
        self.handle_redirect(url)
    }

    /// Follow a redirect by recording the visit and re-issuing the request.
    ///
    /// # Arguments
    ///
    /// - `String` - The absolute redirect target URL taken from the `Location` header.
    ///
    /// # Returns
    ///
    /// - `Result<HttpResponse, RequestError>` - The response of the redirect target, or a `RequestError` when redirects are disabled, looping, or exhausted.
    fn handle_redirect(&mut self, url: String) -> Result<HttpResponse, RequestError> {
        if !self.get_config_ref().redirect {
            return Err(RequestError::Request("Redirect Not Enabled".to_string()));
        }
        if self.get_tmp_ref().get_visit_url().contains(&url) {
            return Err(RequestError::Request("Redirect URL Dead Loop".to_string()));
        }
        self.get_tmp_mut().get_mut_visit_url().insert(url.clone());
        if self.get_config_ref().max_redirect_times == 0 {
            return Err(RequestError::Request(
                "Max Redirect Times Exceeded".to_string(),
            ));
        }
        self.get_config_mut().max_redirect_times -= 1;
        self.set_url(url);
        self.send_sync()
    }

    /// Check whether a chunked response body has received its terminating zero-length chunk.
    ///
    /// # Arguments
    ///
    /// - `&[u8]` - The raw chunked body bytes received so far.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` when the terminating chunk is present, `false` while more bytes are needed.
    fn is_chunked_response_complete(body_bytes: &[u8]) -> bool {
        let mut pos = 0;
        while pos < body_bytes.len() {
            let chunk_size_end = match body_bytes[pos..]
                .windows(2)
                .position(|w: &[u8]| w == b"\r\n")
            {
                Some(p) => pos + p,
                None => return false,
            };
            let raw = &body_bytes[pos..chunk_size_end];
            let chunk_size_str: &[u8] = match raw.iter().position(|&b: &u8| b == b';') {
                Some(p) => &raw[..p],
                None => raw,
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
            let start = chunk_size_end + 2;
            let end = start + chunk_size;
            if end + 2 > body_bytes.len() {
                return false;
            }
            pos = end + 2;
        }
        false
    }

    // ---------- async stream plumbing ----------

    /// Open an asynchronous stream to the target host, directly or through a proxy.
    ///
    /// # Arguments
    ///
    /// - `String` - The target host name or IP address.
    /// - `u16` - The target TCP port.
    ///
    /// # Returns
    ///
    /// - `Result<BoxAsyncReadWrite, RequestError>` - The connected stream, TLS-wrapped for HTTPS, or a `RequestError` on failure.
    async fn open_async_stream(
        &self,
        host: String,
        port: u16,
    ) -> Result<BoxAsyncReadWrite, RequestError> {
        if let Some(proxy) = &self.get_config_ref().proxy {
            return self.open_async_proxy_stream(host, port, proxy).await;
        }
        let tcp = http_type::tokio::net::TcpStream::connect((host.clone(), port))
            .await
            .map_err(|e: std::io::Error| RequestError::Request(e.to_string()))?;
        if self.is_https() {
            let roots: RootCertStore = self.get_tmp_ref().get_root_cert().clone();
            let tls_cfg = ClientConfig::builder()
                .with_root_certificates(roots)
                .with_no_client_auth();
            let connector = TlsConnector::from(Arc::new(tls_cfg));
            let dns = ServerName::try_from(host.clone())
                .map_err(|e: InvalidDnsNameError| RequestError::Request(e.to_string()))?;
            let tls = connector
                .connect(dns, tcp)
                .await
                .map_err(|e: std::io::Error| RequestError::Request(e.to_string()))?;
            Ok(Box::new(tls))
        } else {
            Ok(Box::new(tcp))
        }
    }

    /// Write a GET request to the stream and read the response.
    ///
    /// # Arguments
    ///
    /// - `&mut BoxAsyncReadWrite` - The connected stream used to write the request and read the response.
    ///
    /// # Returns
    ///
    /// - `Result<HttpResponse, RequestError>` - The parsed response, or a `RequestError` on I/O or parse failure.
    async fn send_get_request_async(
        &mut self,
        stream: &mut BoxAsyncReadWrite,
    ) -> Result<HttpResponse, RequestError> {
        let path = self.full_path();
        let header_bytes = self.header_bytes(0);
        let version = self.get_config_ref().http_version.to_string();
        let request = build_http_request("GET", path, header_bytes, None, version);
        stream
            .write_all(&request)
            .await
            .map_err(|e: std::io::Error| RequestError::Request(e.to_string()))?;
        stream
            .flush()
            .await
            .map_err(|e: std::io::Error| RequestError::Request(e.to_string()))?;
        self.read_response_async(stream).await
    }

    /// Write a POST request with the encoded body to the stream and read the response.
    ///
    /// # Arguments
    ///
    /// - `&mut BoxAsyncReadWrite` - The connected stream used to write the request and read the response.
    ///
    /// # Returns
    ///
    /// - `Result<HttpResponse, RequestError>` - The parsed response, or a `RequestError` on I/O or parse failure.
    async fn send_post_request_async(
        &mut self,
        stream: &mut BoxAsyncReadWrite,
    ) -> Result<HttpResponse, RequestError> {
        let body_bytes = self.body_bytes();
        let path = self.full_path();
        let header_bytes = self.header_bytes(body_bytes.len());
        let version = self.get_config_ref().http_version.to_string();
        let request = build_http_request("POST", path, header_bytes, Some(body_bytes), version);
        stream
            .write_all(&request)
            .await
            .map_err(|e: std::io::Error| RequestError::Request(e.to_string()))?;
        stream
            .flush()
            .await
            .map_err(|e: std::io::Error| RequestError::Request(e.to_string()))?;
        self.read_response_async(stream).await
    }

    /// Read a full response from the stream, handling content-length and chunked bodies.
    ///
    /// # Arguments
    ///
    /// - `&mut BoxAsyncReadWrite` - The connected stream to read the response from.
    ///
    /// # Returns
    ///
    /// - `Result<HttpResponse, RequestError>` - The parsed response, or a `RequestError` on I/O or parse failure.
    async fn read_response_async(
        &mut self,
        stream: &mut BoxAsyncReadWrite,
    ) -> Result<HttpResponse, RequestError> {
        let buffer_size = self.get_config_ref().buffer_size;
        let mut buffer = vec![0u8; buffer_size];
        let mut response_bytes: Vec<u8> = Vec::with_capacity(buffer_size.max(8192));
        let mut headers_done = false;
        let mut content_length = 0usize;
        let mut redirect_url: Option<Vec<u8>> = None;
        let mut headers_end_pos = 0usize;
        let mut is_chunked = false;
        let version_bytes = self
            .config
            .http_version
            .to_string()
            .to_ascii_lowercase()
            .into_bytes();
        let location_key = format!("{}:", LOCATION.to_ascii_lowercase()).into_bytes();
        loop {
            let n = stream
                .read(&mut buffer)
                .await
                .map_err(|e: std::io::Error| RequestError::Request(e.to_string()))?;
            if n == 0 {
                break;
            }
            let new_cap = calculate_buffer_capacity(&response_bytes, n, response_bytes.capacity());
            if new_cap > 0 {
                response_bytes.reserve(new_cap - response_bytes.capacity());
            }
            let old_len = response_bytes.len();
            response_bytes.extend_from_slice(&buffer[..n]);
            if !headers_done {
                let search_start = old_len.saturating_sub(3);
                if let Some(pos) = find_double_crlf(&response_bytes, search_start) {
                    headers_done = true;
                    headers_end_pos = pos + 4;
                    parse_response_headers(
                        &response_bytes[..headers_end_pos],
                        &version_bytes,
                        &location_key,
                        &mut content_length,
                        &mut redirect_url,
                        &mut is_chunked,
                    )?;
                }
            }
            if headers_done {
                if is_chunked {
                    if Self::is_chunked_response_complete(&response_bytes[headers_end_pos..]) {
                        break;
                    }
                } else if response_bytes.len() >= headers_end_pos + content_length {
                    response_bytes.truncate(headers_end_pos + content_length);
                    break;
                }
            }
        }
        if is_chunked {
            let body_bytes = response_bytes[headers_end_pos..].to_vec();
            let decoded = parse_chunked_body(&body_bytes);
            response_bytes.truncate(headers_end_pos);
            response_bytes.extend_from_slice(&decoded);
        }
        let mut response = HttpResponse::from_bytes(&response_bytes);
        if !self.get_config_ref().redirect || redirect_url.is_none() {
            if self.get_config_ref().decode {
                response = response.decode(self.get_config_ref().buffer_size);
            }
            return Ok(response);
        }
        let url_bytes: Vec<u8> = redirect_url
            .take()
            .ok_or_else(|| RequestError::Request("Missing Redirect URL".to_string()))?;
        let url = String::from_utf8(url_bytes)
            .map_err(|e: FromUtf8Error| RequestError::Request(e.to_string()))?;
        self.handle_redirect_async(url).await
    }

    /// Follow a redirect by recording the visit and re-issuing the request.
    ///
    /// # Arguments
    ///
    /// - `String` - The absolute redirect target URL taken from the `Location` header.
    ///
    /// # Returns
    ///
    /// - `Result<HttpResponse, RequestError>` - The response of the redirect target, or a `RequestError` when redirects are disabled, looping, or exhausted.
    async fn handle_redirect_async(&mut self, url: String) -> Result<HttpResponse, RequestError> {
        if !self.get_config_ref().redirect {
            return Err(RequestError::Request("Redirect Not Enabled".to_string()));
        }
        if self.get_tmp_ref().get_visit_url().contains(&url) {
            return Err(RequestError::Request("Redirect URL Dead Loop".to_string()));
        }
        self.get_tmp_mut().get_mut_visit_url().insert(url.clone());
        if self.get_config_ref().max_redirect_times == 0 {
            return Err(RequestError::Request(
                "Max Redirect Times Exceeded".to_string(),
            ));
        }
        self.get_config_mut().max_redirect_times -= 1;
        self.set_url(url);
        Box::pin(self.send_async()).await
    }

    // ---------- proxy: sync ----------

    /// Open a synchronous stream to the target through the configured proxy.
    ///
    /// # Arguments
    ///
    /// - `String` - The target host name or IP address.
    /// - `u16` - The target TCP port.
    /// - `&Proxy` - The proxy configuration selecting the HTTP or SOCKS5 handshake.
    ///
    /// # Returns
    ///
    /// - `Result<BoxReadWrite, RequestError>` - The tunnelled stream, or a `RequestError` when the handshake fails.
    fn open_sync_proxy_stream(
        &self,
        target_host: String,
        target_port: u16,
        proxy: &Proxy,
    ) -> Result<BoxReadWrite, RequestError> {
        match proxy.proxy_type {
            ProxyType::Http | ProxyType::Https => {
                self.open_sync_http_proxy(target_host, target_port, proxy)
            }
            ProxyType::Socks5 => self.open_sync_socks5_proxy(target_host, target_port, proxy),
        }
    }

    /// Open a synchronous stream to the target through an HTTP or HTTPS proxy.
    ///
    /// # Arguments
    ///
    /// - `String` - The target host name or IP address.
    /// - `u16` - The target TCP port.
    /// - `&Proxy` - The proxy configuration selecting the HTTP or SOCKS5 handshake.
    ///
    /// # Returns
    ///
    /// - `Result<BoxReadWrite, RequestError>` - The tunnelled stream, or a `RequestError` when the handshake fails.
    fn open_sync_http_proxy(
        &self,
        target_host: String,
        target_port: u16,
        proxy: &Proxy,
    ) -> Result<BoxReadWrite, RequestError> {
        let timeout = Duration::from_millis(self.get_config_ref().timeout);
        let tcp = TcpStream::connect((proxy.host.clone(), proxy.port))
            .map_err(|e: std::io::Error| RequestError::Request(e.to_string()))?;
        tcp.set_read_timeout(Some(timeout))
            .map_err(|e: std::io::Error| RequestError::Request(e.to_string()))?;
        tcp.set_write_timeout(Some(timeout))
            .map_err(|e: std::io::Error| RequestError::Request(e.to_string()))?;
        let mut proxy_stream: BoxReadWrite = if proxy.proxy_type == ProxyType::Https {
            let roots: RootCertStore = self.get_tmp_ref().get_root_cert().clone();
            let tls_cfg = ClientConfig::builder()
                .with_root_certificates(roots)
                .with_no_client_auth();
            let dns = ServerName::try_from(proxy.host.clone())
                .map_err(|e: InvalidDnsNameError| RequestError::Request(e.to_string()))?;
            let session = ClientConnection::new(Arc::new(tls_cfg), dns)
                .map_err(|e: rustls::Error| RequestError::Request(e.to_string()))?;
            Box::new(StreamOwned::new(session, tcp))
        } else {
            Box::new(tcp)
        };
        let connect_request: String = if let (Some(u), Some(p)) = (&proxy.username, &proxy.password)
        {
            let auth = format!("{u}:{p}");
            let encoded = crate::utils::base64_encode(auth.as_bytes());
            format!(
                "CONNECT {target_host}:{target_port} HTTP/1.1\r\nHost: {target_host}:{target_port}\r\nProxy-Authorization: Basic {encoded}\r\n\r\n"
            )
        } else {
            format!(
                "CONNECT {target_host}:{target_port} HTTP/1.1\r\nHost: {target_host}:{target_port}\r\n\r\n"
            )
        };
        proxy_stream
            .write_all(connect_request.as_bytes())
            .map_err(|e: std::io::Error| RequestError::Request(e.to_string()))?;
        proxy_stream
            .flush()
            .map_err(|e: std::io::Error| RequestError::Request(e.to_string()))?;
        let mut buf = [0u8; 1024];
        let n = proxy_stream
            .read(&mut buf)
            .map_err(|e: std::io::Error| RequestError::Request(e.to_string()))?;
        let s = std::str::from_utf8(&buf[..n]).unwrap_or("");
        let pre_read = if let Some(pos) = s.find("\r\n\r\n") {
            let header_part = &s[..pos];
            if !header_part.starts_with("HTTP/1.1 200") && !header_part.starts_with("HTTP/1.0 200")
            {
                return Err(RequestError::Request("Internal Server Error".to_string()));
            }
            buf[pos + 4..n].to_vec()
        } else {
            if !s.starts_with("HTTP/1.1 200") && !s.starts_with("HTTP/1.0 200") {
                return Err(RequestError::Request("Internal Server Error".to_string()));
            }
            Vec::new()
        };
        if self.is_https() {
            let roots: RootCertStore = self.get_tmp_ref().get_root_cert().clone();
            let tls_cfg = ClientConfig::builder()
                .with_root_certificates(roots)
                .with_no_client_auth();
            let dns = ServerName::try_from(target_host.clone())
                .map_err(|e: InvalidDnsNameError| RequestError::Request(e.to_string()))?;
            let session = ClientConnection::new(Arc::new(tls_cfg), dns)
                .map_err(|e: rustls::Error| RequestError::Request(e.to_string()))?;
            let tunnel = SyncProxyTunnelStream::new(proxy_stream, pre_read);
            return Ok(Box::new(StreamOwned::new(session, tunnel)));
        }
        Ok(Box::new(SyncProxyTunnelStream::new(proxy_stream, pre_read)))
    }

    /// Open a synchronous stream to the target through a SOCKS5 proxy.
    ///
    /// # Arguments
    ///
    /// - `String` - The target host name or IP address.
    /// - `u16` - The target TCP port.
    /// - `&Proxy` - The proxy configuration selecting the HTTP or SOCKS5 handshake.
    ///
    /// # Returns
    ///
    /// - `Result<BoxReadWrite, RequestError>` - The tunnelled stream, or a `RequestError` when the handshake fails.
    fn open_sync_socks5_proxy(
        &self,
        target_host: String,
        target_port: u16,
        proxy: &Proxy,
    ) -> Result<BoxReadWrite, RequestError> {
        let timeout = Duration::from_millis(self.get_config_ref().timeout);
        let mut tcp = TcpStream::connect((proxy.host.clone(), proxy.port))
            .map_err(|e: std::io::Error| RequestError::Request(e.to_string()))?;
        tcp.set_read_timeout(Some(timeout))
            .map_err(|e: std::io::Error| RequestError::Request(e.to_string()))?;
        tcp.set_write_timeout(Some(timeout))
            .map_err(|e: std::io::Error| RequestError::Request(e.to_string()))?;
        let auth_methods: Vec<u8> = if proxy.username.is_some() && proxy.password.is_some() {
            vec![0x05, 0x02, 0x00, 0x02]
        } else {
            vec![0x05, 0x01, 0x00]
        };
        tcp.write_all(&auth_methods)
            .map_err(|e: std::io::Error| RequestError::Request(e.to_string()))?;
        let mut resp = [0u8; 2];
        tcp.read_exact(&mut resp)
            .map_err(|e: std::io::Error| RequestError::Request(e.to_string()))?;
        if resp[0] != 0x05 {
            return Err(RequestError::Request("Internal Server Error".to_string()));
        }
        match resp[1] {
            0x00 => {}
            0x02 => {
                let (Some(u), Some(p)) = (&proxy.username, &proxy.password) else {
                    return Err(RequestError::Request("Internal Server Error".to_string()));
                };
                let mut auth_req = vec![0x01];
                auth_req.push(u.len() as u8);
                auth_req.extend_from_slice(u.as_bytes());
                auth_req.push(p.len() as u8);
                auth_req.extend_from_slice(p.as_bytes());
                tcp.write_all(&auth_req)
                    .map_err(|e: std::io::Error| RequestError::Request(e.to_string()))?;
                let mut auth_resp = [0u8; 2];
                tcp.read_exact(&mut auth_resp)
                    .map_err(|e: std::io::Error| RequestError::Request(e.to_string()))?;
                if auth_resp[1] != 0x00 {
                    return Err(RequestError::Request("Internal Server Error".to_string()));
                }
            }
            _ => return Err(RequestError::Request("Internal Server Error".to_string())),
        }
        let mut connect_req: Vec<u8> = vec![0x05, 0x01, 0x00];
        if let Ok(ip) = target_host.parse::<Ipv4Addr>() {
            connect_req.push(0x01);
            connect_req.extend_from_slice(&ip.octets());
        } else if let Ok(ip) = target_host.parse::<Ipv6Addr>() {
            connect_req.push(0x04);
            connect_req.extend_from_slice(&ip.octets());
        } else {
            connect_req.push(0x03);
            connect_req.push(target_host.len() as u8);
            connect_req.extend_from_slice(target_host.as_bytes());
        }
        connect_req.extend_from_slice(&target_port.to_be_bytes());
        tcp.write_all(&connect_req)
            .map_err(|e: std::io::Error| RequestError::Request(e.to_string()))?;
        let mut connect_resp = [0u8; 4];
        tcp.read_exact(&mut connect_resp)
            .map_err(|e: std::io::Error| RequestError::Request(e.to_string()))?;
        if connect_resp[0] != 0x05 || connect_resp[1] != 0x00 {
            return Err(RequestError::Request("Internal Server Error".to_string()));
        }
        match connect_resp[3] {
            0x01 => {
                let mut skip = [0u8; 6];
                tcp.read_exact(&mut skip)
                    .map_err(|e: std::io::Error| RequestError::Request(e.to_string()))?;
            }
            0x03 => {
                let mut len = [0u8; 1];
                tcp.read_exact(&mut len)
                    .map_err(|e: std::io::Error| RequestError::Request(e.to_string()))?;
                let mut skip = vec![0u8; len[0] as usize + 2];
                tcp.read_exact(&mut skip)
                    .map_err(|e: std::io::Error| RequestError::Request(e.to_string()))?;
            }
            0x04 => {
                let mut skip = [0u8; 18];
                tcp.read_exact(&mut skip)
                    .map_err(|e: std::io::Error| RequestError::Request(e.to_string()))?;
            }
            _ => return Err(RequestError::Request("Internal Server Error".to_string())),
        }
        if self.is_https() {
            let roots: RootCertStore = self.get_tmp_ref().get_root_cert().clone();
            let tls_cfg = ClientConfig::builder()
                .with_root_certificates(roots)
                .with_no_client_auth();
            let dns = ServerName::try_from(target_host)
                .map_err(|e: InvalidDnsNameError| RequestError::Request(e.to_string()))?;
            let session = ClientConnection::new(Arc::new(tls_cfg), dns)
                .map_err(|e: rustls::Error| RequestError::Request(e.to_string()))?;
            let proxy_box: BoxReadWrite = Box::new(tcp);
            let tunnel = SyncProxyTunnelStream::new(proxy_box, Vec::new());
            return Ok(Box::new(StreamOwned::new(session, tunnel)));
        }
        Ok(Box::new(tcp))
    }

    // ---------- proxy: async ----------

    /// Open an asynchronous stream to the target through the configured proxy.
    ///
    /// # Arguments
    ///
    /// - `String` - The target host name or IP address.
    /// - `u16` - The target TCP port.
    /// - `&Proxy` - The proxy configuration selecting the HTTP or SOCKS5 handshake.
    ///
    /// # Returns
    ///
    /// - `Result<BoxAsyncReadWrite, RequestError>` - The tunnelled stream, or a `RequestError` when the handshake fails.
    async fn open_async_proxy_stream(
        &self,
        target_host: String,
        target_port: u16,
        proxy: &Proxy,
    ) -> Result<BoxAsyncReadWrite, RequestError> {
        match proxy.proxy_type {
            ProxyType::Http | ProxyType::Https => {
                self.open_async_http_proxy(target_host, target_port, proxy)
                    .await
            }
            ProxyType::Socks5 => {
                self.open_async_socks5_proxy(target_host, target_port, proxy)
                    .await
            }
        }
    }

    /// Open an asynchronous stream to the target through an HTTP or HTTPS proxy.
    ///
    /// # Arguments
    ///
    /// - `String` - The target host name or IP address.
    /// - `u16` - The target TCP port.
    /// - `&Proxy` - The proxy configuration selecting the HTTP or SOCKS5 handshake.
    ///
    /// # Returns
    ///
    /// - `Result<BoxAsyncReadWrite, RequestError>` - The tunnelled stream, or a `RequestError` when the handshake fails.
    async fn open_async_http_proxy(
        &self,
        target_host: String,
        target_port: u16,
        proxy: &Proxy,
    ) -> Result<BoxAsyncReadWrite, RequestError> {
        let tcp = http_type::tokio::net::TcpStream::connect((proxy.host.clone(), proxy.port))
            .await
            .map_err(|e: std::io::Error| RequestError::Request(e.to_string()))?;
        let mut proxy_stream: BoxAsyncReadWrite = if proxy.proxy_type == ProxyType::Https {
            let roots: RootCertStore = self.get_tmp_ref().get_root_cert().clone();
            let tls_cfg = ClientConfig::builder()
                .with_root_certificates(roots)
                .with_no_client_auth();
            let connector = TlsConnector::from(Arc::new(tls_cfg));
            let dns = ServerName::try_from(proxy.host.clone())
                .map_err(|e: InvalidDnsNameError| RequestError::Request(e.to_string()))?;
            let tls = connector
                .connect(dns, tcp)
                .await
                .map_err(|e: std::io::Error| RequestError::Request(e.to_string()))?;
            Box::new(tls)
        } else {
            Box::new(tcp)
        };
        let connect_request: String = if let (Some(u), Some(p)) = (&proxy.username, &proxy.password)
        {
            let auth = format!("{u}:{p}");
            let encoded = crate::utils::base64_encode(auth.as_bytes());
            format!(
                "CONNECT {target_host}:{target_port} HTTP/1.1\r\nHost: {target_host}:{target_port}\r\nProxy-Authorization: Basic {encoded}\r\n\r\n"
            )
        } else {
            format!(
                "CONNECT {target_host}:{target_port} HTTP/1.1\r\nHost: {target_host}:{target_port}\r\n\r\n"
            )
        };
        proxy_stream
            .write_all(connect_request.as_bytes())
            .await
            .map_err(|e: std::io::Error| RequestError::Request(e.to_string()))?;
        proxy_stream
            .flush()
            .await
            .map_err(|e: std::io::Error| RequestError::Request(e.to_string()))?;
        let mut buf = [0u8; 1024];
        let n = proxy_stream
            .read(&mut buf)
            .await
            .map_err(|e: std::io::Error| RequestError::Request(e.to_string()))?;
        let s = std::str::from_utf8(&buf[..n]).unwrap_or("");
        let pre_read = if let Some(pos) = s.find("\r\n\r\n") {
            let header_part = &s[..pos];
            if !header_part.starts_with("HTTP/1.1 200") && !header_part.starts_with("HTTP/1.0 200")
            {
                return Err(RequestError::Request("Internal Server Error".to_string()));
            }
            buf[pos + 4..n].to_vec()
        } else {
            if !s.starts_with("HTTP/1.1 200") && !s.starts_with("HTTP/1.0 200") {
                return Err(RequestError::Request("Internal Server Error".to_string()));
            }
            Vec::new()
        };
        if self.is_https() {
            let roots: RootCertStore = self.get_tmp_ref().get_root_cert().clone();
            let tls_cfg = ClientConfig::builder()
                .with_root_certificates(roots)
                .with_no_client_auth();
            let connector = TlsConnector::from(Arc::new(tls_cfg));
            let dns = ServerName::try_from(target_host.clone())
                .map_err(|e: InvalidDnsNameError| RequestError::Request(e.to_string()))?;
            let tunnel = ProxyTunnelStream::new(proxy_stream, pre_read);
            let tls = connector
                .connect(dns, tunnel)
                .await
                .map_err(|e: std::io::Error| RequestError::Request(e.to_string()))?;
            Ok(Box::new(tls))
        } else {
            Ok(Box::new(ProxyTunnelStream::new(proxy_stream, pre_read)))
        }
    }

    /// Open an asynchronous stream to the target through a SOCKS5 proxy.
    ///
    /// # Arguments
    ///
    /// - `String` - The target host name or IP address.
    /// - `u16` - The target TCP port.
    /// - `&Proxy` - The proxy configuration selecting the HTTP or SOCKS5 handshake.
    ///
    /// # Returns
    ///
    /// - `Result<BoxAsyncReadWrite, RequestError>` - The tunnelled stream, or a `RequestError` when the handshake fails.
    async fn open_async_socks5_proxy(
        &self,
        target_host: String,
        target_port: u16,
        proxy: &Proxy,
    ) -> Result<BoxAsyncReadWrite, RequestError> {
        let mut tcp = http_type::tokio::net::TcpStream::connect((proxy.host.clone(), proxy.port))
            .await
            .map_err(|e: std::io::Error| RequestError::Request(e.to_string()))?;
        let auth_methods: Vec<u8> = if proxy.username.is_some() && proxy.password.is_some() {
            vec![0x05, 0x02, 0x00, 0x02]
        } else {
            vec![0x05, 0x01, 0x00]
        };
        tcp.write_all(&auth_methods)
            .await
            .map_err(|e: std::io::Error| RequestError::Request(e.to_string()))?;
        let mut resp = [0u8; 2];
        tcp.read_exact(&mut resp)
            .await
            .map_err(|e: std::io::Error| RequestError::Request(e.to_string()))?;
        if resp[0] != 0x05 {
            return Err(RequestError::Request("Internal Server Error".to_string()));
        }
        match resp[1] {
            0x00 => {}
            0x02 => {
                let (Some(u), Some(p)) = (&proxy.username, &proxy.password) else {
                    return Err(RequestError::Request("Internal Server Error".to_string()));
                };
                let mut auth_req = vec![0x01u8];
                auth_req.push(u.len() as u8);
                auth_req.extend_from_slice(u.as_bytes());
                auth_req.push(p.len() as u8);
                auth_req.extend_from_slice(p.as_bytes());
                tcp.write_all(&auth_req)
                    .await
                    .map_err(|e: std::io::Error| RequestError::Request(e.to_string()))?;
                let mut auth_resp = [0u8; 2];
                tcp.read_exact(&mut auth_resp)
                    .await
                    .map_err(|e: std::io::Error| RequestError::Request(e.to_string()))?;
                if auth_resp[1] != 0x00 {
                    return Err(RequestError::Request("Internal Server Error".to_string()));
                }
            }
            _ => return Err(RequestError::Request("Internal Server Error".to_string())),
        }
        let mut connect_req: Vec<u8> = vec![0x05, 0x01, 0x00];
        if let Ok(ip) = target_host.parse::<Ipv4Addr>() {
            connect_req.push(0x01);
            connect_req.extend_from_slice(&ip.octets());
        } else if let Ok(ip) = target_host.parse::<Ipv6Addr>() {
            connect_req.push(0x04);
            connect_req.extend_from_slice(&ip.octets());
        } else {
            connect_req.push(0x03);
            connect_req.push(target_host.len() as u8);
            connect_req.extend_from_slice(target_host.as_bytes());
        }
        connect_req.extend_from_slice(&target_port.to_be_bytes());
        tcp.write_all(&connect_req)
            .await
            .map_err(|e: std::io::Error| RequestError::Request(e.to_string()))?;
        let mut connect_resp = [0u8; 4];
        tcp.read_exact(&mut connect_resp)
            .await
            .map_err(|e: std::io::Error| RequestError::Request(e.to_string()))?;
        if connect_resp[0] != 0x05 || connect_resp[1] != 0x00 {
            return Err(RequestError::Request("Internal Server Error".to_string()));
        }
        match connect_resp[3] {
            0x01 => {
                let mut skip = [0u8; 6];
                tcp.read_exact(&mut skip)
                    .await
                    .map_err(|e: std::io::Error| RequestError::Request(e.to_string()))?;
            }
            0x03 => {
                let mut len = [0u8; 1];
                tcp.read_exact(&mut len)
                    .await
                    .map_err(|e: std::io::Error| RequestError::Request(e.to_string()))?;
                let mut skip = vec![0u8; len[0] as usize + 2];
                tcp.read_exact(&mut skip)
                    .await
                    .map_err(|e: std::io::Error| RequestError::Request(e.to_string()))?;
            }
            0x04 => {
                let mut skip = [0u8; 18];
                tcp.read_exact(&mut skip)
                    .await
                    .map_err(|e: std::io::Error| RequestError::Request(e.to_string()))?;
            }
            _ => return Err(RequestError::Request("Internal Server Error".to_string())),
        }
        if self.is_https() {
            let roots: RootCertStore = self.get_tmp_ref().get_root_cert().clone();
            let tls_cfg = ClientConfig::builder()
                .with_root_certificates(roots)
                .with_no_client_auth();
            let connector = TlsConnector::from(Arc::new(tls_cfg));
            let dns = ServerName::try_from(target_host)
                .map_err(|e: InvalidDnsNameError| RequestError::Request(e.to_string()))?;
            let proxy_box: BoxAsyncReadWrite = Box::new(tcp);
            let tunnel = ProxyTunnelStream::new(proxy_box, Vec::new());
            let tls = connector
                .connect(dns, tunnel)
                .await
                .map_err(|e: std::io::Error| RequestError::Request(e.to_string()))?;
            Ok(Box::new(tls))
        } else {
            Ok(Box::new(tcp))
        }
    }
}

impl HttpRequest {
    /// Create a GET request for `url`.
    ///
    /// # Arguments
    ///
    /// - `T` - The request URL, converted into an owned `String`.
    pub fn get<T: Into<String>>(url: T) -> Self {
        Self {
            method: Method::Get,
            url: url.into(),
            headers: HashMap::new(),
            body: Body::default(),
            config: RequestConfig::default(),
            tmp: Tmp::default(),
        }
    }

    /// Create a POST request for `url`.
    ///
    /// # Arguments
    ///
    /// - `T` - The request URL, converted into an owned `String`.
    pub fn post<T: Into<String>>(url: T) -> Self {
        Self {
            method: Method::Post,
            url: url.into(),
            headers: HashMap::new(),
            body: Body::default(),
            config: RequestConfig::default(),
            tmp: Tmp::default(),
        }
    }

    /// Builder-style: set method.
    ///
    /// # Arguments
    ///
    /// - `Method` - The HTTP method to use for the request.
    ///
    /// # Returns
    ///
    /// - `&mut Self` - The request itself, for chaining.
    pub fn set_method(&mut self, method: Method) -> &mut Self {
        self.method = method;
        self
    }

    /// Builder-style: set URL.
    ///
    /// # Arguments
    ///
    /// - `T` - The request URL, converted into an owned `String`.
    ///
    /// # Returns
    ///
    /// - `&mut Self` - The request itself, for chaining.
    pub fn set_url<T: Into<String>>(&mut self, url: T) -> &mut Self {
        self.url = url.into();
        self
    }

    /// Builder-style: set a single header (case-insensitive on read; the
    /// last value wins for repeated keys).
    ///
    /// # Arguments
    ///
    /// - `K` - The header name, normalised to lowercase.
    /// - `V` - The header value, stored as an owned `String`.
    ///
    /// # Returns
    ///
    /// - `&mut Self` - The request itself, for chaining.
    pub fn set_header<K: AsRef<str>, V: AsRef<str>>(&mut self, key: K, value: V) -> &mut Self {
        let normalized = Self::normalize_header_key(key.as_ref());
        self.headers.insert(normalized, value.as_ref().to_owned());
        self
    }

    /// Remove a header by key.
    ///
    /// # Arguments
    ///
    /// - `K` - The header name to remove, matched case-insensitively.
    ///
    /// # Returns
    ///
    /// - `&mut Self` - The request itself, for chaining.
    pub fn remove_header<K: AsRef<str>>(&mut self, key: K) -> &mut Self {
        let normalized = Self::normalize_header_key(key.as_ref());
        self.get_mut_headers().remove(&normalized);
        self
    }

    /// Clear all headers.
    ///
    /// # Returns
    ///
    /// - `&mut Self` - The request itself, for chaining.
    pub fn clear_headers(&mut self) -> &mut Self {
        self.get_mut_headers().clear();
        self
    }

    /// Set body (raw bytes).
    ///
    /// # Arguments
    ///
    /// - `Body` - The body payload to send.
    ///
    /// # Returns
    ///
    /// - `&mut Self` - The request itself, for chaining.
    pub fn set_body(&mut self, body: Body) -> &mut Self {
        self.body = body;
        self
    }

    /// Set a single config field by mutating the embedded `RequestConfig`.
    ///
    /// # Arguments
    ///
    /// - `RequestConfig` - The configuration replacing the current one.
    ///
    /// # Returns
    ///
    /// - `&mut Self` - The request itself, for chaining.
    pub fn set_config(&mut self, config: RequestConfig) -> &mut Self {
        self.config = config;
        self
    }

    /// Get a copy of the HTTP method.
    ///
    /// # Returns
    ///
    /// - `Method` - A clone of the configured HTTP method.
    pub fn get_method(&self) -> Method {
        self.method.clone()
    }

    /// Get a clone of the URL string.
    ///
    /// # Returns
    ///
    /// - `String` - A clone of the configured URL.
    pub fn get_url(&self) -> String {
        self.url.clone()
    }

    /// Get a reference to the URL string.
    ///
    /// # Returns
    ///
    /// - `&str` - A borrowed slice of the configured URL.
    pub fn get_url_ref(&self) -> &str {
        self.url.as_str()
    }

    /// Get a clone of the request headers map.
    ///
    /// # Returns
    ///
    /// - `HashMap<String, String>` - A clone of the configured header map.
    pub fn get_headers(&self) -> HashMap<String, String> {
        self.headers.clone()
    }

    /// Get a reference to the request headers map.
    ///
    /// # Returns
    ///
    /// - `&HashMap<String, String>` - A borrowed reference to the configured header map.
    pub fn get_headers_ref(&self) -> &HashMap<String, String> {
        &self.headers
    }

    /// Returns a mutable reference to the headers map.
    ///
    /// # Returns
    ///
    /// - `&mut HashMap<String, String>` - The mutable headers map.
    pub fn get_headers_mut(&mut self) -> &mut HashMap<String, String> {
        &mut self.headers
    }

    /// Get a clone of the body.
    ///
    /// # Returns
    ///
    /// - `Body` - A clone of the configured body.
    pub fn get_body(&self) -> Body {
        self.body.clone()
    }

    /// Get a reference to the body.
    ///
    /// # Returns
    ///
    /// - `&Body` - A borrowed reference to the configured body.
    pub fn get_body_ref(&self) -> &Body {
        &self.body
    }

    /// Get a clone of the request config.
    ///
    /// # Returns
    ///
    /// - `RequestConfig` - A clone of the configured request options.
    pub fn get_config(&self) -> RequestConfig {
        self.config.clone()
    }

    /// Get a reference to the request config.
    ///
    /// # Returns
    ///
    /// - `&RequestConfig` - A borrowed reference to the configured request options.
    pub fn get_config_ref(&self) -> &RequestConfig {
        &self.config
    }

    /// Get a mutable reference to the request config.
    ///
    /// # Returns
    ///
    /// - `&mut RequestConfig` - A mutable reference to the configured request options.
    pub fn get_config_mut(&mut self) -> &mut RequestConfig {
        &mut self.config
    }

    /// Get a reference to the internal scratch `Tmp`.
    ///
    /// # Returns
    ///
    /// - `&Tmp` - A borrowed reference to the internal redirect-tracking state.
    pub(crate) fn get_tmp_ref(&self) -> &Tmp {
        &self.tmp
    }

    /// Get a mutable reference to the internal scratch `Tmp`.
    ///
    /// # Returns
    ///
    /// - `&mut Tmp` - A mutable reference to the internal redirect-tracking state.
    pub(crate) fn get_tmp_mut(&mut self) -> &mut Tmp {
        &mut self.tmp
    }

    /// Normalize a header key to lowercase so `set_header` /
    /// `remove_header` lookups are case-insensitive.
    ///
    /// # Arguments
    ///
    /// - `&str` - The header key to normalise.
    ///
    /// # Returns
    ///
    /// - `String` - The lower-cased header key.
    fn normalize_header_key(key: &str) -> String {
        key.to_ascii_lowercase()
    }
}
