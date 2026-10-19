use super::*;

/// Provides a default implementation for Server.
impl Default for Server {
    /// Creates a new Server instance with default values.
    ///
    /// # Returns
    ///
    /// - `Self` - A new instance with default configuration.
    #[inline(always)]
    fn default() -> Self {
        Self {
            server_config: ServerConfig::default(),
            request_config: RequestConfig::default(),
            task_panic: Vec::new(),
            request_error: Vec::new(),
            route_matcher: RouteMatcher::new(),
            request_middleware: Vec::new(),
            response_middleware: Vec::new(),
        }
    }
}

/// Implements the `PartialEq` trait for `Server`.
///
/// This allows for comparing two `Server` instances for equality.
impl PartialEq for Server {
    /// Checks if two `Server` instances are equal.
    ///
    /// # Arguments
    ///
    /// - `&Self` - The other `Server` instance to compare against.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` if the instances are equal, `false` otherwise.
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.get_server_config() == other.get_server_config()
            && self.get_request_config() == other.get_request_config()
            && self.get_route_matcher() == other.get_route_matcher()
            && self.get_task_panic().len() == other.get_task_panic().len()
            && self.get_request_error().len() == other.get_request_error().len()
            && self.get_request_middleware().len() == other.get_request_middleware().len()
            && self.get_response_middleware().len() == other.get_response_middleware().len()
            && self
                .get_task_panic()
                .iter()
                .zip(other.get_task_panic().iter())
                .all(|pair: (&ServerHookHandler, &ServerHookHandler)| Arc::ptr_eq(pair.0, pair.1))
            && self
                .get_request_error()
                .iter()
                .zip(other.get_request_error().iter())
                .all(|pair: (&ServerHookHandler, &ServerHookHandler)| Arc::ptr_eq(pair.0, pair.1))
            && self
                .get_request_middleware()
                .iter()
                .zip(other.get_request_middleware().iter())
                .all(|pair: (&ServerHookHandler, &ServerHookHandler)| Arc::ptr_eq(pair.0, pair.1))
            && self
                .get_response_middleware()
                .iter()
                .zip(other.get_response_middleware().iter())
                .all(|pair: (&ServerHookHandler, &ServerHookHandler)| Arc::ptr_eq(pair.0, pair.1))
    }
}

/// Implements the `Eq` trait for `Server`.
///
/// This indicates that `Server` has a total equality relation.
impl Eq for Server {}

/// Implementation of `From` trait for converting `usize` address into `Server`.
impl From<usize> for Server {
    /// Converts a memory address into an owned `Server` by cloning from the reference.
    ///
    /// # Arguments
    ///
    /// - `usize` - The memory address of the `Server` instance.
    ///
    /// # Returns
    ///
    /// - `Server` - A cloned `Server` instance from the given address.
    #[inline(always)]
    fn from(address: usize) -> Self {
        let server: &Server = address.into();
        server.clone()
    }
}

/// Implementation of `From` trait for converting `usize` address into `&Server`.
impl From<usize> for &'static Server {
    /// Converts a memory address into a reference to `Server`.
    ///
    /// # Arguments
    ///
    /// - `usize` - The memory address of the `Server` instance.
    ///
    /// # Returns
    ///
    /// - `&'static Server` - A reference to the `Server` at the given address.
    ///
    /// # Safety
    ///
    /// - The address is guaranteed to be a valid `Server` instance
    ///   that was previously converted from a reference and is managed by the runtime.
    #[inline(always)]
    fn from(address: usize) -> &'static Server {
        unsafe { &*(address as *const Server) }
    }
}

/// Implementation of `From` trait for converting `usize` address into `&mut Server`.
impl From<usize> for &'static mut Server {
    /// Converts a memory address into a mutable reference to `Server`.
    ///
    /// # Arguments
    ///
    /// - `usize` - The memory address of the `Server` instance.
    ///
    /// # Returns
    ///
    /// - `&'static mut Server` - A mutable reference to the `Server` at the given address.
    ///
    /// # Safety
    ///
    /// - The address is guaranteed to be a valid `Server` instance
    ///   that was previously converted from a reference and is managed by the runtime.
    #[inline(always)]
    fn from(address: usize) -> &'static mut Server {
        unsafe { &mut *(address as *mut Server) }
    }
}

