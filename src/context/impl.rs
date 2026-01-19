use crate::*;

/// Implementation of `From` trait for `Context`.
impl From<ContextInner> for Context {
    /// Converts a `ContextInner` into a `Context`.
    ///
    /// # Arguments
    ///
    /// - `ContextInner` - The wrapped context data.
    ///
    /// # Returns
    ///
    /// - `Context` - The newly created context instance.
    #[inline(always)]
    fn from(ctx: ContextInner) -> Self {
        Self(arc_rwlock(ctx))
    }
}

/// Implementation of `From` trait for converting `&ArcRwLockStream` into `Context`.
impl From<&ArcRwLockStream> for Context {
    /// Converts a reference to a network stream into a `Context` with default request.
    ///
    /// # Arguments
    ///
    /// - `&ArcRwLockStream` - The network stream reference to convert.
    ///
    /// # Returns
    ///
    /// - `Context` - The newly created context instance.
    #[inline(always)]
    fn from(stream: &ArcRwLockStream) -> Self {
        let request: Request = Request::default();
        let mut internal_ctx: ContextInner = ContextInner::default();
        internal_ctx
            .set_stream(Some(stream.clone()))
            .set_request(request.clone())
            .get_mut_response()
            .set_version(request.get_version().clone());
        internal_ctx.into()
    }
}

/// Implementation of `From` trait for converting `ArcRwLockStream` into `Context`.
impl From<ArcRwLockStream> for Context {
    /// Converts a network stream into a `Context` with default request.
    ///
    /// # Arguments
    ///
    /// - `ArcRwLockStream` - The network stream to convert.
    ///
    /// # Returns
    ///
    /// - `Context` - The newly created context instance.
    #[inline(always)]
    fn from(stream: ArcRwLockStream) -> Self {
        (&stream).into()
    }
}

/// Implementation of methods for `Context` structure.
impl Context {
    /// Creates a new `Context` with the provided network stream and HTTP request.
    ///
    /// # Arguments
    ///
    /// - `&ArcRwLockStream` - The network stream.
    /// - `&Request` - The HTTP request.
    ///
    /// # Returns
    ///
    /// - `Context` - The newly created context.
    #[inline(always)]
    pub(crate) fn new(stream: &ArcRwLockStream, request: &Request) -> Context {
        let mut internal_ctx: ContextInner = ContextInner::default();
        internal_ctx
            .set_stream(Some(stream.clone()))
            .set_request(request.clone())
            .get_mut_response()
            .set_version(request.get_version().clone());
        internal_ctx.into()
    }

