use crate::*;

/// Provides a default implementation for ServerInner.
impl Default for ServerInner {
    /// Creates a new ServerInner instance with default values.
    ///
    /// # Returns
    ///
    /// - `Self` - A new instance with default configuration.
    #[inline(always)]
    fn default() -> Self {
        Self {
            config: ServerConfigInner::default(),
            task_panic: vec![],
            request_error: vec![],
            route_matcher: RouteMatcher::new(),
            request_middleware: vec![],
            response_middleware: vec![],
        }
    }
}

/// Implements the `PartialEq` trait for `ServerInner`.
///
/// This allows for comparing two `ServerInner` instances for equality.
impl PartialEq for ServerInner {
    /// Checks if two `ServerInner` instances are equal.
    ///
    /// # Arguments
    ///
    /// - `&Self`- The other `ServerInner` instance to compare against.
    ///
    /// # Returns
    ///
    /// - `bool`- `true` if the instances are equal, `false` otherwise.
    fn eq(&self, other: &Self) -> bool {
        self.config == other.config
            && self.route_matcher == other.route_matcher
            && self.task_panic.len() == other.task_panic.len()
            && self.request_error.len() == other.request_error.len()
            && self.request_middleware.len() == other.request_middleware.len()
            && self.response_middleware.len() == other.response_middleware.len()
            && self
                .task_panic
                .iter()
                .zip(other.task_panic.iter())
                .all(|(a, b)| Arc::ptr_eq(a, b))
            && self
                .request_error
                .iter()
                .zip(other.request_error.iter())
                .all(|(a, b)| Arc::ptr_eq(a, b))
            && self
                .request_middleware
                .iter()
                .zip(other.request_middleware.iter())
                .all(|(a, b)| Arc::ptr_eq(a, b))
            && self
                .response_middleware
                .iter()
                .zip(other.response_middleware.iter())
                .all(|(a, b)| Arc::ptr_eq(a, b))
    }
}

/// Implements the `Eq` trait for `ServerInner`.
///
/// This indicates that `ServerInner` has a total equality relation.
impl Eq for ServerInner {}

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
    #[inline(always)]
    fn eq(&self, other: &Self) -> bool {
        if Arc::ptr_eq(self.get_0(), other.get_0()) {
            return true;
        }
        if let (Ok(s), Ok(o)) = (self.get_0().try_read(), other.get_0().try_read()) {
            *s == *o
        } else {
            false
        }
    }
}

/// Implements the `Eq` trait for `Server`.
///
/// This indicates that `Server` has a total equality relation.
impl Eq for Server {}

/// Manages the state for handling a single connection, including the stream and context.
///
/// This struct provides a convenient way to pass around the necessary components
/// for processing a request or WebSocket frame.
impl HandlerState {
    /// Creates a new HandlerState instance.
    ///
    /// # Arguments
    ///
    /// - `&'a ArcRwLockStream` - The network stream.
    /// - `RequestConfig` - The request config.
    ///
    /// # Returns
    ///
    /// - `Self` - The newly created hook state.
    #[inline(always)]
    pub(super) fn new(stream: ArcRwLockStream, request_config: RequestConfig) -> Self {
        Self {
            stream,
            request_config,
        }
    }
}

/// Represents the server, providing methods to configure and run it.
///
/// This struct wraps the `ServerInner` configuration and routing logic,
/// offering a high-level API for setting up the HTTP and WebSocket server.
impl Server {
    /// Creates a new Server instance with default settings.
    ///
    /// # Returns
    ///
    /// - `Self` - A new Server instance.
    pub async fn new() -> Self {
        let server: ServerInner = ServerInner::default();
        Self(arc_rwlock(server))
    }

    /// Creates a new Server instance from a configuration.
    ///
    /// # Arguments
    ///
    /// - `ServerConfig` - The server configuration.
    ///
    /// # Returns
    ///
    /// - `Self` - A new Server instance.
    pub async fn from(config: ServerConfig) -> Self {
        let server: Self = Self::new().await;
        server.config(config).await;
        server
    }

