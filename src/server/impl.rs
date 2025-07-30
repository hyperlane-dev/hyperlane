use crate::*;

/// Provides a default implementation for ServerInner.
impl Default for ServerInner {
    /// Creates a new ServerInner instance with default values.
    ///
    /// # Returns
    ///
    /// - `ServerInner` - A new instance with default configuration.
    fn default() -> Self {
        Self {
            config: ServerConfig::default(),
            route: RouteMatcher::new(),
            request_middleware: vec![],
            response_middleware: vec![],
            pre_upgrade_hook: vec![],
            connected_hook: vec![],
            disable_http_hook: RouteMatcher::new(),
            disable_ws_hook: RouteMatcher::new(),
            panic_hook: Arc::new(|ctx: Context| Box::pin(default_panic_hook(ctx))),
        }
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
    /// - `HandlerState` - The newly created handler state.
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
    /// - `Server` - A new Server instance.
    pub fn new() -> Self {
        let server: ServerInner = ServerInner::default();
        Self(arc_rwlock(server))
    }

    /// Acquires a read lock on the inner server data.
    ///
    /// # Returns
    ///
    /// - `RwLockReadGuardServerInner` - The read guard for ServerInner.
    async fn get_read(&self) -> RwLockReadGuardServerInner {
        self.get_0().read().await
    }

    /// Acquires a write lock on the inner server data.
    ///
    /// # Returns
    ///
    /// - `RwLockWriteGuardServerInner` - The write guard for ServerInner.
    async fn get_write(&self) -> RwLockWriteGuardServerInner {
        self.get_0().write().await
    }

    /// Sets the host address for the server.
    ///
    /// # Arguments
    ///
    /// - `T` - The host address implementing ToString.
    ///
    /// # Returns
    ///
    /// - `&Self` - Reference to self for method chaining.
    pub async fn host<T: ToString>(&self, host: T) -> &Self {
        self.get_write()
            .await
            .get_mut_config()
            .set_host(host.to_string());
        self
    }

    /// Sets the port number for the server.
    ///
    /// # Arguments
    ///
    /// - `usize` - The port number.
    ///
    /// # Returns
    ///
    /// - `&Self` - Reference to self for method chaining.
    pub async fn port(&self, port: usize) -> &Self {
        self.get_write().await.get_mut_config().set_port(port);
        self
    }

    /// Sets the read buffer size for HTTP connections.
    ///
    /// # Arguments
    ///
    /// - `usize` - The buffer size in bytes.
    ///
    /// # Returns
    ///
    /// - `&Self` - Reference to self for method chaining.
    pub async fn http_buffer(&self, buffer: usize) -> &Self {
        let buffer: usize = if buffer == 0 {
            DEFAULT_BUFFER_SIZE
        } else {
            buffer
        };
        self.get_write()
            .await
            .get_mut_config()
            .set_http_buffer(buffer);
        self
    }

    /// Sets the read buffer size for WebSocket connections.
    ///
    /// # Arguments
    ///
    /// - `usize` - The buffer size in bytes.
    ///
    /// # Returns
    ///
    /// - `&Self` - Reference to self for method chaining.
    pub async fn ws_buffer(&self, buffer: usize) -> &Self {
        let buffer: usize = if buffer == 0 {
            DEFAULT_BUFFER_SIZE
        } else {
            buffer
        };
        self.get_write()
            .await
            .get_mut_config()
            .set_ws_buffer(buffer);
        self
    }

    /// Sets a custom panic hook for request processing.
    ///
    /// # Arguments
    ///
    /// - `F` - The panic handler function implementing ErrorHandler<Fut>.
    /// - `Fut` - The future type implementing FutureSendStatic<()>.
    ///
    /// # Returns
    ///
    /// - `&Self` - Reference to self for method chaining.
    pub async fn panic_hook<F, Fut>(&self, func: F) -> &Self
    where
        F: ErrorHandler<Fut>,
        Fut: FutureSendStatic<()>,
    {
        self.get_write()
            .await
            .set_panic_hook(Arc::new(move |ctx: Context| Box::pin(func(ctx))));
        self
    }

    /// Sets the TCP_NODELAY socket option.
    ///
    /// # Arguments
    ///
    /// - `bool` - Whether to enable TCP_NODELAY.
    ///
    /// # Returns
    ///
    /// - `&Self` - Reference to self for method chaining.
    pub async fn set_nodelay(&self, nodelay: bool) -> &Self {
        self.get_write()
            .await
            .get_mut_config()
            .set_nodelay(Some(nodelay));
        self
    }

    /// Enables the TCP_NODELAY socket option.
    ///
    /// # Returns
    ///
    /// - `&Self` - Reference to self for method chaining.
    pub async fn enable_nodelay(&self) -> &Self {
        self.set_nodelay(true).await
    }

    /// Disables the TCP_NODELAY socket option.
    ///
    /// # Returns
    ///
    /// - `&Self` - Reference to self for method chaining.
    pub async fn disable_nodelay(&self) -> &Self {
        self.set_nodelay(false).await
    }

    /// Sets the SO_LINGER socket option.
    ///
    /// # Arguments
    ///
    /// - `OptionDuration` - The linger duration.
    ///
    /// # Returns
    ///
    /// - `&Self` - Reference to self for method chaining.
    pub async fn set_linger(&self, linger: OptionDuration) -> &Self {
        self.get_write().await.get_mut_config().set_linger(linger);
        self
    }

    /// Enables the SO_LINGER socket option.
    ///
    /// # Arguments
    ///
    /// - `Duration` - The linger duration.
    ///
    /// # Returns
    ///
    /// - `&Self` - Reference to self for method chaining.
    pub async fn enable_linger(&self, linger: Duration) -> &Self {
        self.set_linger(Some(linger)).await
    }

    /// Disables the SO_LINGER socket option.
    ///
    /// # Returns
    ///
    /// - `&Self` - Reference to self for method chaining.
    pub async fn disable_linger(&self) -> &Self {
        self.set_linger(None).await
    }

    /// Sets the IP_TTL socket option.
    ///
    /// # Arguments
    ///
    /// - `u32` - The TTL value.
    ///
    /// # Returns
    ///
    /// - `&Self` - Reference to self for method chaining.
    pub async fn set_ttl(&self, ttl: u32) -> &Self {
        self.get_write().await.get_mut_config().set_ttl(Some(ttl));
        self
    }

    /// Adds a route handler for a specific path.
    ///
    /// # Arguments
    ///
    /// - `R` - The route path pattern implementing ToString.
    /// - `F` - The handler function implementing FnSendSyncStatic<Fut>.
    ///
    /// # Returns
    ///
    /// - `&Self` - Reference to self for method chaining.
    pub async fn route<R, F, Fut>(&self, route: R, func: F) -> &Self
    where
        R: ToString,
        F: FnSendSyncStatic<Fut>,
        Fut: FutureSendStatic<()>,
    {
        let route_str: String = route.to_string();
        self.get_write()
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
    /// - `F` - The middleware function implementing FnSendSyncStatic<Fut>.
    ///
    /// # Returns
    ///
    /// - `&Self` - Reference to self for method chaining.
    pub async fn request_middleware<F, Fut>(&self, func: F) -> &Self
    where
        F: FnSendSyncStatic<Fut>,
        Fut: FutureSendStatic<()>,
    {
        self.get_write()
            .await
            .get_mut_request_middleware()
            .push(Arc::new(move |ctx: Context| Box::pin(func(ctx))));
        self
    }

    /// Adds response middleware to the processing pipeline.
    ///
    /// # Arguments
    ///
    /// - `F: FnSendSyncStatic<Fut>` - The middleware function.
    ///
    /// # Returns
    ///
    /// - `&Self` - Reference to self for method chaining.
    pub async fn response_middleware<F, Fut>(&self, func: F) -> &Self
    where
        F: FnSendSyncStatic<Fut>,
        Fut: FutureSendStatic<()>,
    {
        self.get_write()
            .await
            .get_mut_response_middleware()
            .push(Arc::new(move |ctx: Context| Box::pin(func(ctx))));
        self
    }

    /// Adds a hook executed before WebSocket connection upgrade.
    ///
    /// # Arguments
    ///
    /// - `F: FnSendSyncStatic<Fut>` - The hook function.
    ///
    /// # Returns
    ///
    /// - `&Self` - Reference to self for method chaining.
    pub async fn pre_upgrade_hook<F, Fut>(&self, func: F) -> &Self
    where
        F: FnSendSyncStatic<Fut>,
        Fut: FutureSendStatic<()>,
    {
        self.get_write()
            .await
            .get_mut_pre_upgrade_hook()
            .push(Arc::new(move |ctx: Context| Box::pin(func(ctx))));
        self
    }

    /// Adds a hook executed after new client connection is established.
    ///
    /// # Arguments
    ///
    /// - `F: FnSendSyncStatic<Fut>` - The hook function.
    ///
    /// # Returns
    ///
    /// - `&Self` - Reference to self for method chaining.
    pub async fn connected_hook<F, Fut>(&self, func: F) -> &Self
    where
        F: FnSendSyncStatic<Fut>,
        Fut: FutureSendStatic<()>,
    {
        self.get_write()
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
        self.get_write()
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
            .get_write()
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
        self.get_write()
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
            .get_write()
            .await
            .get_mut_disable_ws_hook()
            .add(&route_string, Arc::new(|_: Context| Box::pin(async {})));
        self
    }

    /// Checks if default HTTP handling is disabled for a route.
    ///
    /// # Arguments
    ///
    /// - `&str` - The route path.
    ///
    /// # Returns
    ///
    /// - `bool` - True if HTTP handling is disabled.
    async fn contains_disable_http_hook<'a>(&self, route: &'a str) -> bool {
        self.get_read()
            .await
            .get_disable_http_hook()
            .match_route(route)
    }

    /// Checks if default WebSocket handling is disabled for a route.
    ///
    /// # Arguments
    ///
    /// - `&str` - The route path.
    ///
    /// # Returns
    ///
    /// - `bool` - True if WebSocket handling is disabled.
    async fn contains_disable_ws_hook<'a>(&self, route: &'a str) -> bool {
        self.get_read()
            .await
            .get_disable_ws_hook()
            .match_route(route)
    }

