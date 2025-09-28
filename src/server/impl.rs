use crate::*;

/// Provides a default implementation for ServerInner.
impl Default for ServerInner {
    /// Creates a new ServerInner instance with default values.
    ///
    /// # Returns
    ///
    /// - `Self` - A new instance with default configuration.
    fn default() -> Self {
        Self {
            config: ServerConfigInner::default(),
            panic_hook: vec![],
            request_middleware: vec![],
            route: RouteMatcher::new(),
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
    /// - `&Self`: The other `ServerInner` instance to compare against.
    ///
    /// # Returns
    ///
    /// - `bool`: `true` if the instances are equal, `false` otherwise.
    fn eq(&self, other: &Self) -> bool {
        self.config == other.config
            && self.route == other.route
            && self.request_middleware.len() == other.request_middleware.len()
            && self.response_middleware.len() == other.response_middleware.len()
            && self.panic_hook.len() == other.panic_hook.len()
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
            && self
                .panic_hook
                .iter()
                .zip(other.panic_hook.iter())
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
    /// - `&Self`: The other `Server` instance to compare against.
    ///
    /// # Returns
    ///
    /// - `bool`: `true` if the instances are equal, `false` otherwise.
    fn eq(&self, other: &Self) -> bool {
        if Arc::ptr_eq(&self.get_0(), &other.get_0()) {
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
impl<'a> HandlerState {
    /// Creates a new HandlerState instance.
    ///
    /// # Arguments
    ///
    /// - `&'a ArcRwLockStream` - The network stream.
    /// - `&'a Context` - The request context.
    /// - `usize` - The buffer size for reading HTTP requests.
    ///
    /// # Returns
    ///
    /// - `Self` - The newly created handler state.
    pub(super) fn new(stream: ArcRwLockStream, ctx: Context, buffer: usize) -> Self {
        Self {
            stream,
            ctx,
            buffer,
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
    /// - `RwLockReadGuardServerInner` - The read guard for ServerInner.
    async fn read(&self) -> RwLockReadGuardServerInner {
        self.get_0().read().await
    }

    /// Acquires a write lock on the inner server data.
    ///
    /// # Returns
    ///
    /// - `RwLockWriteGuardServerInner` - The write guard for ServerInner.
    async fn write(&self) -> RwLockWriteGuardServerInner {
        self.get_0().write().await
    }

    /// Handle a given hook macro asynchronously.
    ///
    /// This function dispatches the provided `HookMacro` to the appropriate
    /// internal handler based on its `HookType`. Supported hook types include
    /// panic hooks, disable HTTP/WS hooks, connected hooks, pre-upgrade hooks,
    /// request/response middleware, and routes.
    ///
    /// # Arguments
    ///
    /// - `HookMacro`: The `HookMacro` instance containing the `HookType` and its handler.
    pub async fn handle_hook(&self, hook: HookMacro) {
        match hook.hook_type {
            HookType::PanicHook(_) => {
                self.panic_hook(hook.handler).await;
            }
            HookType::RequestMiddleware(_) => {
                self.request_middleware(hook.handler).await;
            }
            HookType::Route(path) => {
                self.route(path, hook.handler).await;
            }
            HookType::ResponseMiddleware(_) => {
                self.response_middleware(hook.handler).await;
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
        let config: ServerConfig = ServerConfig::from_str(&config_str.to_string()).unwrap();
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

    /// Sets a custom panic hook for request processing.
    ///
    /// # Arguments
    ///
    /// - `F: FnContextSendSyncStatic<Fut, ()>` - The panic handler function.
    /// - `Fut: FutureSendStatic<()>` - The future returned by the panic handler.
    ///
    /// # Returns
    ///
    /// - `&Self` - Reference to self for method chaining.
    pub async fn panic_hook<F, Fut>(&self, hook: F) -> &Self
    where
        F: FnContextSendSyncStatic<Fut, ()>,
        Fut: FutureSendStatic<()>,
    {
        let panic_hook: ArcFnContextPinBoxSendSync<()> =
            Arc::new(move |ctx: Context| -> PinBoxFutureSend<()> { Box::pin(hook(ctx)) });
        self.write().await.get_mut_panic_hook().push(panic_hook);
        self
    }

    /// Adds a route handler for a specific path.
    ///
    /// # Arguments
    ///
    /// - `R: ToString` - The route path pattern.
    /// - `F: FnContextSendSyncStatic<Fut, ()>` - The handler function for the route.
    /// - `Fut: FutureSendStatic<()>` - The future returned by the handler.
    ///
    /// # Returns
    ///
    /// - `&Self` - Reference to self for method chaining.
    pub async fn route<R, F, Fut>(&self, route: R, hook: F) -> &Self
    where
        R: ToString,
        F: FnContextSendSyncStatic<Fut, ()>,
        Fut: FutureSendStatic<()>,
    {
        let route_str: String = route.to_string();
        let route_hook: ArcFnContextPinBoxSendSync<()> =
            Arc::new(move |ctx: Context| -> PinBoxFutureSend<()> { Box::pin(hook(ctx)) });
        self.write()
            .await
            .get_mut_route()
            .add(&route_str, route_hook)
            .unwrap();
        self
    }

    /// Adds request middleware to the processing pipeline.
    ///
    /// # Arguments
    ///
    /// - `F: FnContextSendSyncStatic<Fut, ()>` - The middleware function.
    /// - `Fut: FutureSendStatic<()>` - The future returned by the middleware.
    ///
    /// # Returns
    ///
    /// - `&Self` - Reference to self for method chaining.
    pub async fn request_middleware<F, Fut>(&self, hook: F) -> &Self
    where
        F: FnContextSendSyncStatic<Fut, ()>,
        Fut: FutureSendStatic<()>,
    {
        let request_middleware_hook: ArcFnContextPinBoxSendSync<()> =
            Arc::new(move |ctx: Context| -> PinBoxFutureSend<()> { Box::pin(hook(ctx)) });
        self.write()
            .await
            .get_mut_request_middleware()
            .push(request_middleware_hook);
        self
    }

    /// Adds response middleware to the processing pipeline.
    ///
    /// # Arguments
    ///
    /// - `F: FnContextSendSyncStatic<Fut, ()>` - The middleware function.
    /// - `Fut: FutureSendStatic<()>` - The future returned by the middleware.
    ///
    /// # Returns
    ///
    /// - `&Self` - Reference to self for method chaining.
    pub async fn response_middleware<F, Fut>(&self, hook: F) -> &Self
    where
        F: FnContextSendSyncStatic<Fut, ()>,
        Fut: FutureSendStatic<()>,
    {
        let response_middleware_hook: ArcFnContextPinBoxSendSync<()> =
            Arc::new(move |ctx: Context| -> PinBoxFutureSend<()> { Box::pin(hook(ctx)) });
        self.write()
            .await
            .get_mut_response_middleware()
            .push(response_middleware_hook);
        self
    }

    /// Formats the host and port into a bindable address string.
    ///
    /// # Arguments
    ///
    /// - `H: ToString` - The host address.
    /// - `usize` - The port number.
    ///
    /// # Returns
    ///
    /// - `String` - The formatted address string.
    pub fn format_host_port<H: ToString>(host: H, port: usize) -> String {
        format!("{}{}{}", host.to_string(), COLON_SPACE_SYMBOL, port)
    }

    /// Handles a panic that has been captured and associated with a specific request `Context`.
    ///
    /// This function is invoked when a panic occurs within a task that has access to the request
    /// context, such as a route handler or middleware. It ensures that the panic information is
    /// recorded in the `Context` and then passed to the server's configured panic hook for
    /// processing.
    ///
    /// By associating the panic with the context, the handler can access request-specific details
    /// to provide more meaningful error logging and responses.
    ///
    /// # Arguments
    ///
    /// - `&Context` - The context of the request during which the panic occurred.
    /// - `&Panic` - The captured panic information.
    async fn handle_panic_with_context(&self, ctx: &Context, panic: &Panic) {
        let panic_clone: Panic = panic.clone();
        ctx.cancel_aborted().await.set_panic(panic_clone).await;
        for hook in self.read().await.get_panic_hook().iter() {
            hook(ctx.clone()).await;
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
        let panic: Panic = Panic::from_join_error(join_error);
        self.handle_panic_with_context(&ctx, &panic).await;
    }

    /// Executes a given hook function within a spawned task and manages the request lifecycle.
    ///
    /// This function also handles panics that may occur within the hook's execution.
    ///
    /// # Arguments
    ///
    /// - `ctx: &Context` - The request context.
    /// - `lifecycle: &mut Lifecycle` - A mutable reference to the current `Lifecycle` state.
    /// - `hook: ArcFnContextPinBoxSendSync<()>` - The hook function to execute.
    async fn run_hook_with_lifecycle(
        &self,
        ctx: &Context,
        lifecycle: &mut Lifecycle,
        hook: &ArcFnContextPinBoxSendSync<()>,
    ) {
        let result: ResultJoinError<()> = spawn(hook(ctx.clone())).await;
        ctx.update_lifecycle_status(lifecycle).await;
        if let Err(join_error) = result {
            if join_error.is_panic() {
                self.handle_task_panic(&ctx, join_error).await;
            }
        }
    }

    /// Creates and binds a `TcpListener` based on the server's configuration.
    ///
    /// # Returns
    ///
    /// Returns a `ServerResult` containing the bound `TcpListener` on success,
    /// or a `ServerError` on failure.
    async fn create_tcp_listener(&self) -> ServerResult<TcpListener> {
        let config: ServerConfigInner = self.read().await.get_config().clone();
        let host: String = config.get_host().clone();
        let port: usize = *config.get_port();
        let addr: String = Self::format_host_port(host, port);
        TcpListener::bind(&addr)
            .await
            .map_err(|err| ServerError::TcpBind(err.to_string()))
    }

    /// Enters a loop to accept incoming TCP connections and spawn handlers for them.
    ///
    /// # Arguments
    ///
    /// - `&TcpListener` - A reference to the `TcpListener` to accept connections from.
    ///
    /// # Returns
    ///
    /// - `ServerResult<()>` - A `ServerResult` which is typically `Ok(())` unless an unrecoverable
    /// error occurs.
    async fn accept_connections(&self, tcp_listener: &TcpListener) -> ServerResult<()> {
        while let Ok((stream, _socket_addr)) = tcp_listener.accept().await {
            self.configure_stream(&stream).await;
            let stream: ArcRwLockStream = ArcRwLockStream::from_stream(stream);
            self.spawn_connection_handler(stream).await;
        }
        Ok(())
    }

    /// Configures socket options for a newly accepted `TcpStream`.
    ///
    /// This applies settings like `SO_LINGER`, `TCP_NODELAY`, and `IP_TTL` from the server's configuration.
    ///
    /// # Arguments
    ///
    /// - `&TcpStream` - A reference to the `TcpStream` to configure.
    async fn configure_stream(&self, stream: &TcpStream) {
        let server_inner: RwLockReadGuardServerInner = self.read().await;
        let config: &ServerConfigInner = server_inner.get_config();
        let linger_opt: &OptionDuration = config.get_linger();
        let nodelay_opt: &OptionBool = config.get_nodelay();
        let ttl_opt: &OptionU32 = config.get_ttl();
        let _ = stream.set_linger(*linger_opt);
        if let Some(nodelay) = nodelay_opt {
            let _ = stream.set_nodelay(*nodelay);
        }
        if let Some(ttl) = ttl_opt {
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
        let buffer: usize = *self.read().await.get_config().get_buffer();
        spawn(async move {
            server.handle_connection(stream, buffer).await;
        });
    }

    /// Handles a single client connection, determining whether it's an HTTP or WebSocket request.
    ///
    /// It reads the initial request from the stream and dispatches it to the appropriate handler.
    ///
    /// # Arguments
    ///
    /// - `ArcRwLockStream` - The stream for the client connection.
    /// - `usize` - The buffer size to use for reading the initial HTTP request.
    async fn handle_connection(&self, stream: ArcRwLockStream, buffer: usize) {
        if let Ok(request) = Request::http_from_stream(&stream, buffer).await {
            let ctx: Context = Context::create_context(&stream, &request);
            let handler: HandlerState = HandlerState::new(stream, ctx, buffer);
            self.handle_http_requests(&handler, &request).await;
        }
    }

    /// Executes all registered request middleware in sequence.
    ///
    /// # Arguments
    ///
    /// - `&Context` - The request context.
    /// - `&mut Lifecycle` - A mutable reference to the request lifecycle state.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` if the lifecycle was aborted, `false` otherwise.
    async fn run_request_middleware(&self, ctx: &Context, lifecycle: &mut Lifecycle) -> bool {
        for hook in self.read().await.get_request_middleware().iter() {
            self.run_hook_with_lifecycle(ctx, lifecycle, hook).await;
            if lifecycle.is_abort() {
                return true;
            }
        }
        false
    }

    /// Executes the matched route handler.
    ///
    /// # Arguments
    ///
    /// - `&Context` - The request context.
    /// - `&OptionArcFnContextPinBoxSendSync` - An `Option` containing the handler function if a route was matched.
    /// - `&mut Lifecycle` - A mutable reference to the request lifecycle state.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` if the lifecycle was aborted, `false` otherwise.
    async fn run_route_hook(
        &self,
        ctx: &Context,
        handler: &OptionArcFnContextPinBoxSendSync<()>,
        lifecycle: &mut Lifecycle,
    ) -> bool {
        if let Some(hook) = handler {
            self.run_hook_with_lifecycle(ctx, lifecycle, hook).await;
        }
        lifecycle.is_abort()
    }

    /// Executes all registered response middleware in sequence.
    ///
    /// # Arguments
    ///
    /// - `&Context` - The request context.
    /// - `&mut Lifecycle` - A mutable reference to the request lifecycle state.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` if the lifecycle was aborted, `false` otherwise.
    async fn run_response_middleware(&self, ctx: &Context, lifecycle: &mut Lifecycle) -> bool {
        for hook in self.read().await.get_response_middleware().iter() {
            self.run_hook_with_lifecycle(ctx, lifecycle, hook).await;
            if lifecycle.is_abort() {
                return true;
            }
        }
        false
    }

    /// The core request handling pipeline.
    ///
    /// This function orchestrates the execution of request middleware, the route handler,
    /// and response middleware.
    ///
    /// # Arguments
    ///
    /// - `&HandlerState` - The `HandlerState` for the current connection.
    /// - `&Request` - The incoming request to be processed.
    ///
    /// # Returns
    ///
    /// - `bool` - A boolean indicating whether the connection should be kept alive.
    async fn request_hook<'a>(&self, state: &HandlerState, request: &Request) -> bool {
        let route: &str = request.get_path();
        let ctx: &Context = state.get_ctx();
        ctx.set_request(request).await;
        let mut lifecycle: Lifecycle = Lifecycle::new(request.is_enable_keep_alive());
        let route_hook: OptionArcFnContextPinBoxSendSync<()> = self
            .read()
            .await
            .get_route()
            .try_resolve_route(ctx, route)
            .await;
        if self.run_request_middleware(ctx, &mut lifecycle).await {
            return lifecycle.keep_alive();
        }
        if self.run_route_hook(ctx, &route_hook, &mut lifecycle).await {
            return lifecycle.keep_alive();
        }
        self.run_response_middleware(ctx, &mut lifecycle).await;
        lifecycle.keep_alive()
    }

    /// Handles subsequent HTTP requests on a persistent (keep-alive) connection.
    ///
    /// # Arguments
    ///
    /// - `&HandlerState` - The `HandlerState` for the current connection.
    /// - `&Request` - The initial request that established the keep-alive connection.
    async fn handle_http_requests<'a>(&self, state: &HandlerState, request: &Request) {
        if self.request_hook(state, request).await {
            return;
        }
        let stream: &ArcRwLockStream = state.get_stream();
        let buffer: usize = *state.get_buffer();
        while let Ok(new_request) = &Request::http_from_stream(stream, buffer).await {
            if !self.request_hook(state, new_request).await {
                return;
            }
        }
    }

    /// Starts the server, binds to the configured address, and begins listening for connections.
    ///
    /// This is the main entry point to launch the server. It will initialize the panic hook,
    /// create a TCP listener, and then enter the connection acceptance loop in a background task.
    ///
    /// # Returns
    ///
    /// Returns a `ServerResult` containing a shutdown function on success.
    /// Calling this function will shut down the server by aborting its main task.
    /// Returns an error if the server fails to start.
    pub async fn run(&self) -> ServerResult<ServerHook> {
        let tcp_listener: TcpListener = self.create_tcp_listener().await?;
        let server: Server = self.clone();
        let (wait_sender, wait_receiver) = channel(());
        let (shutdown_sender, mut shutdown_receiver) = channel(());
        let accept_connections: JoinHandle<()> = spawn(async move {
            let _ = server.accept_connections(&tcp_listener).await;
            let _ = wait_sender.send(());
        });
        let wait_hook: ArcFnPinBoxFutureSend<()> = Arc::new(move || {
            let mut wait_receiver_clone: Receiver<()> = wait_receiver.clone();
            Box::pin(async move {
                let _ = wait_receiver_clone.changed().await;
            })
        });
        let shutdown_hook: ArcFnPinBoxFutureSend<()> = Arc::new(move || {
            let shutdown_sender_clone: Sender<()> = shutdown_sender.clone();
            Box::pin(async move {
                let _ = shutdown_sender_clone.send(());
            })
        });
        spawn(async move {
            let _ = shutdown_receiver.changed().await;
            accept_connections.abort();
        });
        let mut server_hook: ServerHook = ServerHook::default();
        server_hook.set_shutdown_hook(shutdown_hook);
        server_hook.set_wait_hook(wait_hook);
        Ok(server_hook)
    }
}
