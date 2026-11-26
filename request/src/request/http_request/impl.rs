use super::*;

impl<T> AsyncReadWrite for T where T: AsyncRead + AsyncWrite + Unpin + Send {}
impl<T> ReadWrite for T where T: Read + Write + Send {}

/// Inherent implementation of [`HttpRequest`].
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
        let (scheme, remainder): (&str, &str) = self
            .get_url()
            .split_once(SCHEME_SEPARATOR)
            .ok_or(RequestError::InvalidUrlScheme(HttpStatus::BadRequest))?;
        if !scheme.eq_ignore_ascii_case(HTTP_SCHEME)
            && !scheme.eq_ignore_ascii_case(HTTPS_LOWERCASE)
        {
            return Err(RequestError::InvalidUrlScheme(HttpStatus::BadRequest));
        }
        let authority_end: usize = remainder
            .find([URL_SLASH, URL_QUERY, URL_FRAGMENT])
            .unwrap_or(remainder.len());
        let authority: &str = &remainder[..authority_end];
        if authority.is_empty()
            || authority.contains(URL_BACKSLASH)
            || authority
                .bytes()
                .any(|byte: u8| byte.is_ascii_control() || byte == SPACE_U8)
        {
            return Err(RequestError::InvalidUrlHost(HttpStatus::BadRequest));
        }
        let target: &str = remainder[authority_end..]
            .split(URL_FRAGMENT)
            .next()
            .unwrap_or_default();
        let (path, query): (&str, &str) = target
            .split_once(URL_QUERY)
            .unwrap_or((target, Default::default()));
        if path
            .bytes()
            .any(|byte: u8| byte.is_ascii_control() || byte == SPACE_U8)
        {
            return Err(RequestError::InvalidUrlPath(HttpStatus::BadRequest));
        }
        if query
            .bytes()
            .any(|byte: u8| byte.is_ascii_control() || byte == SPACE_U8)
        {
            return Err(RequestError::InvalidUrlQuery(HttpStatus::BadRequest));
        }
        let components: HttpUrlComponents = match HttpUrlComponents::parse(self.get_url()) {
            Ok(value) => value,
            Err(_) => return Err(RequestError::InvalidUrlHost(HttpStatus::BadRequest)),
        };
        let host: String = components.get_host().unwrap_or_default();
        if host.is_empty()
            || host
                .bytes()
                .any(|byte: u8| byte.is_ascii_control() || byte == SPACE_U8)
        {
            return Err(RequestError::InvalidUrlHost(HttpStatus::BadRequest));
        }
        Ok(components)
    }

    /// `Host` + path (including query string) for the request line.
    ///
    /// # Returns
    ///
    /// - `Result<String, RequestError>` - The safe request target, or an error for an invalid URL.
    #[cfg(test)]
    pub(crate) fn full_path(&self) -> Result<String, RequestError> {
        let url_obj: HttpUrlComponents = self.parse_url()?;
        let query: String = url_obj.get_query().unwrap_or_default();
        let path: String = url_obj.get_path().unwrap_or_default();
        if query.is_empty() {
            Ok(path)
        } else {
            Ok(format!("{}{}{}", path, QUERY, query))
        }
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
    /// - `Result<Vec<u8>, RequestError>` - The validated wire-format headers, or an error for invalid fields.
    #[cfg(test)]
    pub(crate) fn header_bytes(&self, body_length: usize) -> Result<Vec<u8>, RequestError> {
        let mut header: HashMap<String, String> = self.get_headers().clone();
        let host_value: String = self.parse_url()?.get_host().unwrap_or_default();
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
        validate_request_headers(&header)?;
        let estimated_size: usize = header
            .iter()
            .map(|(k, v): (&String, &String)| k.len() + v.len() + 4)
            .sum();
        let mut out: Vec<u8> = Vec::with_capacity(estimated_size);
        for (key, value) in &header {
            out.extend_from_slice(key.as_bytes());
            out.extend_from_slice(HEADER_VALUE_SEPARATOR);
            out.extend_from_slice(value.as_bytes());
            out.extend_from_slice(HTTP_BR_BYTES);
        }
        Ok(out)
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
    #[cfg(test)]
    fn header_has_key(header: &HashMap<String, String>, target_key: &str) -> bool {
        header
            .keys()
            .any(|key: &String| key.eq_ignore_ascii_case(target_key))
    }

    /// Remove every header entry matching `target_key` case-insensitively.
    ///
    /// # Arguments
    ///
    /// - `&str` - The header name to remove.
    fn remove_header_ci(&mut self, target_key: &str) {
        self.get_mut_headers()
            .retain(|key: &String, _: &mut String| !key.eq_ignore_ascii_case(target_key));
    }

    /// Return whether a parsed URL targets HTTPS.
    ///
    /// # Arguments
    ///
    /// - `&HttpUrlComponents` - Parsed URL components.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` when the URL scheme is `https` (case-insensitive).
    fn scheme_is_https(url_obj: &HttpUrlComponents) -> bool {
        url_obj.get_protocol().eq_ignore_ascii_case(HTTPS_LOWERCASE)
    }

    /// Resolve the effective TCP port from the parsed port and URL scheme.
    ///
    /// # Arguments
    ///
    /// - `u16` - Parsed URL port (`0` when omitted).
    /// - `bool` - Whether the URL uses HTTPS.
    ///
    /// # Returns
    ///
    /// - `u16` - Effective TCP port.
    fn resolve_port_for_scheme(port: u16, is_https: bool) -> u16 {
        if port != 0 {
            return port;
        }
        if is_https {
            HTTPS_DEFAULT_PORT
        } else {
            HTTP_DEFAULT_PORT
        }
    }

    /// Strip optional square brackets surrounding an IPv6 literal host.
    ///
    /// # Arguments
    ///
    /// - `&str` - Raw host string from the URL.
    ///
    /// # Returns
    ///
    /// - `&str` - Bare IPv6 address or original hostname.
    fn strip_ipv6_brackets(host: &str) -> &str {
        host.strip_prefix('[')
            .and_then(|inner: &str| inner.strip_suffix(']'))
            .unwrap_or(host)
    }

    /// Format a host and port into `host:port` or `[ipv6]:port` for CONNECT / Host.
    ///
    /// # Arguments
    ///
    /// - `&str` - Hostname or IP literal.
    /// - `u16` - TCP port.
    ///
    /// # Returns
    ///
    /// - `String` - Properly bracketed authority string.
    fn format_authority(host: &str, port: u16) -> String {
        if host.contains(':') && !host.starts_with('[') {
            format!("[{host}]:{port}")
        } else {
            format!("{host}:{port}")
        }
    }

    /// Build a validated rustls `ServerName` for a DNS name or IPv4/IPv6 literal.
    ///
    /// # Arguments
    ///
    /// - `&str` - Target host string.
    ///
    /// # Returns
    ///
    /// - `Result<ServerName<'static>, RequestError>` - Valid TLS SNI / IP server name.
    fn server_name_for_host(host: &str) -> Result<ServerName<'static>, RequestError> {
        let clean: &str = Self::strip_ipv6_brackets(host);
        ServerName::try_from(clean.to_owned())
            .map_err(|e: InvalidDnsNameError| RequestError::Request(e.to_string()))
    }

    /// Return the effective non-zero read buffer size.
    ///
    /// # Returns
    ///
    /// - `usize` - Configured `buffer_size`, or `DEFAULT_EFFECTIVE_BUFFER_SIZE` when zero.
    fn effective_buffer_size(&self) -> usize {
        let configured: usize = self.get_config().get_buffer_size();
        if configured == 0 {
            DEFAULT_EFFECTIVE_BUFFER_SIZE
        } else {
            configured
        }
    }

    /// Return a cached `Arc<ClientConfig>` for HTTP/1.1 TLS connections.
    ///
    /// # Returns
    ///
    /// - `Arc<ClientConfig>` - Shared rustls client configuration with `http/1.1` ALPN.
    fn tls_client_config(&self) -> Arc<ClientConfig> {
        resolve_tls_client_config(self.get_tmp().get_root_cert(), false)
    }

    /// Return a cached `Arc<ClientConfig>` for HTTP/2 TLS connections.
    ///
    /// # Returns
    ///
    /// - `Arc<ClientConfig>` - Shared rustls client configuration with `h2` ALPN.
    fn tls_client_config_h2(&self) -> Arc<ClientConfig> {
        resolve_tls_client_config(self.get_tmp().get_root_cert(), true)
    }

    /// Return the effective TCP connect timeout in milliseconds (`connect_timeout` or fallback `timeout`).
    ///
    /// # Returns
    ///
    /// - `u64` - Effective connect timeout in milliseconds (`0` disables the connect deadline).
    fn effective_connect_timeout_ms(&self) -> u64 {
        let connect_ms: u64 = self.get_config().get_connect_timeout();
        if connect_ms > 0 {
            connect_ms
        } else {
            self.get_config().get_timeout()
        }
    }

    /// Validate and serialize the configured HTTP method token.
    ///
    /// # Returns
    ///
    /// - `Result<(String, bool), RequestError>` - `(method_token, is_head)` or an error on invalid tokens.
    fn validated_http_method(&self) -> Result<(String, bool), RequestError> {
        let method: Method = self.get_method();
        let token: String = method.to_string();
        if token.is_empty() || !is_valid_header_name(token.as_bytes()) {
            return Err(RequestError::Request(INVALID_HTTP_METHOD.to_owned()));
        }
        Ok((token, method.is_head()))
    }

    /// Validate and serialize the configured HTTP version token.
    ///
    /// # Returns
    ///
    /// - `Result<String, RequestError>` - Valid wire HTTP version token or an error.
    fn validated_http_version(&self) -> Result<String, RequestError> {
        let version: String = self.get_config().get_http_version().to_string();
        if !version.as_bytes().starts_with(HTTP_STATUS_PREFIX)
            || version.len() <= HTTP_STATUS_PREFIX.len()
            || version
                .bytes()
                .any(|byte: u8| byte <= SPACE_U8 || byte >= 0x7f)
        {
            return Err(RequestError::Request(INVALID_HTTP_VERSION.to_owned()));
        }
        Ok(version)
    }

    /// Validate an HTTP CONNECT proxy response head and return whether status is `2xx`.
    ///
    /// # Arguments
    ///
    /// - `&[u8]` - Proxy response header block up to `\r\n\r\n`.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` when the proxy returned a valid `HTTP/1.x 2xx` status line.
    fn is_connect_success_head(head: &[u8]) -> bool {
        let has_prefix: bool =
            head.starts_with(HTTP_CONNECT_PREFIX_1_1) || head.starts_with(HTTP_CONNECT_PREFIX_1_0);
        if !has_prefix {
            return false;
        }
        let status_start: usize = HTTP_CONNECT_PREFIX_1_1.len();
        let status_code: usize = head
            .get(status_start..status_start + 3)
            .map_or(0, parse_status_code);
        (200..300).contains(&status_code)
            && matches!(head.get(status_start + 3), Some(b' ' | b'\r') | None)
    }

    /// Build a validated HTTP CONNECT request string for an HTTP/HTTPS proxy.
    ///
    /// # Arguments
    ///
    /// - `&str` - Target host.
    /// - `u16` - Target port.
    /// - `&Proxy` - Proxy configuration.
    ///
    /// # Returns
    ///
    /// - `Result<String, RequestError>` - Validated CONNECT request bytes as a string.
    fn build_connect_request(
        target_host: &str,
        target_port: u16,
        proxy: &Proxy,
    ) -> Result<String, RequestError> {
        if !is_valid_header_value(target_host.as_bytes())
            || target_host
                .bytes()
                .any(|byte: u8| byte <= SPACE_U8 || byte >= 0x7f)
        {
            return Err(RequestError::InvalidUrlHost(HttpStatus::BadRequest));
        }
        let authority: String = Self::format_authority(target_host, target_port);
        if let (Some(u), Some(p)) = (proxy.get_username(), proxy.get_password()) {
            if u.contains(':')
                || !is_valid_header_value(u.as_bytes())
                || !is_valid_header_value(p.as_bytes())
            {
                return Err(RequestError::Request(PROXY_AUTH_ERROR.to_owned()));
            }
            let auth: String = format!("{u}:{p}");
            let encoded: String = crate::utils::base64_encode(auth.as_bytes());
            Ok(format!(
                "CONNECT {authority} HTTP/1.1\r\nHost: {authority}\r\nProxy-Authorization: Basic {encoded}\r\n\r\n"
            ))
        } else {
            Ok(format!(
                "CONNECT {authority} HTTP/1.1\r\nHost: {authority}\r\n\r\n"
            ))
        }
    }

    /// Resolve a `Location` redirect target (absolute or relative) against the current URL.
    ///
    /// # Arguments
    ///
    /// - `&HttpUrlComponents` - Current parsed URL.
    /// - `&str` - Raw `Location` header value.
    ///
    /// # Returns
    ///
    /// - `Result<String, RequestError>` - Resolved absolute `http://` or `https://` URL.
    fn resolve_redirect_url(
        current: &HttpUrlComponents,
        location: &str,
    ) -> Result<String, RequestError> {
        let trimmed: &str = location.trim();
        if trimmed.is_empty() {
            return Err(RequestError::Request(MISSING_REDIRECT_URL.to_owned()));
        }
        if trimmed.contains(SCHEME_SEPARATOR) {
            return Ok(trimmed.to_owned());
        }
        let raw_protocol: &str = current.get_protocol();
        let scheme: String = if raw_protocol.is_empty() {
            HTTP_SCHEME.to_owned()
        } else {
            raw_protocol.to_ascii_lowercase()
        };
        if let Some(rest) = trimmed.strip_prefix("//") {
            return Ok(format!("{scheme}://{rest}"));
        }
        let host: String = current.get_host().unwrap_or_default();
        let authority: String = match current.get_port() {
            Some(port) if port != 0 => Self::format_authority(&host, port),
            _ => host,
        };
        if trimmed.starts_with(URL_SLASH) {
            return Ok(format!("{scheme}://{authority}{trimmed}"));
        }
        if trimmed.starts_with(URL_QUERY) {
            let base_path: String = current.get_path().unwrap_or_else(|| URL_SLASH.to_string());
            return Ok(format!("{scheme}://{authority}{base_path}{trimmed}"));
        }
        let current_path: String = current.get_path().unwrap_or_else(|| URL_SLASH.to_string());
        let base_dir: &str = match current_path.rfind(URL_SLASH) {
            Some(pos) => &current_path[..=pos],
            None => "/",
        };
        Ok(format!("{scheme}://{authority}{base_dir}{trimmed}"))
    }

    /// Apply RFC 9110 redirect state transitions (loop detection, budget, credential stripping, method rewrite).
    ///
    /// # Arguments
    ///
    /// - `&HttpUrlComponents` - Parsed URL of the hop that produced the redirect.
    /// - `usize` - HTTP redirect status code (`301`/`302`/`303`/`307`/`308`).
    /// - `String` - Raw `Location` header value.
    ///
    /// # Returns
    ///
    /// - `Result<(), RequestError>` - `Ok(())` when the request is updated for the next hop.
    fn prepare_redirect_hop(
        &mut self,
        current_url: &HttpUrlComponents,
        status_code: usize,
        location: String,
    ) -> Result<(), RequestError> {
        let resolved: String = Self::resolve_redirect_url(current_url, &location)?;
        if self.get_tmp().get_visit_url().contains(&resolved) {
            return Err(RequestError::Request(REDIRECT_URL_DEAD_LOOP.to_string()));
        }
        self.get_mut_tmp()
            .get_mut_visit_url()
            .insert(resolved.clone());
        let remaining: usize = self.get_config().get_max_redirect_times();
        if remaining == 0 {
            return Err(RequestError::Request(
                MAX_REDIRECT_TIMES_EXCEEDED.to_string(),
            ));
        }
        self.get_mut_config()
            .set_max_redirect_times(remaining.saturating_sub(1));
        let prev_https: bool = Self::scheme_is_https(current_url);
        let prev_host: String = current_url.get_host().unwrap_or_default();
        let prev_port: u16 =
            Self::resolve_port_for_scheme(current_url.get_port().unwrap_or_default(), prev_https);
        self.set_url(resolved);
        let next_url: HttpUrlComponents = self.parse_url()?;
        let next_https: bool = Self::scheme_is_https(&next_url);
        let next_host: String = next_url.get_host().unwrap_or_default();
        let next_port: u16 =
            Self::resolve_port_for_scheme(next_url.get_port().unwrap_or_default(), next_https);
        let same_origin: bool = prev_https == next_https
            && prev_host.eq_ignore_ascii_case(&next_host)
            && prev_port == next_port;
        if !same_origin || (prev_https && !next_https) {
            self.remove_header_ci(HEADER_AUTHORIZATION);
            self.remove_header_ci(HEADER_PROXY_AUTHORIZATION);
            self.remove_header_ci(HEADER_COOKIE);
        }
        self.remove_header_ci(HOST);
        let current_method: Method = self.get_method();
        let rewrite_to_get: bool =
            (status_code == 303 && !current_method.is_get() && !current_method.is_head())
                || ([301, 302].contains(&status_code) && current_method.is_post());
        if rewrite_to_get {
            self.set_method(Method::Get);
            self.set_body(Body::empty());
            self.remove_header_ci(CONTENT_TYPE);
            self.remove_header_ci(CONTENT_LENGTH);
        }
        Ok(())
    }

    /// Synchronous send with connection pooling, idempotent retries, and iterative redirect handling.
    ///
    /// # Returns
    ///
    /// - `RequestResult` - The parsed HTTP response, or a `RequestError` on failure.
    pub(crate) fn send_sync(&mut self) -> RequestResult {
        let initial_url: String = self.get_url().to_owned();
        self.get_mut_tmp().get_mut_visit_url().insert(initial_url);
        loop {
            let url_obj: HttpUrlComponents = self.parse_url()?;
            validate_request_headers(self.get_headers())?;
            let (method_token, is_head): (String, bool) = self.validated_http_method()?;
            let version: String = self.validated_http_version()?;
            let request_bytes: Vec<u8> =
                build_wire_request_direct(self, &url_obj, &method_token, is_head, &version)?;
            let is_https: bool = Self::scheme_is_https(&url_obj);
            let host: String = url_obj.get_host().clone().unwrap_or_default();
            let port: u16 =
                Self::resolve_port_for_scheme(url_obj.get_port().unwrap_or_default(), is_https);
            let max_retries: usize = if is_idempotent_method(&self.get_method()) {
                self.get_config().get_max_retries()
            } else {
                0
            };
            let mut attempt: usize = 0;
            let (mut response, redirect_url): (HttpResponse, Option<Vec<u8>>) = loop {
                let res: Result<(HttpResponse, Option<Vec<u8>>), RequestError> =
                    self.exchange_sync_once(&host, port, is_https, &request_bytes, is_head);
                match res {
                    Ok((resp, redir)) => {
                        if attempt < max_retries && is_retryable_status(resp.get_status_code()) {
                            thread::sleep(retry_backoff_duration(attempt));
                            attempt += 1;
                            continue;
                        }
                        break (resp, redir);
                    }
                    Err(error) => {
                        if attempt < max_retries && is_retryable_error(&error) {
                            thread::sleep(retry_backoff_duration(attempt));
                            attempt += 1;
                            continue;
                        }
                        return Err(error);
                    }
                }
            };
            if !self.get_config().get_redirect() || redirect_url.is_none() {
                if !is_head && self.get_config().get_decode() {
                    response = response.decode(self.effective_buffer_size());
                }
                return Ok(response);
            }
            let url_bytes: Vec<u8> = redirect_url
                .ok_or_else(|| RequestError::Request(MISSING_REDIRECT_URL.to_string()))?;
            let location: String = String::from_utf8(url_bytes)
                .map_err(|e: FromUtf8Error| RequestError::Request(e.to_string()))?;
            self.prepare_redirect_hop(&url_obj, response.get_status_code(), location)?;
        }
    }

    /// Executes one synchronous HTTP/1 exchange using the idle keep-alive pool with transparent stale redial.
    ///
    /// # Arguments
    ///
    /// - `&str` - Target host.
    /// - `u16` - Target TCP port.
    /// - `bool` - Whether TLS is required.
    /// - `&[u8]` - Pre-serialized wire request bytes.
    /// - `bool` - Whether the request method is `HEAD`.
    ///
    /// # Returns
    ///
    /// - `Result<(HttpResponse, Option<Vec<u8>>), RequestError>` - Parsed response and optional redirect target.
    fn exchange_sync_once(
        &self,
        host: &str,
        port: u16,
        is_https: bool,
        request_bytes: &[u8],
        is_head: bool,
    ) -> Result<(HttpResponse, Option<Vec<u8>>), RequestError> {
        let disable_keep_alive: bool = request_disables_keep_alive(self);
        let pool_key: PoolKey = make_pool_key(
            is_https,
            host,
            port,
            self.get_config().get_proxy().as_ref(),
            self.get_tmp().get_root_cert(),
            self.get_config().get_timeout(),
        );
        let mut attempted_pooled: bool = false;
        loop {
            let (mut stream, from_pool): (BoxReadWrite, bool) = if !disable_keep_alive
                && !attempted_pooled
                && let Some(cached) = take_sync_h1_conn(&pool_key)
            {
                (cached, true)
            } else {
                (
                    self.open_sync_stream(host.to_owned(), port, is_https)?,
                    false,
                )
            };
            let write_res: Result<(), RequestError> = stream
                .write_all(request_bytes)
                .and_then(|_: ()| stream.flush())
                .map_err(|e: io::Error| RequestError::Request(e.to_string()));
            if let Err(error) = write_res {
                if from_pool {
                    attempted_pooled = true;
                    continue;
                }
                return Err(error);
            }
            match self.read_response_once_sync(&mut stream, is_head, disable_keep_alive) {
                Ok((response, redirect_url, reusable)) => {
                    if reusable && !disable_keep_alive {
                        put_sync_h1_conn(pool_key, stream);
                    }
                    return Ok((response, redirect_url));
                }
                Err(error) => {
                    if from_pool && is_retryable_error(&error) {
                        attempted_pooled = true;
                        continue;
                    }
                    return Err(error);
                }
            }
        }
    }

    /// Asynchronous send with H1/H2 pooling, idempotent retries, timeout enforcement, and iterative redirects.
    ///
    /// # Returns
    ///
    /// - `RequestResult` - The parsed HTTP response, or a `RequestError` on failure.
    async fn send_async_impl(&mut self) -> RequestResult {
        let initial_url: String = self.get_url().to_owned();
        self.get_mut_tmp().get_mut_visit_url().insert(initial_url);
        loop {
            let url_obj: HttpUrlComponents = self.parse_url()?;
            validate_request_headers(self.get_headers())?;
            let (method_token, is_head): (String, bool) = self.validated_http_method()?;
            let is_https: bool = Self::scheme_is_https(&url_obj);
            let host: String = url_obj.get_host().clone().unwrap_or_default();
            let port: u16 =
                Self::resolve_port_for_scheme(url_obj.get_port().unwrap_or_default(), is_https);
            let is_h2: bool = *self.get_config().get_http_version() == HttpVersion::Http2;
            let request_bytes: Option<Vec<u8>> = if is_h2 {
                None
            } else {
                let version: String = self.validated_http_version()?;
                Some(build_wire_request_direct(
                    self,
                    &url_obj,
                    &method_token,
                    is_head,
                    &version,
                )?)
            };
            let timeout_ms: u64 = self.get_config().get_timeout();
            let max_retries: usize = if is_idempotent_method(&self.get_method()) {
                self.get_config().get_max_retries()
            } else {
                0
            };
            let mut attempt: usize = 0;
            let (mut response, redirect_url): (HttpResponse, Option<Vec<u8>>) = loop {
                let attempt_res: Result<(HttpResponse, Option<Vec<u8>>), RequestError> = if is_h2 {
                    let pool_key: PoolKey = make_pool_key(
                        is_https,
                        &host,
                        port,
                        self.get_config().get_proxy().as_ref(),
                        self.get_tmp().get_root_cert(),
                        0,
                    );
                    let target: H2ExchangeTarget<'_> = H2ExchangeTarget {
                        url_obj: &url_obj,
                        method_token: &method_token,
                        is_head,
                        host: &host,
                        port,
                        is_https,
                        pool_key,
                    };
                    if timeout_ms > 0 {
                        match tokio::time::timeout(
                            Duration::from_millis(timeout_ms),
                            send_h2_exchange_async(self, target),
                        )
                        .await
                        {
                            Ok(inner) => inner,
                            Err(_) => Err(RequestError::ReadTimeout(HttpStatus::RequestTimeout)),
                        }
                    } else {
                        send_h2_exchange_async(self, target).await
                    }
                } else {
                    let wire: &[u8] = request_bytes.as_deref().unwrap_or_default();
                    if timeout_ms > 0 {
                        match tokio::time::timeout(
                            Duration::from_millis(timeout_ms),
                            self.exchange_async_once(&host, port, is_https, wire, is_head),
                        )
                        .await
                        {
                            Ok(inner) => inner,
                            Err(_) => Err(RequestError::ReadTimeout(HttpStatus::RequestTimeout)),
                        }
                    } else {
                        self.exchange_async_once(&host, port, is_https, wire, is_head)
                            .await
                    }
                };
                match attempt_res {
                    Ok((resp, redir)) => {
                        if attempt < max_retries && is_retryable_status(resp.get_status_code()) {
                            tokio::time::sleep(retry_backoff_duration(attempt)).await;
                            attempt += 1;
                            continue;
                        }
                        break (resp, redir);
                    }
                    Err(error) => {
                        if attempt < max_retries && is_retryable_error(&error) {
                            tokio::time::sleep(retry_backoff_duration(attempt)).await;
                            attempt += 1;
                            continue;
                        }
                        return Err(error);
                    }
                }
            };
            if !self.get_config().get_redirect() || redirect_url.is_none() {
                if !is_head && self.get_config().get_decode() {
                    response = response.decode(self.effective_buffer_size());
                }
                return Ok(response);
            }
            let url_bytes: Vec<u8> = redirect_url
                .ok_or_else(|| RequestError::Request(MISSING_REDIRECT_URL.to_string()))?;
            let location: String = String::from_utf8(url_bytes)
                .map_err(|e: FromUtf8Error| RequestError::Request(e.to_string()))?;
            self.prepare_redirect_hop(&url_obj, response.get_status_code(), location)?;
        }
    }

    /// Execute one asynchronous HTTP/1 exchange using the idle keep-alive pool with transparent stale redial.
    ///
    /// # Arguments
    ///
    /// - `&str` - Target host.
    /// - `u16` - Target TCP port.
    /// - `bool` - Whether to establish a TLS session.
    /// - `&[u8]` - Prebuilt wire request bytes.
    /// - `bool` - Whether the request method is `HEAD`.
    ///
    /// # Returns
    ///
    /// - `Result<(HttpResponse, Option<Vec<u8>>), RequestError>` - Response and optional redirect location.
    async fn exchange_async_once(
        &self,
        host: &str,
        port: u16,
        is_https: bool,
        request_bytes: &[u8],
        is_head: bool,
    ) -> Result<(HttpResponse, Option<Vec<u8>>), RequestError> {
        let disable_keep_alive: bool = request_disables_keep_alive(self);
        let pool_key: PoolKey = make_pool_key(
            is_https,
            host,
            port,
            self.get_config().get_proxy().as_ref(),
            self.get_tmp().get_root_cert(),
            0,
        );
        let mut attempted_pooled: bool = false;
        loop {
            let (mut stream, from_pool): (BoxAsyncReadWrite, bool) = if !disable_keep_alive
                && !attempted_pooled
                && let Some(cached) = take_async_h1_conn(&pool_key)
            {
                (cached, true)
            } else {
                (
                    self.open_async_stream(host.to_owned(), port, is_https)
                        .await?,
                    false,
                )
            };
            let write_res: Result<(), RequestError> = match stream.write_all(request_bytes).await {
                Ok(()) => stream
                    .flush()
                    .await
                    .map_err(|e: io::Error| RequestError::Request(e.to_string())),
                Err(error) => Err(RequestError::Request(error.to_string())),
            };
            if let Err(error) = write_res {
                if from_pool {
                    attempted_pooled = true;
                    continue;
                }
                return Err(error);
            }
            match self
                .read_response_once_async(&mut stream, is_head, disable_keep_alive)
                .await
            {
                Ok((response, redirect_url, reusable)) => {
                    if reusable && !disable_keep_alive {
                        put_async_h1_conn(pool_key, stream);
                    }
                    return Ok((response, redirect_url));
                }
                Err(error) => {
                    if from_pool && is_retryable_error(&error) {
                        attempted_pooled = true;
                        continue;
                    }
                    return Err(error);
                }
            }
        }
    }

    /// Connect a synchronous TCP stream with `TCP_NODELAY` and optional connect/read/write timeouts.
    ///
    /// # Arguments
    ///
    /// - `&str` - Hostname or IP literal (brackets stripped automatically).
    /// - `u16` - Target TCP port.
    ///
    /// # Returns
    ///
    /// - `Result<TcpStream, RequestError>` - Connected TCP stream with timeouts configured.
    fn connect_sync_tcp(&self, host: &str, port: u16) -> Result<TcpStream, RequestError> {
        let clean_host: &str = Self::strip_ipv6_brackets(host);
        let connect_ms: u64 = self.effective_connect_timeout_ms();
        let io_timeout_ms: u64 = self.get_config().get_timeout();
        let tcp: TcpStream = if connect_ms > 0 {
            let connect_dur: Duration = Duration::from_millis(connect_ms);
            let addrs: Vec<SocketAddr> = (clean_host, port)
                .to_socket_addrs()
                .map_err(|e: io::Error| RequestError::Request(e.to_string()))?
                .collect();
            let mut last_err: Option<io::Error> = None;
            let mut connected: Option<TcpStream> = None;
            for addr in addrs {
                match TcpStream::connect_timeout(&addr, connect_dur) {
                    Ok(stream) => {
                        connected = Some(stream);
                        break;
                    }
                    Err(error) => {
                        last_err = Some(error);
                    }
                }
            }
            match connected {
                Some(stream) => stream,
                None => {
                    return Err(RequestError::Request(
                        last_err
                            .map(|e: io::Error| e.to_string())
                            .unwrap_or_else(|| INTERNAL_SERVER_ERROR.to_owned()),
                    ));
                }
            }
        } else {
            TcpStream::connect((clean_host, port))
                .map_err(|e: io::Error| RequestError::Request(e.to_string()))?
        };
        let _: Result<(), io::Error> = tcp.set_nodelay(true);
        if io_timeout_ms > 0 {
            let io_dur: Duration = Duration::from_millis(io_timeout_ms);
            tcp.set_read_timeout(Some(io_dur))
                .map_err(|e: io::Error| RequestError::Request(e.to_string()))?;
            tcp.set_write_timeout(Some(io_dur))
                .map_err(|e: io::Error| RequestError::Request(e.to_string()))?;
        }
        Ok(tcp)
    }

    /// Connect an asynchronous TCP stream with `TCP_NODELAY` and optional connect timeout.
    ///
    /// # Arguments
    ///
    /// - `&str` - Hostname or IP literal (brackets stripped automatically).
    /// - `u16` - Target TCP port.
    ///
    /// # Returns
    ///
    /// - `Result<tokio::net::TcpStream, RequestError>` - Connected async TCP stream.
    async fn connect_async_tcp(
        &self,
        host: &str,
        port: u16,
    ) -> Result<tokio::net::TcpStream, RequestError> {
        let clean_host: String = Self::strip_ipv6_brackets(host).to_owned();
        let connect_ms: u64 = self.effective_connect_timeout_ms();
        let tcp: tokio::net::TcpStream = if connect_ms > 0 {
            tokio::time::timeout(
                Duration::from_millis(connect_ms),
                tokio::net::TcpStream::connect((clean_host, port)),
            )
            .await
            .map_err(|_| RequestError::ReadTimeout(HttpStatus::RequestTimeout))?
            .map_err(|e: io::Error| RequestError::Request(e.to_string()))?
        } else {
            tokio::net::TcpStream::connect((clean_host, port))
                .await
                .map_err(|e: io::Error| RequestError::Request(e.to_string()))?
        };
        let _: Result<(), io::Error> = tcp.set_nodelay(true);
        Ok(tcp)
    }

    /// Open a synchronous stream to the target host, directly or through a proxy.
    ///
    /// # Arguments
    ///
    /// - `String` - The target host name or IP address.
    /// - `u16` - The target TCP port.
    /// - `bool` - Whether the target URL requires TLS.
    ///
    /// # Returns
    ///
    /// - `Result<BoxReadWrite, RequestError>` - The connected stream, TLS-wrapped for HTTPS, or a `RequestError` on failure.
    fn open_sync_stream(
        &self,
        host: String,
        port: u16,
        is_https: bool,
    ) -> Result<BoxReadWrite, RequestError> {
        if let Some(proxy) = &self.get_config().get_proxy() {
            return self.open_sync_proxy_stream(host, port, is_https, proxy);
        }
        let tcp: TcpStream = self.connect_sync_tcp(&host, port)?;
        if is_https {
            let tls_cfg: Arc<ClientConfig> = self.tls_client_config();
            let dns: ServerName<'static> = Self::server_name_for_host(&host)?;
            let session: ClientConnection = ClientConnection::new(tls_cfg, dns)
                .map_err(|e: Error| RequestError::Request(e.to_string()))?;
            Ok(Box::new(StreamOwned::new(session, tcp)))
        } else {
            Ok(Box::new(tcp))
        }
    }

    /// Reads one HTTP/1 response synchronously with pooled scratch buffers and keep-alive reuse detection.
    ///
    /// # Arguments
    ///
    /// - `&mut BoxReadWrite` - The connected stream to read the response from.
    /// - `bool` - Whether this response answers a HEAD request.
    /// - `bool` - Whether the caller disabled keep-alive.
    ///
    /// # Returns
    ///
    /// - `Result<(HttpResponse, Option<Vec<u8>>, bool), RequestError>` - Response, optional redirect target, and pool reusability flag.
    fn read_response_once_sync(
        &self,
        stream: &mut BoxReadWrite,
        is_head: bool,
        disable_keep_alive: bool,
    ) -> Result<(HttpResponse, Option<Vec<u8>>, bool), RequestError> {
        let buffer_size: usize = self.effective_buffer_size();
        let mut buffer: Vec<u8> = take_client_buffer(buffer_size);
        buffer.resize(buffer_size, 0u8);
        let mut response_bytes: Vec<u8> = take_client_buffer(buffer_size.max(8192));
        let mut head: Option<FinalResponseHead> = None;
        let mut interim_count: usize = 0;
        let mut read_error: Option<RequestError> = None;
        loop {
            let n: usize = match stream.read(&mut buffer) {
                Ok(n) => n,
                Err(error) => {
                    read_error = Some(RequestError::Request(error.to_string()));
                    break;
                }
            };
            if n == 0 {
                break;
            }
            let new_cap: usize =
                calculate_buffer_capacity(&response_bytes, n, response_bytes.capacity());
            if new_cap > 0 {
                response_bytes.reserve(new_cap.saturating_sub(response_bytes.capacity()));
            }
            let old_len: usize = response_bytes.len();
            response_bytes.extend_from_slice(&buffer[..n]);
            if head.is_none() {
                match find_final_response_head(
                    &mut response_bytes,
                    old_len.saturating_sub(3),
                    is_head,
                    &mut interim_count,
                ) {
                    Ok(found) => head = found,
                    Err(error) => {
                        read_error = Some(error);
                        break;
                    }
                }
            }
            if let Some((end, framing, _)) = &head {
                let body_slice: &[u8] = &response_bytes[*end..];
                if body_slice.len() > MAX_RESPONSE_BODY_BYTES {
                    read_error = Some(RequestError::ContentLengthTooLarge(HttpStatus::BadRequest));
                    break;
                }
                match response_body_is_complete(*framing, body_slice) {
                    Ok(true) => break,
                    Ok(false) => {}
                    Err(error) => {
                        read_error = Some(error);
                        break;
                    }
                }
            }
        }
        return_client_buffer(buffer);
        if let Some(error) = read_error {
            return_client_buffer(response_bytes);
            return Err(error);
        }
        let Some((head_end, framing, redirect_url)) = head else {
            return_client_buffer(response_bytes);
            return Err(RequestError::HttpRequestPartsInsufficient(
                HttpStatus::BadRequest,
            ));
        };
        let body_slice: &[u8] = &response_bytes[head_end..];
        let has_trailing: bool = match framing {
            ResponseFraming::Fixed(len) => body_slice.len() != len,
            ResponseFraming::Chunked => match chunked_body_end(body_slice) {
                Ok(Some(end)) => end != body_slice.len(),
                _ => true,
            },
            ResponseFraming::Close => true,
        };
        let reusable: bool = response_allows_pool_reuse(
            &response_bytes[..head_end],
            framing,
            has_trailing,
            disable_keep_alive,
        );
        let parsed: Result<HttpResponse, RequestError> =
            finish_framed_response(&response_bytes, head_end, framing);
        return_client_buffer(response_bytes);
        let response: HttpResponse = parsed?;
        Ok((response, redirect_url, reusable))
    }

    /// Open an asynchronous HTTP/1 stream to the target host, directly or through a proxy.
    ///
    /// # Arguments
    ///
    /// - `String` - The target host name or IP address.
    /// - `u16` - The target TCP port.
    /// - `bool` - Whether the target URL requires TLS.
    ///
    /// # Returns
    ///
    /// - `Result<BoxAsyncReadWrite, RequestError>` - The connected stream, TLS-wrapped for HTTPS, or a `RequestError` on failure.
    async fn open_async_stream(
        &self,
        host: String,
        port: u16,
        is_https: bool,
    ) -> Result<BoxAsyncReadWrite, RequestError> {
        if let Some(proxy) = &self.get_config().get_proxy() {
            return self
                .open_async_proxy_stream(host, port, is_https, proxy, false)
                .await;
        }
        let tcp: tokio::net::TcpStream = self.connect_async_tcp(&host, port).await?;
        if is_https {
            let connector: TlsConnector = TlsConnector::from(self.tls_client_config());
            let dns: ServerName<'static> = Self::server_name_for_host(&host)?;
            let tls: TlsStream<tokio::net::TcpStream> = connector
                .connect(dns, tcp)
                .await
                .map_err(|e: io::Error| RequestError::Request(e.to_string()))?;
            Ok(Box::new(tls))
        } else {
            Ok(Box::new(tcp))
        }
    }

    /// Open an asynchronous HTTP/2 (`h2` ALPN over TLS or cleartext `h2c`) transport stream.
    ///
    /// # Arguments
    ///
    /// - `&str` - Target host name or IP address.
    /// - `u16` - Target TCP port.
    /// - `bool` - Whether to negotiate TLS + `h2` ALPN (`false` uses cleartext `h2c`).
    ///
    /// # Returns
    ///
    /// - `Result<BoxAsyncReadWrite, RequestError>` - Connected transport ready for `h2::client::handshake`.
    pub(crate) async fn open_async_stream_h2(
        &self,
        host: &str,
        port: u16,
        is_https: bool,
    ) -> Result<BoxAsyncReadWrite, RequestError> {
        if let Some(proxy) = &self.get_config().get_proxy() {
            return self
                .open_async_proxy_stream(host.to_owned(), port, is_https, proxy, true)
                .await;
        }
        let tcp: tokio::net::TcpStream = self.connect_async_tcp(host, port).await?;
        if is_https {
            let connector: TlsConnector = TlsConnector::from(self.tls_client_config_h2());
            let dns: ServerName<'static> = Self::server_name_for_host(host)?;
            let tls: TlsStream<tokio::net::TcpStream> = connector
                .connect(dns, tcp)
                .await
                .map_err(|e: io::Error| RequestError::Request(e.to_string()))?;
            if tls.get_ref().1.alpn_protocol() != Some(ALPN_H2) {
                return Err(RequestError::Request(H2_ALPN_MISMATCH.to_owned()));
            }
            Ok(Box::new(tls))
        } else {
            Ok(Box::new(tcp))
        }
    }

    /// Reads one HTTP/1 response asynchronously with pooled scratch buffers and keep-alive reuse detection.
    ///
    /// # Arguments
    ///
    /// - `&mut BoxAsyncReadWrite` - The connected stream to read the response from.
    /// - `bool` - Whether this response answers a HEAD request.
    /// - `bool` - Whether the caller disabled keep-alive.
    ///
    /// # Returns
    ///
    /// - `Result<(HttpResponse, Option<Vec<u8>>, bool), RequestError>` - Response, optional redirect target, and pool reusability flag.
    async fn read_response_once_async(
        &self,
        stream: &mut BoxAsyncReadWrite,
        is_head: bool,
        disable_keep_alive: bool,
    ) -> Result<(HttpResponse, Option<Vec<u8>>, bool), RequestError> {
        let buffer_size: usize = self.effective_buffer_size();
        let mut buffer: Vec<u8> = take_client_buffer(buffer_size);
        buffer.resize(buffer_size, 0u8);
        let mut response_bytes: Vec<u8> = take_client_buffer(buffer_size.max(8192));
        let mut head: Option<FinalResponseHead> = None;
        let mut interim_count: usize = 0;
        let mut read_error: Option<RequestError> = None;
        loop {
            let n: usize = match stream.read(&mut buffer).await {
                Ok(n) => n,
                Err(error) => {
                    read_error = Some(RequestError::Request(error.to_string()));
                    break;
                }
            };
            if n == 0 {
                break;
            }
            let new_cap: usize =
                calculate_buffer_capacity(&response_bytes, n, response_bytes.capacity());
            if new_cap > 0 {
                response_bytes.reserve(new_cap.saturating_sub(response_bytes.capacity()));
            }
            let old_len: usize = response_bytes.len();
            response_bytes.extend_from_slice(&buffer[..n]);
            if head.is_none() {
                match find_final_response_head(
                    &mut response_bytes,
                    old_len.saturating_sub(3),
                    is_head,
                    &mut interim_count,
                ) {
                    Ok(found) => head = found,
                    Err(error) => {
                        read_error = Some(error);
                        break;
                    }
                }
            }
            if let Some((end, framing, _)) = &head {
                let body_slice: &[u8] = &response_bytes[*end..];
                if body_slice.len() > MAX_RESPONSE_BODY_BYTES {
                    read_error = Some(RequestError::ContentLengthTooLarge(HttpStatus::BadRequest));
                    break;
                }
                match response_body_is_complete(*framing, body_slice) {
                    Ok(true) => break,
                    Ok(false) => {}
                    Err(error) => {
                        read_error = Some(error);
                        break;
                    }
                }
            }
        }
        return_client_buffer(buffer);
        if let Some(error) = read_error {
            return_client_buffer(response_bytes);
            return Err(error);
        }
        let Some((head_end, framing, redirect_url)) = head else {
            return_client_buffer(response_bytes);
            return Err(RequestError::HttpRequestPartsInsufficient(
                HttpStatus::BadRequest,
            ));
        };
        let body_slice: &[u8] = &response_bytes[head_end..];
        let has_trailing: bool = match framing {
            ResponseFraming::Fixed(len) => body_slice.len() != len,
            ResponseFraming::Chunked => match chunked_body_end(body_slice) {
                Ok(Some(end)) => end != body_slice.len(),
                _ => true,
            },
            ResponseFraming::Close => true,
        };
        let reusable: bool = response_allows_pool_reuse(
            &response_bytes[..head_end],
            framing,
            has_trailing,
            disable_keep_alive,
        );
        let parsed: Result<HttpResponse, RequestError> =
            finish_framed_response(&response_bytes, head_end, framing);
        return_client_buffer(response_bytes);
        let response: HttpResponse = parsed?;
        Ok((response, redirect_url, reusable))
    }

    /// Open a synchronous stream to the target through the configured proxy.
    ///
    /// # Arguments
    ///
    /// - `String` - The target host name or IP address.
    /// - `u16` - The target TCP port.
    /// - `bool` - Whether the target URL requires TLS over the tunnel.
    /// - `&Proxy` - The proxy configuration selecting the HTTP or SOCKS5 handshake.
    ///
    /// # Returns
    ///
    /// - `Result<BoxReadWrite, RequestError>` - The tunnelled stream, or a `RequestError` when the handshake fails.
    fn open_sync_proxy_stream(
        &self,
        target_host: String,
        target_port: u16,
        is_https: bool,
        proxy: &Proxy,
    ) -> Result<BoxReadWrite, RequestError> {
        match *proxy.get_proxy_type() {
            ProxyType::Http | ProxyType::Https => {
                self.open_sync_http_proxy(target_host, target_port, is_https, proxy)
            }
            ProxyType::Socks5 => {
                self.open_sync_socks5_proxy(target_host, target_port, is_https, proxy)
            }
        }
    }

    /// Open a synchronous stream to the target through an HTTP or HTTPS proxy.
    ///
    /// # Arguments
    ///
    /// - `String` - The target host name or IP address.
    /// - `u16` - The target TCP port.
    /// - `bool` - Whether the target URL requires TLS over the tunnel.
    /// - `&Proxy` - The proxy configuration.
    ///
    /// # Returns
    ///
    /// - `Result<BoxReadWrite, RequestError>` - The tunnelled stream, or a `RequestError` when the handshake fails.
    fn open_sync_http_proxy(
        &self,
        target_host: String,
        target_port: u16,
        is_https: bool,
        proxy: &Proxy,
    ) -> Result<BoxReadWrite, RequestError> {
        let connect_request: String =
            Self::build_connect_request(&target_host, target_port, proxy)?;
        let tcp: TcpStream = self.connect_sync_tcp(proxy.get_host(), proxy.get_port())?;
        let mut proxy_stream: BoxReadWrite = if proxy.get_proxy_type() == &ProxyType::Https {
            let tls_cfg: Arc<ClientConfig> = self.tls_client_config();
            let dns: ServerName<'static> = Self::server_name_for_host(proxy.get_host())?;
            let session: ClientConnection = ClientConnection::new(tls_cfg, dns)
                .map_err(|e: Error| RequestError::Request(e.to_string()))?;
            Box::new(StreamOwned::new(session, tcp))
        } else {
            Box::new(tcp)
        };
        proxy_stream
            .write_all(connect_request.as_bytes())
            .map_err(|e: io::Error| RequestError::Request(e.to_string()))?;
        proxy_stream
            .flush()
            .map_err(|e: io::Error| RequestError::Request(e.to_string()))?;
        let mut raw_resp: Vec<u8> = Vec::with_capacity(512);
        let mut chunk: [u8; 512] = [0u8; 512];
        let head_end: usize = loop {
            if let Some(pos) = find_double_crlf(&raw_resp, 0) {
                break pos;
            }
            if raw_resp.len() >= MAX_CONNECT_HEAD_BYTES {
                return Err(RequestError::Request(INTERNAL_SERVER_ERROR.to_string()));
            }
            let n: usize = proxy_stream
                .read(&mut chunk)
                .map_err(|e: io::Error| RequestError::Request(e.to_string()))?;
            if n == 0 {
                return Err(RequestError::Request(INTERNAL_SERVER_ERROR.to_string()));
            }
            raw_resp.extend_from_slice(&chunk[..n]);
        };
        if !Self::is_connect_success_head(&raw_resp[..head_end]) {
            return Err(RequestError::Request(INTERNAL_SERVER_ERROR.to_string()));
        }
        let pre_read: Vec<u8> = raw_resp[head_end + 4..].to_vec();
        if is_https {
            let tls_cfg: Arc<ClientConfig> = self.tls_client_config();
            let dns: ServerName<'static> = Self::server_name_for_host(&target_host)?;
            let session: ClientConnection = ClientConnection::new(tls_cfg, dns)
                .map_err(|e: Error| RequestError::Request(e.to_string()))?;
            let tunnel: SyncProxyTunnelStream = SyncProxyTunnelStream::new(proxy_stream, pre_read);
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
    /// - `bool` - Whether the target URL requires TLS over the tunnel.
    /// - `&Proxy` - The proxy configuration.
    ///
    /// # Returns
    ///
    /// - `Result<BoxReadWrite, RequestError>` - The tunnelled stream, or a `RequestError` when the handshake fails.
    fn open_sync_socks5_proxy(
        &self,
        target_host: String,
        target_port: u16,
        is_https: bool,
        proxy: &Proxy,
    ) -> Result<BoxReadWrite, RequestError> {
        let has_auth: bool = proxy.get_username().is_some() && proxy.get_password().is_some();
        if let (Some(u), Some(p)) = (proxy.get_username(), proxy.get_password())
            && (u.len() > usize::from(u8::MAX) || p.len() > usize::from(u8::MAX))
        {
            return Err(RequestError::Request(PROXY_AUTH_ERROR.to_owned()));
        }
        let clean_target: &str = Self::strip_ipv6_brackets(&target_host);
        let mut connect_req: Vec<u8> = vec![0x05, 0x01, 0x00];
        if let Ok(ip) = clean_target.parse::<Ipv4Addr>() {
            connect_req.push(0x01);
            connect_req.extend_from_slice(&ip.octets());
        } else if let Ok(ip) = clean_target.parse::<Ipv6Addr>() {
            connect_req.push(0x04);
            connect_req.extend_from_slice(&ip.octets());
        } else {
            let host_len: u8 = u8::try_from(clean_target.len())
                .map_err(|_| RequestError::Request(PROXY_AUTH_ERROR.to_owned()))?;
            connect_req.push(0x03);
            connect_req.push(host_len);
            connect_req.extend_from_slice(clean_target.as_bytes());
        }
        connect_req.extend_from_slice(&target_port.to_be_bytes());
        let mut tcp: TcpStream = self.connect_sync_tcp(proxy.get_host(), proxy.get_port())?;
        let auth_methods: [u8; 3] = if has_auth {
            [0x05, 0x01, 0x02]
        } else {
            [0x05, 0x01, 0x00]
        };
        tcp.write_all(&auth_methods)
            .map_err(|e: io::Error| RequestError::Request(e.to_string()))?;
        let mut resp: [u8; 2] = [0u8; 2];
        tcp.read_exact(&mut resp)
            .map_err(|e: io::Error| RequestError::Request(e.to_string()))?;
        if resp[0] != 0x05 {
            return Err(RequestError::Request(INTERNAL_SERVER_ERROR.to_string()));
        }
        match resp[1] {
            0x00 if !has_auth => {}
            0x02 => {
                let (Some(u), Some(p)) = (proxy.get_username(), proxy.get_password()) else {
                    return Err(RequestError::Request(INTERNAL_SERVER_ERROR.to_string()));
                };
                let mut auth_req: Vec<u8> = Vec::with_capacity(3 + u.len() + p.len());
                auth_req.push(0x01);
                auth_req.push(u.len() as u8);
                auth_req.extend_from_slice(u.as_bytes());
                auth_req.push(p.len() as u8);
                auth_req.extend_from_slice(p.as_bytes());
                tcp.write_all(&auth_req)
                    .map_err(|e: io::Error| RequestError::Request(e.to_string()))?;
                let mut auth_resp: [u8; 2] = [0u8; 2];
                tcp.read_exact(&mut auth_resp)
                    .map_err(|e: io::Error| RequestError::Request(e.to_string()))?;
                if auth_resp[0] != 0x01 || auth_resp[1] != 0x00 {
                    return Err(RequestError::Request(INTERNAL_SERVER_ERROR.to_string()));
                }
            }
            _ => return Err(RequestError::Request(INTERNAL_SERVER_ERROR.to_string())),
        }
        tcp.write_all(&connect_req)
            .map_err(|e: io::Error| RequestError::Request(e.to_string()))?;
        let mut connect_resp: [u8; 4] = [0u8; 4];
        tcp.read_exact(&mut connect_resp)
            .map_err(|e: io::Error| RequestError::Request(e.to_string()))?;
        if connect_resp[0] != 0x05 || connect_resp[1] != 0x00 {
            return Err(RequestError::Request(INTERNAL_SERVER_ERROR.to_string()));
        }
        match connect_resp[3] {
            0x01 => {
                let mut skip: [u8; 6] = [0u8; 6];
                tcp.read_exact(&mut skip)
                    .map_err(|e: io::Error| RequestError::Request(e.to_string()))?;
            }
            0x03 => {
                let mut len: [u8; 1] = [0u8; 1];
                tcp.read_exact(&mut len)
                    .map_err(|e: io::Error| RequestError::Request(e.to_string()))?;
                let mut skip: Vec<u8> = vec![0u8; len[0] as usize + 2];
                tcp.read_exact(&mut skip)
                    .map_err(|e: io::Error| RequestError::Request(e.to_string()))?;
            }
            0x04 => {
                let mut skip: [u8; 18] = [0u8; 18];
                tcp.read_exact(&mut skip)
                    .map_err(|e: io::Error| RequestError::Request(e.to_string()))?;
            }
            _ => return Err(RequestError::Request(INTERNAL_SERVER_ERROR.to_string())),
        }
        if is_https {
            let tls_cfg: Arc<ClientConfig> = self.tls_client_config();
            let dns: ServerName<'static> = Self::server_name_for_host(&target_host)?;
            let session: ClientConnection = ClientConnection::new(tls_cfg, dns)
                .map_err(|e: Error| RequestError::Request(e.to_string()))?;
            let proxy_box: BoxReadWrite = Box::new(tcp);
            let tunnel: SyncProxyTunnelStream = SyncProxyTunnelStream::new(proxy_box, Vec::new());
            return Ok(Box::new(StreamOwned::new(session, tunnel)));
        }
        Ok(Box::new(tcp))
    }

    /// Open an asynchronous stream to the target through the configured proxy.
    ///
    /// # Arguments
    ///
    /// - `String` - The target host name or IP address.
    /// - `u16` - The target TCP port.
    /// - `bool` - Whether the target URL requires TLS over the tunnel.
    /// - `&Proxy` - The proxy configuration selecting the HTTP or SOCKS5 handshake.
    /// - `bool` - Whether to negotiate `h2` ALPN on target TLS handshakes.
    ///
    /// # Returns
    ///
    /// - `Result<BoxAsyncReadWrite, RequestError>` - The tunnelled stream, or a `RequestError` when the handshake fails.
    async fn open_async_proxy_stream(
        &self,
        target_host: String,
        target_port: u16,
        is_https: bool,
        proxy: &Proxy,
        enable_h2: bool,
    ) -> Result<BoxAsyncReadWrite, RequestError> {
        match *proxy.get_proxy_type() {
            ProxyType::Http | ProxyType::Https => {
                self.open_async_http_proxy(target_host, target_port, is_https, proxy, enable_h2)
                    .await
            }
            ProxyType::Socks5 => {
                self.open_async_socks5_proxy(target_host, target_port, is_https, proxy, enable_h2)
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
    /// - `bool` - Whether the target URL requires TLS over the tunnel.
    /// - `&Proxy` - The proxy configuration.
    /// - `bool` - Whether to negotiate `h2` ALPN on target TLS handshakes.
    ///
    /// # Returns
    ///
    /// - `Result<BoxAsyncReadWrite, RequestError>` - The tunnelled stream, or a `RequestError` when the handshake fails.
    async fn open_async_http_proxy(
        &self,
        target_host: String,
        target_port: u16,
        is_https: bool,
        proxy: &Proxy,
        enable_h2: bool,
    ) -> Result<BoxAsyncReadWrite, RequestError> {
        let connect_request: String =
            Self::build_connect_request(&target_host, target_port, proxy)?;
        let tcp: tokio::net::TcpStream = self
            .connect_async_tcp(proxy.get_host(), proxy.get_port())
            .await?;
        let mut proxy_stream: BoxAsyncReadWrite = if proxy.get_proxy_type() == &ProxyType::Https {
            let connector: TlsConnector = TlsConnector::from(self.tls_client_config());
            let dns: ServerName<'static> = Self::server_name_for_host(proxy.get_host())?;
            let tls: TlsStream<tokio::net::TcpStream> = connector
                .connect(dns, tcp)
                .await
                .map_err(|e: io::Error| RequestError::Request(e.to_string()))?;
            Box::new(tls)
        } else {
            Box::new(tcp)
        };
        proxy_stream
            .write_all(connect_request.as_bytes())
            .await
            .map_err(|e: io::Error| RequestError::Request(e.to_string()))?;
        proxy_stream
            .flush()
            .await
            .map_err(|e: io::Error| RequestError::Request(e.to_string()))?;
        let mut raw_resp: Vec<u8> = Vec::with_capacity(512);
        let mut chunk: [u8; 512] = [0u8; 512];
        let head_end: usize = loop {
            if let Some(pos) = find_double_crlf(&raw_resp, 0) {
                break pos;
            }
            if raw_resp.len() >= MAX_CONNECT_HEAD_BYTES {
                return Err(RequestError::Request(INTERNAL_SERVER_ERROR.to_string()));
            }
            let n: usize = proxy_stream
                .read(&mut chunk)
                .await
                .map_err(|e: io::Error| RequestError::Request(e.to_string()))?;
            if n == 0 {
                return Err(RequestError::Request(INTERNAL_SERVER_ERROR.to_string()));
            }
            raw_resp.extend_from_slice(&chunk[..n]);
        };
        if !Self::is_connect_success_head(&raw_resp[..head_end]) {
            return Err(RequestError::Request(INTERNAL_SERVER_ERROR.to_string()));
        }
        let pre_read: Vec<u8> = raw_resp[head_end + 4..].to_vec();
        if is_https {
            let tls_cfg: Arc<ClientConfig> = if enable_h2 {
                self.tls_client_config_h2()
            } else {
                self.tls_client_config()
            };
            let connector: TlsConnector = TlsConnector::from(tls_cfg);
            let dns: ServerName<'static> = Self::server_name_for_host(&target_host)?;
            let tunnel: ProxyTunnelStream = ProxyTunnelStream::new(proxy_stream, pre_read);
            let tls: TlsStream<ProxyTunnelStream> = connector
                .connect(dns, tunnel)
                .await
                .map_err(|e: io::Error| RequestError::Request(e.to_string()))?;
            if enable_h2 && tls.get_ref().1.alpn_protocol() != Some(ALPN_H2) {
                return Err(RequestError::Request(H2_ALPN_MISMATCH.to_owned()));
            }
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
    /// - `bool` - Whether the target URL requires TLS over the tunnel.
    /// - `&Proxy` - The proxy configuration.
    /// - `bool` - Whether to negotiate `h2` ALPN on target TLS handshakes.
    ///
    /// # Returns
    ///
    /// - `Result<BoxAsyncReadWrite, RequestError>` - The tunnelled stream, or a `RequestError` when the handshake fails.
    async fn open_async_socks5_proxy(
        &self,
        target_host: String,
        target_port: u16,
        is_https: bool,
        proxy: &Proxy,
        enable_h2: bool,
    ) -> Result<BoxAsyncReadWrite, RequestError> {
        let has_auth: bool = proxy.get_username().is_some() && proxy.get_password().is_some();
        if let (Some(u), Some(p)) = (proxy.get_username(), proxy.get_password())
            && (u.len() > usize::from(u8::MAX) || p.len() > usize::from(u8::MAX))
        {
            return Err(RequestError::Request(PROXY_AUTH_ERROR.to_owned()));
        }
        let clean_target: &str = Self::strip_ipv6_brackets(&target_host);
        let mut connect_req: Vec<u8> = vec![0x05, 0x01, 0x00];
        if let Ok(ip) = clean_target.parse::<Ipv4Addr>() {
            connect_req.push(0x01);
            connect_req.extend_from_slice(&ip.octets());
        } else if let Ok(ip) = clean_target.parse::<Ipv6Addr>() {
            connect_req.push(0x04);
            connect_req.extend_from_slice(&ip.octets());
        } else {
            let host_len: u8 = u8::try_from(clean_target.len())
                .map_err(|_| RequestError::Request(PROXY_AUTH_ERROR.to_owned()))?;
            connect_req.push(0x03);
            connect_req.push(host_len);
            connect_req.extend_from_slice(clean_target.as_bytes());
        }
        connect_req.extend_from_slice(&target_port.to_be_bytes());
        let mut tcp: tokio::net::TcpStream = self
            .connect_async_tcp(proxy.get_host(), proxy.get_port())
            .await?;
        let auth_methods: [u8; 3] = if has_auth {
            [0x05, 0x01, 0x02]
        } else {
            [0x05, 0x01, 0x00]
        };
        tcp.write_all(&auth_methods)
            .await
            .map_err(|e: io::Error| RequestError::Request(e.to_string()))?;
        let mut resp: [u8; 2] = [0u8; 2];
        tcp.read_exact(&mut resp)
            .await
            .map_err(|e: io::Error| RequestError::Request(e.to_string()))?;
        if resp[0] != 0x05 {
            return Err(RequestError::Request(INTERNAL_SERVER_ERROR.to_string()));
        }
        match resp[1] {
            0x00 if !has_auth => {}
            0x02 => {
                let (Some(u), Some(p)) = (proxy.get_username(), proxy.get_password()) else {
                    return Err(RequestError::Request(INTERNAL_SERVER_ERROR.to_string()));
                };
                let mut auth_req: Vec<u8> = Vec::with_capacity(3 + u.len() + p.len());
                auth_req.push(0x01);
                auth_req.push(u.len() as u8);
                auth_req.extend_from_slice(u.as_bytes());
                auth_req.push(p.len() as u8);
                auth_req.extend_from_slice(p.as_bytes());
                tcp.write_all(&auth_req)
                    .await
                    .map_err(|e: io::Error| RequestError::Request(e.to_string()))?;
                let mut auth_resp: [u8; 2] = [0u8; 2];
                tcp.read_exact(&mut auth_resp)
                    .await
                    .map_err(|e: io::Error| RequestError::Request(e.to_string()))?;
                if auth_resp[0] != 0x01 || auth_resp[1] != 0x00 {
                    return Err(RequestError::Request(INTERNAL_SERVER_ERROR.to_string()));
                }
            }
            _ => return Err(RequestError::Request(INTERNAL_SERVER_ERROR.to_string())),
        }
        tcp.write_all(&connect_req)
            .await
            .map_err(|e: io::Error| RequestError::Request(e.to_string()))?;
        let mut connect_resp: [u8; 4] = [0u8; 4];
        tcp.read_exact(&mut connect_resp)
            .await
            .map_err(|e: io::Error| RequestError::Request(e.to_string()))?;
        if connect_resp[0] != 0x05 || connect_resp[1] != 0x00 {
            return Err(RequestError::Request(INTERNAL_SERVER_ERROR.to_string()));
        }
        match connect_resp[3] {
            0x01 => {
                let mut skip: [u8; 6] = [0u8; 6];
                tcp.read_exact(&mut skip)
                    .await
                    .map_err(|e: io::Error| RequestError::Request(e.to_string()))?;
            }
            0x03 => {
                let mut len: [u8; 1] = [0u8; 1];
                tcp.read_exact(&mut len)
                    .await
                    .map_err(|e: io::Error| RequestError::Request(e.to_string()))?;
                let mut skip: Vec<u8> = vec![0u8; len[0] as usize + 2];
                tcp.read_exact(&mut skip)
                    .await
                    .map_err(|e: io::Error| RequestError::Request(e.to_string()))?;
            }
            0x04 => {
                let mut skip: [u8; 18] = [0u8; 18];
                tcp.read_exact(&mut skip)
                    .await
                    .map_err(|e: io::Error| RequestError::Request(e.to_string()))?;
            }
            _ => return Err(RequestError::Request(INTERNAL_SERVER_ERROR.to_string())),
        }
        if is_https {
            let tls_cfg: Arc<ClientConfig> = if enable_h2 {
                self.tls_client_config_h2()
            } else {
                self.tls_client_config()
            };
            let connector: TlsConnector = TlsConnector::from(tls_cfg);
            let dns: ServerName<'static> = Self::server_name_for_host(&target_host)?;
            let proxy_box: BoxAsyncReadWrite = Box::new(tcp);
            let tunnel: ProxyTunnelStream = ProxyTunnelStream::new(proxy_box, Vec::new());
            let tls: TlsStream<ProxyTunnelStream> = connector
                .connect(dns, tunnel)
                .await
                .map_err(|e: io::Error| RequestError::Request(e.to_string()))?;
            if enable_h2 && tls.get_ref().1.alpn_protocol() != Some(ALPN_H2) {
                return Err(RequestError::Request(H2_ALPN_MISMATCH.to_owned()));
            }
            Ok(Box::new(tls))
        } else {
            Ok(Box::new(tcp))
        }
    }
}

/// Inherent implementation of [`HttpRequest`].
impl HttpRequest {
    /// Create a GET request for `url`.
    ///
    /// # Arguments
    ///
    /// - `T` - The request URL, converted into an owned `String`.
    ///
    /// # Returns
    ///
    /// - `Self` - A GET `HttpRequest` targeting `url`.
    pub fn get<T>(url: T) -> Self
    where
        T: Into<String>,
    {
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
    ///
    /// # Returns
    ///
    /// - `Self` - A POST `HttpRequest` targeting `url`.
    pub fn post<T>(url: T) -> Self
    where
        T: Into<String>,
    {
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
    pub fn set_url<T>(&mut self, url: T) -> &mut Self
    where
        T: Into<String>,
    {
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
    pub fn set_header<K, V>(&mut self, key: K, value: V) -> &mut Self
    where
        K: AsRef<str>,
        V: AsRef<str>,
    {
        let normalized: String = Self::normalize_header_key(key.as_ref());
        self.get_mut_headers()
            .insert(normalized, value.as_ref().to_owned());
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
    pub fn remove_header<K>(&mut self, key: K) -> &mut Self
    where
        K: AsRef<str>,
    {
        let normalized: String = Self::normalize_header_key(key.as_ref());
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