    /// Acquires a read lock on the inner server data.
    ///
    /// # Returns
    ///
    /// - `ServerStateReadGuard` - The read guard for ServerInner.
    pub(super) async fn read(&self) -> ServerStateReadGuard<'_> {
        self.get_0().read().await
    }

    /// Acquires a write lock on the inner server data.
    ///
    /// # Returns
    ///
    /// - `ServerStateWriteGuard` - The write guard for ServerInner.
    async fn write(&self) -> ServerStateWriteGuard<'_> {
        self.get_0().write().await
    }

    /// Gets the route matcher.
    ///
    /// # Returns
    /// - `RouteMatcher` - The route matcher.
    pub async fn get_route_matcher(&self) -> RouteMatcher {
        self.read().await.get_route_matcher().clone()
    }

    /// Registers a hook into the server's processing pipeline.
    ///
    /// This function dispatches the provided `HookType` to the appropriate
    /// internal hook collection based on its variant. The hook will be executed
    /// at the corresponding stage of request processing according to its type:
    /// - `Panic`: Added to panic handlers for error recovery
    /// - `RequestError`: Added to request error handlers
    /// - `RequestMiddleware`: Added to pre-route middleware chain
    /// - `Route`: Registered as a route handler for the specified path
    /// - `ResponseMiddleware`: Added to post-route middleware chain
    ///
    /// # Arguments
    ///
    /// - `HookType` - The `HookType` instance containing the hook configuration and factory.
    pub async fn handle_hook(&self, hook: HookType) {
        match hook {
            HookType::TaskPanic(_, hook) => {
                self.write().await.get_mut_task_panic().push(hook());
            }
            HookType::RequestError(_, hook) => {
                self.write().await.get_mut_request_error().push(hook());
            }
            HookType::RequestMiddleware(_, hook) => {
                self.write().await.get_mut_request_middleware().push(hook());
            }
            HookType::Route(path, hook) => {
                self.write()
                    .await
                    .get_mut_route_matcher()
                    .add(path, hook())
                    .unwrap();
            }
            HookType::ResponseMiddleware(_, hook) => {
                self.write()
                    .await
                    .get_mut_response_middleware()
                    .push(hook());
            }
        };
    }

    /// Sets the server configuration from a string.
    ///
    /// # Arguments
    ///
    /// - `C: ToString` - The configuration.
    ///
    /// # Returns
    ///
    /// - `&Self` - Reference to self for method chaining.
    pub async fn config_str<C: ToString>(&self, config_str: C) -> &Self {
        let config: ServerConfig = ServerConfig::from_json_str(&config_str.to_string()).unwrap();
        self.write().await.set_config(config.get_inner().await);
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
    /// - `&Self` - Reference to self for method chaining.
    pub async fn config(&self, config: ServerConfig) -> &Self {
        self.write().await.set_config(config.get_inner().await);
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
    /// - `&Self` - Reference to self for method chaining.
    pub async fn task_panic<S>(&self) -> &Self
    where
        S: ServerHook,
    {
        self.write()
            .await
            .get_mut_task_panic()
            .push(server_hook_factory::<S>());
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
    /// - `&Self` - Reference to self for method chaining.
    pub async fn request_error<S>(&self) -> &Self
    where
        S: ServerHook,
    {
        self.write()
            .await
            .get_mut_request_error()
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
    /// - `path` - The route path pattern.
    ///
    /// # Returns
    ///
    /// - `&Self` - Reference to self for method chaining.
    pub async fn route<S>(&self, path: impl ToString) -> &Self
    where
        S: ServerHook,
    {
        self.write()
            .await
            .get_mut_route_matcher()
            .add(&path.to_string(), server_hook_factory::<S>())
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
    /// - `&Self` - Reference to self for method chaining.
    pub async fn request_middleware<S>(&self) -> &Self
    where
        S: ServerHook,
    {
        self.write()
            .await
            .get_mut_request_middleware()
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
    /// - `&Self` - Reference to self for method chaining.
    pub async fn response_middleware<S>(&self) -> &Self
    where
        S: ServerHook,
    {
        self.write()
            .await
            .get_mut_response_middleware()
            .push(server_hook_factory::<S>());
        self
    }

    /// Formats the host and port into a bindable address string.
    ///
    /// # Arguments
    ///
    /// - `H: ToString` - The host address.
    /// - `u16` - The port number.
    ///
    /// # Returns
    ///
    /// - `String` - The formatted address string.
    #[inline(always)]
    pub fn format_host_port<H: ToString>(host: H, port: u16) -> String {
        format!("{}{COLON}{port}", host.to_string())
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
    /// - `&Context` - The context of the request during which the panic occurred.
    /// - `&PanicData` - The captured panic information.
    async fn handle_panic_with_context(&self, ctx: &Context, panic: &PanicData) {
        let panic_clone: PanicData = panic.clone();
        ctx.cancel_aborted().await.set_task_panic(panic_clone).await;
        for hook in self.read().await.get_task_panic().iter() {
            Box::pin(self.task_handler(ctx, hook, false)).await;
            if ctx.get_aborted().await {
                return;
            }
        }
    }

    /// Handles a panic that occurred within a spawned Tokio task.
    ///
    /// It extracts the panic information from the `JoinError` and processes it.
    ///
    /// # Arguments
    ///
    /// - `&Context` - The context associated with the task.
    /// - `JoinError` - The `JoinError` returned from the panicked task.
    async fn handle_task_panic(&self, ctx: &Context, join_error: JoinError) {
        let panic: PanicData = PanicData::from_join_error(join_error);
        ctx.set_response_status_code(HttpStatus::InternalServerError.code())
            .await;
        self.handle_panic_with_context(ctx, &panic).await;
    }

    /// Spawns a task handler for a given context and hook.
    ///
    /// # Arguments
    ///
    /// - `&Context` - The context of the request.
    /// - `&ServerHookHandler` - The hook to execute.
    /// - `bool` - Whether to handle panics that occur during execution.
    async fn task_handler(&self, ctx: &Context, hook: &ServerHookHandler, progress: bool) {
        if let Err(join_error) = spawn(hook(ctx)).await
            && join_error.is_panic()
        {
            if progress {
                Box::pin(self.handle_task_panic(ctx, join_error)).await;
            } else {
                eprintln!("Panic occurred in panic handler: {:?}", join_error);
                let _ = Self::try_flush_stdout_and_stderr();
            }
        }
    }

    /// Creates and binds a `TcpListener` based on the server's configuration.
    ///
    /// # Returns
    ///
    /// - `Result<TcpListener, ServerError>` - A `Result` containing the bound `TcpListener` on success,
    ///   or a `ServerError` on failure.
    async fn create_tcp_listener(&self) -> Result<TcpListener, ServerError> {
        let config: ServerConfigInner = self.read().await.get_config().clone();
        let host: String = config.get_host().clone();
        let port: u16 = *config.get_port();
        let addr: String = Self::format_host_port(host, port);
        TcpListener::bind(&addr)
            .await
            .map_err(|error| ServerError::TcpBind(error.to_string()))
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
        while let Ok((stream, _socket_addr)) = tcp_listener.accept().await {
            self.configure_stream(&stream).await;
            let stream: ArcRwLockStream = ArcRwLockStream::from_stream(stream);
            self.spawn_connection_handler(stream).await;
        }
        Ok(())
    }

    /// Configures socket options for a newly accepted `TcpStream`.
    ///
    /// This applies settings like `TCP_NODELAY`, and `IP_TTL` from the server's configuration.
    ///
    /// # Arguments
    ///
    /// - `&TcpStream` - A reference to the `TcpStream` to configure.
    async fn configure_stream(&self, stream: &TcpStream) {
        let server_inner: ServerStateReadGuard = self.read().await;
        let config: &ServerConfigInner = server_inner.get_config();
        if let Some(nodelay) = config.try_get_nodelay() {
            let _ = stream.set_nodelay(*nodelay);
        }
        if let Some(ttl) = config.try_get_ttl() {
            let _ = stream.set_ttl(*ttl);
        }
    }

    /// Spawns a new asynchronous task to handle a single client connection.
    ///
    /// # Arguments
    ///
    /// - `ArcRwLockStream` - The thread-safe stream representing the client connection.
    async fn spawn_connection_handler(&self, stream: ArcRwLockStream) {
        let server: Server = self.clone();
        let request_config: RequestConfig = *self.read().await.get_config().get_request_config();
        spawn(async move {
            server.handle_connection(stream, request_config).await;
        });
    }

    /// Handles errors that occur while processing HTTP requests.
    ///
    /// # Arguments
    ///
    /// - `&Context` - The request context.
    /// - `&RequestError` - The error that occurred.
    pub async fn handle_request_error(&self, ctx: &Context, error: &RequestError) {
        ctx.cancel_aborted()
            .await
            .set_request_error_data(error.clone())
            .await;
        for hook in self.read().await.get_request_error().iter() {
            self.task_handler(ctx, hook, true).await;
            if ctx.get_aborted().await {
                return;
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
    /// - `request_config` - The request config to use for reading the initial HTTP request.
    async fn handle_connection(&self, stream: ArcRwLockStream, request_config: RequestConfig) {
        match Request::http_from_stream(&stream, &request_config).await {
            Ok(request) => {
                let hook: HandlerState = HandlerState::new(stream, request_config);
                self.handle_http_requests(&hook, &request).await;
            }
            Err(error) => {
                self.handle_request_error(&stream.into(), &error).await;
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
    /// - `&HandlerState` - The `HandlerState` for the current connection.
    /// - `&Request` - The incoming request to be processed.
    ///
    /// # Returns
    ///
    /// - `bool` - A boolean indicating whether the connection should be kept alive.
    async fn request_hook(&self, state: &HandlerState, request: &Request) -> bool {
        let route: &str = request.get_path();
        let ctx: &Context = &Context::new(state.get_stream(), request);
        let keep_alive: bool = request.is_enable_keep_alive();
        if self.handle_request_middleware(ctx).await {
            return ctx.is_keep_alive(keep_alive).await;
        }
        if self.handle_route_matcher(ctx, route).await {
            return ctx.is_keep_alive(keep_alive).await;
        }
        if self.handle_response_middleware(ctx).await {
            return ctx.is_keep_alive(keep_alive).await;
        }
        if let Some(panic) = ctx.try_get_task_panic_data().await {
            ctx.set_response_status_code(HttpStatus::InternalServerError.code())
                .await;
            self.handle_panic_with_context(ctx, &panic).await;
        }
        ctx.is_keep_alive(keep_alive).await
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
        let request_config: &RequestConfig = state.get_request_config();
        loop {
            match Request::http_from_stream(stream, request_config).await {
                Ok(new_request) => {
                    if !self.request_hook(state, &new_request).await {
                        return;
                    }
                }
                Err(error) => {
                    self.handle_request_error(&state.get_stream().into(), &error)
                        .await;
                    return;
                }
            }
        }
    }

    /// Executes trait-based request middleware in sequence.
    ///
    /// # Arguments
    ///
    /// - `&Context` - The request context.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` if the lifecycle was aborted, `false` otherwise.
    pub(super) async fn handle_request_middleware(&self, ctx: &Context) -> bool {
        for hook in self.read().await.get_request_middleware().iter() {
            self.task_handler(ctx, hook, true).await;
            if ctx.get_aborted().await {
                return true;
            }
        }
        false
    }

    /// Executes a trait-based route hook if one matches.
    ///
    /// # Arguments
    ///
    /// - `&Context` - The request context.
    /// - `&str` - The request path to match.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` if the lifecycle was aborted, `false` otherwise.
    pub(super) async fn handle_route_matcher(&self, ctx: &Context, path: &str) -> bool {
        if let Some(hook) = self
            .read()
            .await
            .get_route_matcher()
            .try_resolve_route(ctx, path)
            .await
        {
            self.task_handler(ctx, &hook, true).await;
            if ctx.get_aborted().await {
                return true;
            }
        }
        false
    }

    /// Executes trait-based response middleware in sequence.
    ///
    /// # Arguments
    ///
    /// - `&Context` - The request context.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` if the lifecycle was aborted, `false` otherwise.
    pub(super) async fn handle_response_middleware(&self, ctx: &Context) -> bool {
        for hook in self.read().await.get_response_middleware().iter() {
            self.task_handler(ctx, hook, true).await;
            if ctx.get_aborted().await {
                return true;
            }
        }
        false
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
        let wait_hook: SharedAsyncTaskFactory<()> = Arc::new(move || {
            let mut wait_receiver_clone: Receiver<()> = wait_receiver.clone();
            Box::pin(async move {
                let _ = wait_receiver_clone.changed().await;
            })
        });
        let shutdown_hook: SharedAsyncTaskFactory<()> = Arc::new(move || {
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
