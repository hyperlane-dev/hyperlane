use crate::*;

/// Implementation of methods for `Context` structure.
impl Context {
    /// Creates a new `Context` from an internal context instance.
    ///
    /// # Arguments
    ///
    /// - `ContextInner` - The wrapped context data.
    ///
    /// # Returns
    ///
    /// - `Context` - The newly created context instance.
    pub(crate) fn from_internal_context(ctx: ContextInner) -> Self {
        Self(arc_rwlock(ctx))
    }

    /// Creates a new `Context` for a given stream and request.
    ///
    /// # Arguments
    ///
    /// - `&ArcRwLockStream` - The network stream.
    /// - `&Request` - The HTTP request.
    ///
    /// # Returns
    ///
    /// - `Context` - The newly created context.
    pub(crate) fn create_context(stream: &ArcRwLockStream, request: &Request) -> Context {
        Context::from_internal_context({
            let mut internal_ctx: ContextInner = ContextInner::default();
            internal_ctx
                .set_stream(Some(stream.clone()))
                .set_request(request.clone());
            internal_ctx
        })
    }

    /// Acquires a read lock on the inner context data.
    ///
    /// # Returns
    ///
    /// - `RwLockReadContextInner` - The read guard for the inner context.
    async fn read(&self) -> RwLockReadContextInner {
        self.get_0().read().await
    }

    /// Acquires a write lock on the inner context data.
    ///
    /// # Returns
    ///
    /// - `RwLockWriteContextInner` - The write guard for the inner context.
    async fn write(&self) -> RwLockWriteContextInner {
        self.get_0().write().await
    }

