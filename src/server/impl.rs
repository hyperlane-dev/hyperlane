use crate::*;

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
            task_panic: vec![],
            request_error: vec![],
            route_matcher: RouteMatcher::new(),
            request_middleware: vec![],
            response_middleware: vec![],
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
    /// - `&Self`- The other `Server` instance to compare against.
    ///
    /// # Returns
    ///
    /// - `bool`- `true` if the instances are equal, `false` otherwise.
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
                .all(|(a, b)| Arc::ptr_eq(a, b))
            && self
                .get_request_error()
                .iter()
                .zip(other.get_request_error().iter())
                .all(|(a, b)| Arc::ptr_eq(a, b))
            && self
                .get_request_middleware()
                .iter()
                .zip(other.get_request_middleware().iter())
                .all(|(a, b)| Arc::ptr_eq(a, b))
            && self
                .get_response_middleware()
                .iter()
                .zip(other.get_response_middleware().iter())
                .all(|(a, b)| Arc::ptr_eq(a, b))
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
    fn from(addr: usize) -> Self {
        let server: &Server = addr.into();
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
    #[inline(always)]
    fn from(addr: usize) -> &'static Server {
        unsafe { &*(addr as *const Server) }
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
    #[inline(always)]
    fn from(addr: usize) -> &'static mut Server {
        unsafe { &mut *(addr as *mut Server) }
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
    /// - `&Server` - A reference to the `Server` instance.
    #[inline(always)]
    fn as_ref(&self) -> &Server {
        let addr: usize = (self as &Server).into();
        addr.into()
    }
}

/// Implementation of `AsMut` trait for `Server`.
impl AsMut<Server> for Server {
    /// Converts `&mut Server` to `&mut Server` via memory address conversion.
    ///
    /// # Returns
    ///
    /// - `&mut Server` - A mutable reference to the `Server` instance.
    #[inline(always)]
    fn as_mut(&mut self) -> &mut Server {
        let addr: usize = (self as &mut Server).into();
        addr.into()
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
    /// - `AsRef<str>` - The configuration.
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
    /// - `RequestConfig`- The HTTP request config to set.
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
    /// # Type Parameters
    ///
    /// - `ServerHook` - The task panic handler type that implements `ServerHook`.
    ///
    /// # Returns
    ///
    /// - `&mut Self` - Reference to self for method chaining.
    #[inline(always)]
    pub fn task_panic<S>(&mut self) -> &mut Self
    where
        S: ServerHook,
    {
        self.get_mut_task_panic().push(server_hook_factory::<S>());
        self
    }

    /// Registers a request error handler to the processing pipeline.
    ///
    /// This method allows registering request error handlers that implement the `ServerHook` trait,
    /// which will be executed when a request error occurs during HTTP request processing.
    ///
    /// # Type Parameters
    ///
    /// - `ServerHook` - The request error handler type that implements `ServerHook`.
    ///
    /// # Returns
    ///
    /// - `&mut Self` - Reference to self for method chaining.
    #[inline(always)]
    pub fn request_error<S>(&mut self) -> &mut Self
    where
        S: ServerHook,
    {
        self.get_mut_request_error()
            .push(server_hook_factory::<S>());
        self
    }

    /// Registers a route hook for a specific path.
    ///
    /// This method allows registering route handlers that implement the `ServerHook` trait,
    /// providing type safety and better code organization.
    ///
    /// # Type Parameters
    ///
    /// - `ServerHook` - The route hook type that implements `ServerHook`.
    ///
    /// # Arguments
    ///
    /// - `AsRef<str>` - The route path pattern.
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
            .add(path.as_ref(), server_hook_factory::<S>())
            .unwrap();
        self
    }