/// Implementation of `From` trait for converting `&Server` into `usize` address.
impl From<&Server> for usize {
    /// Converts a reference to `Server` into its memory address.
    ///
    /// # Arguments
    ///
    /// - `&Server` - The reference to the `Server` instance.
    ///
    /// # Returns
    ///
    /// - `usize` - The memory address of the `Server` instance.
    #[inline(always)]
    fn from(server: &Server) -> Self {
        server as *const Server as usize
    }
}

/// Implementation of `From` trait for converting `&mut Server` into `usize` address.
impl From<&mut Server> for usize {
    /// Converts a mutable reference to `Server` into its memory address.
    ///
    /// # Arguments
    ///
    /// - `&mut Server` - The mutable reference to the `Server` instance.
    ///
    /// # Returns
    ///
    /// - `usize` - The memory address of the `Server` instance.
    #[inline(always)]
    fn from(server: &mut Server) -> Self {
        server as *mut Server as usize
    }
}

/// Implementation of `AsRef` trait for `Server`.
impl AsRef<Server> for Server {
    /// Converts `&Server` to `&Server` via memory address conversion.
    ///
    /// # Returns
    ///
    /// - `&Self` - A reference to the `Server` instance.
    #[inline(always)]
    fn as_ref(&self) -> &Self {
        let address: usize = self.into();
        address.into()
    }
}

/// Implementation of `AsMut` trait for `Server`.
impl AsMut<Server> for Server {
    /// Converts `&mut Server` to `&mut Server` via memory address conversion.
    ///
    /// # Returns
    ///
    /// - `&mut Self` - A mutable reference to the `Server` instance.
    #[inline(always)]
    fn as_mut(&mut self) -> &mut Self {
        let address: usize = self.into();
        address.into()
    }
}

/// Converts a `ServerConfig` into a `Server` instance.
///
/// This allows creating a `Server` directly from its configuration,
/// using default values for other fields.
impl From<ServerConfig> for Server {
    /// Creates a new `Server` instance from the given `ServerConfig`.
    ///
    /// # Arguments
    ///
    /// - `ServerConfig` - The server configuration to use.
    ///
    /// # Returns
    ///
    /// - `Self` - A new `Server` instance with the provided configuration.
    #[inline(always)]
    fn from(server_config: ServerConfig) -> Self {
        Self {
            server_config,
            ..Default::default()
        }
    }
}

/// Converts a `RequestConfig` into a `Server` instance.
///
/// This allows creating a `Server` directly from its request configuration,
/// using default values for other fields.
impl From<RequestConfig> for Server {
    /// Creates a new `Server` instance from the given `RequestConfig`.
    ///
    /// # Arguments
    ///
    /// - `RequestConfig` - The request configuration to use.
    ///
    /// # Returns
    ///
    /// - `Self` - A new `Server` instance with the provided request configuration.
    #[inline(always)]
    fn from(request_config: RequestConfig) -> Self {
        Self {
            request_config,
            ..Default::default()
        }
    }
}

/// Implementation of `Lifetime` trait for `Server`.
impl Lifetime for Server {
    /// Converts a reference to the server into a `'static` reference.
    ///
    /// # Returns
    ///
    /// - `&'static Self` - A reference to the server with a `'static` lifetime.
    ///
    /// # Safety
    ///
    /// - The address is guaranteed to be a valid `Server` instance
    ///   that was previously converted from a reference and is managed by the runtime.
    #[inline(always)]
    unsafe fn leak(&self) -> &'static Self {
        let address: usize = self.into();
        address.into()
    }

    /// Converts a reference to the server into a `'static` mutable reference.
    ///
    /// # Returns
    ///
    /// - `&'static mut Self` - A mutable reference to the server with a `'static` lifetime.
    ///
    /// # Safety
    ///
    /// - The address is guaranteed to be a valid `Server` instance
    ///   that was previously converted from a reference and is managed by the runtime.
    #[inline(always)]
    unsafe fn leak_mut(&self) -> &'static mut Self {
        let address: usize = self.into();
        address.into()
    }
}

