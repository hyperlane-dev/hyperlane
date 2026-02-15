use crate::*;

/// Implementation of `Default` trait for `Context`.
impl Default for Context {
    /// Creates a default `Context` instance.
    ///
    /// # Returns
    ///
    /// - `Context` - A new context with default values and a static default server.
    #[inline(always)]
    fn default() -> Self {
        Self {
            aborted: false,
            closed: false,
            stream: None,
            request: Request::default(),
            response: Response::default(),
            route_params: RouteParams::default(),
            attributes: ThreadSafeAttributeStore::default(),
            server: default_server(),
        }
    }
}

/// Implementation of `PartialEq` trait for `Context`.
impl PartialEq for Context {
    /// Compares two `Context` instances for equality.
    ///
    /// # Arguments
    ///
    /// - `&Self` - The first `Context` instance.
    /// - `&Self` - The second `Context` instance.
    ///
    /// # Returns
    ///
    /// - `bool` - True if the instances are equal, otherwise false.
    #[inline(always)]
    fn eq(&self, other: &Self) -> bool {
        self.get_aborted() == other.get_aborted()
            && self.get_closed() == other.get_closed()
            && self.get_request() == other.get_request()
            && self.get_response() == other.get_response()
            && self.get_route_params() == other.get_route_params()
            && self.get_attributes().len() == other.get_attributes().len()
            && self.try_get_stream().is_some() == other.try_get_stream().is_some()
            && self.get_server() == other.get_server()
    }
}

/// Implementation of `Eq` trait for `Context`.
impl Eq for Context {}

/// Implementation of `From` trait for `Context` from `&'static Server`.
impl From<&'static Server> for Context {
    /// Converts a `&'static Server` into a `Context` with default request and response.
    ///
    /// # Arguments
    ///
    /// - `&'static Server` - The server reference to convert.
    ///
    /// # Returns
    ///
    /// - `Context` - The newly created context instance.
    #[inline(always)]
    fn from(server: &'static Server) -> Self {
        let mut ctx: Context = Context::default();
        ctx.set_server(server);
        ctx
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
        let mut ctx: Context = Context::default();
        ctx.set_stream(Some(stream.clone()));
        ctx
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

/// Implementation of `From` trait for converting `usize` address into `Context`.
impl From<usize> for Context {
    /// Converts a memory address into an owned `Context` by cloning from the reference.
    ///
    /// # Arguments
    ///
    /// - `usize` - The memory address of the `Context` instance.
    ///
    /// # Returns
    ///
    /// - `Context` - A cloned `Context` instance from the given address.
    #[inline(always)]
    fn from(addr: usize) -> Self {
        let ctx: &Context = addr.into();
        ctx.clone()
    }
}

/// Implementation of `From` trait for converting `usize` address into `&Context`.
impl From<usize> for &'static Context {
    /// Converts a memory address into a reference to `Context`.
    ///
    /// # Arguments
    ///
    /// - `usize` - The memory address of the `Context` instance.
    ///
    /// # Returns
    ///
    /// - `&'static Context` - A reference to the `Context` at the given address.
    #[inline(always)]
    fn from(addr: usize) -> &'static Context {
        unsafe { &*(addr as *const Context) }
    }
}

/// Implementation of `From` trait for converting `usize` address into `&mut Context`.
impl From<usize> for &'static mut Context {
    /// Converts a memory address into a mutable reference to `Context`.
    ///
    /// # Arguments
    ///
    /// - `usize` - The memory address of the `Context` instance.
    ///
    /// # Returns
    ///
    /// - `&'static mut Context` - A mutable reference to the `Context` at the given address.
    #[inline(always)]
    fn from(addr: usize) -> &'static mut Context {
        unsafe { &mut *(addr as *mut Context) }
    }
}

/// Implementation of `From` trait for converting `&Context` into `usize` address.
impl From<&Context> for usize {
    /// Converts a reference to `Context` into its memory address.
    ///
    /// # Arguments
    ///
    /// - `&Context` - The reference to the `Context` instance.
    ///
    /// # Returns
    ///
    /// - `usize` - The memory address of the `Context` instance.
    #[inline(always)]
    fn from(ctx: &Context) -> Self {
        ctx as *const Context as usize
    }
}

/// Implementation of `From` trait for converting `&mut Context` into `usize` address.
impl From<&mut Context> for usize {
    /// Converts a mutable reference to `Context` into its memory address.
    ///
    /// # Arguments
    ///
    /// - `&mut Context` - The mutable reference to the `Context` instance.
    ///
    /// # Returns
    ///
    /// - `usize` - The memory address of the `Context` instance.
    #[inline(always)]
    fn from(ctx: &mut Context) -> Self {
        ctx as *mut Context as usize
    }
}

