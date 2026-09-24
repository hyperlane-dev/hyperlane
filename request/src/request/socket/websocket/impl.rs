use super::*;

impl WebSocket {
    fn get_url(&self) -> String {
        self.url.as_ref().clone()
    }

    fn generate_websocket_key() -> String {
        let mut key_bytes: [u8; 16] = [0u8; 16];
        let now: u64 = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos() as u64;
        let ptr: usize = &key_bytes as *const _ as usize;
        for (i, byte) in key_bytes.iter_mut().enumerate() {
            *byte = ((now.wrapping_add(ptr as u64).wrapping_add(i as u64)) % 256) as u8;
        }
        base64_encode(&key_bytes)
    }

    fn get_headers(&self) -> Vec<(String, String)> {
        let mut headers: Vec<(String, String)> = Vec::new();
        for (key, value) in self.header.iter() {
            if let Some(first_value) = value.front() {
                headers.push((key.clone(), first_value.clone()));
            }
        }
        headers
    }

    async fn connect_async_internal(&self) -> Result<(), WebSocketError> {
        if self.connected.load(Ordering::Relaxed) {
            return Ok(());
        }
        let url: String = self.get_url();
        if url.is_empty() {
            return Err(WebSocketError::invalid_url("URL is empty"));
        }
        let url_obj: HttpUrlComponents = SharedWebSocketBuilder::parse_url(&url)?;
        if let Ok(mut config) = self.config.write() {
            config.url_obj = url_obj;
        }
        let timeout_duration: Duration = Duration::from_millis(
            self.config
                .read()
                .map(|config| config.timeout)
                .unwrap_or(DEFAULT_HIGH_SECURITY_READ_TIMEOUT_MS),
        );
        let headers: Vec<(String, String)> = self.get_headers();
        let mut request_builder = Request::builder().uri(&url);
        for (key, value) in &headers {
            request_builder = request_builder.header(key, value);
        }
        let request: Request = request_builder.body(()).map_err(|error| {
            WebSocketError::invalid_url(format!("Failed to build request: {error}"))
        })?;
        let proxy_config: Option<ProxyConfig> = self
            .config
            .read()
            .ok()
            .and_then(|config| config.proxy.clone());
        let ws_stream: WebSocketConnectionType = if let Some(proxy_config) = proxy_config {
            let url_obj: HttpUrlComponents = self
                .config
                .read()
                .map(|config| config.url_obj.clone())
                .unwrap_or_default();
            let target_host: String = url_obj.host.clone().unwrap_or_default();
            let target_port: u16 = url_obj.port.unwrap_or_default();
            let proxy_stream: BoxAsyncReadWrite = self
                .get_proxy_connection_stream_async(target_host.clone(), target_port, &proxy_config)
                .await?;
            let proxy_tunnel_stream: WebSocketProxyTunnelStream =
                WebSocketProxyTunnelStream::new(proxy_stream);
            let mut proxy_request_builder = Request::builder().uri(&url);
            proxy_request_builder = proxy_request_builder
                .header(HOST, format!("{target_host}:{target_port}"))
                .header(UPGRADE, "websocket")
                .header(CONNECTION, "Upgrade")
                .header(SEC_WEBSOCKET_VERSION, "13")
                .header(SEC_WEBSOCKET_KEY, Self::generate_websocket_key());
            for (key, value) in &headers {
                proxy_request_builder = proxy_request_builder.header(key, value);
            }
            let protocols: Vec<String> = self
                .config
                .read()
                .map(|config| config.protocols.clone())
                .unwrap_or_default();
            if !protocols.is_empty() {
                proxy_request_builder =
                    proxy_request_builder.header("Sec-WebSocket-String", protocols.join(", "));
            }
            let proxy_request: Request = proxy_request_builder.body(()).map_err(|e| {
                WebSocketError::invalid_url(format!("Failed to build proxy request: {e}"))
            })?;
            let connect_future = client_async_with_config(proxy_request, proxy_tunnel_stream, None);
            let (ws_stream, _) = timeout(timeout_duration, connect_future)
                .await
                .map_err(|_| WebSocketError::timeout("Connection timeout"))?
                .map_err(|e| {
                    let error_msg: String = e.to_string();
                    if error_msg.contains("tls")
                        || error_msg.contains("TLS")
                        || error_msg.contains("ssl")
                        || error_msg.contains("SSL")
                        || error_msg.contains("certificate")
                        || error_msg.contains("handshake")
                    {
                        WebSocketError::tls(error_msg)
                    } else {
                        WebSocketError::connection(error_msg)
                    }
                })?;
            WebSocketConnectionType::Proxy(ws_stream)
        } else {
            let connect_future = connect_async_with_config(request, None, false);
            let (ws_stream, _) = timeout(timeout_duration, connect_future)
                .await
                .map_err(|_| WebSocketError::timeout("Connection timeout"))?
                .map_err(|e| {
                    let error_msg: String = e.to_string();
                    if error_msg.contains("tls")
                        || error_msg.contains("TLS")
                        || error_msg.contains("ssl")
                        || error_msg.contains("SSL")
                        || error_msg.contains("certificate")
                        || error_msg.contains("handshake")
                    {
                        WebSocketError::tls(error_msg)
                    } else {
                        WebSocketError::connection(error_msg)
                    }
                })?;
            WebSocketConnectionType::Direct(ws_stream)
        };
        let mut connection: http_type::tokio::sync::MutexGuard<
            '_,
            Option<WebSocketConnectionType>,
        > = self.connection.lock().await;
        *connection = Some(ws_stream);
        self.connected.store(true, Ordering::Relaxed);
        Ok(())
    }