/// Represents the server, providing methods to configure and run it.
///
/// This struct wraps the `Server` configuration and routing logic,
/// offering a high-level API for setting up the HTTP and WebSocket server.
impl Server {
    /// Registers a hook into the server's processing pipeline.
    ///
    /// This function dispatches the provided `HookType` to the appropriate
    /// internal hook collection based on its variant. The hook will be executed
    /// at the corresponding stage of request processing according to its type:
    /// - `Panic` - Added to panic handlers for error recovery
    /// - `RequestError` - Added to request error handlers
    /// - `RequestMiddleware` - Added to pre-route middleware chain
    /// - `Route` - Registered as a route handler for the specified path
    /// - `ResponseMiddleware` - Added to post-route middleware chain
    ///
    /// # Arguments
    ///
    /// - `HookType` - The `HookType` instance containing the hook configuration and factory.
    #[inline]
    pub fn handle_hook(&mut self, hook: HookType) {
        match hook {
            HookType::TaskPanic(_, hook) => {
                self.get_mut_task_panic().push(hook());
            }
            HookType::RequestError(_, hook) => {
                self.get_mut_request_error().push(hook());
            }
            HookType::RequestMiddleware(_, hook) => {
                self.get_mut_request_middleware().push(hook());
            }
            HookType::Route(path, hook) => {
                self.get_mut_route_matcher().add(path, hook()).unwrap();
            }
            HookType::ResponseMiddleware(_, hook) => {
                self.get_mut_response_middleware().push(hook());
            }
        };
    }

    /// Sets the server configuration from a JSON string.
    ///
    /// # Arguments
    ///
    /// - `C` - The configuration.
    ///
    /// # Returns
    ///
    /// - `&mut Self` - Reference to self for method chaining.
    #[inline]
    pub fn config_from_json<C>(&mut self, json: C) -> &mut Self
    where
        C: AsRef<str>,
    {
        let config: ServerConfig = serde_json::from_str(json.as_ref()).unwrap();
        self.set_server_config(config);
        self
    }

    /// Sets the server configuration.
    ///
    /// # Arguments
    ///
    /// - `ServerConfig` - The server configuration.
    ///
    /// # Returns
    ///
    /// - `&mut Self` - Reference to self for method chaining.
    #[inline(always)]
    pub fn server_config(&mut self, config: ServerConfig) -> &mut Self {
        self.set_server_config(config);
        self
    }

    /// Sets the HTTP request config.
    ///
    /// # Arguments
    ///
    /// - `RequestConfig` - The HTTP request config to set.
    ///
    /// # Returns
    ///
    /// - `&mut Self` - Reference to self for method chaining.
    #[inline(always)]
    pub fn request_config(&mut self, config: RequestConfig) -> &mut Self {
        self.set_request_config(config);
        self
    }

    /// Registers a task panic handler to the processing pipeline.
    ///
    /// This method allows registering task panic handlers that implement the `ServerHook` trait,
    /// which will be executed when a panic occurs during request processing.
    ///
    /// # Returns
    ///
    /// - `&mut Self` - Reference to self for method chaining.
    #[inline(always)]
    pub fn task_panic<S>(&mut self) -> &mut Self
    where
        S: ServerHook,
    {
        self.get_mut_task_panic().push(Hook::factory::<S>());
        self
    }

    /// Registers a request error handler to the processing pipeline.
    ///
    /// This method allows registering request error handlers that implement the `ServerHook` trait,
    /// which will be executed when a request error occurs during HTTP request processing.
    ///
    /// # Returns
    ///
    /// - `&mut Self` - Reference to self for method chaining.
    #[inline(always)]
    pub fn request_error<S>(&mut self) -> &mut Self
    where
        S: ServerHook,
    {
        self.get_mut_request_error().push(Hook::factory::<S>());
        self
    }