    /// Formats the host and port into a bindable address string.
    ///
    /// # Arguments
    ///
    /// - `str` - The host address.
    /// - `usize` - The port number.
    ///
    /// # Returns
    ///
    /// - `String` - The formatted address string.
    pub fn format_host_port(host: &str, port: &usize) -> String {
        format!("{}{}{}", host, COLON_SPACE_SYMBOL, port)
    }

    /// Initializes the global panic hook for the entire application.
    ///
    /// This sets a custom panic hook that captures panic information and forwards it
    /// to the server's configured panic handler.
    async fn init_panic_hook(&self) {
        let server_clone: Server = self.clone();
        let panic_hook: ArcErrorHandlerSendSync =
            server_clone.get_read().await.get_panic_hook().clone();
        set_hook(Box::new(move |panic: &PanicHookInfo<'_>| {
            let panic_struct: Panic = Panic::from_panic_hook(panic);
            let ctx: Context = Context::default();
            let panic_hook_clone: ArcErrorHandlerSendSync = panic_hook.clone();
            tokio::spawn(async move {
                ctx.set_panic(panic_struct).await;
                panic_hook_clone(ctx).await;
            });
        }));
    }

    /// Handles a panic that occurred within a request's context.
    ///
    /// This function associates the panic information with the context and invokes the
    /// server's configured panic hook.
    ///
    /// # Arguments
    ///
    /// - `ctx` - The context of the request during which the panic occurred.
    /// - `panic` - The captured panic information.
    async fn handle_panic_with_context(&self, ctx: &Context, panic: &Panic) {
        let panic_clone: Panic = panic.clone();
        let _ = ctx.set_panic(panic_clone).await;
        self.get_read().await.get_panic_hook()(ctx.clone()).await;
    }

    /// Handles a panic that occurred within a spawned Tokio task.
    ///
    /// It extracts the panic information from the `JoinError` and processes it.
    ///
    /// # Arguments
    ///
    /// - `ctx` - The context associated with the task.
    /// - `join_error` - The `JoinError` returned from the panicked task.
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
    /// - `ctx` - The request context.
    /// - `lifecycle` - A mutable reference to the current `Lifecycle` state.
    /// - `hook_func` - The hook function to execute.
    async fn run_hook_with_lifecycle<F>(
        &self,
        ctx: &Context,
        lifecycle: &mut Lifecycle,
        hook_func: F,
    ) where
        F: Fn(Context) -> PinBoxFutureSendStatic,
    {
        let result: ResultJoinError<()> = tokio::spawn(hook_func(ctx.clone())).await;
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
        let config: ServerConfig = self.get_read().await.get_config().clone();
        let host: &str = config.get_host();
        let port: usize = *config.get_port();
        let addr: String = Self::format_host_port(host, &port);
        TcpListener::bind(&addr)
            .await
            .map_err(|err| ServerError::TcpBind(err.to_string()))
    }

    /// Enters a loop to accept incoming TCP connections and spawn handlers for them.
    ///
    /// # Arguments
    ///
    /// - `tcp_listener` - A reference to the `TcpListener` to accept connections from.
    ///
    /// # Returns
    ///
    /// A `ServerResult` which is typically `Ok(())` unless an unrecoverable
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
    /// - `stream` - A reference to the `TcpStream` to configure.
    async fn configure_stream(&self, stream: &TcpStream) {
        let config: ServerConfig = self.get_read().await.get_config().clone();
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
    /// - `stream` - The thread-safe stream representing the client connection.
    async fn spawn_connection_handler(&self, stream: ArcRwLockStream) {
        let server: Server = self.clone();
        let http_buffer: usize = *self.get_read().await.get_config().get_http_buffer();
        tokio::spawn(async move {
            server.handle_connection(stream, http_buffer).await;
        });
    }

    /// Handles a single client connection, determining whether it's an HTTP or WebSocket request.
    ///
    /// It reads the initial request from the stream and dispatches it to the appropriate handler.
    ///
    /// # Arguments
    ///
    /// - `stream` - The stream for the client connection.
    /// - `http_buffer` - The buffer size to use for reading the initial HTTP request.
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
    /// - `ctx` - The request context.
    /// - `lifecycle` - A mutable reference to the request lifecycle state.
    async fn run_pre_upgrade_hook(&self, ctx: &Context, lifecycle: &mut Lifecycle) {
        for func in self.get_read().await.get_pre_upgrade_hook().iter() {
            self.run_hook_with_lifecycle(ctx, lifecycle, move |ctx: Context| func(ctx))
                .await;
        }
    }

    /// Executes all registered `connected` hooks.
    ///
    /// # Arguments
    ///
    /// - `ctx` - The request context.
    /// - `lifecycle` - A mutable reference to the request lifecycle state.
    async fn run_connected_hook(&self, ctx: &Context, lifecycle: &mut Lifecycle) {
        for func in self.get_read().await.get_connected_hook().iter() {
            self.run_hook_with_lifecycle(ctx, lifecycle, move |ctx: Context| func(ctx))
                .await;
        }
    }

    /// Executes all registered request middleware in sequence.
    ///
    /// # Arguments
    ///
    /// - `ctx` - The request context.
    /// - `lifecycle` - A mutable reference to the request lifecycle state.
    async fn run_request_middleware(&self, ctx: &Context, lifecycle: &mut Lifecycle) {
        for func in self.get_read().await.get_request_middleware().iter() {
            self.run_hook_with_lifecycle(ctx, lifecycle, move |ctx: Context| func(ctx))
                .await;
        }
    }

    /// Executes the matched route handler.
    ///
    /// # Arguments
    ///
    /// - `ctx` - The request context.
    /// - `handler` - An `Option` containing the handler function if a route was matched.
    /// - `lifecycle` - A mutable reference to the request lifecycle state.
    async fn run_route_hook(
        &self,
        ctx: &Context,
        handler: &OptionArcFnPinBoxSendSync,
        lifecycle: &mut Lifecycle,
    ) {
        if let Some(func) = handler {
            self.run_hook_with_lifecycle(ctx, lifecycle, move |ctx: Context| func(ctx))
                .await;
        }
    }

    /// Executes all registered response middleware in sequence.
    ///
    /// # Arguments
    ///
    /// - `ctx` - The request context.
    /// - `lifecycle` - A mutable reference to the request lifecycle state.
    async fn run_response_middleware(&self, ctx: &Context, lifecycle: &mut Lifecycle) {
        for func in self.get_read().await.get_response_middleware().iter() {
            self.run_hook_with_lifecycle(ctx, lifecycle, move |ctx: Context| func(ctx))
                .await;
        }
    }

    /// The core request handling pipeline.
    ///
    /// This function orchestrates the execution of request middleware, the route handler,
    /// and response middleware.
    ///
    /// # Arguments
    ///
    /// - `state` - The `HandlerState` for the current connection.
    /// - `request` - The incoming request to be processed.
    ///
    /// # Returns
    ///
    /// A boolean indicating whether the connection should be kept alive.
    async fn request_hook<'a>(&self, state: &HandlerState<'a>, request: &Request) -> bool {
        let route: &str = request.get_path();
        let ctx: &Context = state.ctx;
        ctx.set_request(request).await;
        let mut lifecycle: Lifecycle = Lifecycle::new_continue(request.is_enable_keep_alive());
        let route_hook: OptionArcFnPinBoxSendSync = self
            .get_read()
            .await
            .get_route()
            .resolve_route(ctx, route)
            .await;
        self.run_request_middleware(ctx, &mut lifecycle).await;
        if lifecycle.is_abort() {
            return lifecycle.keep_alive();
        }
        self.run_route_hook(ctx, &route_hook, &mut lifecycle).await;
        if lifecycle.is_abort() {
            return lifecycle.keep_alive();
        }
        self.run_response_middleware(ctx, &mut lifecycle).await;
        lifecycle.keep_alive()
    }

    /// Handles subsequent HTTP requests on a persistent (keep-alive) connection.
    ///
    /// # Arguments
    ///
    /// - `state` - The `HandlerState` for the current connection.
    /// - `request` - The initial request that established the keep-alive connection.
    async fn handle_http_requests<'a>(&self, state: &HandlerState<'a>, request: &Request) {
        let route: &String = request.get_path();
        let contains_disable_http_hook: bool = self.contains_disable_http_hook(route).await;
        let buffer: usize = *self.get_read().await.get_config().get_http_buffer();
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
    /// - `state` - The `HandlerState` for the current connection.
    /// - `request` - The initial HTTP request from the client.
    async fn http_hook<'a>(&self, state: &HandlerState<'a>, request: &Request) {
        let ctx: &Context = state.ctx;
        let mut lifecycle: Lifecycle = Lifecycle::new();
        self.run_connected_hook(ctx, &mut lifecycle).await;
        if lifecycle.is_abort() {
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
    /// - `state` - The `HandlerState` for the current connection.
    /// - `request` - The mutable request object, which will be updated with each new frame.
    /// - `route` - The route path that the WebSocket connection was established on.
    async fn handle_ws_requests<'a>(
        &self,
        state: &HandlerState<'a>,
        request: &mut Request,
        route: &str,
    ) {
        let disable_ws_hook_contains: bool = self.contains_disable_ws_hook(route).await;
        let buffer: usize = *self.get_read().await.get_config().get_ws_buffer();
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
    /// - `state` - The `HandlerState` for the current connection.
    /// - `request` - The mutable HTTP request that initiated the WebSocket upgrade.
    async fn ws_hook<'a>(&self, state: &HandlerState<'a>, request: &mut Request) {
        let route: String = request.get_path().clone();
        let ctx: &Context = state.ctx;
        let mut lifecycle: Lifecycle = Lifecycle::new();
        self.get_read()
            .await
            .get_route()
            .resolve_route(ctx, &route)
            .await;
        self.run_pre_upgrade_hook(ctx, &mut lifecycle).await;
        if lifecycle.is_abort() {
            return;
        }
        if ctx.upgrade_to_ws().await.is_err() {
            return;
        }
        self.run_connected_hook(ctx, &mut lifecycle).await;
        if lifecycle.is_abort() {
            return;
        }
        self.handle_ws_requests(state, request, &route).await;
    }

    /// Starts the server, binds to the configured address, and begins listening for connections.
    ///
    /// This is the main entry point to launch the server. It will initialize the panic hook,
    /// create a TCP listener, and then enter the connection acceptance loop.
    ///
    /// # Returns
    ///
    /// Returns a `ServerResult` which will be an error if the server fails to start.
    pub async fn run(&self) -> ServerResult<()> {
        self.init_panic_hook().await;
        let tcp_listener: TcpListener = self.create_tcp_listener().await?;
        self.accept_connections(&tcp_listener).await
    }
}