    async fn send_message_async(&self, message: Message) -> Result<(), WebSocketError> {
        if !self.connected.load(Ordering::Relaxed) {
            self.connect_async_internal().await?;
        }
        let mut connection: http_type::tokio::sync::MutexGuard<
            '_,
            Option<WebSocketConnectionType>,
        > = self.connection.lock().await;
        if let Some(ref mut ws_stream) = *connection {
            ws_stream
                .send(message)
                .await
                .map_err(|error: tungstenite::Error| WebSocketError::protocol(error.to_string()))?;
        } else {
            return Err(WebSocketError::connection("Not connected"));
        }
        Ok(())
    }

    fn send_message_sync(&self, message: Message) -> Result<(), WebSocketError> {
        let rt: Runtime = Runtime::new()
            .map_err(|error: std::io::Error| WebSocketError::io(error.to_string()))?;
        rt.block_on(self.send_message_async(message))
    }

    async fn receive_message_async(&self) -> Result<WebSocketMessage, WebSocketError> {
        if !self.connected.load(Ordering::Relaxed) {
            return Err(WebSocketError::connection("Not connected"));
        }
        let timeout_duration: Duration = Duration::from_millis(
            self.config
                .read()
                .map(|config| config.timeout)
                .unwrap_or(DEFAULT_HIGH_SECURITY_READ_TIMEOUT_MS),
        );
        let mut connection: http_type::tokio::sync::MutexGuard<
            '_,
            Option<WebSocketConnectionType>,
        > = self.connection.lock().await;
        if let Some(ref mut ws_stream) = *connection {
            let receive_future = ws_stream.next();
            if let Some(msg_result) = timeout(timeout_duration, receive_future)
                .await
                .map_err(|_| WebSocketError::timeout("Receive timeout"))?
            {
                let message: Message = msg_result.map_err(|error: tungstenite::Error| {
                    WebSocketError::protocol(error.to_string())
                })?;
                return Ok(self.convert_message(message));
            }
        }
        Err(WebSocketError::connection("Connection closed"))
    }