    /// Registers a route hook for a specific path.
    ///
    /// This method allows registering route handlers that implement the `ServerHook` trait,
    /// providing type safety and better code organization.
    ///
    /// # Arguments
    ///
    /// - `impl AsRef<str>` - The route path pattern.
    ///
    /// # Returns
    ///
    /// - `&mut Self` - Reference to self for method chaining.
    #[inline(always)]
    pub fn route<S>(&mut self, path: impl AsRef<str>) -> &mut Self
    where
        S: ServerHook,
    {
        self.get_mut_route_matcher()
            .add(path.as_ref(), Hook::factory::<S>())
            .unwrap();
        self
    }

    /// Registers request middleware to the processing pipeline.
    ///
    /// This method allows registering middleware that implements the `ServerHook` trait,
    /// which will be executed before route handlers for every incoming request.
    ///
    /// # Returns
    ///
    /// - `&mut Self` - Reference to self for method chaining.
    #[inline(always)]
    pub fn request_middleware<S>(&mut self) -> &mut Self
    where
        S: ServerHook,
    {
        self.get_mut_request_middleware().push(Hook::factory::<S>());
        self
    }

    /// Registers response middleware to the processing pipeline.
    ///
    /// This method allows registering middleware that implements the `ServerHook` trait,
    /// which will be executed after route handlers for every outgoing response.
    ///
    /// # Returns
    ///
    /// - `&mut Self` - Reference to self for method chaining.
    #[inline(always)]
    pub fn response_middleware<S>(&mut self) -> &mut Self
    where
        S: ServerHook,
    {
        self.get_mut_response_middleware()
            .push(Hook::factory::<S>());
        self
    }

    /// Format the host and port into a bindable address string.
    ///
    /// # Arguments
    ///
    /// - `H` - The host address.
    /// - `u16` - The port number.
    ///
    /// # Returns
    ///
    /// - `String` - The formatted address string in the form "host:port".
    #[inline(always)]
    pub fn format_bind_address<H>(host: H, port: u16) -> String
    where
        H: AsRef<str>,
    {
        format!("{}{COLON}{port}", host.as_ref())
    }

    /// Flushes the standard output stream.
    ///
    /// # Returns
    ///
    /// - `io::Result<()>` - The result of the flush operation.
    #[inline(always)]
    pub fn try_flush_stdout() -> io::Result<()> {
        stdout().flush()
    }

    /// Flushes the standard output stream.
    ///
    /// # Panics
    ///
    /// This function will panic if the flush operation fails.
    #[inline(always)]
    pub fn flush_stdout() {
        stdout().flush().unwrap();
    }

    /// Flushes the standard error stream.
    ///
    /// # Returns
    ///
    /// - `io::Result<()>` - The result of the flush operation.
    #[inline(always)]
    pub fn try_flush_stderr() -> io::Result<()> {
        stderr().flush()
    }

    /// Flushes the standard error stream.
    ///
    /// # Panics
    ///
    /// This function will panic if the flush operation fails.
    #[inline(always)]
    pub fn flush_stderr() {
        stderr().flush().unwrap();
    }

    /// Flushes both the standard output and error streams.
    ///
    /// # Returns
    ///
    /// - `io::Result<()>` - The result of the flush operation.
    #[inline(always)]
    pub fn try_flush_stdout_and_stderr() -> io::Result<()> {
        Self::try_flush_stdout()?;
        Self::try_flush_stderr()
    }

    /// Flushes both the standard output and error streams.
    ///
    /// # Panics
    ///
    /// This function will panic if either flush operation fails.
    #[inline(always)]
    pub fn flush_stdout_and_stderr() {
        Self::flush_stdout();
        Self::flush_stderr();
    }

    /// Executes trait-based request middleware in sequence.
    ///
    /// # Arguments
    ///
    /// - `&mut Stream` - The `Stream` for the current request.
    /// - `&mut Context` - The `Context` for the current request.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` if the lifecycle was aborted, `false` otherwise.
    pub(super) async fn handle_request_middleware(
        &self,
        stream: &mut Stream,
        ctx: &mut Context,
    ) -> bool {
        for hook in self.get_request_middleware().iter() {
            if hook(stream, ctx).await.is_reject() {
                return true;
            }
        }
        false
    }

