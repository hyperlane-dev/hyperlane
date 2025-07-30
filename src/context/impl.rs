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
    /// - `ArcRwLockStream` - The network stream.
    /// - `Request` - The HTTP request.
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

    /// Retrieves the underlying network stream, if available.
    ///
    /// # Returns
    ///
    /// - `OptionArcRwLockStream` - The thread-safe, shareable network stream if it exists.
    pub async fn get_stream(&self) -> OptionArcRwLockStream {
        self.read().await.get_stream().clone()
    }

    /// Retrieves the remote socket address of the connection.
    ///
    /// # Returns
    ///
    /// - `OptionSocketAddr` - The socket address of the remote peer if available.
    pub async fn get_socket_addr(&self) -> OptionSocketAddr {
        let stream_result: OptionArcRwLockStream = self.get_stream().await;
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
    pub async fn get_socket_addr_or_default(&self) -> SocketAddr {
        let stream_result: OptionArcRwLockStream = self.get_stream().await;
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
    pub async fn get_socket_addr_string(&self) -> OptionString {
        self.get_socket_addr().await.map(|data| data.to_string())
    }

    /// Retrieves the remote socket address as a string, or a default value if unavailable.
    ///
    /// # Returns
    ///
    /// - `String` - The string representation of the socket address, or default if unavailable.
    pub async fn get_socket_addr_or_default_string(&self) -> String {
        self.get_socket_addr_or_default().await.to_string()
    }

    /// Retrieves the IP address part of the remote socket address.
    ///
    /// # Returns
    ///
    /// - `OptionSocketHost` - The IP address of the remote peer if available.
    pub async fn get_socket_host(&self) -> OptionSocketHost {
        self.get_socket_addr()
            .await
            .map(|socket_addr: SocketAddr| socket_addr.ip())
    }

    /// Retrieves the port number part of the remote socket address.
    ///
    /// # Returns
    ///
    /// - `OptionSocketPort` - The port number of the remote peer if available.
    pub async fn get_socket_port(&self) -> OptionSocketPort {
        self.get_socket_addr()
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
    /// - `RequestHeadersKey` - The query parameter key.
    ///
    /// # Returns
    ///
    /// - `OptionRequestQuerysValue` - The query parameter value if exists.
    pub async fn get_request_query<K>(&self, key: K) -> OptionRequestQuerysValue
    where
        K: Into<RequestHeadersKey>,
    {
        self.read().await.get_request().get_query(key)
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
    /// - `RequestHeadersKey` - The header key.
    ///
    /// # Returns
    ///
    /// - `OptionRequestHeadersValue` - The header values if exists.
    pub async fn get_request_header<K>(&self, key: K) -> OptionRequestHeadersValue
    where
        K: Into<RequestHeadersKey>,
    {
        self.read().await.get_request().get_header(key)
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
    /// - `K` - The key of the header.
    ///
    /// # Returns
    ///
    /// - `OptionRequestHeadersValueItem` - The first value of the header if it exists.
    pub async fn get_request_header_front<K>(&self, key: K) -> OptionRequestHeadersValueItem
    where
        K: Into<RequestHeadersKey>,
    {
        self.read().await.get_request().get_header_front(key)
    }

    /// Retrieves the last value of a specific request header.
    ///
    /// # Arguments
    ///
    /// - `K` - The key of the header.
    ///
    /// # Returns
    ///
    /// - `OptionRequestHeadersValueItem` - The last value of the header if it exists.
    pub async fn get_request_header_back<K>(&self, key: K) -> OptionRequestHeadersValueItem
    where
        K: Into<RequestHeadersKey>,
    {
        self.read().await.get_request().get_header_back(key)
    }

    /// Retrieves the number of values for a specific request header.
    ///
    /// # Arguments
    ///
    /// - `K` - The key of the header.
    ///
    /// # Returns
    ///
    /// - `usize` - The number of values for the specified header.
    pub async fn get_request_header_len<K>(&self, key: K) -> usize
    where
        K: Into<RequestHeadersKey>,
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
    /// - `K` - The key of the header to check.
    ///
    /// # Returns
    ///
    /// - `bool` - True if the header exists, otherwise false.
    pub async fn has_request_header<K>(&self, key: K) -> bool
    where
        K: Into<RequestHeadersKey>,
    {
        self.read().await.get_request().has_header(key)
    }

    /// Checks if a request header has a specific value.
    ///
    /// # Arguments
    ///
    /// - `RequestHeadersKey` - The header key.
    /// - `RequestHeadersValueItem` - The value to check.
    ///
    /// # Returns
    ///
    /// - `bool` - True if header contains the value.
    pub async fn has_request_header_value<K, V>(&self, key: K, value: V) -> bool
    where
        K: Into<RequestHeadersKey>,
        V: Into<RequestHeadersValueItem>,
    {
        self.read().await.get_request().has_header_value(key, value)
    }

    /// Parses and retrieves all cookies from the request headers.
    ///
    /// # Returns
    ///
    /// - `Cookies` - A map of cookies parsed from the request's Cookie header.
    pub async fn get_request_cookies(&self) -> Cookies {
        self.get_request_header_back(COOKIE)
            .await
            .map(|data| Cookie::parse(&data))
            .unwrap_or_default()
    }

    /// Retrieves a specific cookie by its name from the request.
    ///
    /// # Arguments
    ///
    /// - `CookieKey` - The cookie name.
    ///
    /// # Returns
    ///
    /// - `OptionCookiesValue` - The cookie value if exists.
    pub async fn get_request_cookie<K>(&self, key: K) -> OptionCookiesValue
    where
        K: Into<CookieKey>,
    {
        self.get_request_cookies().await.get(&key.into()).cloned()
    }

    /// Retrieves the upgrade type of the request, e.g., for WebSockets.
    ///
    /// # Returns
    ///
    /// - `UpgradeType` - Indicates if the request is for a WebSocket connection.
    pub async fn get_request_upgrade_type(&self) -> UpgradeType {
        self.read().await.get_request().get_upgrade_type()
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
    /// - `Response` - The response to set in the context.
    ///
    /// # Returns
    ///
    /// - `&Self` - Reference to the modified context.
    pub async fn set_response(&self, response: Response) -> &Self {
        self.write().await.set_response(response);
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
    /// - `K` - The key of the header to retrieve.
    ///
    /// # Returns
    ///
    /// - `OptionResponseHeadersValue` - The header values if the header exists.
    pub async fn get_response_header<K>(&self, key: K) -> OptionResponseHeadersValue
    where
        K: Into<ResponseHeadersKey>,
    {
        self.read().await.get_response().get_header(key)
    }

    /// Sets a response header, adding it if it doesn't exist or appending to it if it does.
    ///
    /// # Arguments
    ///
    /// - `String` - The header key.
    /// - `String` - The header value.
    ///
    /// # Returns
    ///
    /// - `&Self` - Reference to self for method chaining.
    pub async fn set_response_header<K, V>(&self, key: K, value: V) -> &Self
    where
        K: Into<String>,
        V: Into<String>,
    {
        self.write().await.get_mut_response().set_header(key, value);
        self
    }

    /// Retrieves the first value of a specific response header.
    ///
    /// # Arguments
    ///
    /// - `K` - The key of the header.
    ///
    /// # Returns
    ///
    /// - `OptionResponseHeadersValueItem` - The first value of the header if it exists.
    pub async fn get_response_header_front<K>(&self, key: K) -> OptionResponseHeadersValueItem
    where
        K: Into<ResponseHeadersKey>,
    {
        self.read().await.get_response().get_header_front(key)
    }

    /// Retrieves the last value of a specific response header.
    ///
    /// # Arguments
    ///
    /// - `K` - The key of the header.
    ///
    /// # Returns
    ///
    /// - `OptionResponseHeadersValueItem` - The last value of the header if it exists.
    pub async fn get_response_header_back<K>(&self, key: K) -> OptionResponseHeadersValueItem
    where
        K: Into<ResponseHeadersKey>,
    {
        self.read().await.get_response().get_header_back(key)
    }

    /// Checks if a specific response header exists.
    ///
    /// # Arguments
    ///
    /// - `K` - The key of the header to check.
    ///
    /// # Returns
    ///
    /// - `bool` - True if the header exists, otherwise false.
    pub async fn get_response_has_header<K>(&self, key: K) -> bool
    where
        K: Into<ResponseHeadersKey>,
    {
        self.read().await.get_response().has_header(key)
    }

    /// Checks if a response header has a specific value.
    ///
    /// # Arguments
    ///
    /// - `K` - The key of the header.
    /// - `V` - The value to check for.
    ///
    /// # Returns
    ///
    /// - `bool` - True if the header contains the specified value, otherwise false.
    pub async fn has_response_header_value<K, V>(&self, key: K, value: V) -> bool
    where
        K: Into<ResponseHeadersKey>,
        V: Into<ResponseHeadersValueItem>,
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
    /// - `K` - The key of the header.
    ///
    /// # Returns
    ///
    /// - `usize` - The number of values for the specified header.
    pub async fn get_response_header_len<K>(&self, key: K) -> usize
    where
        K: Into<ResponseHeadersKey>,
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

    /// Replaces a response header with a new value, removing any existing values.
    ///
    /// # Arguments
    ///
    /// - `K` - The key of the header to replace.
    /// - `V` - The new value for the header.
    ///
    /// # Returns
    ///
    /// - `&Self` - Reference to the modified context.
    pub async fn replace_response_header<K, V>(&self, key: K, value: V) -> &Self
    where
        K: Into<ResponseHeadersKey>,
        V: Into<String>,
    {
        self.write()
            .await
            .get_mut_response()
            .replace_header(key, value);
        self
    }

    /// Removes a response header and all its values.
    ///
    /// # Arguments
    ///
    /// - `K` - The key of the header to remove.
    ///
    /// # Returns
    ///
    /// - `&Self` - Reference to the modified context.
    pub async fn remove_response_header<K>(&self, key: K) -> &Self
    where
        K: Into<ResponseHeadersKey>,
    {
        self.write().await.get_mut_response().remove_header(key);
        self
    }

    /// Removes a specific value from a response header.
    ///
    /// # Arguments
    ///
    /// - `ResponseHeadersKey` - The header key.
    /// - `String` - The value to remove.
    ///
    /// # Returns
    ///
    /// - `&Self` - Reference to self for method chaining.
    pub async fn remove_response_header_value<K, V>(&self, key: K, value: V) -> &Self
    where
        K: Into<ResponseHeadersKey>,
        V: Into<String>,
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
        self.get_response_header_back(COOKIE)
            .await
            .map(|data| Cookie::parse(&data))
            .unwrap_or_default()
    }

    /// Retrieves a specific cookie by its name from the response.
    ///
    /// # Arguments
    ///
    /// - `K` - The name of the cookie to retrieve.
    ///
    /// # Returns
    ///
    /// - `OptionCookiesValue` - The cookie's value if it exists.
    pub async fn get_response_cookie<K>(&self, key: K) -> OptionCookiesValue
    where
        K: Into<CookieKey>,
    {
        self.get_response_cookies().await.get(&key.into()).cloned()
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
        B: Into<ResponseBody>,
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
    /// - `P` - The reason phrase to set.
    ///
    /// # Returns
    ///
    /// - `&Self` - Reference to the modified context.
    pub async fn set_response_reason_phrase<P>(&self, reason_phrase: P) -> &Self
    where
        P: Into<ResponseReasonPhrase>,
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
    /// - `status_code` - The status code to set for the response.
    ///
    /// # Returns
    ///
    /// A reference to the modified context.
    pub async fn set_response_status_code(&self, status_code: ResponseStatusCode) -> &Self {
        self.write()
            .await
            .get_mut_response()
            .set_status_code(status_code);
        self
    }

    /// Resets the response body to be empty.
    ///
    /// # Returns
    ///
    /// A reference to the modified context.
    pub async fn reset_response_body(&self) -> &Self {
        self.set_response_body(ResponseBody::default()).await;
        self
    }

    /// Retrieves the parameters extracted from the route path.
    ///
    /// # Returns
    ///
    /// A map containing the route parameters.
    pub async fn get_route_params(&self) -> RouteParams {
        self.read().await.get_route_params().clone()
    }

    /// Sets the route parameters for the context.
    ///
    /// # Arguments
    ///
    /// - `params` - The route parameters to set.
    ///
    /// # Returns
    ///
    /// A reference to the modified `Context`.
    pub(crate) async fn set_route_params(&self, params: RouteParams) -> &Self {
        self.write().await.set_route_params(params);
        self
    }

    /// Retrieves a specific route parameter by its name.
    ///
    /// # Arguments
    ///
    /// - `name` - The name of the route parameter to retrieve.
    ///
    /// # Returns
    ///
    /// The value of the route parameter if it exists.
    pub async fn get_route_param(&self, name: &str) -> OptionString {
        self.read().await.get_route_params().get(name).cloned()
    }

    /// Retrieves all attributes stored in the context.
    ///
    /// # Returns
    ///
    /// A map containing all attributes.
    pub async fn get_attributes(&self) -> HashMapArcAnySendSync {
        self.read().await.get_attributes().clone()
    }

    /// Retrieves a specific attribute by its key, casting it to the specified type.
    ///
    /// # Arguments
    ///
    /// - `key` - The key of the attribute to retrieve.
    ///
    /// # Returns
    ///
    /// The attribute's value if it exists and can be cast to the specified type.
    pub async fn get_attribute<V>(&self, key: &str) -> Option<V>
    where
        V: AnySendSyncClone,
    {
        self.read()
            .await
            .get_attributes()
            .get(&AttributeKey::External(key.to_owned()).to_string())
            .and_then(|arc| arc.downcast_ref::<V>())
            .cloned()
    }

    /// Sets an attribute in the context.
    ///
    /// # Arguments
    ///
    /// - `key` - The key of the attribute to set.
    /// - `value` - The value of the attribute.
    ///
    /// # Returns
    ///
    /// A reference to the modified context.
    pub async fn set_attribute<V>(&self, key: &str, value: V) -> &Self
    where
        V: AnySendSyncClone,
    {
        self.write().await.get_mut_attributes().insert(
            AttributeKey::External(key.to_owned()).to_string(),
            Arc::new(value),
        );
        self
    }

    /// Removes an attribute from the context.
    ///
    /// # Arguments
    ///
    /// - `key` - The key of the attribute to remove.
    ///
    /// # Returns
    ///
    /// A reference to the modified context.
    pub async fn remove_attribute(&self, key: &str) -> &Self {
        self.write()
            .await
            .get_mut_attributes()
            .remove(&AttributeKey::External(key.to_owned()).to_string());
        self
    }

    /// Clears all attributes from the context.
    ///
    /// # Returns
    ///
    /// A reference to the modified context.
    pub async fn clear_attribute(&self) -> &Self {
        self.write().await.get_mut_attributes().clear();
        self
    }

    /// Retrieves an internal framework attribute.
    ///
    /// # Arguments
    ///
    /// - `key` - The internal attribute key to retrieve.
    ///
    /// # Returns
    ///
    /// The attribute's value if it exists and can be cast to the specified type.
    async fn get_internal_attribute<V>(&self, key: InternalAttributeKey) -> Option<V>
    where
        V: AnySendSyncClone,
    {
        self.read()
            .await
            .get_attributes()
            .get(&AttributeKey::Internal(key).to_string())
            .and_then(|arc| arc.downcast_ref::<V>())
            .cloned()
    }

    /// Sets an internal framework attribute.
    ///
    /// # Arguments
    ///
    /// - `key` - The internal attribute key to set.
    /// - `value` - The value of the attribute.
    ///
    /// # Returns
    ///
    /// A reference to the modified context.
    async fn set_internal_attribute<V>(&self, key: InternalAttributeKey, value: V) -> &Self
    where
        V: AnySendSyncClone,
    {
        self.write()
            .await
            .get_mut_attributes()
            .insert(AttributeKey::Internal(key).to_string(), Arc::new(value));
        self
    }

    /// Retrieves panic information if a panic has occurred during handling.
    ///
    /// # Returns
    ///
    /// The panic information if a panic was caught.
    pub async fn get_panic(&self) -> OptionPanic {
        self.get_internal_attribute(InternalAttributeKey::Panic)
            .await
    }

    /// Sets the panic information for the context.
    ///
    /// # Arguments
    ///
    /// - `panic` - The panic information to store.
    ///
    /// # Returns
    ///
    /// A reference to the modified context.
    pub(crate) async fn set_panic(&self, panic: Panic) -> &Self {
        self.set_internal_attribute(InternalAttributeKey::Panic, panic)
            .await
    }

    /// Checks if the connection has been terminated (aborted and closed).
    ///
    /// # Returns
    ///
    /// True if the connection is both aborted and closed, otherwise false.
    pub async fn is_terminated(&self) -> bool {
        self.get_aborted().await || self.get_closed().await
    }

    /// Checks if the connection should be kept alive based on request headers.
    ///
    /// # Returns
    ///
    /// True if the Connection header suggests keeping the connection alive, otherwise false.
    pub async fn is_enable_keep_alive(&self) -> bool {
        self.get_request().await.is_enable_keep_alive()
    }

    /// Handles the WebSocket upgrade handshake and sends the appropriate response.
    ///
    /// This method constructs and sends the WebSocket handshake response if the
    /// request is a valid upgrade request.
    ///
    /// # Returns
    ///
    /// The outcome of the handshake operation.
    pub async fn upgrade_to_ws(&self) -> ResponseResult {
        if let Some(key) = &self.get_request_header_back(SEC_WEBSOCKET_KEY).await {
            let accept_key: String = WebSocketFrame::generate_accept_key(key);
            let result: ResponseResult = self
                .set_response_version(HttpVersion::HTTP1_1)
                .await
                .set_response_status_code(101)
                .await
                .replace_response_header(UPGRADE, WEBSOCKET)
                .await
                .replace_response_header(CONNECTION, UPGRADE)
                .await
                .replace_response_header(SEC_WEBSOCKET_ACCEPT, accept_key)
                .await
                .send()
                .await;
            return result;
        }
        Err(ResponseError::WebSocketHandShake(format!(
            "missing {} header",
            SEC_WEBSOCKET_KEY
        )))
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
        self.reset_response_body().await;
        if self.get_aborted().await {
            return Err(RequestError::RequestAborted);
        }
        if let Some(stream) = self.get_stream().await.as_ref() {
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
        self.reset_response_body().await;
        if self.get_aborted().await {
            return Err(RequestError::RequestAborted);
        }
        if let Some(stream) = self.get_stream().await.as_ref() {
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

    /// Updates the lifecycle status based on the current context state.
    ///
    /// # Arguments
    ///
    /// - `lifecycle` - The lifecycle to update.
    pub(crate) async fn update_lifecycle_status(&self, lifecycle: &mut Lifecycle) {
        let keep_alive: bool = !self.get_closed().await && lifecycle.is_keep_alive();
        let aborted: bool = self.get_aborted().await;
        lifecycle.update_status(aborted, keep_alive);
    }

    /// Sends the response headers and body to the client.
    ///
    /// # Returns
    ///
    /// The outcome of the send operation.
    pub async fn send(&self) -> ResponseResult {
        if self.is_terminated().await {
            return Err(ResponseError::Terminated);
        }
        if let Some(stream) = self.get_stream().await {
            let response_res: ResponseData = self.write().await.get_mut_response().build();
            return stream.send(&response_res).await;
        }
        Err(ResponseError::NotFoundStream)
    }

    /// Sends the response and then closes the connection.
    ///
    /// After sending, the connection will be marked as closed.
    ///
    /// # Returns
    ///
    /// The outcome of the send operation.
    pub async fn send_once(&self) -> ResponseResult {
        let res: ResponseResult = self.send().await;
        self.closed().await;
        res
    }

    /// Sends only the response body to the client.
    ///
    /// This is useful for streaming data or for responses where headers have already been sent.
    ///
    /// # Returns
    ///
    /// The outcome of the send operation.
    pub async fn send_body(&self) -> ResponseResult {
        if self.is_terminated().await {
            return Err(ResponseError::Terminated);
        }
        if let Some(stream) = self.get_stream().await {
            let is_ws: bool = self.get_request_upgrade_type().await.is_ws();
            let response_body: ResponseBody = self.get_response_body().await;
            return stream.send_body_conditional(&response_body, is_ws).await;
        }
        Err(ResponseError::NotFoundStream)
    }

    /// Sends only the response body and then closes the connection.
    ///
    /// After sending the body, the connection will be marked as closed.
    ///
    /// # Returns
    ///
    /// The outcome of the send operation.
    pub async fn send_once_body(&self) -> ResponseResult {
        let res: ResponseResult = self.send_body().await;
        self.closed().await;
        res
    }

    /// Flushes the underlying network stream, ensuring all buffered data is sent.
    ///
    /// # Returns
    ///
    /// The outcome of the flush operation.
    pub async fn flush(&self) -> ResponseResult {
        if let Some(stream) = self.get_stream().await {
            stream.flush().await;
            return Ok(());
        }
        Err(ResponseError::NotFoundStream)
    }
}