/// Implementation of methods for `Context` structure.
impl Context {
    /// Creates a new `Context` with the provided network stream, HTTP request and server.
    ///
    /// # Arguments
    ///
    /// - `&ArcRwLockStream` - The network stream.
    /// - `&Request` - The HTTP request.
    /// - `&'static Server` - The server.
    ///
    /// # Returns
    ///
    /// - `Context` - The newly created context.
    #[inline(always)]
    pub(crate) fn new(
        stream: &ArcRwLockStream,
        request: &Request,
        server: &'static Server,
    ) -> Context {
        let mut ctx: Context = Context::default();
        ctx.set_stream(Some(stream.clone()))
            .set_request(request.clone())
            .set_server(server)
            .get_mut_response()
            .set_version(request.get_version().clone());
        ctx
    }

    /// Reads an HTTP request from the underlying stream.
    ///
    /// # Returns
    ///
    /// - `Result<Request, RequestError>` - The parsed request or error.
    pub async fn http_from_stream(&mut self) -> Result<Request, RequestError> {
        if self.get_aborted() {
            return Err(RequestError::RequestAborted(HttpStatus::BadRequest));
        }
        if let Some(stream) = self.try_get_stream().as_ref() {
            let request_res: Result<Request, RequestError> =
                Request::http_from_stream(stream, self.get_server().get_request_config()).await;
            if let Ok(request) = request_res.as_ref() {
                self.set_request(request.clone());
            }
            return request_res;
        };
        Err(RequestError::GetTcpStream(HttpStatus::BadRequest))
    }

    /// Reads a WebSocket frame from the underlying stream.
    ///
    /// # Returns
    ///
    /// - `Result<Request, RequestError>` - The parsed frame or error.
    pub async fn ws_from_stream(&mut self) -> Result<Request, RequestError> {
        if self.get_aborted() {
            return Err(RequestError::RequestAborted(HttpStatus::BadRequest));
        }
        if let Some(stream) = self.try_get_stream().as_ref() {
            let last_request: &Request = self.get_request();
            let request_res: Result<Request, RequestError> = last_request
                .ws_from_stream(stream, self.get_server().get_request_config())
                .await;
            match request_res.as_ref() {
                Ok(request) => {
                    self.set_request(request.clone());
                }
                Err(_) => {
                    self.set_request(last_request.clone());
                }
            }
            return request_res;
        };
        Err(RequestError::GetTcpStream(HttpStatus::BadRequest))
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
    pub(crate) fn is_keep_alive(&self, keep_alive: bool) -> bool {
        !self.get_closed() && keep_alive
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
        self.get_route_params().get(name.as_ref()).cloned()
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
    pub fn try_get_attribute<V>(&self, key: impl AsRef<str>) -> Option<V>
    where
        V: AnySendSyncClone,
    {
        self.get_attributes()
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
    /// - `AnySendSyncClone` - The attribute value if it exists and can be cast to the specified type.
    ///
    /// # Panics
    ///
    /// - If the attribute is not found.
    #[inline(always)]
    pub fn get_attribute<V>(&self, key: impl AsRef<str>) -> V
    where
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
    /// - `&mut Self` - A reference to the modified context.
    #[inline(always)]
    pub fn set_attribute<K, V>(&mut self, key: K, value: V) -> &mut Self
    where
        K: AsRef<str>,
        V: AnySendSyncClone,
    {
        self.get_mut_attributes().insert(
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
    /// - `&mut Self` - A reference to the modified context.
    #[inline(always)]
    pub fn remove_attribute<K>(&mut self, key: K) -> &mut Self
    where
        K: AsRef<str>,
    {
        self.get_mut_attributes()
            .remove(&Attribute::External(key.as_ref().to_owned()).to_string());
        self
    }

    /// Clears all attributes from the context.
    ///
    /// # Returns
    ///
    /// - `&mut Self` - A reference to the modified context.
    #[inline(always)]
    pub fn clear_attribute(&mut self) -> &mut Self {
        self.get_mut_attributes().clear();
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
        self.get_attributes()
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
    /// - `AnySendSyncClone` - The attribute value if it exists and can be cast to the specified type.
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
    /// - `&mut Self` - A reference to the modified context.
    #[inline(always)]
    fn set_internal_attribute<V>(&mut self, key: InternalAttribute, value: V) -> &mut Self
    where
        V: AnySendSyncClone,
    {
        self.get_mut_attributes()
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
    /// - `&mut Self` - Reference to the modified context for method chaining.
    #[inline(always)]
    pub(crate) fn set_task_panic(&mut self, panic_data: PanicData) -> &mut Self {
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
    /// - `&mut Self` - A reference to the modified context.
    #[inline(always)]
    pub(crate) fn set_request_error_data(&mut self, request_error: RequestError) -> &mut Self {
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

    /// Sends HTTP response data over the stream.
    ///
    /// # Returns
    ///
    /// - `Result<(), ResponseError>` - Result indicating success or failure.
    pub async fn try_send(&mut self) -> Result<(), ResponseError> {
        if self.is_terminated() {
            return Err(ResponseError::Terminated);
        }
        let response_data: ResponseData = self.get_mut_response().build();
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
    pub async fn send(&mut self) {
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
        self.try_send_body_with_data(self.get_response().get_body())
            .await
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