    /// Executes a trait-based route hook if one matches.
    ///
    /// # Arguments
    ///
    /// - `&mut Stream` - The `Stream` for the current request.
    /// - `&mut Context` - The `Context` for the current request.
    /// - `&str` - The request path to match.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` if the lifecycle was aborted, `false` otherwise.
    pub(super) async fn handle_route_matcher(
        &self,
        stream: &mut Stream,
        ctx: &mut Context,
        path: &str,
    ) -> bool {
        if let Some(hook) = self.get_route_matcher().try_resolve_route(ctx, path)
            && hook(stream, ctx).await.is_reject()
        {
            return true;
        }
        false
    }

    /// Executes trait-based response middleware in sequence.
    ///
    /// # Arguments
    ///
    /// - `&mut Stream` - The `Stream` for the current request.
    /// - `&mut Context` - The `Context` for the current request.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` if the lifecycle was aborted, `false` otherwise.
    pub(super) async fn handle_response_middleware(
        &self,
        stream: &mut Stream,
        ctx: &mut Context,
    ) -> bool {
        for hook in self.get_response_middleware().iter() {
            if hook(stream, ctx).await.is_reject() {
                return true;
            }
        }
        false
    }

    /// Handles errors that occur while processing HTTP requests.
    ///
    /// # Arguments
    ///
    /// - `&mut Stream` - The `Stream` for the current request.
    /// - `&mut Context` - The `Context` for the current request.
    /// - `&RequestError` - The error that occurred.
    pub async fn handle_request_error(
        &self,
        stream: &mut Stream,
        ctx: &mut Context,
        error: &RequestError,
    ) {
        ctx.set_request_error_data(error.clone());
        stream.set_closed(false);
        for hook in self.get_request_error().iter() {
            if hook(stream, ctx).await.is_reject() {
                return;
            }
        }
    }

    /// The core request handling pipeline.
    ///
    /// This function orchestrates the execution of request middleware, the route hook,
    /// and response middleware. It supports both function-based and trait-based handlers.
    ///
    /// # Arguments
    ///
    /// - `&mut Stream` - The `Stream` for the current request.
    /// - `&mut Context` - The `Context` for the current request.
    /// - `Request` - The incoming request to be processed.
    ///
    /// # Returns
    ///
    /// - `bool` - A boolean indicating whether the connection should be kept alive.
    pub(crate) async fn request_hook(
        &self,
        stream: &mut Stream,
        ctx: &mut Context,
        request: Request,
    ) -> bool {
        let keep_alive: bool = request.is_enable_keep_alive();
        let version: RequestVersion = request.get_version().clone();
        let route: RequestPath = request.get_path().clone();
        ctx.set_request(request);
        ctx.get_mut_response().reset().set_version(version);
        ctx.clear_route_params();
        ctx.clear_attribute();
        stream.set_closed(false);
        if self.handle_request_middleware(stream, ctx).await {
            return stream.is_keep_alive(keep_alive);
        }
        if self.handle_route_matcher(stream, ctx, &route).await {
            return stream.is_keep_alive(keep_alive);
        }
        if self.handle_response_middleware(stream, ctx).await {
            return stream.is_keep_alive(keep_alive);
        }
        stream.is_keep_alive(keep_alive)
    }

    /// Handles subsequent HTTP requests on a persistent (keep-alive) connection.
    ///
    /// # Arguments
    ///
    /// - `&mut Stream` - The `Stream` for the current request.
    /// - `&mut Context` - The `Context` for the current request.
    /// - `Request` - The initial request that established the keep-alive connection.
    async fn handle_http_requests(&self, stream: &mut Stream, ctx: &mut Context, request: Request) {
        if !self.request_hook(stream, ctx, request).await {
            return;
        }
        loop {
            let mut reused_request: Request = mem::take(ctx.get_mut_request());
            match stream.try_fill_http_request(&mut reused_request).await {
                Ok(()) => {
                    if !self.request_hook(stream, ctx, reused_request).await {
                        return;
                    }
                }
                Err(error) => {
                    ctx.set_request(reused_request);
                    self.handle_request_error(stream, ctx, &error).await;
                    return;
                }
            }
        }
    }