    /// Registers request middleware to the processing pipeline.
    ///
    /// This method allows registering middleware that implements the `ServerHook` trait,
    /// which will be executed before route handlers for every incoming request.
    ///
    /// # Type Parameters
    ///
    /// - `ServerHook` - The middleware type that implements `ServerHook`.
    ///
    /// # Returns
    ///
    /// - `&mut Self` - Reference to self for method chaining.
    #[inline(always)]
    pub fn request_middleware<S>(&mut self) -> &mut Self
    where
        S: ServerHook,
    {
        self.get_mut_request_middleware()
            .push(server_hook_factory::<S>());
        self
    }

    /// Registers response middleware to the processing pipeline.
    ///
    /// This method allows registering middleware that implements the `ServerHook` trait,
    /// which will be executed after route handlers for every outgoing response.
    ///
    /// # Type Parameters
    ///
    /// - `ServerHook` - The middleware type that implements `ServerHook`.
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
            .push(server_hook_factory::<S>());
        self
    }

    /// Format the host and port into a bindable address string.
    ///
    /// # Arguments
    ///
    /// - `AsRef<str>` - The host address.
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

    /// Flushes the standard error stream.
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

    /// Handles a panic that has been captured and associated with a specific request `Context`.
    ///
    /// This function is invoked when a panic occurs within a task that has access to the request
    /// context, such as a route hook or middleware. It ensures that the panic information is
    /// recorded in the `Context` and then passed to the server's configured panic hook for
    /// processing.
    ///
    /// By associating the panic with the context, the hook can access request-specific details
    /// to provide more meaningful error logging and responses.
    ///
    /// # Arguments
    ///
    /// - `&mut Context` - The context of the request during which the panic occurred.
    /// - `&PanicData` - The captured panic information.
    async fn handle_panic_with_context(&self, ctx: &mut Context, panic: &PanicData) {
        ctx.set_aborted(false)
            .set_closed(false)
            .set_task_panic(panic.clone());
        for hook in self.get_task_panic().iter() {
            Box::pin(self.task_handler(ctx, hook, false)).await;
            if ctx.get_aborted() {
                return;
            }
        }
        ctx.set_aborted(true).set_closed(true);
    }

    /// Handles a panic that occurred within a spawned Tokio task.
    ///
    /// It extracts the panic information from the `JoinError` and processes it.
    ///
    /// # Arguments
    ///
    /// - `&mut Context` - The context associated with the task.
    /// - `JoinError` - The `JoinError` returned from the panicked task.
    async fn handle_task_panic(&self, ctx: &mut Context, join_error: JoinError) {
        let panic: PanicData = PanicData::from_join_error(join_error);
        ctx.get_mut_response()
            .set_status_code(HttpStatus::InternalServerError.code());
        self.handle_panic_with_context(ctx, &panic).await
    }

    /// Spawns a task handler for a given context and hook.
    ///
    /// # Arguments
    ///
    /// - `&mut Context` - The context of the request.
    /// - `&ServerHookHandler` - The hook to execute.
    /// - `bool` - Whether to handle panics that occur during execution.
    async fn task_handler(&self, ctx: &mut Context, hook: &ServerHookHandler, progress: bool) {
        if let Err(join_error) = spawn(hook(ctx)).await {
            if !join_error.is_panic() {
                return;
            }
            if progress {
                Box::pin(self.handle_task_panic(ctx, join_error)).await;
                return;
            }
            eprintln!("{}", join_error);
            let _ = Self::try_flush_stdout_and_stderr();
        };
    }

    /// Configures socket options for a newly accepted `TcpStream`.
    ///
    /// This applies settings like `TCP_NODELAY`, and `IP_TTL` from the server's configuration.
    ///
    /// # Arguments
    ///
    /// - `&TcpStream` - A reference to the `TcpStream` to configure.
    async fn configure_stream(&self, stream: &TcpStream) {
        let config: ServerConfig = self.get_server_config().clone();
        if let Some(nodelay) = config.try_get_nodelay() {
            let _ = stream.set_nodelay(*nodelay);
        }
        if let Some(ttl) = config.try_get_ttl() {
            let _ = stream.set_ttl(*ttl);
        }
    }

    /// Executes trait-based request middleware in sequence.
    ///
    /// # Arguments
    ///
    /// - `&mut Context` - The request context.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` if the lifecycle was aborted, `false` otherwise.
    pub(super) async fn handle_request_middleware(&self, ctx: &mut Context) -> bool {
        for hook in self.get_request_middleware().iter() {
            self.task_handler(ctx, hook, true).await;
            if ctx.get_aborted() {
                return true;
            }
        }
        false
    }

    /// Executes a trait-based route hook if one matches.
    ///
    /// # Arguments
    ///
    /// - `&mut Context` - The request context.
    /// - `&str` - The request path to match.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` if the lifecycle was aborted, `false` otherwise.
    pub(super) async fn handle_route_matcher(&self, ctx: &mut Context, path: &str) -> bool {
        if let Some(hook) = self.get_route_matcher().try_resolve_route(ctx, path) {
            self.task_handler(ctx, &hook, true).await;
            if ctx.get_aborted() {
                return true;
            }
        }
        false
    }

    /// Executes trait-based response middleware in sequence.
    ///
    /// # Arguments
    ///
    /// - `&mut Context` - The request context.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` if the lifecycle was aborted, `false` otherwise.
    pub(super) async fn handle_response_middleware(&self, ctx: &mut Context) -> bool {
        for hook in self.get_response_middleware().iter() {
            self.task_handler(ctx, hook, true).await;
            if ctx.get_aborted() {
                return true;
            }
        }
        false
    }

    /// Spawns a new asynchronous task to handle a single client connection.
    ///
    /// # Arguments
    ///
    /// - `ArcRwLockStream` - The thread-safe stream representing the client connection.
    async fn spawn_connection_handler(&self, stream: ArcRwLockStream) {
        let server_address: usize = self.into();
        spawn(async move {
            let server: &'static Server = server_address.into();
            server.handle_connection(stream).await;
        });
    }

    /// Handles errors that occur while processing HTTP requests.
    ///
    /// # Arguments
    ///
    /// - `&mut Context` - The request context.
    /// - `&RequestError` - The error that occurred.
    pub async fn handle_request_error(&self, ctx: &mut Context, error: &RequestError) {
        ctx.set_aborted(false)
            .set_closed(false)
            .set_request_error_data(error.clone());
        for hook in self.get_request_error().iter() {
            self.task_handler(ctx, hook, true).await;
            if ctx.get_aborted() {
                return;
            }
        }
        ctx.set_aborted(true).set_closed(true);
    }

    /// The core request handling pipeline.
    ///
    /// This function orchestrates the execution of request middleware, the route hook,
    /// and response middleware. It supports both function-based and trait-based handlers.
    ///
    /// # Arguments
    ///
    /// - `&HandlerState` - The `HandlerState` for the current connection.
    /// - `&Request` - The incoming request to be processed.
    ///
    /// # Returns
    ///
    /// - `bool` - A boolean indicating whether the connection should be kept alive.
    async fn request_hook(&self, state: &HandlerState, request: &Request) -> bool {
        let route: &str = request.get_path();
        let ctx: &mut Context = &mut Context::new(state.get_stream(), request, state.get_server());
        let keep_alive: bool = request.is_enable_keep_alive();
        if self.handle_request_middleware(ctx).await {
            return ctx.is_keep_alive(keep_alive);
        }
        if self.handle_route_matcher(ctx, route).await {
            return ctx.is_keep_alive(keep_alive);
        }
        if self.handle_response_middleware(ctx).await {
            return ctx.is_keep_alive(keep_alive);
        }
        ctx.is_keep_alive(keep_alive)
    }

    /// Handles subsequent HTTP requests on a persistent (keep-alive) connection.
    ///
    /// # Arguments
    ///
    /// - `&HandlerState` - The `HandlerState` for the current connection.
    /// - `&Request` - The initial request that established the keep-alive connection.
    async fn handle_http_requests(&self, state: &HandlerState, request: &Request) {
        if !self.request_hook(state, request).await {
            return;
        }
        let stream: &ArcRwLockStream = state.get_stream();
        let request_config: &RequestConfig = state.get_server().get_request_config();
        loop {
            match Request::http_from_stream(stream, request_config).await {
                Ok(new_request) => {
                    if !self.request_hook(state, &new_request).await {
                        return;
                    }
                }
                Err(error) => {
                    self.handle_request_error(&mut state.get_stream().into(), &error)
                        .await;
                    return;
                }
            }
        }
    }

    /// Handles a single client connection, determining whether it's an HTTP or WebSocket request.
    ///
    /// It reads the initial request from the stream and dispatches it to the appropriate hook.
    ///
    /// # Arguments
    ///
    /// - `ArcRwLockStream` - The stream for the client connection.
    async fn handle_connection(&self, stream: ArcRwLockStream) {
        match Request::http_from_stream(&stream, self.get_request_config()).await {
            Ok(request) => {
                let server_address: usize = self.into();
                let hook: HandlerState = HandlerState::new(stream, server_address.into());
                self.handle_http_requests(&hook, &request).await;
            }
            Err(error) => {
                self.handle_request_error(&mut stream.into(), &error).await;
            }
        }
    }

    /// Enters a loop to accept incoming TCP connections and spawn handlers for them.
    ///
    /// # Arguments
    ///
    /// - `&TcpListener` - A reference to the `TcpListener` to accept connections from.
    ///
    /// # Returns
    ///
    /// - `Result<(), ServerError>` - A `Result` which is typically `Ok(())` unless an unrecoverable
    ///   error occurs.
    async fn accept_connections(&self, tcp_listener: &TcpListener) -> Result<(), ServerError> {
        while let Ok((stream, _)) = tcp_listener.accept().await {
            self.configure_stream(&stream).await;
            let stream: ArcRwLockStream = ArcRwLockStream::from_stream(stream);
            self.spawn_connection_handler(stream).await;
        }
        Ok(())
    }

    /// Creates and binds a `TcpListener` based on the server's configuration.
    ///
    /// # Returns
    ///
    /// - `Result<TcpListener, ServerError>` - A `Result` containing the bound `TcpListener` on success,
    ///   or a `ServerError` on failure.
    async fn create_tcp_listener(&self) -> Result<TcpListener, ServerError> {
        Ok(TcpListener::bind(self.get_server_config().get_address()).await?)
    }

    /// Starts the server, binds to the configured address, and begins listening for connections.
    ///
    /// This is the main entry point to launch the server. It will initialize the panic hook,
    /// create a TCP listener, and then enter the connection acceptance loop in a background task.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing a shutdown function on success.
    /// Calling this function will shut down the server by aborting its main task.
    /// Returns an error if the server fails to start.
    pub async fn run(&self) -> Result<ServerControlHook, ServerError> {
        let tcp_listener: TcpListener = self.create_tcp_listener().await?;
        let server: Server = self.clone();
        let (wait_sender, wait_receiver) = channel(());
        let (shutdown_sender, mut shutdown_receiver) = channel(());
        let accept_connections: JoinHandle<()> = spawn(async move {
            let _ = server.accept_connections(&tcp_listener).await;
            let _ = wait_sender.send(());
        });
        let wait_hook: ServerControlHookHandler<()> = Arc::new(move || {
            let mut wait_receiver_clone: Receiver<()> = wait_receiver.clone();
            Box::pin(async move {
                let _ = wait_receiver_clone.changed().await;
            })
        });
        let shutdown_hook: ServerControlHookHandler<()> = Arc::new(move || {
            let shutdown_sender_clone: Sender<()> = shutdown_sender.clone();
            Box::pin(async move {
                let _ = shutdown_sender_clone.send(());
            })
        });
        spawn(async move {
            let _ = shutdown_receiver.changed().await;
            accept_connections.abort();
        });
        let mut server_control_hook: ServerControlHook = ServerControlHook::default();
        server_control_hook.set_shutdown_hook(shutdown_hook);
        server_control_hook.set_wait_hook(wait_hook);
        Ok(server_control_hook)
    }
}
