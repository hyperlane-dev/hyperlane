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
            route: RouteMatcher::new(),
            request_middleware: vec![],
            response_middleware: vec![],
            pre_upgrade_hook: vec![],
            connected_hook: vec![],
            disable_http_hook: RouteMatcher::new(),
            disable_ws_hook: RouteMatcher::new(),
            panic_hook: vec![],
        }
    }
}

/// Provides a default implementation for `ServerHook`.
impl Default for ServerHook {
    /// Creates a new `ServerHook` instance with default no-op hooks.
    ///
    /// The default `wait_hook` and `shutdown_hook` do nothing, allowing the server
    /// to run without specific shutdown or wait logic unless configured otherwise.
    ///
    /// # Returns
    ///
    /// - `Self` - A new `ServerHook` instance with default hooks.
    fn default() -> Self {
        Self {
            wait_hook: Arc::new(|| Box::pin(async move {})),
            shutdown_hook: Arc::new(|| Box::pin(async move {})),
        }
    }
}

/// Manages server lifecycle hooks, including waiting and shutdown procedures.
///
/// This struct holds closures that are executed during specific server lifecycle events.
impl ServerHook {
    /// Waits for the server's shutdown signal or completion.
    ///
    /// This method asynchronously waits until the server's `wait_hook` is triggered,
    /// typically indicating that the server has finished its operations or is ready to shut down.
    pub async fn wait(&self) {
        self.get_wait_hook()().await;
    }

    /// Initiates the server shutdown process.
    ///
    /// This method asynchronously calls the `shutdown_hook`, which is responsible for
    /// performing any necessary cleanup or graceful shutdown procedures.
    pub async fn shutdown(&self) {
        self.get_shutdown_hook()().await;
    }
}