    /// Handles a single client connection, determining whether it's an HTTP or WebSocket request.
    ///
    /// It reads the initial request from the stream and dispatches it to the appropriate hook.
    /// Both arguments are borrowed: the stream and the context belong to the
    /// caller, so this leaves them alive for whatever runs next — the panic
    /// hooks after an unwind, or the next request on a keep alive connection.
    ///
    /// # Arguments
    ///
    /// - `&mut Stream` - The `Stream` for the current request.
    /// - `&mut Context` - The `Context` for the current request.
    async fn handle_connection(&self, stream: &mut Stream, ctx: &mut Context) {
        match stream.try_get_http_request().await {
            Ok(request) => {
                self.handle_http_requests(stream, ctx, request).await;
            }
            Err(error) => {
                self.handle_request_error(stream, ctx, &error).await;
            }
        }
    }

    /// Serves one HTTP/1.1 connection through the existing keep alive pipeline.
    ///
    /// This is the same pipeline a plaintext connection has always used: the
    /// request parser runs in a loop, so keep alive, upgrades and raw writes
    /// behave exactly as they did before TLS and the multiplexed protocols
    /// existed. A handler that panics is caught here and reported through the
    /// existing task panic hooks, so the accept loop survives it and the hooks
    /// see the panic while the stream and context are still alive.
    ///
    /// # Arguments
    ///
    /// - `&mut Stream` - The stream carrying the connection.
    ///
    /// # Returns
    ///
    /// - `()` - Resolves once the connection is finished.
    pub(crate) async fn serve_h1_connection_stream(&self, stream: &mut Stream) {
        let mut ctx: Context = Context::default();
        let panicked: Option<PanicData> =
            catch_connection_panic(self.handle_connection(stream, &mut ctx)).await;
        let Some(panic) = panicked else {
            return;
        };
        ctx.set_task_panic(panic)
            .get_mut_response()
            .set_status_code(HttpStatus::InternalServerError.code());
        stream.set_closed(false);
        for hook in self.get_task_panic().iter() {
            if hook(stream, &mut ctx).await.is_reject() {
                break;
            }
        }
    }

    /// Serves one request that arrived on a multiplexed protocol stream.
    ///
    /// The lifecycle is the one an HTTP/1.1 request runs: the request
    /// middleware, the matched route and the response middleware, in that
    /// order, against the same context and the same error hook. A panic inside
    /// it is caught the same way a connection panic is.
    ///
    /// # Arguments
    ///
    /// - `&mut Stream` - The stream the pipeline writes its response onto.
    /// - `&mut Context` - The context for the current request.
    /// - `Request` - The request converted from the protocol head and body.
    ///
    /// # Returns
    ///
    /// - `()` - Resolves once the pipeline has finished writing.
    pub(crate) async fn run_protocol_lifecycle(
        &self,
        stream: &mut Stream,
        ctx: &mut Context,
        request: Request,
    ) {
        let panicked: Option<PanicData> = catch_connection_panic(async {
            let _keep_alive: bool = self.request_hook(stream, ctx, request).await;
        })
        .await;
        let Some(panic) = panicked else {
            return;
        };
        ctx.set_task_panic(panic)
            .get_mut_response()
            .set_status_code(HttpStatus::InternalServerError.code());
        stream.set_closed(false);
        for hook in self.get_task_panic().iter() {
            if hook(stream, ctx).await.is_reject() {
                break;
            }
        }
    }