    /// Acquires a read lock on the inner context data.
    ///
    /// # Returns
    ///
    /// - `ContextReadGuard` - The read guard for the inner context.
    #[inline(always)]
    fn read(&self) -> ContextReadGuard<'_> {
        self.get_0().try_read().unwrap()
    }

    /// Acquires a write lock on the inner context data.
    ///
    /// # Returns
    ///
    /// - `ContextWriteGuard` - The write guard for the inner context.
    #[inline(always)]
    fn write(&self) -> ContextWriteGuard<'_> {
        self.get_0().try_write().unwrap()
    }

    /// Reads an HTTP request from the underlying stream.
    ///
    /// # Arguments
    ///
    /// - `RequestConfig` - The request config.
    ///
    /// # Returns
    ///
    /// - `Result<Request, RequestError>` - The parsed request or error.
    pub async fn http_from_stream(
        &self,
        request_config: RequestConfig,
    ) -> Result<Request, RequestError> {
        if self.get_aborted() {
            return Err(RequestError::RequestAborted(HttpStatus::BadRequest));
        }
        if let Some(stream) = self.try_get_stream().as_ref() {
            let request_res: Result<Request, RequestError> =
                Request::http_from_stream(stream, &request_config).await;
            if let Ok(request) = request_res.as_ref() {
                self.set_request(request);
            }
            return request_res;
        };
        Err(RequestError::GetTcpStream(HttpStatus::BadRequest))
    }

    /// Reads a WebSocket frame from the underlying stream.
    ///
    /// # Arguments
    ///
    /// - `RequestConfig` - The request config.
    ///
    /// # Returns
    ///
    /// - `Result<Request, RequestError>` - The parsed frame or error.
    pub async fn ws_from_stream(
        &self,
        request_config: RequestConfig,
    ) -> Result<Request, RequestError> {
        if self.get_aborted() {
            return Err(RequestError::RequestAborted(HttpStatus::BadRequest));
        }
        if let Some(stream) = self.try_get_stream().as_ref() {
            let mut last_request: Request = self.get_request();
            let request_res: Result<Request, RequestError> =
                last_request.ws_from_stream(stream, &request_config).await;
            match request_res.as_ref() {
                Ok(request) => {
                    self.set_request(request);
                }
                Err(_) => {
                    self.set_request(&last_request);
                }
            }
            return request_res;
        };
        Err(RequestError::GetTcpStream(HttpStatus::BadRequest))
    }

    /// Checks if the context has been marked as aborted.
    ///
    /// # Returns
    ///
    /// - `bool` - True if the context is aborted, otherwise false.
    #[inline(always)]
    pub fn get_aborted(&self) -> bool {
        self.read().get_aborted()
    }

    /// Sets the aborted flag for the context.
    ///
    /// # Arguments
    ///
    /// - `bool` - The aborted state to set.
    ///
    /// # Returns
    ///
    /// - `&Self` - Reference to self for method chaining.
    #[inline(always)]
    pub fn set_aborted(&self, aborted: bool) -> &Self {
        self.write().set_aborted(aborted);
        self
    }

    /// Marks the context as aborted.
    ///
    /// # Returns
    ///
    /// - `&Self` - Reference to the modified context.
    #[inline(always)]
    pub fn aborted(&self) -> &Self {
        self.set_aborted(true);
        self
    }

    /// Cancels the aborted state of the context.
    ///
    /// # Returns
    ///
    /// - `&Self` - Reference to the modified context.
    #[inline(always)]
    pub fn cancel_aborted(&self) -> &Self {
        self.set_aborted(false);
        self
    }

    /// Checks if the connection is marked as closed.
    ///
    /// # Returns
    ///
    /// - `bool` - True if the connection is closed, otherwise false.
    #[inline(always)]
    pub fn get_closed(&self) -> bool {
        self.read().get_closed()
    }

    /// Sets the closed flag for the connection.
    ///
    /// # Arguments
    ///
    /// - `bool` - The new value for the closed flag.
    ///
    /// # Returns
    ///
    /// - `&Self` - Reference to the modified context.
    #[inline(always)]
    pub fn set_closed(&self, closed: bool) -> &Self {
        self.write().set_closed(closed);
        self
    }

    /// Marks the connection as closed.
    ///
    /// # Returns
    ///
    /// - `&Self` - Reference to the modified context.
    #[inline(always)]
    pub fn closed(&self) -> &Self {
        self.set_closed(true);
        self
    }

    /// Cancels the closed state of the connection.
    ///
    /// # Returns
    ///
    /// - `&Self` - Reference to the modified context.
    #[inline(always)]
    pub fn cancel_closed(&self) -> &Self {
        self.set_closed(false);
        self
    }

    /// Checks if the connection has been terminated (aborted or closed).
    ///
    /// # Returns
    ///
    /// - `bool` - True if the connection is either aborted or closed, otherwise false.
    #[inline(always)]
    pub fn is_terminated(&self) -> bool {
        self.get_aborted() || self.get_closed()
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

    /// Retrieves the underlying network stream, if available.
    ///
    /// # Returns
    ///
    /// - `Option<ArcRwLockStream>` - The thread-safe, shareable network stream if it exists.
    #[inline(always)]
    pub fn try_get_stream(&self) -> Option<ArcRwLockStream> {
        self.read().try_get_stream().clone()
    }

    /// Retrieves the underlying network stream.
    ///
    /// # Returns
    ///
    /// - `ArcRwLockStream` - The thread-safe, shareable network stream.
    ///
    /// # Panics
    ///
    /// - If the network stream is not found.
    #[inline(always)]
    pub fn get_stream(&self) -> ArcRwLockStream {
        self.try_get_stream().unwrap()
    }

    /// Retrieves the remote socket address of the connection.
    ///
    /// # Returns
    ///
    /// - `Option<SocketAddr>` - The socket address of the remote peer if available.
    pub async fn try_get_socket_addr(&self) -> Option<SocketAddr> {
        self.try_get_stream()
            .as_ref()?
            .read()
            .await
            .peer_addr()
            .ok()
    }

    /// Retrieves the remote socket address.
    ///
    /// # Returns
    ///
    /// - `SocketAddr` - The socket address of the remote peer.
    ///
    /// # Panics
    ///
    /// - If the socket address is not found.
    pub async fn get_socket_addr(&self) -> SocketAddr {
        self.try_get_socket_addr().await.unwrap()
    }

    /// Retrieves the remote socket address as a string.
    ///
    /// # Returns
    ///
    /// - `Option<String>` - The string representation of the socket address if available.
    pub async fn try_get_socket_addr_string(&self) -> Option<String> {
        self.try_get_socket_addr()
            .await
            .map(|data| data.to_string())
    }

    /// Retrieves the remote socket address as a string.
    ///
    /// # Returns
    ///
    /// - `String` - The string representation of the socket address.
    ///
    /// # Panics
    ///
    /// - If the socket address is not found.
    pub async fn get_socket_addr_string(&self) -> String {
        self.get_socket_addr().await.to_string()
    }

    /// Retrieves the IP address part of the remote socket address.
    ///
    /// # Returns
    ///
    /// - `Option<SocketHost>` - The IP address of the remote peer.
    pub async fn try_get_socket_host(&self) -> Option<SocketHost> {
        self.try_get_socket_addr()
            .await
            .map(|socket_addr: SocketAddr| socket_addr.ip())
    }

    /// Retrieves the IP address part of the remote socket address.
    ///
    /// # Returns
    ///
    /// - `SocketHost` - The IP address of the remote peer.
    ///
    /// # Panics
    ///
    /// - If the socket address is not found.
    pub async fn get_socket_host(&self) -> SocketHost {
        self.try_get_socket_host().await.unwrap()
    }

    /// Retrieves the port number part of the remote socket address.
    ///
    /// # Returns
    ///
    /// - `Option<SocketPort>` - The port number of the remote peer if available.
    pub async fn try_get_socket_port(&self) -> Option<SocketPort> {
        self.try_get_socket_addr()
            .await
            .map(|socket_addr: SocketAddr| socket_addr.port())
    }

    /// Retrieves the port number part of the remote socket address.
    ///
    /// # Returns
    ///
    /// - `SocketPort` - The port number of the remote peer.
    ///
    /// # Panics
    ///
    /// - If the socket address is not found.
    pub async fn get_socket_port(&self) -> SocketPort {
        self.try_get_socket_port().await.unwrap()
    }

    /// Retrieves the current HTTP request.
    ///
    /// # Returns
    ///
    /// - `Request` - A clone of the current request.
    #[inline(always)]
    pub fn get_request(&self) -> Request {
        self.read().get_request().clone()
    }

    /// Sets the current HTTP request for the context.
    ///
    /// # Arguments
    ///
    /// - `&Request` - The request to set in the context.
    ///
    /// # Returns
    ///
    /// - `&Self` - Reference to the modified context.
    #[inline(always)]
    pub(crate) fn set_request(&self, request_data: &Request) -> &Self {
        self.write().set_request(request_data.clone());
        self
    }

    /// Executes an asynchronous closure with the current request.
    ///
    /// This method provides temporary access to the request data without needing to clone it.
    ///
    /// # Arguments
    ///
    /// - `F` - A closure that takes the `Request` and returns a future.
    ///
    /// # Returns
    ///
    /// - `R` - The result of the provided closure's future.
    pub async fn with_request<F, Fut, R>(&self, func: F) -> R
    where
        F: Fn(Request) -> Fut,
        Fut: FutureSendStatic<R>,
    {
        func(self.read().get_request().clone()).await
    }

    /// Retrieves the string representation of the current request.
    ///
    /// # Returns
    ///
    /// - `String` - The full request as a string.
    #[inline(always)]
    pub fn get_request_string(&self) -> String {
        self.read().get_request().get_string()
    }

    /// Retrieves the HTTP version of the request.
    ///
    /// # Returns
    ///
    /// - `RequestVersion` - The HTTP version of the request.
    #[inline(always)]
    pub fn get_request_version(&self) -> RequestVersion {
        self.read().get_request().get_version().clone()
    }

    /// Retrieves the HTTP method of the request.
    ///
    /// # Returns
    ///
    /// - `RequestMethod` - The HTTP method of the request.
    #[inline(always)]
    pub fn get_request_method(&self) -> RequestMethod {
        self.read().get_request().get_method().clone()
    }

    /// Retrieves the host from the request headers.
    ///
    /// # Returns
    ///
    /// - `RequestHost` - The host part of the request's URI.
    #[inline(always)]
    pub fn get_request_host(&self) -> RequestHost {
        self.read().get_request().get_host().clone()
    }

    /// Retrieves the path of the request.
    ///
    /// # Returns
    ///
    /// - `RequestPath` - The path part of the request's URI.
    #[inline(always)]
    pub fn get_request_path(&self) -> RequestPath {
        self.read().get_request().get_path().clone()
    }

    /// Retrieves the query parameters of the request.
    ///
    /// # Returns
    ///
    /// - `RequestQuerys` - A map containing the query parameters.
    #[inline(always)]
    pub fn get_request_querys(&self) -> RequestQuerys {
        self.read().get_request().get_querys().clone()
    }

    /// Attempts to retrieve a specific query parameter by its key.
    ///
    /// # Arguments
    ///
    /// - `AsRef<str>` - The query parameter key.
    ///
    /// # Returns
    ///
    /// - `Option<RequestQuerysValue>` - The query parameter value if exists.
    #[inline(always)]
    pub fn try_get_request_query<K>(&self, key: K) -> Option<RequestQuerysValue>
    where
        K: AsRef<str>,
    {
        self.read().get_request().try_get_query(key)
    }

    /// Retrieves a specific query parameter by its key, panicking if not found.
    ///
    /// # Arguments
    ///
    /// - `AsRef<str>` - The query parameter key.
    ///
    /// # Returns
    ///
    /// - `RequestQuerysValue` - The query parameter value if exists.
    ///
    /// # Panics
    ///
    /// - If the query parameter is not found.
    #[inline(always)]
    pub fn get_request_query<K>(&self, key: K) -> RequestQuerysValue
    where
        K: AsRef<str>,
    {
        self.read().get_request().get_query(key)
    }

    /// Retrieves the body of the request.
    ///
    /// # Returns
    ///
    /// - `RequestBody` - A clone of the request's body.
    #[inline(always)]
    pub fn get_request_body(&self) -> RequestBody {
        self.read().get_request().get_body().clone()
    }

    /// Retrieves the request body as a string.
    ///
    /// # Returns
    ///
    /// - `String` - The request body converted to a string.
    #[inline(always)]
    pub fn get_request_body_string(&self) -> String {
        self.read().get_request().get_body_string()
    }

    /// Deserializes the request body from JSON into a specified type.
    ///
    /// # Returns
    ///
    /// - `Result<J, serde_json::Error>` - The deserialized type `J` or a JSON error.
    pub fn try_get_request_body_json<J>(&self) -> Result<J, serde_json::Error>
    where
        J: DeserializeOwned,
    {
        self.read().get_request().try_get_body_json()
    }

    /// Deserializes the request body from JSON into a specified type, panicking if not found.
    ///
    /// # Returns
    ///
    /// - `J` - The deserialized type `J`.
    ///
    /// # Panics
    ///
    /// - If deserialization fails.
    pub fn get_request_body_json<J>(&self) -> J
    where
        J: DeserializeOwned,
    {
        self.read().get_request().get_body_json()
    }

    /// Retrieves all request headers.
    ///
    /// # Returns
    ///
    /// - `RequestHeaders` - A clone of the request's header map.
    #[inline(always)]
    pub fn get_request_headers(&self) -> RequestHeaders {
        self.read().get_request().get_headers().clone()
    }

    /// Retrieves the total number of request headers.
    ///
    /// # Returns
    ///
    /// - `usize` - The total number of headers in the request.
    #[inline(always)]
    pub fn get_request_headers_length(&self) -> usize {
        self.read().get_request().get_headers_length()
    }

    /// Attempts to retrieve a specific request header by its key.
    ///
    /// # Arguments
    ///
    /// - `AsRef<str>` - The header key.
    ///
    /// # Returns
    ///
    /// - `Option<RequestHeadersValue>` - The header values if exists.
    #[inline(always)]
    pub fn try_get_request_header<K>(&self, key: K) -> Option<RequestHeadersValue>
    where
        K: AsRef<str>,
    {
        self.read().get_request().try_get_header(key)
    }

    /// Retrieves a specific request header by its key, panicking if not found.
    ///
    /// # Arguments
    ///
    /// - `AsRef<str>` - The key of the header to retrieve.
    ///
    /// # Returns
    ///
    /// - `RequestHeadersValue` - The header values if exists.
    ///
    /// # Panics
    ///
    /// - If the header is not found.
    #[inline(always)]
    pub fn get_request_header<K>(&self, key: K) -> RequestHeadersValue
    where
        K: AsRef<str>,
    {
        self.read().get_request().get_header(key)
    }

    /// Attempts to retrieve the first value of a specific request header.
    ///
    /// # Arguments
    ///
    /// - `AsRef<str>` - The key of the header.
    ///
    /// # Returns
    ///
    /// - `Option<RequestHeadersValueItem>` - The first value of the header if it exists.
    #[inline(always)]
    pub fn try_get_request_header_front<K>(&self, key: K) -> Option<RequestHeadersValueItem>
    where
        K: AsRef<str>,
    {
        self.read().get_request().try_get_header_front(key)
    }

    /// Retrieves the first value of a specific request header, panicking if not found.
    ///
    /// # Arguments
    ///
    /// - `AsRef<str>` - The key of the header.
    ///
    /// # Returns
    ///
    /// - `RequestHeadersValueItem` - The first value of the header if it exists.
    ///
    /// # Panics
    ///
    /// - If the header is not found.
    #[inline(always)]
    pub fn get_request_header_front<K>(&self, key: K) -> RequestHeadersValueItem
    where
        K: AsRef<str>,
    {
        self.read().get_request().get_header_front(key)
    }

    /// Attempts to retrieve the last value of a specific request header.
    ///
    /// # Arguments
    ///
    /// - `AsRef<str>` - The key of the header.
    ///
    /// # Returns
    ///
    /// - `Option<RequestHeadersValueItem>` - The last value of the header if it exists.
    #[inline(always)]
    pub fn try_get_request_header_back<K>(&self, key: K) -> Option<RequestHeadersValueItem>
    where
        K: AsRef<str>,
    {
        self.read().get_request().try_get_header_back(key)
    }

    /// Retrieves the last value of a specific request header, panicking if not found.
    ///
    /// # Arguments
    ///
    /// - `AsRef<str>` - The key of the header.
    ///
    /// # Returns
    ///
    /// - `RequestHeadersValueItem` - The last value of the header if it exists.
    ///
    /// # Panics
    ///
    /// - If the header is not found.
    #[inline(always)]
    pub fn get_request_header_back<K>(&self, key: K) -> RequestHeadersValueItem
    where
        K: AsRef<str>,
    {
        self.read().get_request().get_header_back(key)
    }

    /// Attempts to retrieve the number of values for a specific request header.
    ///
    /// # Arguments
    ///
    /// - `AsRef<str>` - The key of the header.
    ///
    /// # Returns
    ///
    /// - `Option<usize>` - The number of values for the specified header if it exists.
    #[inline(always)]
    pub fn try_get_request_header_len<K>(&self, key: K) -> Option<usize>
    where
        K: AsRef<str>,
    {
        self.read().get_request().try_get_header_length(key)
    }

    /// Retrieves the number of values for a specific request header.
    ///
    /// # Arguments
    ///
    /// - `AsRef<str>` - The key of the header.
    ///
    /// # Returns
    ///
    /// - `usize` - The number of values for the specified header.
    ///
    /// # Panics
    ///
    /// - If the header is not found.
    #[inline(always)]
    pub fn get_request_header_len<K>(&self, key: K) -> usize
    where
        K: AsRef<str>,
    {
        self.read().get_request().get_header_length(key)
    }

    /// Retrieves the total number of values across all request headers.
    ///
    /// # Returns
    ///
    /// - `usize` - The total count of all values in all headers.
    #[inline(always)]
    pub fn get_request_headers_values_length(&self) -> usize {
        self.read().get_request().get_headers_values_length()
    }

    /// Checks if a specific request header exists.
    ///
    /// # Arguments
    ///
    /// - `AsRef<str>` - The key of the header to check.
    ///
    /// # Returns
    ///
    /// - `bool` - True if the header exists, otherwise false.
    #[inline(always)]
    pub fn get_request_has_header<K>(&self, key: K) -> bool
    where
        K: AsRef<str>,
    {
        self.read().get_request().has_header(key)
    }

    /// Checks if a request header has a specific value.
    ///
    /// # Arguments
    ///
    /// - `AsRef<str>` - The header key.
    /// - `AsRef<str>` - The value to check.
    ///
    /// # Returns
    ///
    /// - `bool` - True if header contains the value.
    #[inline(always)]
    pub fn get_request_has_header_value<K, V>(&self, key: K, value: V) -> bool
    where
        K: AsRef<str>,
        V: AsRef<str>,
    {
        self.read().get_request().has_header_value(key, value)
    }

    /// Parses and retrieves all cookies from the request headers.
    ///
    /// # Returns
    ///
    /// - `Cookies` - A map of cookies parsed from the request's Cookie header.
    #[inline(always)]
    pub fn get_request_cookies(&self) -> Cookies {
        self.try_get_request_header_back(COOKIE)
            .map(|data| Cookie::parse(&data))
            .unwrap_or_default()
    }

    /// Attempts to retrieve a specific cookie by its name from the request.
    ///
    /// # Arguments
    ///
    /// - `AsRef<str>` - The cookie name.
    ///
    /// # Returns
    ///
    /// - `Option<CookieValue>` - The cookie value if exists.
    #[inline(always)]
    pub fn try_get_request_cookie<K>(&self, key: K) -> Option<CookieValue>
    where
        K: AsRef<str>,
    {
        self.get_request_cookies().get(key.as_ref()).cloned()
    }

    /// Retrieves a specific cookie by its name from the request, panicking if not found.
    ///
    /// # Arguments
    ///
    /// - `AsRef<str>` - The cookie name.
    ///
    /// # Returns
    ///
    /// - `CookieValue` - The cookie value if exists.
    ///
    /// # Panics
    ///
    /// - If the cookie is not found.
    #[inline(always)]
    pub fn get_request_cookie<K>(&self, key: K) -> CookieValue
    where
        K: AsRef<str>,
    {
        self.try_get_request_cookie(key).unwrap()
    }

    /// Retrieves the upgrade type of the request.
    ///
    /// # Returns
    ///
    /// - `UpgradeType` - The upgrade type of the request.
    #[inline(always)]
    pub fn get_request_upgrade_type(&self) -> UpgradeType {
        self.read().get_request().get_upgrade_type()
    }

    /// Checks if the request is a WebSocket upgrade request.
    ///
    /// # Returns
    ///
    /// - `bool` - True if this is a WebSocket upgrade request.
    #[inline(always)]
    pub fn get_request_is_ws(&self) -> bool {
        self.read().get_request().is_ws()
    }

    /// Checks if the request is an HTTP/2 cleartext (h2c) upgrade.
    ///
    /// # Returns
    ///
    /// - `bool` - True if this is an h2c upgrade request.
    #[inline(always)]
    pub fn get_request_is_h2c(&self) -> bool {
        self.read().get_request().is_h2c()
    }

    /// Checks if the request is a TLS upgrade.
    ///
    /// # Returns
    ///
    /// - `bool` - True if this is a TLS upgrade request.
    #[inline(always)]
    pub fn get_request_is_tls(&self) -> bool {
        self.read().get_request().is_tls()
    }

    /// Checks if the request has an unknown upgrade type.
    ///
    /// # Returns
    ///
    /// - `bool` - True if the upgrade type is unknown.
    #[inline(always)]
    pub fn get_request_is_unknown_upgrade(&self) -> bool {
        self.read().get_request().is_unknown_upgrade()
    }

    /// Checks if the request HTTP version is HTTP/1.1 or higher.
    ///
    /// # Returns
    ///
    /// - `bool` - True if the version is HTTP/1.1 or higher.
    #[inline(always)]
    pub fn get_request_is_http1_1_or_higher(&self) -> bool {
        self.read().get_request().is_http1_1_or_higher()
    }

    /// Checks if the request HTTP version is HTTP/0.9.
    ///
    /// # Returns
    ///
    /// - `bool` - True if the version is HTTP/0.9.
    #[inline(always)]
    pub fn get_request_is_http0_9(&self) -> bool {
        self.read().get_request().is_http0_9()
    }

    /// Checks if the request HTTP version is HTTP/1.0.
    ///
    /// # Returns
    ///
    /// - `bool` - True if the version is HTTP/1.0.
    #[inline(always)]
    pub fn get_request_is_http1_0(&self) -> bool {
        self.read().get_request().is_http1_0()
    }

    /// Checks if the request HTTP version is HTTP/1.1.
    ///
    /// # Returns
    ///
    /// - `bool` - True if the version is HTTP/1.1.
    #[inline(always)]
    pub fn get_request_is_http1_1(&self) -> bool {
        self.read().get_request().is_http1_1()
    }

    /// Checks if the request HTTP version is HTTP/2.
    ///
    /// # Returns
    ///
    /// - `bool` - True if the version is HTTP/2.
    #[inline(always)]
    pub fn get_request_is_http2(&self) -> bool {
        self.read().get_request().is_http2()
    }

    /// Checks if the request HTTP version is HTTP/3.
    ///
    /// # Returns
    ///
    /// - `bool` - True if the version is HTTP/3.
    #[inline(always)]
    pub fn get_request_is_http3(&self) -> bool {
        self.read().get_request().is_http3()
    }

    /// Checks if the request has an unknown HTTP version.
    ///
    /// # Returns
    ///
    /// - `bool` - True if the version is unknown.
    #[inline(always)]
    pub fn get_request_is_unknown_version(&self) -> bool {
        self.read().get_request().is_unknown_version()
    }

    /// Checks if the request uses HTTP protocol.
    ///
    /// # Returns
    ///
    /// - `bool` - True if the version belongs to HTTP family.
    #[inline(always)]
    pub fn get_request_is_http(&self) -> bool {
        self.read().get_request().is_http()
    }

    /// Checks if the request method is GET.
    ///
    /// # Returns
    ///
    /// - `bool` - True if the method is GET.
    #[inline(always)]
    pub fn get_request_is_get(&self) -> bool {
        self.read().get_request().is_get()
    }

    /// Checks if the request method is POST.
    ///
    /// # Returns
    ///
    /// - `bool` - True if the method is POST.
    #[inline(always)]
    pub fn get_request_is_post(&self) -> bool {
        self.read().get_request().is_post()
    }

    /// Checks if the request method is PUT.
    ///
    /// # Returns
    ///
    /// - `bool` - True if the method is PUT.
    #[inline(always)]
    pub fn get_request_is_put(&self) -> bool {
        self.read().get_request().is_put()
    }

    /// Checks if the request method is DELETE.
    ///
    /// # Returns
    ///
    /// - `bool` - True if the method is DELETE.
    #[inline(always)]
    pub fn get_request_is_delete(&self) -> bool {
        self.read().get_request().is_delete()
    }

    /// Checks if the request method is PATCH.
    ///
    /// # Returns
    ///
    /// - `bool` - True if the method is PATCH.
    #[inline(always)]
    pub fn get_request_is_patch(&self) -> bool {
        self.read().get_request().is_patch()
    }

    /// Checks if the request method is HEAD.
    ///
    /// # Returns
    ///
    /// - `bool` - True if the method is HEAD.
    #[inline(always)]
    pub fn get_request_is_head(&self) -> bool {
        self.read().get_request().is_head()
    }

    /// Checks if the request method is OPTIONS.
    ///
    /// # Returns
    ///
    /// - `bool` - True if the method is OPTIONS.
    #[inline(always)]
    pub fn get_request_is_options(&self) -> bool {
        self.read().get_request().is_options()
    }

    /// Checks if the request method is CONNECT.
    ///
    /// # Returns
    ///
    /// - `bool` - True if the method is CONNECT.
    #[inline(always)]
    pub fn get_request_is_connect(&self) -> bool {
        self.read().get_request().is_connect()
    }

    /// Checks if the request method is TRACE.
    ///
    /// # Returns
    ///
    /// - `bool` - True if the method is TRACE.
    #[inline(always)]
    pub fn get_request_is_trace(&self) -> bool {
        self.read().get_request().is_trace()
    }

    /// Checks if the request method is unknown.
    ///
    /// # Returns
    ///
    /// - `bool` - True if the method is unknown.
    #[inline(always)]
    pub fn get_request_is_unknown_method(&self) -> bool {
        self.read().get_request().is_unknown_method()
    }

    /// Checks if the connection should be kept alive based on request headers.
    ///
    /// # Returns
    ///
    /// - `bool` - True if the Connection header suggests keeping the connection alive, otherwise false.
    #[inline(always)]
    pub fn get_request_is_enable_keep_alive(&self) -> bool {
        self.read().get_request().is_enable_keep_alive()
    }

    /// Checks if keep-alive should be disabled for the request.
    ///
    /// # Returns
    ///
    /// - `bool` - True if keep-alive should be disabled.
    #[inline(always)]
    pub fn get_request_is_disable_keep_alive(&self) -> bool {
        self.read().get_request().is_disable_keep_alive()
    }

    /// Retrieves the current HTTP response.
    ///
    /// # Returns
    ///
    /// - `Response` - A clone of the current response.
    #[inline(always)]
    pub fn get_response(&self) -> Response {
        self.read().get_response().clone()
    }

    /// Sets the HTTP response for the context.
    ///
    /// # Arguments
    ///
    /// - `Borrow<Response>` - The response to set in the context.
    ///
    /// # Returns
    ///
    /// - `&Self` - Reference to the modified context.
    #[inline(always)]
    pub fn set_response<T>(&self, response: T) -> &Self
    where
        T: Borrow<Response>,
    {
        self.write().set_response(response.borrow().clone());
        self
    }

    /// Executes an asynchronous closure with the current response.
    ///
    /// # Arguments
    ///
    /// - `F` - A closure that takes the `Response` and returns a future.
    ///
    /// # Returns
    ///
    /// - `R` - The result of the provided closure's future.
    pub async fn with_response<F, Fut, R>(&self, func: F) -> R
    where
        F: Fn(Response) -> Fut,
        Fut: FutureSendStatic<R>,
    {
        func(self.read().get_response().clone()).await
    }

    /// Retrieves the string representation of the current response.
    ///
    /// # Returns
    ///
    /// - `String` - The full response as a string.
    #[inline(always)]
    pub fn get_response_string(&self) -> String {
        self.read().get_response().get_string()
    }

    /// Retrieves the HTTP version of the response.
    ///
    /// # Returns
    ///
    /// - `ResponseVersion` - The HTTP version of the response.
    #[inline(always)]
    pub fn get_response_version(&self) -> ResponseVersion {
        self.read().get_response().get_version().clone()
    }

    /// Sets the HTTP version for the response.
    ///
    /// # Arguments
    ///
    /// - `ResponseVersion` - The HTTP version to set for the response.
    ///
    /// # Returns
    ///
    /// - `&Self` - Reference to the modified context.
    #[inline(always)]
    pub fn set_response_version(&self, version: ResponseVersion) -> &Self {
        self.write().get_mut_response().set_version(version);
        self
    }

    /// Retrieves all response headers.
    ///
    /// # Returns
    ///
    /// - `ResponseHeaders` - A clone of the response's header map.
    #[inline(always)]
    pub fn get_response_headers(&self) -> ResponseHeaders {
        self.read().get_response().get_headers().clone()
    }

    /// Attempts to retrieve a specific response header by its key.
    ///
    /// # Arguments
    ///
    /// - `AsRef<str>` - The key of the header to retrieve.
    ///
    /// # Returns
    ///
    /// - `Option<ResponseHeadersValue>` - The header values if the header exists.
    #[inline(always)]
    pub fn try_get_response_header<K>(&self, key: K) -> Option<ResponseHeadersValue>
    where
        K: AsRef<str>,
    {
        self.read().get_response().try_get_header(key)
    }

    /// Retrieves a specific response header by its key, panicking if not found.
    ///
    /// # Arguments
    ///
    /// - `AsRef<str>` - The key of the header to retrieve.
    ///
    /// # Returns
    ///
    /// - `ResponseHeadersValue` - The header values if the header exists.
    ///
    /// # Panics
    ///
    /// - If the header is not found.
    #[inline(always)]
    pub fn get_response_header<K>(&self, key: K) -> ResponseHeadersValue
    where
        K: AsRef<str>,
    {
        self.read().get_response().get_header(key)
    }

    /// Sets a response header with a new value, removing any existing values.
    ///
    /// # Arguments
    ///
    /// - `K` - The key of the header to set.
    /// - `V` - The new value for the header.
    ///
    /// # Returns
    ///
    /// - `&Self` - Reference to the modified context.
    #[inline(always)]
    pub fn set_response_header<K, V>(&self, key: K, value: V) -> &Self
    where
        K: AsRef<str>,
        V: AsRef<str>,
    {
        self.write().get_mut_response().set_header(key, value);
        self
    }

    /// Attempts to retrieve the first value of a specific response header.
    ///
    /// # Arguments
    ///
    /// - `AsRef<str>` - The key of the header.
    ///
    /// # Returns
    ///
    /// - `Option<ResponseHeadersValueItem>` - The first value of the header if it exists.
    #[inline(always)]
    pub fn try_get_response_header_front<K>(&self, key: K) -> Option<ResponseHeadersValueItem>
    where
        K: AsRef<str>,
    {
        self.read().get_response().try_get_header_front(key)
    }

    /// Retrieves the first value of a specific response header, panicking if not found.
    ///
    /// # Arguments
    ///
    /// - `AsRef<str>` - The key of the header.
    ///
    /// # Returns
    ///
    /// - `ResponseHeadersValueItem` - The first value of the header if it exists.
    ///
    /// # Panics
    ///
    /// - If the header is not found.
    #[inline(always)]
    pub fn get_response_header_front<K>(&self, key: K) -> ResponseHeadersValueItem
    where
        K: AsRef<str>,
    {
        self.read().get_response().get_header_front(key)
    }

    /// Attempts to retrieve the last value of a specific response header.
    ///
    /// # Arguments
    ///
    /// - `AsRef<str>` - The key of the header.
    ///
    /// # Returns
    ///
    /// - `Option<ResponseHeadersValueItem>` - The last value of the header if it exists.
    #[inline(always)]
    pub fn try_get_response_header_back<K>(&self, key: K) -> Option<ResponseHeadersValueItem>
    where
        K: AsRef<str>,
    {
        self.read().get_response().try_get_header_back(key)
    }

    /// Retrieves the last value of a specific response header, panicking if not found.
    ///
    /// # Arguments
    ///
    /// - `AsRef<str>` - The key of the header.
    ///
    /// # Returns
    ///
    /// - `ResponseHeadersValueItem` - The last value of the header if it exists.
    ///
    /// # Panics
    ///
    /// - If the header is not found.
    #[inline(always)]
    pub fn get_response_header_back<K>(&self, key: K) -> ResponseHeadersValueItem
    where
        K: AsRef<str>,
    {
        self.read().get_response().get_header_back(key)
    }

    /// Checks if a specific response header exists.
    ///
    /// # Arguments
    ///
    /// - `AsRef<str>` - The key of the header to check.
    ///
    /// # Returns
    ///
    /// - `bool` - True if the header exists, otherwise false.
    #[inline(always)]
    pub fn get_response_has_header<K>(&self, key: K) -> bool
    where
        K: AsRef<str>,
    {
        self.read().get_response().has_header(key)
    }

    /// Checks if a response header has a specific value.
    ///
    /// # Arguments
    ///
    /// - `AsRef<str>` - The key of the header.
    /// - `AsRef<str>` - The value to check for.
    ///
    /// # Returns
    ///
    /// - `bool` - True if the header contains the specified value, otherwise false.
    #[inline(always)]
    pub fn get_response_header_value<K, V>(&self, key: K, value: V) -> bool
    where
        K: AsRef<str>,
        V: AsRef<str>,
    {
        self.read().get_response().has_header_value(key, value)
    }

    /// Retrieves the total number of response headers.
    ///
    /// # Returns
    ///
    /// - `usize` - The total number of headers in the response.
    #[inline(always)]
    pub fn get_response_headers_length(&self) -> usize {
        self.read().get_response().get_headers_length()
    }

    /// Attempts to retrieve the number of values for a specific response header.
    ///
    /// # Arguments
    ///
    /// - `AsRef<str>` - The key of the header.
    ///
    /// # Returns
    ///
    /// - `Option<usize>` - The number of values for the specified header if it exists.
    #[inline(always)]
    pub fn try_get_response_header_length<K>(&self, key: K) -> Option<usize>
    where
        K: AsRef<str>,
    {
        self.read().get_response().try_get_header_length(key)
    }

    /// Retrieves the number of values for a specific response header.
    ///
    /// # Arguments
    ///
    /// - `AsRef<str>` - The key of the header.
    ///
    /// # Returns
    ///
    /// - `usize` - The number of values for the specified header.
    ///
    /// # Panics
    ///
    /// - If the header is not found.
    #[inline(always)]
    pub fn get_response_header_length<K>(&self, key: K) -> usize
    where
        K: AsRef<str>,
    {
        self.read().get_response().get_header_length(key)
    }

    /// Retrieves the total number of values across all response headers.
    ///
    /// # Returns
    ///
    /// - `usize` - The total count of all values in all headers.
    #[inline(always)]
    pub fn get_response_headers_values_length(&self) -> usize {
        self.read().get_response().get_headers_values_length()
    }

    /// Adds a response header, adding it if it doesn't exist or appending to it if it does.
    ///
    /// # Arguments
    ///
    /// - `AsRef<str>` - The header key.
    /// - `AsRef<str>` - The header value.
    ///
    /// # Returns
    ///
    /// - `&Self` - Reference to self for method chaining.
    #[inline(always)]
    pub fn add_response_header<K, V>(&self, key: K, value: V) -> &Self
    where
        K: AsRef<str>,
        V: AsRef<str>,
    {
        self.write().get_mut_response().add_header(key, value);
        self
    }

    /// Removes a response header and all its values.
    ///
    /// # Arguments
    ///
    /// - `AsRef<str>` - The key of the header to remove.
    ///
    /// # Returns
    ///
    /// - `&Self` - Reference to the modified context.
    #[inline(always)]
    pub fn remove_response_header<K>(&self, key: K) -> &Self
    where
        K: AsRef<str>,
    {
        self.write().get_mut_response().remove_header(key);
        self
    }

    /// Removes a specific value from a response header.
    ///
    /// # Arguments
    ///
    /// - `AsRef<str>` - The header key.
    /// - `AsRef<str>` - The value to remove.
    ///
    /// # Returns
    ///
    /// - `&Self` - Reference to self for method chaining.
    #[inline(always)]
    pub fn remove_response_header_value<K, V>(&self, key: K, value: V) -> &Self
    where
        K: AsRef<str>,
        V: AsRef<str>,
    {
        self.write()
            .get_mut_response()
            .remove_header_value(key, value);
        self
    }

    /// Clears all headers from the response.
    ///
    /// # Returns
    ///
    /// - `&Self` - Reference to the modified context.
    #[inline(always)]
    pub fn clear_response_headers(&self) -> &Self {
        self.write().get_mut_response().clear_headers();
        self
    }

    /// Parses and retrieves all cookies from the response headers.
    ///
    /// # Returns
    ///
    /// - `Cookies` - A map of cookies parsed from the response's Cookie header.
    #[inline(always)]
    pub fn get_response_cookies(&self) -> Cookies {
        self.try_get_response_header_back(COOKIE)
            .map(|data| Cookie::parse(&data))
            .unwrap_or_default()
    }

    /// Attempts to retrieve a specific cookie by its name from the response.
    ///
    /// # Arguments
    ///
    /// - `AsRef<str>` - The name of the cookie to retrieve.
    ///
    /// # Returns
    ///
    /// - `Option<CookieValue>` - The cookie's value if it exists.
    #[inline(always)]
    pub fn try_get_response_cookie<K>(&self, key: K) -> Option<CookieValue>
    where
        K: AsRef<str>,
    {
        self.get_response_cookies().get(key.as_ref()).cloned()
    }

    /// Retrieves a specific cookie by its name from the response, panicking if not found.
    ///
    /// # Arguments
    ///
    /// - `AsRef<str>` - The name of the cookie to retrieve.
    ///
    /// # Returns
    ///
    /// - `CookieValue` - The cookie's value if it exists.
    ///
    /// # Panics
    ///
    /// - If the cookie is not found.
    #[inline(always)]
    pub fn get_response_cookie<K>(&self, key: K) -> CookieValue
    where
        K: AsRef<str>,
    {
        self.try_get_response_cookie(key).unwrap()
    }

    /// Retrieves the body of the response.
    ///
    /// # Returns
    ///
    /// - `ResponseBody` - The response body.
    #[inline(always)]
    pub fn get_response_body(&self) -> ResponseBody {
        self.read().get_response().get_body().clone()
    }

    /// Sets the body of the response.
    ///
    /// # Arguments
    ///
    /// - `B` - The body data to set for the response.
    ///
    /// # Returns
    ///
    /// - `&Self` - Reference to the modified context.
    #[inline(always)]
    pub fn set_response_body<B>(&self, body: B) -> &Self
    where
        B: AsRef<[u8]>,
    {
        self.write().get_mut_response().set_body(body);
        self
    }

    /// Retrieves the response body as a string.
    ///
    /// # Returns
    ///
    /// - `String` - The response body converted to a string.
    #[inline(always)]
    pub fn get_response_body_string(&self) -> String {
        self.read().get_response().get_body_string()
    }

    /// Deserializes the response body from JSON into a specified type.
    ///
    /// # Returns
    ///
    /// - `Result<J, serde_json::Error>` - The deserialized type `J` or a JSON error.
    pub fn try_get_response_body_json<J>(&self) -> Result<J, serde_json::Error>
    where
        J: DeserializeOwned,
    {
        self.read().get_response().try_get_body_json()
    }

    /// Deserializes the response body from JSON into a specified type, panicking if not found.
    ///
    /// # Returns
    ///
    /// - `J` - The deserialized type `J`.
    ///
    /// # Panics
    ///
    /// - If deserialization fails.
    pub fn get_response_body_json<J>(&self) -> J
    where
        J: DeserializeOwned,
    {
        self.read().get_response().get_body_json()
    }

    /// Retrieves the reason phrase of the response status code.
    ///
    /// # Returns
    ///
    /// - `ResponseReasonPhrase` - The reason phrase associated with the response status code.
    #[inline(always)]
    pub fn get_response_reason_phrase(&self) -> ResponseReasonPhrase {
        self.read().get_response().get_reason_phrase().clone()
    }

    /// Sets the reason phrase for the response status code.
    ///
    /// # Arguments
    ///
    /// - `AsRef<str>` - The reason phrase to set.
    ///
    /// # Returns
    ///
    /// - `&Self` - Reference to the modified context.
    #[inline(always)]
    pub fn set_response_reason_phrase<P>(&self, reason_phrase: P) -> &Self
    where
        P: AsRef<str>,
    {
        self.write()
            .get_mut_response()
            .set_reason_phrase(reason_phrase);
        self
    }

    /// Retrieves the status code of the response.
    ///
    /// # Returns
    ///
    /// - `ResponseStatusCode` - The status code of the response.
    #[inline(always)]
    pub fn get_response_status_code(&self) -> ResponseStatusCode {
        self.read().get_response().get_status_code()
    }

    /// Sets the status code for the response.
    ///
    /// # Arguments
    ///
    /// - `ResponseStatusCode` - The status code to set for the response.
    ///
    /// # Returns
    ///
    /// - `&Self` - A reference to the modified context.
    #[inline(always)]
    pub fn set_response_status_code(&self, status_code: ResponseStatusCode) -> &Self {
        self.write().get_mut_response().set_status_code(status_code);
        self
    }

    /// Retrieves the parameters extracted from the route path.
    ///
    /// # Returns
    ///
    /// - `RouteParams` - A map containing the route parameters.
    #[inline(always)]
    pub fn get_route_params(&self) -> RouteParams {
        self.read().get_route_params().clone()
    }

    /// Sets the route parameters for the context.
    ///
    /// # Arguments
    ///
    /// - `RouteParams` - The route parameters to set.
    ///
    /// # Returns
    ///
    /// - `&Self` - A reference to the modified `Context`.
    #[inline(always)]
    pub(crate) fn set_route_params(&self, params: RouteParams) -> &Self {
        self.write().set_route_params(params);
        self
    }

    /// Attempts to retrieve a specific route parameter by its name.
    ///
    /// # Arguments
    ///
    /// - `AsRef<str>` - The name of the route parameter to retrieve.
    ///
    /// # Returns
    ///
    /// - `Option<String>` - The value of the route parameter if it exists.
    #[inline(always)]
    pub fn try_get_route_param<T>(&self, name: T) -> Option<String>
    where
        T: AsRef<str>,
    {
        self.read().get_route_params().get(name.as_ref()).cloned()
    }

    /// Retrieves a specific route parameter by its name, panicking if not found.
    ///
    /// # Arguments
    ///
    /// - `AsRef<str>` - The name of the route parameter to retrieve.
    ///
    /// # Returns
    ///
    /// - `String` - The value of the route parameter if it exists.
    ///
    /// # Panics
    ///
    /// - If the route parameter is not found.
    #[inline(always)]
    pub fn get_route_param<T>(&self, name: T) -> String
    where
        T: AsRef<str>,
    {
        self.try_get_route_param(name).unwrap()
    }

    /// Retrieves all attributes stored in the context.
    ///
    /// # Returns
    ///
    /// - `ThreadSafeAttributeStore` - A map containing all attributes.
    #[inline(always)]
    pub fn get_attributes(&self) -> ThreadSafeAttributeStore {
        self.read().get_attributes().clone()
    }

    /// Attempts to retrieve a specific attribute by its key, casting it to the specified type.
    ///
    /// # Arguments
    ///
    /// - `AsRef<str>` - The key of the attribute to retrieve.
    ///
    /// # Returns
    ///
    /// - `Option<V>` - The attribute value if it exists and can be cast to the specified type.
    #[inline(always)]
    pub fn try_get_attribute<K, V>(&self, key: K) -> Option<V>
    where
        K: AsRef<str>,
        V: AnySendSyncClone,
    {
        self.read()
            .get_attributes()
            .get(&Attribute::External(key.as_ref().to_owned()).to_string())
            .and_then(|arc| arc.downcast_ref::<V>())
            .cloned()
    }

    /// Retrieves a specific attribute by its key, casting it to the specified type, panicking if not found.
    ///
    /// # Arguments
    ///
    /// - `AsRef<str>` - The key of the attribute to retrieve.
    ///
    /// # Returns
    ///
    /// - `V` - The attribute value if it exists and can be cast to the specified type.
    ///
    /// # Panics
    ///
    /// - If the attribute is not found.
    #[inline(always)]
    pub fn get_attribute<K, V>(&self, key: K) -> V
    where
        K: AsRef<str>,
        V: AnySendSyncClone,
    {
        self.try_get_attribute(key).unwrap()
    }

    /// Sets an attribute in the context.
    ///
    /// # Arguments
    ///
    /// - `AsRef<str>` - The key of the attribute to set.
    /// - `AnySendSyncClone` - The value of the attribute.
    ///
    /// # Returns
    ///
    /// - `&Self` - A reference to the modified context.
    #[inline(always)]
    pub fn set_attribute<K, V>(&self, key: K, value: V) -> &Self
    where
        K: AsRef<str>,
        V: AnySendSyncClone,
    {
        self.write().get_mut_attributes().insert(
            Attribute::External(key.as_ref().to_owned()).to_string(),
            Arc::new(value),
        );
        self
    }

    /// Removes an attribute from the context.
    ///
    /// # Arguments
    ///
    /// - `AsRef<str>` - The key of the attribute to remove.
    ///
    /// # Returns
    ///
    /// - `&Self` - A reference to the modified context.
    #[inline(always)]
    pub fn remove_attribute<K>(&self, key: K) -> &Self
    where
        K: AsRef<str>,
    {
        self.write()
            .get_mut_attributes()
            .remove(&Attribute::External(key.as_ref().to_owned()).to_string());
        self
    }

    /// Clears all attributes from the context.
    ///
    /// # Returns
    ///
    /// - `&Self` - A reference to the modified context.
    #[inline(always)]
    pub fn clear_attribute(&self) -> &Self {
        self.write().get_mut_attributes().clear();
        self
    }

    /// Retrieves an internal framework attribute.
    ///
    /// # Arguments
    ///
    /// - `InternalAttribute` - The internal attribute key to retrieve.
    ///
    /// # Returns
    ///
    /// - `Option<V>` - The attribute value if it exists and can be cast to the specified type.
    #[inline(always)]
    fn try_get_internal_attribute<V>(&self, key: InternalAttribute) -> Option<V>
    where
        V: AnySendSyncClone,
    {
        self.read()
            .get_attributes()
            .get(&Attribute::Internal(key).to_string())
            .and_then(|arc| arc.downcast_ref::<V>())
            .cloned()
    }

    /// Retrieves an internal framework attribute.
    ///
    /// # Arguments
    ///
    /// - `InternalAttribute` - The internal attribute key to retrieve.
    ///
    /// # Returns
    ///
    /// - `V` - The attribute value if it exists and can be cast to the specified type.
    ///
    /// # Panics
    ///
    /// - If the attribute is not found.
    #[inline(always)]
    fn get_internal_attribute<V>(&self, key: InternalAttribute) -> V
    where
        V: AnySendSyncClone,
    {
        self.try_get_internal_attribute(key).unwrap()
    }

    /// Sets an internal framework attribute.
    ///
    /// # Arguments
    ///
    /// - `InternalAttribute` - The internal attribute key to set.
    /// - `AnySendSyncClone` - The value of the attribute.
    ///
    /// # Returns
    ///
    /// - `&Self` - A reference to the modified context.
    #[inline(always)]
    fn set_internal_attribute<V>(&self, key: InternalAttribute, value: V) -> &Self
    where
        V: AnySendSyncClone,
    {
        self.write()
            .get_mut_attributes()
            .insert(Attribute::Internal(key).to_string(), Arc::new(value));
        self
    }

    /// Stores panic data for the current task context.
    ///
    /// # Arguments
    ///
    /// - `PanicData` - The panic data specific to the current task.
    ///
    /// # Returns
    ///
    /// - `&Self` - Reference to the modified context for method chaining.
    #[inline(always)]
    pub(crate) fn set_task_panic(&self, panic_data: PanicData) -> &Self {
        self.set_internal_attribute(InternalAttribute::TaskPanicData, panic_data)
    }

    /// Retrieves panic data associated with the current task.
    ///
    /// # Returns
    ///
    /// - `Option<PanicData>` - Task panic data if a panic was caught during execution.
    #[inline(always)]
    pub fn try_get_task_panic_data(&self) -> Option<PanicData> {
        self.try_get_internal_attribute(InternalAttribute::TaskPanicData)
    }

    /// Retrieves panic data associated with the current task.
    ///
    /// # Returns
    ///
    /// - `PanicData` - Task panic data if available.
    ///
    /// # Panics
    ///
    /// - If no task panic data is found.
    #[inline(always)]
    pub fn get_task_panic_data(&self) -> PanicData {
        self.get_internal_attribute(InternalAttribute::TaskPanicData)
    }

    /// Sets the request error information for the context.
    ///
    /// # Arguments
    ///
    /// - `RequestError` - The request error information to store.
    ///
    /// # Returns
    ///
    /// - `&Self` - A reference to the modified context.
    #[inline(always)]
    pub(crate) fn set_request_error_data(&self, request_error: RequestError) -> &Self {
        self.set_internal_attribute(InternalAttribute::RequestErrorData, request_error)
    }

    /// Retrieves request error information if an error occurred during handling.
    ///
    /// # Returns
    ///
    /// - `Option<RequestError>` - The request error information if an error was caught.
    #[inline(always)]
    pub fn try_get_request_error_data(&self) -> Option<RequestError> {
        self.try_get_internal_attribute(InternalAttribute::RequestErrorData)
    }

    /// Retrieves request error information if an error occurred during handling.
    ///
    /// # Returns
    ///
    /// - `RequestError` - The request error information if an error was caught.
    ///
    /// # Panics
    ///
    /// - If the request error information is not found.
    #[inline(always)]
    pub fn get_request_error_data(&self) -> RequestError {
        self.get_internal_attribute(InternalAttribute::RequestErrorData)
    }

    /// Sets a hook function for the context with a custom key.
    ///
    /// # Arguments
    ///
    /// - `AsRef<str>` - The key to identify this hook.
    /// - `FnContextSendSyncStatic<Fut, ()>, Fut: FutureSendStatic<()>` - The hook function to store.
    ///
    /// # Returns
    ///
    /// - `&Self` - A reference to the modified context.
    #[inline(always)]
    pub fn set_hook<K, F, Fut>(&self, key: K, hook: F) -> &Self
    where
        K: AsRef<str>,
        F: FnContextSendSyncStatic<Fut, ()>,
        Fut: FutureSendStatic<()>,
    {
        let hook_fn: HookHandler<()> =
            Arc::new(move |ctx: Context| -> SendableAsyncTask<()> { Box::pin(hook(ctx)) });
        self.set_internal_attribute(InternalAttribute::Hook(key.as_ref().to_owned()), hook_fn)
    }

    /// Attempts to retrieve a hook function if it has been set.
    ///
    /// # Arguments
    ///
    /// - `AsRef<str>` - The key to identify the hook.
    ///
    /// # Returns
    ///
    /// - `Option<HookHandler<()>>` - The hook function if it has been set.
    #[inline(always)]
    pub fn try_get_hook<K>(&self, key: K) -> Option<HookHandler<()>>
    where
        K: AsRef<str>,
    {
        self.try_get_internal_attribute(InternalAttribute::Hook(key.as_ref().to_owned()))
    }

    /// Retrieves a hook function if it has been set, panicking if not found.
    ///
    /// # Arguments
    ///
    /// - `AsRef<str>` - The key to identify the hook.
    ///
    /// # Returns
    ///
    /// - `HookHandler<()>` - The hook function if it has been set.
    ///
    /// # Panics
    ///
    /// - If the hook function is not found.
    #[inline(always)]
    pub fn get_hook<K>(&self, key: K) -> HookHandler<()>
    where
        K: AsRef<str>,
    {
        self.get_internal_attribute(InternalAttribute::Hook(key.as_ref().to_owned()))
    }

    /// Sends HTTP response data over the stream.
    ///
    /// # Returns
    ///
    /// - `Result<(), ResponseError>` - Result indicating success or failure.
    pub async fn try_send(&self) -> Result<(), ResponseError> {
        if self.is_terminated() {
            return Err(ResponseError::Terminated);
        }
        let response_data: ResponseData = self.write().get_mut_response().build();
        if let Some(stream) = self.try_get_stream() {
            return stream.try_send(response_data).await;
        }
        Err(ResponseError::NotFoundStream)
    }

    /// Sends HTTP response data over the stream.
    ///
    /// # Panics
    ///
    /// Panics if the write operation fails.
    pub async fn send(&self) {
        self.try_send().await.unwrap();
    }

    /// Sends HTTP response body.
    ///
    /// # Returns
    ///
    /// - `Result<(), ResponseError>` - Result indicating success or failure.
    pub async fn try_send_body(&self) -> Result<(), ResponseError> {
        if self.is_terminated() {
            return Err(ResponseError::Terminated);
        }
        let response_body: ResponseBody = self.get_response_body();
        self.try_send_body_with_data(response_body).await
    }

    /// Sends HTTP response body.
    ///
    /// # Panics
    ///
    /// Panics if the write operation fails.
    pub async fn send_body(&self) {
        self.try_send_body().await.unwrap();
    }

    /// Sends only the response body to the client with additional data.
    ///
    /// This method is useful for streaming data or for responses where headers have already been sent.
    ///
    /// # Arguments
    ///
    /// - `AsRef<[u8]>` - The additional data to send as the body.
    ///
    /// # Returns
    ///
    /// - `Result<(), ResponseError>` - The result of the send operation.
    pub async fn try_send_body_with_data<D>(&self, data: D) -> Result<(), ResponseError>
    where
        D: AsRef<[u8]>,
    {
        if self.is_terminated() {
            return Err(ResponseError::Terminated);
        }
        if let Some(stream) = self.try_get_stream() {
            return stream.try_send_body(data).await;
        }
        Err(ResponseError::NotFoundStream)
    }

    /// Sends HTTP response body.
    ///
    /// # Arguments
    ///
    /// - `AsRef<[u8]>` - The response body data (must implement AsRef<[u8]>).
    ///
    /// # Panics
    ///
    /// Panics if the write operation fails.
    pub async fn send_body_with_data<D>(&self, data: D)
    where
        D: AsRef<[u8]>,
    {
        self.try_send_body_with_data(data).await.unwrap();
    }

    /// Sends multiple HTTP response bodies sequentially.
    ///
    /// # Arguments
    ///
    /// - `I: IntoIterator<Item = D>, D: AsRef<[u8]>` - The response body data list to send.
    ///
    /// # Returns
    ///
    /// - `Result<(), ResponseError>` - Result indicating success or failure.
    pub async fn try_send_body_list<I, D>(&self, data_iter: I) -> Result<(), ResponseError>
    where
        I: IntoIterator<Item = D>,
        D: AsRef<[u8]>,
    {
        if self.is_terminated() {
            return Err(ResponseError::Terminated);
        }
        if let Some(stream) = self.try_get_stream() {
            return stream.try_send_body_list(data_iter).await;
        }
        Err(ResponseError::NotFoundStream)
    }

    /// Sends multiple HTTP response bodies sequentially.
    ///
    /// # Arguments
    ///
    /// - `I: IntoIterator<Item = D>, D: AsRef<[u8]>` - The response body data list to send.
    ///
    /// # Panics
    ///
    /// Panics if any write operation fails.
    pub async fn send_body_list<I, D>(&self, data_iter: I)
    where
        I: IntoIterator<Item = D>,
        D: AsRef<[u8]>,
    {
        self.try_send_body_list(data_iter).await.unwrap();
    }

    /// Sends a list of response bodies to the client with additional data.
    ///
    /// This is useful for streaming multiple data chunks or for responses where headers have already been sent.
    ///
    /// # Arguments
    ///
    /// - `I: IntoIterator<Item = D>, D: AsRef<[u8]>` - The additional data to send as a list of bodies.
    ///
    /// # Returns
    ///
    /// - `Result<(), ResponseError>` - The result of the send operation.
    pub async fn try_send_body_list_with_data<I, D>(
        &self,
        data_iter: I,
    ) -> Result<(), ResponseError>
    where
        I: IntoIterator<Item = D>,
        D: AsRef<[u8]>,
    {
        if self.is_terminated() {
            return Err(ResponseError::Terminated);
        }
        if let Some(stream) = self.try_get_stream() {
            return stream.try_send_body_list(data_iter).await;
        }
        Err(ResponseError::NotFoundStream)
    }

    /// Sends a list of response bodies to the client with additional data.
    ///
    /// # Arguments
    ///
    /// - `I: IntoIterator<Item = D>, D: AsRef<[u8]>` - The additional data to send as a list of bodies.
    ///
    /// # Panics
    ///
    /// Panics if any write operation fails.
    pub async fn send_body_list_with_data<I, D>(&self, data_iter: I)
    where
        I: IntoIterator<Item = D>,
        D: AsRef<[u8]>,
    {
        self.try_send_body_list_with_data(data_iter).await.unwrap()
    }

    /// Flushes the underlying network stream, ensuring all buffered data is sent.
    ///
    /// # Returns
    ///
    /// - `Result<(), ResponseError>` - The result of the flush operation.
    pub async fn try_flush(&self) -> Result<(), ResponseError> {
        if self.is_terminated() {
            return Err(ResponseError::Terminated);
        }
        if let Some(stream) = self.try_get_stream() {
            return stream.try_flush().await;
        }
        Err(ResponseError::NotFoundStream)
    }

    /// Flushes all buffered data to the stream.
    ///
    /// # Panics
    ///
    /// Panics if the flush operation fails.
    pub async fn flush(&self) {
        self.try_flush().await.unwrap();
    }
}