/// Manages the state for handling a single connection, including the stream and context.
///
/// This struct provides a convenient way to pass around the necessary components
/// for processing a request or WebSocket frame.
impl<'a> HandlerState<'a> {
    /// Creates a new HandlerState instance.
    ///
    /// # Arguments
    ///
    /// - `&'a ArcRwLockStream` - The network stream.
    /// - `&'a Context` - The request context.
    ///
    /// # Returns
    ///
    /// - `Self` - The newly created handler state.
    pub(super) fn new(stream: &'a ArcRwLockStream, ctx: &'a Context) -> Self {
        Self { stream, ctx }
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
    pub fn new() -> Self {
        let server: ServerInner = ServerInner::default();
        Self(arc_rwlock(server))
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
    pub async fn panic_hook<F, Fut>(&self, func: F) -> &Self
    where
        F: FnContextSendSyncStatic<Fut, ()>,
        Fut: FutureSendStatic<()>,
    {
        self.write()
            .await
            .get_mut_panic_hook()
            .push(Arc::new(move |ctx: Context| Box::pin(func(ctx))));
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
    pub async fn route<R, F, Fut>(&self, route: R, func: F) -> &Self
    where
        R: ToString,
        F: FnContextSendSyncStatic<Fut, ()>,
        Fut: FutureSendStatic<()>,
    {
        let route_str: String = route.to_string();
        self.write()
            .await
            .get_mut_route()
            .add(
                &route_str,
                Arc::new(move |ctx: Context| Box::pin(func(ctx))),
            )
            .unwrap_or_else(|err| panic!("{}", err));
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
    pub async fn request_middleware<F, Fut>(&self, func: F) -> &Self
    where
        F: FnContextSendSyncStatic<Fut, ()>,
        Fut: FutureSendStatic<()>,
    {
        self.write()
            .await
            .get_mut_request_middleware()
            .push(Arc::new(move |ctx: Context| Box::pin(func(ctx))));
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
    pub async fn response_middleware<F, Fut>(&self, func: F) -> &Self
    where
        F: FnContextSendSyncStatic<Fut, ()>,
        Fut: FutureSendStatic<()>,
    {
        self.write()
            .await
            .get_mut_response_middleware()
            .push(Arc::new(move |ctx: Context| Box::pin(func(ctx))));
        self
    }

    /// Adds a hook executed before WebSocket connection upgrade.
    ///
    /// # Arguments
    ///
    /// - `F: FnContextSendSyncStatic<Fut, ()>` - The hook function.
    /// - `Fut: FutureSendStatic<()>` - The future returned by the hook.
    ///
    /// # Returns
    ///
    /// - `&Self` - Reference to self for method chaining.
    pub async fn pre_upgrade_hook<F, Fut>(&self, func: F) -> &Self
    where
        F: FnContextSendSyncStatic<Fut, ()>,
        Fut: FutureSendStatic<()>,
    {
        self.write()
            .await
            .get_mut_pre_upgrade_hook()
            .push(Arc::new(move |ctx: Context| Box::pin(func(ctx))));
        self
    }

    /// Adds a hook executed after new client connection is established.
    ///
    /// # Arguments
    ///
    /// - `F: FnContextSendSyncStatic<Fut, ()>` - The hook function.
    /// - `Fut: FutureSendStatic<()>` - The future returned by the hook.
    ///
    /// # Returns
    ///
    /// - `&Self` - Reference to self for method chaining.
    pub async fn connected_hook<F, Fut>(&self, func: F) -> &Self
    where
        F: FnContextSendSyncStatic<Fut, ()>,
        Fut: FutureSendStatic<()>,
    {
        self.write()
            .await
            .get_mut_connected_hook()
            .push(Arc::new(move |ctx: Context| Box::pin(func(ctx))));
        self
    }

    /// Re-enables default HTTP handling for a route.
    ///
    /// # Arguments
    ///
    /// - `R: ToString` - The route path.
    ///
    /// # Returns
    ///
    /// - `&Self` - Reference to self for method chaining.
    pub async fn enable_http_hook<R: ToString>(&self, route: R) -> &Self {
        let route_string: String = route.to_string();
        self.write()
            .await
            .get_mut_disable_http_hook()
            .remove(&route_string);
        self
    }

    /// Disables default HTTP handling for a route.
    ///
    /// # Arguments
    ///
    /// - `R: ToString` - The route path.
    ///
    /// # Returns
    ///
    /// - `&Self` - Reference to self for method chaining.
    pub async fn disable_http_hook<R: ToString>(&self, route: R) -> &Self {
        let route_string: String = route.to_string();
        let _ = self
            .write()
            .await
            .get_mut_disable_http_hook()
            .add(&route_string, Arc::new(|_: Context| Box::pin(async {})));
        self
    }

    /// Re-enables default WebSocket handling for a route.
    ///
    /// # Arguments
    ///
    /// - `R: ToString` - The route path.
    ///
    /// # Returns
    ///
    /// - `&Self` - Reference to self for method chaining.
    pub async fn enable_ws_hook<R: ToString>(&self, route: R) -> &Self {
        let route_string: String = route.to_string();
        self.write()
            .await
            .get_mut_disable_ws_hook()
            .remove(&route_string);
        self
    }

    /// Disables default WebSocket handling for a route.
    ///
    /// # Arguments
    ///
    /// - `R: ToString` - The route path.
    ///
    /// # Returns
    ///
    /// - `&Self` - Reference to self for method chaining.
    pub async fn disable_ws_hook<R: ToString>(&self, route: R) -> &Self {
        let route_string: String = route.to_string();
        let _ = self
            .write()
            .await
            .get_mut_disable_ws_hook()
            .add(&route_string, Arc::new(|_: Context| Box::pin(async {})));
        self
    }

    /// Checks if default HTTP handling is disabled for a route.
    ///
    /// # Arguments
    ///
    /// - `R: ToString` - The route path.
    ///
    /// # Returns
    ///
    /// - `bool` - True if HTTP handling is disabled.
    async fn contains_disable_http_hook<'a, R: ToString>(&self, route: R) -> bool {
        let route_string: String = route.to_string();
        self.read()
            .await
            .get_disable_http_hook()
            .match_route(&route_string)
    }

    /// Checks if default WebSocket handling is disabled for a route.
    ///
    /// # Arguments
    ///
    /// - `R: ToString` - The route path.
    ///
    /// # Returns
    ///
    /// - `bool` - True if WebSocket handling is disabled.
    async fn contains_disable_ws_hook<'a, R: ToString>(&self, route: R) -> bool {
        let route_string: String = route.to_string();
        self.read()
            .await
            .get_disable_ws_hook()
            .match_route(&route_string)
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
        for func in self.read().await.get_panic_hook().iter() {
            func(ctx.clone()).await;
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
    /// - `F: Fn(Context) -> PinBoxFutureSendStatic` - The hook function to execute.
    async fn run_hook_with_lifecycle<F>(
        &self,
        ctx: &Context,
        lifecycle: &mut Lifecycle,
        hook_func: F,
    ) where
        F: Fn(Context) -> PinBoxFutureSendStatic,
    {
        let result: ResultJoinError<()> = spawn(hook_func(ctx.clone())).await;
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
        let config: ServerConfigInner = self.read().await.get_config().clone();
        let nodelay_opt: OptionBool = *config.get_nodelay();
        let linger_opt: OptionDuration = *config.get_linger();
        let ttl_opt: OptionU32 = *config.get_ttl();
        let _ = stream.set_linger(linger_opt);
        if let Some(nodelay) = nodelay_opt {
            let _ = stream.set_nodelay(nodelay);
        }
        if let Some(ttl) = ttl_opt {
            let _ = stream.set_ttl(ttl);
        }
    }

    /// Spawns a new asynchronous task to handle a single client connection.
    ///
    /// # Arguments
    ///
    /// - `ArcRwLockStream` - The thread-safe stream representing the client connection.
    async fn spawn_connection_handler(&self, stream: ArcRwLockStream) {
        let server: Server = self.clone();
        let http_buffer: usize = *self.read().await.get_config().get_http_buffer();
        spawn(async move {
            server.handle_connection(stream, http_buffer).await;
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
    async fn handle_connection(&self, stream: ArcRwLockStream, http_buffer: usize) {
        if let Ok(mut request) = Request::http_from_stream(&stream, http_buffer).await {
            let ctx: Context = Context::create_context(&stream, &request);
            let handler: HandlerState = HandlerState::new(&stream, &ctx);
            if request.is_ws() {
                self.ws_hook(&handler, &mut request).await;
            } else {
                self.http_hook(&handler, &request).await;
            }
        }
    }

    /// Executes all registered pre-upgrade hooks for a WebSocket connection.
    ///
    /// # Arguments
    ///
    /// - `&Context` - The request context.
    /// - `&mut Lifecycle` - A mutable reference to the request lifecycle state.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` if the lifecycle was aborted, `false` otherwise.
    async fn run_pre_upgrade_hook(&self, ctx: &Context, lifecycle: &mut Lifecycle) -> bool {
        for func in self.read().await.get_pre_upgrade_hook().iter() {
            self.run_hook_with_lifecycle(ctx, lifecycle, move |ctx: Context| func(ctx))
                .await;
            if lifecycle.is_abort() {
                return true;
            }
        }
        false
    }

    /// Executes all registered `connected` hooks.
    ///
    /// # Arguments
    ///
    /// - `&Context` - The request context.
    /// - `&mut Lifecycle` - A mutable reference to the request lifecycle state.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` if the lifecycle was aborted, `false` otherwise.
    async fn run_connected_hook(&self, ctx: &Context, lifecycle: &mut Lifecycle) -> bool {
        for func in self.read().await.get_connected_hook().iter() {
            self.run_hook_with_lifecycle(ctx, lifecycle, move |ctx: Context| func(ctx))
                .await;
            if lifecycle.is_abort() {
                return true;
            }
        }
        false
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
        for func in self.read().await.get_request_middleware().iter() {
            self.run_hook_with_lifecycle(ctx, lifecycle, move |ctx: Context| func(ctx))
                .await;
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
        if let Some(func) = handler {
            self.run_hook_with_lifecycle(ctx, lifecycle, move |ctx: Context| func(ctx))
                .await;
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
        for func in self.read().await.get_response_middleware().iter() {
            self.run_hook_with_lifecycle(ctx, lifecycle, move |ctx: Context| func(ctx))
                .await;
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
    /// - `&HandlerState<'a>` - The `HandlerState` for the current connection.
    /// - `&Request` - The incoming request to be processed.
    ///
    /// # Returns
    ///
    /// - `bool` - A boolean indicating whether the connection should be kept alive.
    async fn request_hook<'a>(&self, state: &HandlerState<'a>, request: &Request) -> bool {
        let route: &str = request.get_path();
        let ctx: &Context = state.ctx;
        ctx.set_request(request).await;
        let mut lifecycle: Lifecycle = Lifecycle::new_continue(request.is_enable_keep_alive());
        let route_hook: OptionArcFnContextPinBoxSendSync<()> = self
            .read()
            .await
            .get_route()
            .resolve_route(ctx, route)
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
    /// - `&HandlerState<'a>` - The `HandlerState` for the current connection.
    /// - `&Request` - The initial request that established the keep-alive connection.
    async fn handle_http_requests<'a>(&self, state: &HandlerState<'a>, request: &Request) {
        let route: &String = request.get_path();
        let contains_disable_http_hook: bool = self.contains_disable_http_hook(route).await;
        let buffer: usize = *self.read().await.get_config().get_http_buffer();
        if contains_disable_http_hook {
            while self.request_hook(state, request).await {}
            return;
        }
        while let Ok(new_request) = &Request::http_from_stream(state.stream, buffer).await {
            if !self.request_hook(state, new_request).await {
                return;
            }
        }
    }

    /// The main entry point for handling an HTTP connection.
    ///
    /// It runs connected hooks and then enters the request handling loop.
    ///
    /// # Arguments
    ///
    /// - `&HandlerState<'a>` - The `HandlerState` for the current connection.
    /// - `&Request` - The initial HTTP request from the client.
    async fn http_hook<'a>(&self, state: &HandlerState<'a>, request: &Request) {
        let ctx: &Context = state.ctx;
        let mut lifecycle: Lifecycle = Lifecycle::new();
        if self.run_connected_hook(ctx, &mut lifecycle).await {
            return;
        }
        if !self.request_hook(state, request).await {
            return;
        }
        self.handle_http_requests(state, request).await;
    }

    /// Handles the stream of incoming WebSocket frames after a connection is established.
    ///
    /// # Arguments
    ///
    /// - `&HandlerState<'a>` - The `HandlerState` for the current connection.
    /// - `&mut Request` - The mutable request object, which will be updated with each new frame.
    /// - `&str` - The route path that the WebSocket connection was established on.
    async fn handle_ws_requests<'a>(
        &self,
        state: &HandlerState<'a>,
        request: &mut Request,
        route: &str,
    ) {
        let disable_ws_hook_contains: bool = self.contains_disable_ws_hook(route).await;
        let buffer: usize = *self.read().await.get_config().get_ws_buffer();
        if disable_ws_hook_contains {
            while self.request_hook(state, request).await {}
            return;
        }
        while let Ok(new_request) = &Request::ws_from_stream(state.stream, buffer, request).await {
            let _ = self.request_hook(state, new_request).await;
        }
    }

    /// The main entry point for handling a WebSocket connection.
    ///
    /// This function manages the upgrade process, runs pre-upgrade and connected hooks,
    /// and then enters the WebSocket frame handling loop.
    ///
    /// # Arguments
    ///
    /// - `&HandlerState<'a>` - The `HandlerState` for the current connection.
    /// - `&mut Request` - The mutable HTTP request that initiated the WebSocket upgrade.
    async fn ws_hook<'a>(&self, state: &HandlerState<'a>, request: &mut Request) {
        let route: String = request.get_path().clone();
        let ctx: &Context = state.ctx;
        let mut lifecycle: Lifecycle = Lifecycle::new();
        self.read()
            .await
            .get_route()
            .resolve_route(ctx, &route)
            .await;
        if self.run_pre_upgrade_hook(ctx, &mut lifecycle).await {
            return;
        }
        if ctx.upgrade_to_ws().await.is_err() {
            return;
        }
        if self.run_connected_hook(ctx, &mut lifecycle).await {
            return;
        }
        self.handle_ws_requests(state, request, &route).await;
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
        let mut server_run: ServerHook = ServerHook::default();
        server_run.set_shutdown_hook(shutdown_hook);
        server_run.set_wait_hook(wait_hook);
        Ok(server_run)
    }
}