    /// Starts the server, binds to the configured address, and begins listening.
    ///
    /// The configured protocol selects the listener. HTTP/1.1 and HTTP/2 share
    /// the TCP listener, the latter speaking its own preface and optionally
    /// wrapping the connection in TLS, while HTTP/3 binds a QUIC UDP endpoint.
    /// Invalid TLS material and an HTTP/3 configuration without TLS are both
    /// rejected before the socket is bound, so a misconfigured server fails to
    /// start rather than accepting connections it cannot serve.
    ///
    /// The accept loop owns a clone of the server instead of borrowing it, so
    /// running a short lived `Server` stays safe and dropping it leaves no
    /// leaked allocation behind.
    ///
    /// # Returns
    ///
    /// - `Result<ServerControlHook, Box<ServerError>>` - The shutdown hook on
    ///   success, or the failure that prevented the server from binding.
    pub async fn run(&self) -> Result<ServerControlHook, Box<ServerError>> {
        let server_config: &ServerConfig = self.get_server_config();
        let protocol: ServerProtocol = server_config.get_protocol();
        let tls_config: Option<TlsConfig> = server_config.get_tls().clone();
        if protocol == ServerProtocol::Http3 && tls_config.is_none() {
            return Err(Box::new(ServerError::MissingTlsForHttp3));
        }
        let stream_tls: Option<Arc<rustls::ServerConfig>> = match tls_config.clone() {
            Some(_) if protocol == ServerProtocol::Http3 => None,
            Some(config) => Some(build_stream_tls_config(&config, &protocol)?),
            None => None,
        };
        let bind_address: &String = server_config.get_address();
        let server: Arc<Server> = Arc::new(Self::clone(self));
        let request_config: RequestConfig = *self.get_request_config();
        let (shutdown_sender, shutdown_receiver) = channel(());
        let (running_sender, running_receiver) = channel(SERVER_RUNNING);
        let _: JoinHandle<()> = match protocol {
            ServerProtocol::Http3 => {
                let Some(config) = tls_config.clone() else {
                    return Err(Box::new(ServerError::MissingTlsForHttp3));
                };
                let quic_config: Arc<quinn::crypto::rustls::QuicServerConfig> = Arc::new(
                    quinn::crypto::rustls::QuicServerConfig::try_from(build_quic_tls_config(
                        &config,
                    )?)
                    .map_err(
                        |error: quinn::crypto::rustls::NoInitialCipherSuite| {
                            Box::new(ServerError::Tls(format!(
                                "no compatible cipher suite for quic: {}",
                                error
                            )))
                        },
                    )?,
                );
                let bind_addr: SocketAddr =
                    bind_address
                        .parse::<SocketAddr>()
                        .map_err(|error: AddrParseError| {
                            Box::new(ServerError::Tls(format!(
                                "invalid bind address {}: {}",
                                bind_address, error
                            )))
                        })?;
                let endpoint: Endpoint =
                    Endpoint::server(quinn::ServerConfig::with_crypto(quic_config), bind_addr)
                        .map_err(|error: io::Error| Box::new(ServerError::from(error)))?;
                let env: ConnectionEnv = ConnectionEnv {
                    server,
                    request_config,
                    protocol,
                };
                spawn(async move {
                    let _: Result<(), Box<ServerError>> =
                        quic_accept(env, endpoint, shutdown_receiver, running_sender).await;
                })
            }
            _ => {
                let tcp_listener: TcpListener = TcpListener::bind(&bind_address)
                    .await
                    .map_err(|error: io::Error| Box::new(ServerError::from(error)))?;
                let env: ConnectionEnv = ConnectionEnv {
                    server,
                    request_config,
                    protocol,
                };
                spawn(async move {
                    tcp_accept(
                        env,
                        stream_tls,
                        AcceptSignal {
                            listener: tcp_listener,
                            shutdown_receiver,
                            running_sender,
                        },
                    )
                    .await;
                })
            }
        };
        let wait_hook: ServerControlHookHandler<()> = Arc::new(move || {
            let mut running_receiver_clone: Receiver<bool> = running_receiver.clone();
            Box::pin(async move {
                while !*running_receiver_clone.borrow_and_update() {
                    let _: Result<(), tokio::sync::watch::error::RecvError> =
                        running_receiver_clone.changed().await;
                }
            })
        });
        let shutdown_hook: ServerControlHookHandler<()> = Arc::new(move || {
            let shutdown_sender_clone: Sender<()> = shutdown_sender.clone();
            Box::pin(async move {
                let _: Result<(), tokio::sync::watch::error::SendError<()>> =
                    shutdown_sender_clone.send(());
            })
        });
        let mut server_control_hook: ServerControlHook = ServerControlHook::default();
        server_control_hook.set_shutdown_hook(shutdown_hook);
        server_control_hook.set_wait_hook(wait_hook);
        Ok(server_control_hook)
    }
}