    fn receive_message_sync(&self) -> Result<WebSocketMessage, WebSocketError> {
        let rt: Runtime = Runtime::new()
            .map_err(|error: std::io::Error| WebSocketError::io(error.to_string()))?;
        rt.block_on(self.receive_message_async())
    }

    fn convert_message(&self, message: Message) -> WebSocketMessage {
        match message {
            Message::Text(text) => WebSocketMessage::Text(text.to_string()),
            Message::Binary(data) => WebSocketMessage::Binary(data.to_vec()),
            Message::Ping(data) => WebSocketMessage::Ping(data.to_vec()),
            Message::Pong(data) => WebSocketMessage::Pong(data.to_vec()),
            Message::Close(_) => WebSocketMessage::Close,
            Message::Frame(_) => WebSocketMessage::Close,
        }
    }

    async fn close_async_internal(&self) -> Result<(), WebSocketError> {
        let mut connection: http_type::tokio::sync::MutexGuard<
            '_,
            Option<WebSocketConnectionType>,
        > = self.connection.lock().await;
        if let Some(ref mut ws_stream) = *connection {
            ws_stream
                .send(Message::Close(None))
                .await
                .map_err(|error: tungstenite::Error| WebSocketError::protocol(error.to_string()))?;
            use futures::SinkExt;
            ws_stream
                .close()
                .await
                .map_err(|error: tungstenite::Error| WebSocketError::protocol(error.to_string()))?;
        }
        *connection = None;
        self.connected.store(false, Ordering::Relaxed);
        Ok(())
    }

    fn close_sync(&self) -> Result<(), WebSocketError> {
        let rt: Runtime = Runtime::new()
            .map_err(|error: std::io::Error| WebSocketError::io(error.to_string()))?;
        rt.block_on(self.close_async_internal())
    }

