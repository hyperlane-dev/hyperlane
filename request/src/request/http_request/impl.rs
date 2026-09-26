use super::*;

/// Blanket AsyncRead+AsyncWrite+Unpin+Send for any compatible stream.
pub(crate) trait AsyncReadWrite: AsyncRead + AsyncWrite + Unpin + Send {}
/// Blanket Read+Write for any compatible stream.
pub(crate) trait ReadWrite: Read + Write {}

impl<T: AsyncRead + AsyncWrite + Unpin + Send> AsyncReadWrite for T {}
impl<T: Read + Write> ReadWrite for T {}

/// Boxed dynamic async stream.
pub(crate) type BoxAsyncReadWrite = Box<dyn AsyncReadWrite>;
/// Boxed dynamic sync stream.
pub(crate) type BoxReadWrite = Box<dyn ReadWrite>;

impl HttpRequest {
    /// Send the HTTP request synchronously, returning the parsed response.
    pub fn send(&mut self) -> RequestResult {
        self.send_sync()
    }

    /// Send the HTTP request asynchronously, returning the parsed response.
    pub async fn send_async(&mut self) -> RequestResult {
        self.send_async_impl().await
    }

    /// Parse the configured URL into a [`HttpUrlComponents`].
    pub(crate) fn parse_url(&self) -> Result<HttpUrlComponents, RequestError> {
        HttpUrlComponents::parse(&self.url).map_err(|e| RequestError::Request(e.to_string()))
    }

    /// `Host` + path (including query string) for the request line.
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
    pub(crate) fn protocol_lower(config: &RequestConfig) -> String {
        config.http_version.to_string().to_ascii_lowercase()
    }

    /// Build the wire-format header bytes, with `Host`, `Content-Length`,
    /// `Accept`, `User-Agent` auto-filled if missing.
    pub(crate) fn header_bytes(&self, body_length: usize) -> Vec<u8> {
        let mut header: HashMap<String, String> = self.headers.clone();
        let host_value: String = self
            .parse_url()
            .ok()
            .and_then(|u| u.host.clone())
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
        let estimated_size: usize = header.iter().map(|(k, v)| k.len() + v.len() + 4).sum();
        let mut out: Vec<u8> = Vec::with_capacity(estimated_size);
        for (key, value) in &header {
            out.extend_from_slice(key.as_bytes());
            out.extend_from_slice(b": ");
            out.extend_from_slice(value.as_bytes());
            out.extend_from_slice(HTTP_BR_BYTES);
        }
        out
    }

    fn header_has_key(header: &HashMap<String, String>, target_key: &str) -> bool {
        let target = target_key.to_ascii_lowercase();
        header.keys().any(|k| k == &target)
    }

    /// Encode `self.body` according to the `Content-Type` header.
    /// Returns empty bytes if no recognised content type.
    pub(crate) fn body_bytes(&self) -> Vec<u8> {
        let ct = self
            .headers
            .iter()
            .find(|(k, _)| k.eq_ignore_ascii_case(CONTENT_TYPE))
            .map(|(_, v)| v.clone());
        match ct {
            Some(value) => value
                .to_lowercase()
                .parse::<ContentType>()
                .unwrap_or_default()
                .get_body_string(&String::from_utf8_lossy(&self.body.bytes))
                .into_bytes(),
            None => Vec::new(),
        }
    }

    /// Synchronous send.
    pub(crate) fn send_sync(&mut self) -> RequestResult {
        let url_obj = self.parse_url()?;
        let host: String = url_obj.host.clone().unwrap_or_default();
        let port: u16 = self.resolve_port(url_obj.port.unwrap_or_default());
        let mut stream: BoxReadWrite = self.open_sync_stream(host, port)?;
        let method = self.method.clone();
        if method.is_get() {
            self.send_get_request_sync(&mut stream)
        } else if method.is_post() {
            self.send_post_request_sync(&mut stream)
        } else {
            Err(RequestError::Request("Method Not Allowed".to_string()))
        }
    }