    /// Checks if the context has been marked as aborted.
    ///
    /// # Returns
    ///
    /// - `bool` - True if the context is aborted, otherwise false.
    pub async fn get_aborted(&self) -> bool {
        *self.read().await.get_aborted()
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
    pub async fn set_aborted(&self, aborted: bool) -> &Self {
        self.write().await.set_aborted(aborted);
        self
    }

    /// Marks the context as aborted.
    ///
    /// # Returns
    ///
    /// - `&Self` - Reference to the modified context.
    pub async fn aborted(&self) -> &Self {
        self.set_aborted(true).await;
        self
    }

    /// Cancels the aborted state of the context.
    ///
    /// # Returns
    ///
    /// - `&Self` - Reference to the modified context.
    pub async fn cancel_aborted(&self) -> &Self {
        self.set_aborted(false).await;
        self
    }

    /// Checks if the connection is marked as closed.
    ///
    /// # Returns
    ///
    /// - `bool` - True if the connection is closed, otherwise false.
    pub async fn get_closed(&self) -> bool {
        *self.read().await.get_closed()
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
    pub async fn set_closed(&self, closed: bool) -> &Self {
        self.write().await.set_closed(closed);
        self
    }

    /// Marks the connection as closed.
    ///
    /// # Returns
    ///
    /// - `&Self` - Reference to the modified context.
    pub async fn closed(&self) -> &Self {
        self.set_closed(true).await;
        self
    }

    /// Cancels the closed state of the connection.
    ///
    /// # Returns
    ///
    /// - `&Self` - Reference to the modified context.
    pub async fn cancel_closed(&self) -> &Self {
        self.set_closed(false).await;
        self
    }

    /// Checks if the connection has been terminated (aborted and closed).
    ///
    /// # Returns
    ///
    /// - `bool` - True if the connection is both aborted and closed, otherwise false.
    pub async fn is_terminated(&self) -> bool {
        self.get_aborted().await || self.get_closed().await
    }

    /// Retrieves the underlying network stream, if available.
    ///
    /// # Returns
    ///
    /// - `OptionArcRwLockStream` - The thread-safe, shareable network stream if it exists.
    pub async fn try_get_stream(&self) -> OptionArcRwLockStream {
        self.read().await.get_stream().clone()
    }

    /// Retrieves the remote socket address of the connection.
    ///
    /// # Returns
    ///
    /// - `OptionSocketAddr` - The socket address of the remote peer if available.
    pub async fn try_get_socket_addr(&self) -> OptionSocketAddr {
        let stream_result: OptionArcRwLockStream = self.try_get_stream().await;
        if stream_result.is_none() {
            return None;
        }
        stream_result.unwrap().read().await.peer_addr().ok()
    }

    /// Retrieves the remote socket address or a default value if unavailable.
    ///
    /// # Returns
    ///
    /// - `SocketAddr` - The socket address of the remote peer, or default if unavailable.
    pub async fn get_socket_addr(&self) -> SocketAddr {
        let stream_result: OptionArcRwLockStream = self.try_get_stream().await;
        if stream_result.is_none() {
            return DEFAULT_SOCKET_ADDR;
        }
        stream_result
            .unwrap()
            .read()
            .await
            .peer_addr()
            .unwrap_or(DEFAULT_SOCKET_ADDR)
    }

    /// Retrieves the remote socket address as a string.
    ///
    /// # Returns
    ///
    /// - `OptionString` - The string representation of the socket address if available.
    pub async fn try_get_socket_addr_string(&self) -> OptionString {
        self.try_get_socket_addr()
            .await
            .map(|data| data.to_string())
    }

    /// Retrieves the remote socket address as a string, or a default value if unavailable.
    ///
    /// # Returns
    ///
    /// - `String` - The string representation of the socket address, or default if unavailable.
    pub async fn get_socket_addr_string(&self) -> String {
        self.get_socket_addr().await.to_string()
    }

    /// Retrieves the IP address part of the remote socket address.
    ///
    /// # Returns
    ///
    /// - `OptionSocketHost` - The IP address of the remote peer if available.
    pub async fn try_get_socket_host(&self) -> OptionSocketHost {
        self.try_get_socket_addr()
            .await
            .map(|socket_addr: SocketAddr| socket_addr.ip())
    }

    /// Retrieves the port number part of the remote socket address.
    ///
    /// # Returns
    ///
    /// - `OptionSocketPort` - The port number of the remote peer if available.
    pub async fn try_get_socket_port(&self) -> OptionSocketPort {
        self.try_get_socket_addr()
            .await
            .map(|socket_addr: SocketAddr| socket_addr.port())
    }

    /// Retrieves the current HTTP request.
    ///
    /// # Returns
    ///
    /// - `Request` - A clone of the current request.
    pub async fn get_request(&self) -> Request {
        self.read().await.get_request().clone()
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
    pub(crate) async fn set_request(&self, request_data: &Request) -> &Self {
        self.write().await.set_request(request_data.clone());
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
        func(self.read().await.get_request().clone()).await
    }

    /// Retrieves the string representation of the current request.
    ///
    /// # Returns
    ///
    /// - `String` - The full request as a string.
    pub async fn get_request_string(&self) -> String {
        self.read().await.get_request().get_string()
    }

    /// Retrieves the HTTP version of the request.
    ///
    /// # Returns
    ///
    /// - `RequestVersion` - The HTTP version of the request.
    pub async fn get_request_version(&self) -> RequestVersion {
        self.read().await.get_request().get_version().clone()
    }

    /// Retrieves the HTTP method of the request.
    ///
    /// # Returns
    ///
    /// - `RequestMethod` - The HTTP method of the request.
    pub async fn get_request_method(&self) -> RequestMethod {
        self.read().await.get_request().get_method().clone()
    }

    /// Retrieves the host from the request headers.
    ///
    /// # Returns
    ///
    /// - `RequestHost` - The host part of the request's URI.
    pub async fn get_request_host(&self) -> RequestHost {
        self.read().await.get_request().get_host().clone()
    }

    /// Retrieves the path of the request.
    ///
    /// # Returns
    ///
    /// - `RequestPath` - The path part of the request's URI.
    pub async fn get_request_path(&self) -> RequestPath {
        self.read().await.get_request().get_path().clone()
    }

    /// Retrieves the query parameters of the request.
    ///
    /// # Returns
    ///
    /// - `RequestQuerys` - A map containing the query parameters.
    pub async fn get_request_querys(&self) -> RequestQuerys {
        self.read().await.get_request().get_querys().clone()
    }

    /// Retrieves a specific query parameter by its key.
    ///
    /// # Arguments
    ///
    /// - `AsRef<str>` - The query parameter key.
    ///
    /// # Returns
    ///
    /// - `OptionRequestQuerysValue` - The query parameter value if exists.
    pub async fn try_get_request_query<K>(&self, key: K) -> OptionRequestQuerysValue
    where
        K: AsRef<str>,
    {
        self.read().await.get_request().try_get_query(key)
    }

    /// Retrieves the body of the request.
    ///
    /// # Returns
    ///
    /// - `RequestBody` - A clone of the request's body.
    pub async fn get_request_body(&self) -> RequestBody {
        self.read().await.get_request().get_body().clone()
    }

    /// Retrieves the request body as a string.
    ///
    /// # Returns
    ///
    /// - `String` - The request body converted to a string.
    pub async fn get_request_body_string(&self) -> String {
        self.read().await.get_request().get_body_string()
    }

    /// Deserializes the request body from JSON into a specified type.
    ///
    /// # Returns
    ///
    /// - `ResultJsonError<J>` - The deserialized type `J` or a JSON error.
    pub async fn get_request_body_json<J>(&self) -> ResultJsonError<J>
    where
        J: DeserializeOwned,
    {
        self.read().await.get_request().get_body_json()
    }

    /// Retrieves a specific request header by its key.
    ///
    /// Gets a request header by key.
    ///
    /// # Arguments
    ///
    /// - `AsRef<str>` - The header key.
    ///
    /// # Returns
    ///
    /// - `OptionRequestHeadersValue` - The header values if exists.
    pub async fn try_get_request_header<K>(&self, key: K) -> OptionRequestHeadersValue
    where
        K: AsRef<str>,
    {
        self.read().await.get_request().try_get_header(key)
    }

    /// Retrieves all request headers.
    ///
    /// # Returns
    ///
    /// - `RequestHeaders` - A clone of the request's header map.
    pub async fn get_request_headers(&self) -> RequestHeaders {
        self.read().await.get_request().get_headers().clone()
    }

    /// Retrieves the first value of a specific request header.
    ///
    /// # Arguments
    ///
    /// - `AsRef<str>` - The key of the header.
    ///
    /// # Returns
    ///
    /// - `OptionRequestHeadersValueItem` - The first value of the header if it exists.
    pub async fn try_get_request_header_front<K>(&self, key: K) -> OptionRequestHeadersValueItem
    where
        K: AsRef<str>,
    {
        self.read().await.get_request().try_get_header_front(key)
    }

    /// Retrieves the last value of a specific request header.
    ///
    /// # Arguments
    ///
    /// - `AsRef<str>` - The key of the header.
    ///
    /// # Returns
    ///
    /// - `OptionRequestHeadersValueItem` - The last value of the header if it exists.
    pub async fn try_get_request_header_back<K>(&self, key: K) -> OptionRequestHeadersValueItem
    where
        K: AsRef<str>,
    {
        self.read().await.get_request().try_get_header_back(key)
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
    pub async fn get_request_header_len<K>(&self, key: K) -> usize
    where
        K: AsRef<str>,
    {
        self.read().await.get_request().get_header_length(key)
    }

    /// Retrieves the total number of values across all request headers.
    ///
    /// # Returns
    ///
    /// - `usize` - The total count of all values in all headers.
    pub async fn get_request_headers_values_length(&self) -> usize {
        self.read().await.get_request().get_headers_values_length()
    }

    /// Retrieves the total number of request headers.
    ///
    /// # Returns
    ///
    /// - `usize` - The total number of headers in the request.
    pub async fn get_request_headers_length(&self) -> usize {
        self.read().await.get_request().get_headers_length()
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
    pub async fn get_request_has_header<K>(&self, key: K) -> bool
    where
        K: AsRef<str>,
    {
        self.read().await.get_request().has_header(key)
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
    pub async fn get_request_has_header_value<K, V>(&self, key: K, value: V) -> bool
    where
        K: AsRef<str>,
        V: AsRef<str>,
    {
        self.read().await.get_request().has_header_value(key, value)
    }

    /// Parses and retrieves all cookies from the request headers.
    ///
    /// # Returns
    ///
    /// - `Cookies` - A map of cookies parsed from the request's Cookie header.
    pub async fn get_request_cookies(&self) -> Cookies {
        self.try_get_request_header_back(COOKIE)
            .await
            .map(|data| Cookie::parse(&data))
            .unwrap_or_default()
    }

    /// Retrieves a specific cookie by its name from the request.
    ///
    /// # Arguments
    ///
    /// - `AsRef<str>` - The cookie name.
    ///
    /// # Returns
    ///
    /// - `OptionCookiesValue` - The cookie value if exists.
    pub async fn try_get_request_cookie<K>(&self, key: K) -> OptionCookiesValue
    where
        K: AsRef<str>,
    {
        self.get_request_cookies().await.get(key.as_ref()).cloned()
    }

    /// Retrieves the upgrade type of the request.
    ///
    /// # Returns
    ///
    /// - `UpgradeType` - Indicates if the request is for a WebSocket connection.
    pub async fn get_request_upgrade_type(&self) -> UpgradeType {
        self.read().await.get_request().get_upgrade_type()
    }

    /// Checks if the request is a WebSocket upgrade request.
    ///
    /// # Returns
    ///
    /// - `bool` - True if this is a WebSocket upgrade request.
    pub async fn get_request_is_ws(&self) -> bool {
        self.read().await.get_request().is_ws()
    }

    /// Checks if the request is an HTTP/2 cleartext (h2c) upgrade.
    ///
    /// # Returns
    ///
    /// - `bool` - True if this is an h2c upgrade request.
    pub async fn get_request_is_h2c(&self) -> bool {
        self.read().await.get_request().is_h2c()
    }

    /// Checks if the request is a TLS upgrade.
    ///
    /// # Returns
    ///
    /// - `bool` - True if this is a TLS upgrade request.
    pub async fn get_request_is_tls(&self) -> bool {
        self.read().await.get_request().is_tls()
    }

    /// Checks if the request has an unknown upgrade type.
    ///
    /// # Returns
    ///
    /// - `bool` - True if the upgrade type is unknown.
    pub async fn get_request_is_unknown_upgrade(&self) -> bool {
        self.read().await.get_request().is_unknown_upgrade()
    }

    /// Checks if the request HTTP version is HTTP/1.1 or higher.
    ///
    /// # Returns
    ///
    /// - `bool` - True if the version is HTTP/1.1 or higher.
    pub async fn get_request_is_http1_1_or_higher(&self) -> bool {
        self.read().await.get_request().is_http1_1_or_higher()
    }

    /// Checks if the request HTTP version is HTTP/0.9.
    ///
    /// # Returns
    ///
    /// - `bool` - True if the version is HTTP/0.9.
    pub async fn get_request_is_http0_9(&self) -> bool {
        self.read().await.get_request().is_http0_9()
    }

    /// Checks if the request HTTP version is HTTP/1.0.
    ///
    /// # Returns
    ///
    /// - `bool` - True if the version is HTTP/1.0.
    pub async fn get_request_is_http1_0(&self) -> bool {
        self.read().await.get_request().is_http1_0()
    }

    /// Checks if the request HTTP version is HTTP/1.1.
    ///
    /// # Returns
    ///
    /// - `bool` - True if the version is HTTP/1.1.
    pub async fn get_request_is_http1_1(&self) -> bool {
        self.read().await.get_request().is_http1_1()
    }

    /// Checks if the request HTTP version is HTTP/2.
    ///
    /// # Returns
    ///
    /// - `bool` - True if the version is HTTP/2.
    pub async fn get_request_is_http2(&self) -> bool {
        self.read().await.get_request().is_http2()
    }

    /// Checks if the request HTTP version is HTTP/3.
    ///
    /// # Returns
    ///
    /// - `bool` - True if the version is HTTP/3.
    pub async fn get_request_is_http3(&self) -> bool {
        self.read().await.get_request().is_http3()
    }

    /// Checks if the request has an unknown HTTP version.
    ///
    /// # Returns
    ///
    /// - `bool` - True if the version is unknown.
    pub async fn get_request_is_unknown_version(&self) -> bool {
        self.read().await.get_request().is_unknown_version()
    }

    /// Checks if the request uses HTTP protocol.
    ///
    /// # Returns
    ///
    /// - `bool` - True if the version belongs to HTTP family.
    pub async fn get_request_is_http(&self) -> bool {
        self.read().await.get_request().is_http()
    }

    /// Checks if the request method is GET.
    ///
    /// # Returns
    ///
    /// - `bool` - True if the method is GET.
    pub async fn get_request_is_get(&self) -> bool {
        self.read().await.get_request().is_get()
    }

    /// Checks if the request method is POST.
    ///
    /// # Returns
    ///
    /// - `bool` - True if the method is POST.
    pub async fn get_request_is_post(&self) -> bool {
        self.read().await.get_request().is_post()
    }

    /// Checks if the request method is PUT.
    ///
    /// # Returns
    ///
    /// - `bool` - True if the method is PUT.
    pub async fn get_request_is_put(&self) -> bool {
        self.read().await.get_request().is_put()
    }

    /// Checks if the request method is DELETE.
    ///
    /// # Returns
    ///
    /// - `bool` - True if the method is DELETE.
    pub async fn get_request_is_delete(&self) -> bool {
        self.read().await.get_request().is_delete()
    }

    /// Checks if the request method is PATCH.
    ///
    /// # Returns
    ///
    /// - `bool` - True if the method is PATCH.
    pub async fn get_request_is_patch(&self) -> bool {
        self.read().await.get_request().is_patch()
    }

    /// Checks if the request method is HEAD.
    ///
    /// # Returns
    ///
    /// - `bool` - True if the method is HEAD.
    pub async fn get_request_is_head(&self) -> bool {
        self.read().await.get_request().is_head()
    }

    /// Checks if the request method is OPTIONS.
    ///
    /// # Returns
    ///
    /// - `bool` - True if the method is OPTIONS.
    pub async fn get_request_is_options(&self) -> bool {
        self.read().await.get_request().is_options()
    }

    /// Checks if the request method is CONNECT.
    ///
    /// # Returns
    ///
    /// - `bool` - True if the method is CONNECT.
    pub async fn get_request_is_connect(&self) -> bool {
        self.read().await.get_request().is_connect()
    }

    /// Checks if the request method is TRACE.
    ///
    /// # Returns
    ///
    /// - `bool` - True if the method is TRACE.
    pub async fn get_request_is_trace(&self) -> bool {
        self.read().await.get_request().is_trace()
    }

    /// Checks if the request method is unknown.
    ///
    /// # Returns
    ///
    /// - `bool` - True if the method is unknown.
    pub async fn get_request_is_unknown_method(&self) -> bool {
        self.read().await.get_request().is_unknown_method()
    }

    /// Checks if the connection should be kept alive based on request headers.
    ///
    /// # Returns
    ///
    /// - `bool` - True if the Connection header suggests keeping the connection alive, otherwise false.
    pub async fn get_request_is_enable_keep_alive(&self) -> bool {
        self.read().await.get_request().is_enable_keep_alive()
    }

    /// Checks if keep-alive should be disabled for the request.
    ///
    /// # Returns
    ///
    /// - `bool` - True if keep-alive should be disabled.
    pub async fn get_request_is_disable_keep_alive(&self) -> bool {
        self.read().await.get_request().is_disable_keep_alive()
    }

    /// Retrieves the current HTTP response.
    ///
    /// # Returns
    ///
    /// - `Response` - A clone of the current response.
    pub async fn get_response(&self) -> Response {
        self.read().await.get_response().clone()
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
    pub async fn set_response<T>(&self, response: T) -> &Self
    where
        T: Borrow<Response>,
    {
        self.write().await.set_response(response.borrow().clone());
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
        func(self.read().await.get_response().clone()).await
    }

    /// Retrieves the string representation of the current response.
    ///
    /// # Returns
    ///
    /// - `String` - The full response as a string.
    pub async fn get_response_string(&self) -> String {
        self.read().await.get_response().get_string()
    }

    /// Retrieves the HTTP version of the response.
    ///
    /// # Returns
    ///
    /// - `ResponseVersion` - The HTTP version of the response.
    pub async fn get_response_version(&self) -> ResponseVersion {
        self.read().await.get_response().get_version().clone()
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
    pub async fn set_response_version(&self, version: ResponseVersion) -> &Self {
        self.write().await.get_mut_response().set_version(version);
        self
    }

    /// Retrieves all response headers.
    ///
    /// # Returns
    ///
    /// - `ResponseHeaders` - A clone of the response's header map.
    pub async fn get_response_headers(&self) -> ResponseHeaders {
        self.read().await.get_response().get_headers().clone()
    }

    /// Retrieves a specific response header by its key.
    ///
    /// # Arguments
    ///
    /// - `AsRef<str>` - The key of the header to retrieve.
    ///
    /// # Returns
    ///
    /// - `OptionResponseHeadersValue` - The header values if the header exists.
    pub async fn try_get_response_header<K>(&self, key: K) -> OptionResponseHeadersValue
    where
        K: AsRef<str>,
    {
        self.read().await.get_response().try_get_header(key)
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
    pub async fn set_response_header<K, V>(&self, key: K, value: V) -> &Self
    where
        K: AsRef<str>,
        V: AsRef<str>,
    {
        self.write().await.get_mut_response().set_header(key, value);
        self
    }

    /// Retrieves the first value of a specific response header.
    ///
    /// # Arguments
    ///
    /// - `AsRef<str>` - The key of the header.
    ///
    /// # Returns
    ///
    /// - `OptionResponseHeadersValueItem` - The first value of the header if it exists.
    pub async fn try_get_response_header_front<K>(&self, key: K) -> OptionResponseHeadersValueItem
    where
        K: AsRef<str>,
    {
        self.read().await.get_response().try_get_header_front(key)
    }

    /// Retrieves the last value of a specific response header.
    ///
    /// # Arguments
    ///
    /// - `AsRef<str>` - The key of the header.
    ///
    /// # Returns
    ///
    /// - `OptionResponseHeadersValueItem` - The last value of the header if it exists.
    pub async fn try_get_response_header_back<K>(&self, key: K) -> OptionResponseHeadersValueItem
    where
        K: AsRef<str>,
    {
        self.read().await.get_response().try_get_header_back(key)
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
    pub async fn get_response_has_header<K>(&self, key: K) -> bool
    where
        K: AsRef<str>,
    {
        self.read().await.get_response().has_header(key)
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
    pub async fn get_response_header_value<K, V>(&self, key: K, value: V) -> bool
    where
        K: AsRef<str>,
        V: AsRef<str>,
    {
        self.read()
            .await
            .get_response()
            .has_header_value(key, value)
    }

    /// Retrieves the total number of response headers.
    ///
    /// # Returns
    ///
    /// - `usize` - The total number of headers in the response.
    pub async fn get_response_headers_length(&self) -> usize {
        self.read().await.get_response().get_headers_length()
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
    pub async fn get_response_header_length<K>(&self, key: K) -> usize
    where
        K: AsRef<str>,
    {
        self.read().await.get_response().get_header_length(key)
    }

    /// Retrieves the total number of values across all response headers.
    ///
    /// # Returns
    ///
    /// - `usize` - The total count of all values in all headers.
    pub async fn get_response_headers_values_length(&self) -> usize {
        self.read().await.get_response().get_headers_values_length()
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
    pub async fn add_response_header<K, V>(&self, key: K, value: V) -> &Self
    where
        K: AsRef<str>,
        V: AsRef<str>,
    {
        self.write().await.get_mut_response().add_header(key, value);
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
    pub async fn remove_response_header<K>(&self, key: K) -> &Self
    where
        K: AsRef<str>,
    {
        self.write().await.get_mut_response().remove_header(key);
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
    pub async fn remove_response_header_value<K, V>(&self, key: K, value: V) -> &Self
    where
        K: AsRef<str>,
        V: AsRef<str>,
    {
        self.write()
            .await
            .get_mut_response()
            .remove_header_value(key, value);
        self
    }

    /// Clears all headers from the response.
    ///
    /// # Returns
    ///
    /// - `&Self` - Reference to the modified context.
    pub async fn clear_response_headers(&self) -> &Self {
        self.write().await.get_mut_response().clear_headers();
        self
    }

    /// Parses and retrieves all cookies from the response headers.
    ///
    /// # Returns
    ///
    /// - `Cookies` - A map of cookies parsed from the response's Cookie header.
    pub async fn get_response_cookies(&self) -> Cookies {
        self.try_get_response_header_back(COOKIE)
            .await
            .map(|data| Cookie::parse(&data))
            .unwrap_or_default()
    }

    /// Retrieves a specific cookie by its name from the response.
    ///
    /// # Arguments
    ///
    /// - `AsRef<str>` - The name of the cookie to retrieve.
    ///
    /// # Returns
    ///
    /// - `OptionCookiesValue` - The cookie's value if it exists.
    pub async fn try_get_response_cookie<K>(&self, key: K) -> OptionCookiesValue
    where
        K: AsRef<str>,
    {
        self.get_response_cookies().await.get(key.as_ref()).cloned()
    }

    /// Retrieves the body of the response.
    ///
    /// # Returns
    ///
    /// - `ResponseBody` - A clone of the response's body.
    pub async fn get_response_body(&self) -> ResponseBody {
        self.read().await.get_response().get_body().clone()
    }

    /// Sets the body of the response.
    ///
    /// # Arguments
    ///
    /// - `B` - The body to set for the response.
    ///
    /// # Returns
    ///
    /// - `&Self` - Reference to the modified context.
    pub async fn set_response_body<B>(&self, body: B) -> &Self
    where
        B: AsRef<[u8]>,
    {
        self.write().await.get_mut_response().set_body(body);
        self
    }

    /// Retrieves the response body as a string.
    ///
    /// # Returns
    ///
    /// - `String` - The response body converted to a string.
    pub async fn get_response_body_string(&self) -> String {
        self.read().await.get_response().get_body_string()
    }

    /// Deserializes the response body from JSON into a specified type.
    ///
    /// # Returns
    ///
    /// - `ResultJsonError<J>` - The deserialized type `J` or a JSON error.
    pub async fn get_response_body_json<J>(&self) -> ResultJsonError<J>
    where
        J: DeserializeOwned,
    {
        self.read().await.get_response().get_body_json()
    }

    /// Retrieves the reason phrase of the response's status code.
    ///
    /// # Returns
    ///
    /// - `ResponseReasonPhrase` - The reason phrase associated with the response's status code.
    pub async fn get_response_reason_phrase(&self) -> ResponseReasonPhrase {
        self.read().await.get_response().get_reason_phrase().clone()
    }

    /// Sets the reason phrase for the response's status code.
    ///
    /// # Arguments
    ///
    /// - `AsRef<str>` - The reason phrase to set.
    ///
    /// # Returns
    ///
    /// - `&Self` - Reference to the modified context.
    pub async fn set_response_reason_phrase<P>(&self, reason_phrase: P) -> &Self
    where
        P: AsRef<str>,
    {
        self.write()
            .await
            .get_mut_response()
            .set_reason_phrase(reason_phrase);
        self
    }

    /// Retrieves the status code of the response.
    ///
    /// # Returns
    ///
    /// The status code of the response.
    pub async fn get_response_status_code(&self) -> ResponseStatusCode {
        self.read().await.get_response().get_status_code().clone()
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
    pub async fn set_response_status_code(&self, status_code: ResponseStatusCode) -> &Self {
        self.write()
            .await
            .get_mut_response()
            .set_status_code(status_code);
        self
    }

    /// Retrieves the parameters extracted from the route path.
    ///
    /// # Returns
    ///
    /// - `RouteParams` - A map containing the route parameters.
    pub async fn get_route_params(&self) -> RouteParams {
        self.read().await.get_route_params().clone()
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
    pub(crate) async fn set_route_params(&self, params: RouteParams) -> &Self {
        self.write().await.set_route_params(params);
        self
    }

    /// Retrieves a specific route parameter by its name.
    ///
    /// # Arguments
    ///
    /// - `AsRef<str>` - The name of the route parameter to retrieve.
    ///
    /// # Returns
    ///
    /// - `OptionString` - The value of the route parameter if it exists.
    pub async fn try_get_route_param<T>(&self, name: T) -> OptionString
    where
        T: AsRef<str>,
    {
        self.read()
            .await
            .get_route_params()
            .get(name.as_ref())
            .cloned()
    }

    /// Retrieves all attributes stored in the context.
    ///
    /// # Returns
    ///
    /// - `HashMapArcAnySendSync` - A map containing all attributes.
    pub async fn get_attributes(&self) -> HashMapArcAnySendSync {
        self.read().await.get_attributes().clone()
    }

    /// Retrieves a specific attribute by its key, casting it to the specified type.
    ///
    /// # Arguments
    ///
    /// - `AsRef<str>` - The key of the attribute to retrieve.
    ///
    /// # Returns
    ///
    /// - `Option<V>` - The attribute's value if it exists and can be cast to the specified type.
    pub async fn try_get_attribute<K, V>(&self, key: K) -> Option<V>
    where
        K: AsRef<str>,
        V: AnySendSyncClone,
    {
        self.read()
            .await
            .get_attributes()
            .get(&Attribute::External(key.as_ref().to_owned()).to_string())
            .and_then(|arc| arc.downcast_ref::<V>())
            .cloned()
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
    pub async fn set_attribute<K, V>(&self, key: K, value: V) -> &Self
    where
        K: AsRef<str>,
        V: AnySendSyncClone,
    {
        self.write().await.get_mut_attributes().insert(
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
    pub async fn remove_attribute<K>(&self, key: K) -> &Self
    where
        K: AsRef<str>,
    {
        self.write()
            .await
            .get_mut_attributes()
            .remove(&Attribute::External(key.as_ref().to_owned()).to_string());
        self
    }

    /// Clears all attributes from the context.
    ///
    /// # Returns
    ///
    /// - `&Self` - A reference to the modified context.
    pub async fn clear_attribute(&self) -> &Self {
        self.write().await.get_mut_attributes().clear();
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
    /// - `Option<V>` - The attribute's value if it exists and can be cast to the specified type.
    async fn try_get_internal_attribute<V>(&self, key: InternalAttribute) -> Option<V>
    where
        V: AnySendSyncClone,
    {
        self.read()
            .await
            .get_attributes()
            .get(&Attribute::Internal(key).to_string())
            .and_then(|arc| arc.downcast_ref::<V>())
            .cloned()
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
    async fn set_internal_attribute<V>(&self, key: InternalAttribute, value: V) -> &Self
    where
        V: AnySendSyncClone,
    {
        self.write()
            .await
            .get_mut_attributes()
            .insert(Attribute::Internal(key).to_string(), Arc::new(value));
        self
    }

    /// Retrieves panic information if a panic has occurred during handling.
    ///
    /// # Returns
    ///
    /// - `OptionPanic` - The panic information if a panic was caught.
    pub async fn try_get_panic(&self) -> OptionPanic {
        self.try_get_internal_attribute(InternalAttribute::Panic)
            .await
    }

    /// Sets the panic information for the context.
    ///
    /// # Arguments
    ///
    /// - `Panic` - The panic information to store.
    ///
    /// # Returns
    ///
    /// - `&Self` - A reference to the modified context.
    pub(crate) async fn set_panic(&self, panic: Panic) -> &Self {
        self.set_internal_attribute(InternalAttribute::Panic, panic)
            .await
    }

    /// Sets the send function for the context.
    ///
    /// # Arguments
    ///
    /// - `F: FnContextSendSyncStatic<Fut, ()>, Fut: FutureSendStatic<()>` - The send function to store.
    ///
    /// # Returns
    ///
    /// - `&Self` - A reference to the modified context.
    pub async fn set_send_hook<F, Fut>(&self, hook: F) -> &Self
    where
        F: FnContextSendSyncStatic<Fut, ()>,
        Fut: FutureSendStatic<()>,
    {
        let send_hook: ArcFnContextPinBoxSendSync<()> =
            Arc::new(move |ctx: Context| -> PinBoxFutureSend<()> { Box::pin(hook(ctx)) });
        self.set_internal_attribute(InternalAttribute::SendHook, send_hook)
            .await
    }

    /// Retrieves the send function if it has been set.
    ///
    /// # Returns
    ///
    /// - `OptionArcFnContextPinBoxSendSync<()>` - The send function if it has been set.
    pub async fn try_get_send_hook(&self) -> OptionArcFnContextPinBoxSendSync<()> {
        self.try_get_internal_attribute(InternalAttribute::SendHook)
            .await
    }

    /// Sets the send body function for the context.
    ///
    /// # Arguments
    ///
    /// - `F` - The send body function to store.
    ///
    /// # Returns
    ///
    /// - `&Self` - A reference to the modified context.
    pub async fn set_send_body_hook<F, Fut>(&self, hook: F) -> &Self
    where
        F: FnContextSendSyncStatic<Fut, ()>,
        Fut: FutureSendStatic<()>,
    {
        let send_body_hook: ArcFnContextPinBoxSendSync<()> =
            Arc::new(move |ctx: Context| -> PinBoxFutureSend<()> { Box::pin(hook(ctx)) });
        self.set_internal_attribute(InternalAttribute::SendBodyHook, send_body_hook)
            .await
    }

    /// Retrieves the send body function if it has been set.
    ///
    /// # Returns
    ///
    /// - `OptionArcFnContextPinBoxSendSync<()>` - The send body function if it has been set.
    pub async fn try_get_send_body_hook(&self) -> OptionArcFnContextPinBoxSendSync<()> {
        self.try_get_internal_attribute(InternalAttribute::SendBodyHook)
            .await
    }

    /// Updates the lifecycle status based on the current context state.
    ///
    /// # Arguments
    ///
    /// - `&mut Lifecycle` - The lifecycle to update.
    pub(crate) async fn update_lifecycle_status(&self, lifecycle: &mut Lifecycle) {
        let keep_alive: bool = !self.get_closed().await && lifecycle.is_keep_alive();
        let aborted: bool = self.get_aborted().await;
        lifecycle.update_status(aborted, keep_alive);
    }

    /// Sends the response headers and body to the client.
    ///
    /// # Returns
    ///
    /// - `ResponseResult` - The outcome of the send operation.
    pub async fn send(&self) -> ResponseResult {
        let response_data: ResponseData = self.write().await.get_mut_response().build();
        self.send_with_data(response_data).await
    }

    /// Sends the response and then closes the connection.
    ///
    /// After sending, the connection will be marked as closed.
    ///
    /// # Returns
    ///
    /// - `ResponseResult` - The outcome of the send operation.
    pub async fn send_once(&self) -> ResponseResult {
        let response_data: ResponseData = self.write().await.get_mut_response().build();
        self.send_once_with_data(response_data).await
    }

    /// Sends only the response body to the client.
    ///
    /// This is useful for streaming data or for responses where headers have already been sent.
    ///
    /// # Returns
    ///
    /// - `ResponseResult` - The outcome of the send operation.
    pub async fn send_body(&self) -> ResponseResult {
        let response_body: ResponseBody = self.get_response_body().await;
        self.send_body_with_data(response_body).await
    }

    /// Sends only the response body and then closes the connection.
    ///
    /// After sending the body, the connection will be marked as closed.
    ///
    /// # Returns
    ///
    /// - `ResponseResult` - The outcome of the send operation.
    pub async fn send_body_once(&self) -> ResponseResult {
        let response_body: ResponseBody = self.get_response_body().await;
        self.send_body_once_with_data(response_body).await
    }

    /// Sends the response headers and body to the client with additional data.
    ///
    /// # Arguments
    ///
    /// - `AsRef<[u8]>` - The additional data to send.
    ///
    /// # Returns
    ///
    /// - `ResponseResult` - The outcome of the send operation.
    pub async fn send_with_data<D>(&self, data: D) -> ResponseResult
    where
        D: AsRef<[u8]>,
    {
        if self.is_terminated().await {
            return Err(ResponseError::Terminated);
        }
        if let Some(stream) = self.try_get_stream().await {
            return stream.send(data).await;
        }
        Err(ResponseError::NotFoundStream)
    }

    /// Sends the response and then closes the connection with additional data.
    ///
    /// After sending, the connection will be marked as closed.
    ///
    /// # Arguments
    ///
    /// - `AsRef<[u8]>` - The additional data to send.
    ///
    /// # Returns
    ///
    /// - `ResponseResult` - The outcome of the send operation.
    pub async fn send_once_with_data<D>(&self, data: D) -> ResponseResult
    where
        D: AsRef<[u8]>,
    {
        let res: ResponseResult = self.send_with_data(data).await;
        self.closed().await;
        res
    }

    /// Sends only the response body to the client with additional data.
    ///
    /// This is useful for streaming data or for responses where headers have already been sent.
    ///
    /// # Arguments
    ///
    /// - `AsRef<[u8]>` - The additional data to send as the body.
    ///
    /// # Returns
    ///
    /// - `ResponseResult` - The outcome of the send operation.
    pub async fn send_body_with_data<D>(&self, data: D) -> ResponseResult
    where
        D: AsRef<[u8]>,
    {
        if self.is_terminated().await {
            return Err(ResponseError::Terminated);
        }
        if let Some(stream) = self.try_get_stream().await {
            return stream.send_body(data).await;
        }
        Err(ResponseError::NotFoundStream)
    }

    /// Sends only the response body and then closes the connection with additional data.
    ///
    /// After sending the body, the connection will be marked as closed.
    ///
    /// # Arguments
    ///
    /// - `AsRef<[u8]>` - The additional data to send as the body.
    ///
    /// # Returns
    ///
    /// - `ResponseResult` - The outcome of the send operation.
    pub async fn send_body_once_with_data<D>(&self, data: D) -> ResponseResult
    where
        D: AsRef<[u8]>,
    {
        let res: ResponseResult = self.send_body_with_data(data).await;
        self.closed().await;
        res
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
    /// - `ResponseResult` - The outcome of the send operation.
    pub async fn send_body_list_with_data<I, D>(&self, data_iter: I) -> ResponseResult
    where
        I: IntoIterator<Item = D>,
        D: AsRef<[u8]>,
    {
        if self.is_terminated().await {
            return Err(ResponseError::Terminated);
        }
        if let Some(stream) = self.try_get_stream().await {
            return stream.send_body_list(data_iter).await;
        }
        Err(ResponseError::NotFoundStream)
    }

    /// Sends a list of response bodies and then closes the connection with additional data.
    ///
    /// After sending the body list, the connection will be marked as closed.
    ///
    /// # Arguments
    ///
    /// - `I: IntoIterator<Item = D>, D: AsRef<[u8]>` - The additional data to send as a list of bodies.
    ///
    /// # Returns
    ///
    /// - `ResponseResult` - The outcome of the send operation.
    pub async fn send_body_list_once_with_data<I, D>(&self, data_iter: I) -> ResponseResult
    where
        I: IntoIterator<Item = D>,
        D: AsRef<[u8]>,
    {
        let res: ResponseResult = self.send_body_list_with_data(data_iter).await;
        self.closed().await;
        res
    }

    /// Flushes the underlying network stream, ensuring all buffered data is sent.
    ///
    /// # Returns
    ///
    /// - `ResponseResult` - The outcome of the flush operation.
    pub async fn flush(&self) -> ResponseResult {
        if let Some(stream) = self.try_get_stream().await {
            stream.flush().await;
            return Ok(());
        }
        Err(ResponseError::NotFoundStream)
    }

    /// Reads an HTTP request from the underlying stream.
    ///
    /// # Arguments
    ///
    /// - `usize` - The read buffer size.
    ///
    /// # Returns
    ///
    /// - `RequestReaderHandleResult` - The parsed request or error.
    pub async fn http_from_stream(&self, buffer: usize) -> RequestReaderHandleResult {
        if self.get_aborted().await {
            return Err(RequestError::RequestAborted);
        }
        if let Some(stream) = self.try_get_stream().await.as_ref() {
            let request_res: RequestReaderHandleResult =
                Request::http_from_stream(stream, buffer).await;
            if let Ok(request) = request_res.as_ref() {
                self.set_request(request).await;
            }
            return request_res;
        };
        Err(RequestError::GetTcpStream)
    }

    /// Reads a WebSocket frame from the underlying stream.
    ///
    /// # Arguments
    ///
    /// - `usize` - The read buffer size.
    ///
    /// # Returns
    ///
    /// - `RequestReaderHandleResult` - The parsed frame or error.
    pub async fn ws_from_stream(&self, buffer: usize) -> RequestReaderHandleResult {
        if self.get_aborted().await {
            return Err(RequestError::RequestAborted);
        }
        if let Some(stream) = self.try_get_stream().await.as_ref() {
            let mut last_request: Request = self.get_request().await;
            let request_res: RequestReaderHandleResult =
                Request::ws_from_stream(stream, buffer, &mut last_request).await;
            match request_res.as_ref() {
                Ok(request) => {
                    self.set_request(&request).await;
                }
                Err(_) => {
                    self.set_request(&last_request).await;
                }
            }
            return request_res;
        };
        Err(RequestError::GetTcpStream)
    }
}