    async fn get_proxy_connection_stream_async(
        &self,
        target_host: String,
        target_port: u16,
        proxy_config: &ProxyConfig,
    ) -> Result<BoxAsyncReadWrite, WebSocketError> {
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

    async fn get_http_proxy_connection_async(
        &self,
        target_host: String,
        target_port: u16,
        proxy_config: &ProxyConfig,
    ) -> Result<BoxAsyncReadWrite, WebSocketError> {
        let proxy_host_port: (String, u16) = (proxy_config.host.clone(), proxy_config.port);
        let tcp_stream: http_type::tokio::net::TcpStream =
            http_type::tokio::net::TcpStream::connect(proxy_host_port)
                .await
                .map_err(|err| WebSocketError::connection(err.to_string()))?;
        let mut proxy_stream: BoxAsyncReadWrite = if proxy_config.proxy_type == ProxyType::Https {
            let roots: RootCertStore = RootCertStore {
                roots: TLS_SERVER_ROOTS.to_vec(),
            };
            let tls_config: ClientConfig = ClientConfig::builder()
                .with_root_certificates(roots)
                .with_no_client_auth();
            let connector: TlsConnector = TlsConnector::from(Arc::new(tls_config));
            let dns_name: ServerName<'_> = ServerName::try_from(proxy_config.host.clone())
                .map_err(|err| WebSocketError::tls(err.to_string()))?;
            let tls_stream: TlsStream<http_type::tokio::net::TcpStream> = connector
                .connect(dns_name, tcp_stream)
                .await
                .map_err(|err| WebSocketError::tls(err.to_string()))?;
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
            .map_err(|err: std::io::Error| WebSocketError::protocol(err.to_string()))?;
        proxy_stream
            .flush()
            .await
            .map_err(|err: std::io::Error| WebSocketError::protocol(err.to_string()))?;
        let mut response_buffer: [u8; 1024] = [0u8; 1024];
        let bytes_read: usize = proxy_stream
            .read(&mut response_buffer)
            .await
            .map_err(|err: std::io::Error| WebSocketError::protocol(err.to_string()))?;
        let response: Cow<'_, str> = String::from_utf8_lossy(&response_buffer[..bytes_read]);
        if !response.starts_with("HTTP/1.1 200") && !response.starts_with("HTTP/1.0 200") {
            return Err(WebSocketError::connection(format!(
                "Proxy connection failed: {}",
                response.lines().next().unwrap_or("Unknown error")
            )));
        }
        Ok(proxy_stream)
    }

    async fn get_socks5_proxy_connection_async(
        &self,
        target_host: String,
        target_port: u16,
        proxy_config: &ProxyConfig,
    ) -> Result<BoxAsyncReadWrite, WebSocketError> {
        let proxy_host_port: (String, u16) = (proxy_config.host.clone(), proxy_config.port);
        let mut tcp_stream: http_type::tokio::net::TcpStream =
            http_type::tokio::net::TcpStream::connect(proxy_host_port)
                .await
                .map_err(|err| WebSocketError::connection(err.to_string()))?;
        let auth_methods: Vec<u8> =
            if proxy_config.username.is_some() && proxy_config.password.is_some() {
                vec![0x05, 0x02, 0x00, 0x02]
            } else {
                vec![0x05, 0x01, 0x00]
            };
        tcp_stream
            .write_all(&auth_methods)
            .await
            .map_err(|err: std::io::Error| WebSocketError::protocol(err.to_string()))?;
        let mut response: [u8; 2] = [0u8; 2];
        tcp_stream
            .read_exact(&mut response)
            .await
            .map_err(|err: std::io::Error| WebSocketError::protocol(err.to_string()))?;
        if response[0] != 0x05 {
            return Err(WebSocketError::protocol("Invalid SOCKS5 response"));
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

                    tcp_stream
                        .write_all(&auth_request)
                        .await
                        .map_err(|err: std::io::Error| WebSocketError::protocol(err.to_string()))?;

                    let mut auth_response = [0u8; 2];
                    tcp_stream
                        .read_exact(&mut auth_response)
                        .await
                        .map_err(|err: std::io::Error| WebSocketError::protocol(err.to_string()))?;

                    if auth_response[1] != 0x00 {
                        return Err(WebSocketError::protocol("SOCKS5 authentication failed"));
                    }
                } else {
                    return Err(WebSocketError::protocol(
                        "SOCKS5 proxy requires authentication",
                    ));
                }
            }
            0xFF => {
                return Err(WebSocketError::protocol(
                    "No acceptable SOCKS5 authentication methods",
                ));
            }
            _ => {
                return Err(WebSocketError::protocol(
                    "Unsupported SOCKS5 authentication method",
                ));
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
            .map_err(|err: std::io::Error| WebSocketError::protocol(err.to_string()))?;

        let mut connect_response: [u8; 4] = [0u8; 4];
        tcp_stream
            .read_exact(&mut connect_response)
            .await
            .map_err(|err: std::io::Error| WebSocketError::protocol(err.to_string()))?;

        if connect_response[0] != 0x05 || connect_response[1] != 0x00 {
            return Err(WebSocketError::protocol(format!(
                "SOCKS5 connection failed with code: {}",
                connect_response[1]
            )));
        }
        match connect_response[3] {
            0x01 => {
                let mut skip: [u8; 6] = [0u8; 6];
                tcp_stream
                    .read_exact(&mut skip)
                    .await
                    .map_err(|err: std::io::Error| WebSocketError::protocol(err.to_string()))?;
            }
            0x03 => {
                let mut len: [u8; 1] = [0u8; 1];
                tcp_stream
                    .read_exact(&mut len)
                    .await
                    .map_err(|err: std::io::Error| WebSocketError::protocol(err.to_string()))?;
                let mut skip: Vec<u8> = vec![0u8; len[0] as usize + 2];
                tcp_stream
                    .read_exact(&mut skip)
                    .await
                    .map_err(|err: std::io::Error| WebSocketError::protocol(err.to_string()))?;
            }
            0x04 => {
                let mut skip: [u8; 18] = [0u8; 18];
                tcp_stream
                    .read_exact(&mut skip)
                    .await
                    .map_err(|err: std::io::Error| WebSocketError::protocol(err.to_string()))?;
            }
            _ => {
                return Err(WebSocketError::protocol("Invalid SOCKS5 address type"));
            }
        }
        let proxy_stream: BoxAsyncReadWrite = Box::new(tcp_stream);
        Ok(proxy_stream)
    }

    /// Sends a text message synchronously.
    ///
    /// # Arguments
    ///
    /// - `&str` - The text message to send.
    ///
    /// # Returns
    ///
    /// - `WebSocketResult` - Result indicating success or failure.
    pub fn send_text(&mut self, text: &str) -> WebSocketResult {
        let message: Message = Message::Text(text.into());
        self.send_message_sync(message)
    }

    /// Sends a binary message synchronously.
    ///
    /// # Arguments
    ///
    /// - `&[u8]` - The binary data to send.
    ///
    /// # Returns
    ///
    /// - `WebSocketResult` - Result indicating success or failure.
    pub fn send_binary(&mut self, data: &[u8]) -> WebSocketResult {
        let message: Message = Message::Binary(data.to_vec().into());
        self.send_message_sync(message)
    }

    /// Sends a ping message synchronously.
    ///
    /// # Arguments
    ///
    /// - `&[u8]` - The ping data to send.
    ///
    /// # Returns
    ///
    /// - `WebSocketResult` - Result indicating success or failure.
    pub fn send_ping(&mut self, data: &[u8]) -> WebSocketResult {
        let message: Message = Message::Ping(data.to_vec().into());
        self.send_message_sync(message)
    }

    /// Sends a pong message synchronously.
    ///
    /// # Arguments
    ///
    /// - `&[u8]` - The pong data to send.
    ///
    /// # Returns
    ///
    /// - `WebSocketResult` - Result indicating success or failure.
    pub fn send_pong(&mut self, data: &[u8]) -> WebSocketResult {
        let message: Message = Message::Pong(data.to_vec().into());
        self.send_message_sync(message)
    }

    /// Receives a message synchronously.
    ///
    /// # Returns
    ///
    /// - `WebSocketMessageResult` - Result containing the received message or error.
    pub fn receive(&mut self) -> WebSocketMessageResult {
        self.receive_message_sync()
    }

    /// Closes the WebSocket connection synchronously.
    ///
    /// # Returns
    ///
    /// - `WebSocketResult` - Result indicating success or failure.
    pub fn close(&mut self) -> WebSocketResult {
        self.close_sync()
    }

    /// Checks if the WebSocket is currently connected.
    ///
    /// # Returns
    ///
    /// - `bool` - True if connected, false otherwise.
    pub fn is_connected(&self) -> bool {
        self.connected.load(Ordering::Relaxed)
    }

    /// Sends a text message asynchronously.
    ///
    /// # Arguments
    ///
    /// - `&str` - The text message to send.
    ///
    /// # Returns
    ///
    /// - `WebSocketResult` - Result indicating success or failure.
    pub async fn send_text_async(&mut self, text: &str) -> WebSocketResult {
        let message: Message = Message::Text(text.into());
        self.send_message_async(message).await
    }

    /// Sends a binary message asynchronously.
    ///
    /// # Arguments
    ///
    /// - `&[u8]` - The binary data to send.
    ///
    /// # Returns
    ///
    /// - `WebSocketResult` - Result indicating success or failure.
    pub async fn send_binary_async(&mut self, data: &[u8]) -> WebSocketResult {
        let message: Message = Message::Binary(data.to_vec().into());
        self.send_message_async(message).await
    }

    /// Sends a ping message asynchronously.
    ///
    /// # Arguments
    ///
    /// - `&[u8]` - The ping data to send.
    ///
    /// # Returns
    ///
    /// - `WebSocketResult` - Result indicating success or failure.
    pub async fn send_ping_async(&mut self, data: &[u8]) -> WebSocketResult {
        let message: Message = Message::Ping(data.to_vec().into());
        self.send_message_async(message).await
    }

    /// Sends a pong message asynchronously.
    ///
    /// # Arguments
    ///
    /// - `&[u8]` - The pong data to send.
    ///
    /// # Returns
    ///
    /// - `WebSocketResult` - Result indicating success or failure.
    pub async fn send_pong_async(&mut self, data: &[u8]) -> WebSocketResult {
        let message: Message = Message::Pong(data.to_vec().into());
        self.send_message_async(message).await
    }

    /// Receives a message asynchronously.
    ///
    /// # Returns
    ///
    /// - `WebSocketMessageResult` - Result containing the received message or error.
    pub async fn receive_async(&mut self) -> WebSocketMessageResult {
        self.receive_message_async().await
    }

    /// Closes the WebSocket connection asynchronously.
    ///
    /// # Returns
    ///
    /// - `WebSocketResult` - Result indicating success or failure.
    pub async fn close_async_method(&mut self) -> WebSocketResult {
        self.close_async_internal().await
    }
}

/// Synchronous WebSocket trait implementation.
///
/// Provides synchronous methods for WebSocket operations including:
/// - Sending messages (text, binary, ping, pong)
/// - Receiving messages
/// - Closing connections
/// - Checking connection status
impl WebSocketTrait for WebSocket {
    fn send_text(&mut self, text: &str) -> WebSocketResult {
        self.send_text(text)
    }

    fn send_binary(&mut self, data: &[u8]) -> WebSocketResult {
        self.send_binary(data)
    }

    fn send_ping(&mut self, data: &[u8]) -> WebSocketResult {
        self.send_ping(data)
    }

    fn send_pong(&mut self, data: &[u8]) -> WebSocketResult {
        self.send_pong(data)
    }

    fn receive(&mut self) -> WebSocketMessageResult {
        self.receive()
    }

    fn close(&mut self) -> WebSocketResult {
        self.close()
    }

    fn is_connected(&self) -> bool {
        self.is_connected()
    }
}

/// Asynchronous WebSocket trait implementation.
///
/// Provides asynchronous methods for WebSocket operations including:
/// - Sending messages (text, binary, ping, pong)
/// - Receiving messages
/// - Closing connections
/// - Checking connection status
impl AsyncWebSocketTrait for WebSocket {
    fn send_text<'a>(
        &'a mut self,
        text: &'a str,
    ) -> Pin<Box<dyn Future<Output = WebSocketResult> + Send + 'a>> {
        Box::pin(self.send_text_async(text))
    }

    fn send_binary<'a>(
        &'a mut self,
        data: &'a [u8],
    ) -> Pin<Box<dyn Future<Output = WebSocketResult> + Send + 'a>> {
        Box::pin(self.send_binary_async(data))
    }

    fn send_ping<'a>(
        &'a mut self,
        data: &'a [u8],
    ) -> Pin<Box<dyn Future<Output = WebSocketResult> + Send + 'a>> {
        Box::pin(self.send_ping_async(data))
    }

    fn send_pong<'a>(
        &'a mut self,
        data: &'a [u8],
    ) -> Pin<Box<dyn Future<Output = WebSocketResult> + Send + 'a>> {
        Box::pin(self.send_pong_async(data))
    }

    fn receive(&mut self) -> Pin<Box<dyn Future<Output = WebSocketMessageResult> + Send + '_>> {
        Box::pin(self.receive_async())
    }

    fn close(&mut self) -> Pin<Box<dyn Future<Output = WebSocketResult> + Send + '_>> {
        Box::pin(self.close_async_method())
    }

    fn is_connected(&self) -> bool {
        self.is_connected()
    }
}