    /// Asynchronous send.
    async fn send_async_impl(&mut self) -> RequestResult {
        let url_obj = self.parse_url()?;
        let host: String = url_obj.host.clone().unwrap_or_default();
        let port: u16 = self.resolve_port(url_obj.port.unwrap_or_default());
        let mut stream: BoxAsyncReadWrite = self.open_async_stream(host, port).await?;
        let method = self.method.clone();
        if method.is_get() {
            self.send_get_request_async(&mut stream).await
        } else if method.is_post() {
            self.send_post_request_async(&mut stream).await
        } else {
            Err(RequestError::Request("Method Not Allowed".to_string()))
        }
    }

    fn resolve_port(&self, port: u16) -> u16 {
        if port != 0 {
            return port;
        }
        let protocol = Self::protocol_lower(&self.config);
        Protocol::get_port(&protocol)
    }

    fn is_https(&self) -> bool {
        Self::protocol_lower(&self.config) == HTTPS_LOWERCASE
    }

    // ---------- sync stream plumbing ----------

    fn open_sync_stream(&self, host: String, port: u16) -> Result<BoxReadWrite, RequestError> {
        if let Some(proxy) = &self.config.proxy {
            return self.open_sync_proxy_stream(host, port, proxy);
        }
        let timeout = Duration::from_millis(self.config.timeout);
        let tcp = TcpStream::connect((host.clone(), port))
            .map_err(|e: std::io::Error| RequestError::Request(e.to_string()))?;
        tcp.set_read_timeout(Some(timeout))
            .map_err(|e: std::io::Error| RequestError::Request(e.to_string()))?;
        tcp.set_write_timeout(Some(timeout))
            .map_err(|e: std::io::Error| RequestError::Request(e.to_string()))?;
        if self.is_https() {
            let roots = self.tmp.root_cert.clone();
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

    fn send_get_request_sync(
        &mut self,
        stream: &mut BoxReadWrite,
    ) -> Result<HttpResponse, RequestError> {
        let path = self.full_path();
        let header_bytes = self.header_bytes(0);
        let version = self.config.http_version.to_string();
        let request = build_http_request("GET", path, header_bytes, None, version);
        stream
            .write_all(&request)
            .and_then(|_| stream.flush())
            .map_err(|e: std::io::Error| RequestError::Request(e.to_string()))?;
        self.read_response_sync(stream)
    }

    fn send_post_request_sync(
        &mut self,
        stream: &mut BoxReadWrite,
    ) -> Result<HttpResponse, RequestError> {
        let body_bytes = self.body_bytes();
        let path = self.full_path();
        let header_bytes = self.header_bytes(body_bytes.len());
        let version = self.config.http_version.to_string();
        let request = build_http_request("POST", path, header_bytes, Some(body_bytes), version);
        stream
            .write_all(&request)
            .and_then(|_| stream.flush())
            .map_err(|e: std::io::Error| RequestError::Request(e.to_string()))?;
        self.read_response_sync(stream)
    }

    fn read_response_sync(
        &mut self,
        stream: &mut BoxReadWrite,
    ) -> Result<HttpResponse, RequestError> {
        let buffer_size = self.config.buffer_size;
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
        if !self.config.redirect || redirect_url.is_none() {
            if self.config.decode {
                response = response.decode(self.config.buffer_size);
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

    fn handle_redirect(&mut self, url: String) -> Result<HttpResponse, RequestError> {
        if !self.config.redirect {
            return Err(RequestError::Request("Redirect Not Enabled".to_string()));
        }
        if self.tmp.visit_url.contains(&url) {
            return Err(RequestError::Request("Redirect URL Dead Loop".to_string()));
        }
        self.tmp.visit_url.insert(url.clone());
        if self.config.max_redirect_times == 0 {
            return Err(RequestError::Request(
                "Max Redirect Times Exceeded".to_string(),
            ));
        }
        self.config.max_redirect_times -= 1;
        self.url = url;
        self.send_sync()
    }

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
            let chunk_size_str: &[u8] = match raw.iter().position(|&b| b == b';') {
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

    async fn open_async_stream(
        &self,
        host: String,
        port: u16,
    ) -> Result<BoxAsyncReadWrite, RequestError> {
        if let Some(proxy) = &self.config.proxy {
            return self.open_async_proxy_stream(host, port, proxy).await;
        }
        let tcp = http_type::tokio::net::TcpStream::connect((host.clone(), port))
            .await
            .map_err(|e: std::io::Error| RequestError::Request(e.to_string()))?;
        if self.is_https() {
            let roots = self.tmp.root_cert.clone();
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

    async fn send_get_request_async(
        &mut self,
        stream: &mut BoxAsyncReadWrite,
    ) -> Result<HttpResponse, RequestError> {
        let path = self.full_path();
        let header_bytes = self.header_bytes(0);
        let version = self.config.http_version.to_string();
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

    async fn send_post_request_async(
        &mut self,
        stream: &mut BoxAsyncReadWrite,
    ) -> Result<HttpResponse, RequestError> {
        let body_bytes = self.body_bytes();
        let path = self.full_path();
        let header_bytes = self.header_bytes(body_bytes.len());
        let version = self.config.http_version.to_string();
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

    async fn read_response_async(
        &mut self,
        stream: &mut BoxAsyncReadWrite,
    ) -> Result<HttpResponse, RequestError> {
        let buffer_size = self.config.buffer_size;
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
        if !self.config.redirect || redirect_url.is_none() {
            if self.config.decode {
                response = response.decode(self.config.buffer_size);
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

    async fn handle_redirect_async(&mut self, url: String) -> Result<HttpResponse, RequestError> {
        if !self.config.redirect {
            return Err(RequestError::Request("Redirect Not Enabled".to_string()));
        }
        if self.tmp.visit_url.contains(&url) {
            return Err(RequestError::Request("Redirect URL Dead Loop".to_string()));
        }
        self.tmp.visit_url.insert(url.clone());
        if self.config.max_redirect_times == 0 {
            return Err(RequestError::Request(
                "Max Redirect Times Exceeded".to_string(),
            ));
        }
        self.config.max_redirect_times -= 1;
        self.url = url;
        Box::pin(self.send_async()).await
    }

    // ---------- proxy: sync ----------

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

    fn open_sync_http_proxy(
        &self,
        target_host: String,
        target_port: u16,
        proxy: &Proxy,
    ) -> Result<BoxReadWrite, RequestError> {
        let timeout = Duration::from_millis(self.config.timeout);
        let tcp = TcpStream::connect((proxy.host.clone(), proxy.port))
            .map_err(|e: std::io::Error| RequestError::Request(e.to_string()))?;
        tcp.set_read_timeout(Some(timeout))
            .map_err(|e: std::io::Error| RequestError::Request(e.to_string()))?;
        tcp.set_write_timeout(Some(timeout))
            .map_err(|e: std::io::Error| RequestError::Request(e.to_string()))?;
        let mut proxy_stream: BoxReadWrite = if proxy.proxy_type == ProxyType::Https {
            let roots = self.tmp.root_cert.clone();
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
            let roots = self.tmp.root_cert.clone();
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

    fn open_sync_socks5_proxy(
        &self,
        target_host: String,
        target_port: u16,
        proxy: &Proxy,
    ) -> Result<BoxReadWrite, RequestError> {
        let timeout = Duration::from_millis(self.config.timeout);
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
            let roots = self.tmp.root_cert.clone();
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
            let roots = self.tmp.root_cert.clone();
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
            let roots = self.tmp.root_cert.clone();
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
            let roots = self.tmp.root_cert.clone();
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
