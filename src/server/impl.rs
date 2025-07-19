use crate::*;

impl Default for ServerInner {
    fn default() -> Self {
        Self {
            config: ServerConfig::default(),
            route_matcher: RouteMatcher::new(),
            request_middleware: vec![],
            response_middleware: vec![],
            pre_upgrade_hook: vec![],
            connected_hook: vec![],
            disable_http_hook: hash_set_xx_hash3_64(),
            disable_ws_hook: hash_set_xx_hash3_64(),
            error_hook: Arc::new(|ctx: Context| Box::pin(default_error_hook(ctx))),
        }
    }
}

impl<'a> HandlerState<'a> {
    fn new(stream: &'a ArcRwLockStream, ctx: &'a Context) -> Self {
        Self { stream, ctx }
    }
}

impl Server {
    pub fn new() -> Self {
        let server: ServerInner = ServerInner::default();
        Self(arc_rwlock(server))
    }

    async fn get_read(&self) -> RwLockReadGuardServerInner {
        self.get_0().read().await
    }

    async fn get_write(&self) -> RwLockWriteGuardServerInner {
        self.get_0().write().await
    }

    pub fn host<T: ToString>(&self, host: T) -> &Self {
        sync_block_on(async {
            self.get_write()
                .await
                .get_mut_config()
                .set_host(host.to_string());
        });
        self
    }

    pub fn port(&self, port: usize) -> &Self {
        sync_block_on(async {
            self.get_write().await.get_mut_config().set_port(port);
        });
        self
    }

    pub fn http_buffer(&self, buffer: usize) -> &Self {
        let buffer: usize = if buffer == 0 {
            DEFAULT_BUFFER_SIZE
        } else {
            buffer
        };
        sync_block_on(async {
            self.get_write()
                .await
                .get_mut_config()
                .set_http_buffer(buffer);
        });
        self
    }

    pub fn ws_buffer(&self, buffer: usize) -> &Self {
        let buffer: usize = if buffer == 0 {
            DEFAULT_BUFFER_SIZE
        } else {
            buffer
        };
        sync_block_on(async {
            self.get_write()
                .await
                .get_mut_config()
                .set_ws_buffer(buffer);
        });
        self
    }

    pub fn error_hook<F, Fut>(&self, func: F) -> &Self
    where
        F: ErrorHandler<Fut>,
        Fut: FutureSendStatic<()>,
    {
        sync_block_on(async {
            self.get_write()
                .await
                .set_error_hook(Arc::new(move |ctx: Context| {
                    Box::pin(func(ctx)) as PinBoxFutureSendStatic
                }));
        });
        self
    }

    pub fn set_nodelay(&self, nodelay: bool) -> &Self {
        sync_block_on(async {
            self.get_write()
                .await
                .get_mut_config()
                .set_nodelay(Some(nodelay));
        });
        self
    }

    pub fn enable_nodelay(&self) -> &Self {
        self.set_nodelay(true)
    }

    pub fn disable_nodelay(&self) -> &Self {
        self.set_nodelay(false)
    }

    pub fn set_linger(&self, linger: OptionDuration) -> &Self {
        sync_block_on(async {
            self.get_write().await.get_mut_config().set_linger(linger);
        });
        self
    }

    pub fn enable_linger(&self, linger: Duration) -> &Self {
        self.set_linger(Some(linger))
    }

    pub fn disable_linger(&self) -> &Self {
        self.set_linger(None)
    }

    pub fn set_ttl(&self, ttl: u32) -> &Self {
        sync_block_on(async {
            self.get_write().await.get_mut_config().set_ttl(Some(ttl));
        });
        self
    }

    pub fn route<R, F, Fut>(&self, route: R, func: F) -> &Self
    where
        R: ToString,
        F: FnSendSyncStatic<Fut>,
        Fut: FutureSendStatic<()>,
    {
        let route_str: String = route.to_string();
        let arc_func = Arc::new(move |ctx: Context| Box::pin(func(ctx)) as PinBoxFutureSendStatic);
        sync_block_on(async {
            self.get_write()
                .await
                .get_mut_route_matcher()
                .add(&route_str, arc_func)
                .unwrap_or_else(|err| panic!("{}", err));
        });
        self
    }

    pub fn request_middleware<F, Fut>(&self, func: F) -> &Self
    where
        F: FnSendSyncStatic<Fut>,
        Fut: FutureSendStatic<()>,
    {
        sync_block_on(async {
            self.get_write()
                .await
                .get_mut_request_middleware()
                .push(Arc::new(move |ctx: Context| Box::pin(func(ctx))));
        });
        self
    }

    pub fn response_middleware<F, Fut>(&self, func: F) -> &Self
    where
        F: FnSendSyncStatic<Fut>,
        Fut: FutureSendStatic<()>,
    {
        sync_block_on(async {
            self.get_write()
                .await
                .get_mut_response_middleware()
                .push(Arc::new(move |ctx: Context| Box::pin(func(ctx))));
        });
        self
    }

    pub fn pre_upgrade_hook<F, Fut>(&self, func: F) -> &Self
    where
        F: FnSendSyncStatic<Fut>,
        Fut: FutureSendStatic<()>,
    {
        sync_block_on(async {
            self.get_write()
                .await
                .get_mut_pre_upgrade_hook()
                .push(Arc::new(move |ctx: Context| Box::pin(func(ctx))));
        });
        self
    }

    pub fn connected_hook<F, Fut>(&self, func: F) -> &Self
    where
        F: FnSendSyncStatic<Fut>,
        Fut: FutureSendStatic<()>,
    {
        sync_block_on(async {
            self.get_write()
                .await
                .get_mut_connected_hook()
                .push(Arc::new(move |ctx: Context| Box::pin(func(ctx))));
        });
        self
    }

    pub fn enable_http_hook<R: ToString>(&self, route: R) -> &Self {
        let route_string: String = route.to_string();
        sync_block_on(async {
            self.get_write()
                .await
                .get_mut_disable_http_hook()
                .remove(&route_string);
        });
        self
    }

    pub fn disable_http_hook<R: ToString>(&self, route: R) -> &Self {
        let route_string: String = route.to_string();
        sync_block_on(async {
            self.get_write()
                .await
                .get_mut_disable_http_hook()
                .insert(route_string);
        });
        self
    }

    pub fn enable_ws_hook<R: ToString>(&self, route: R) -> &Self {
        let route_string: String = route.to_string();
        sync_block_on(async {
            self.get_write()
                .await
                .get_mut_disable_ws_hook()
                .remove(&route_string);
        });
        self
    }

    pub fn disable_ws_hook<R: ToString>(&self, route: R) -> &Self {
        let route_string: String = route.to_string();
        sync_block_on(async {
            self.get_write()
                .await
                .get_mut_disable_ws_hook()
                .insert(route_string);
        });
        self
    }

    pub fn format_host_port(host: &str, port: &usize) -> String {
        format!("{}{}{}", host, COLON_SPACE_SYMBOL, port)
    }

    fn init_panic_hook(&self) {
        let server_clone = self.clone();
        let error_hook: ArcErrorHandlerSendSync =
            sync_block_on(async { server_clone.get_read().await.get_error_hook().clone() });
        set_hook(Box::new(move |panic: &PanicHookInfo<'_>| {
            let panic_struct: Panic = Panic::from_panic_hook(panic);
            let ctx: Context = Context::default();
            let error_hook_clone: ArcErrorHandlerSendSync = error_hook.clone();
            tokio::spawn(async move {
                let _ = ctx.set_panic(panic_struct).await;
                error_hook_clone(ctx).await;
            });
        }));
    }

    async fn handle_panic_with_context(&self, ctx: &Context, panic: &Panic) {
        let ctx_clone: Context = ctx.clone();
        let panic_clone: Panic = panic.clone();
        let _ = ctx_clone.set_panic(panic_clone).await;
        let error_hook = self.get_read().await.get_error_hook().clone();
        error_hook(ctx_clone).await;
    }

    async fn handle_task_panic(&self, ctx: &Context, join_error: JoinError) {
        let panic: Panic = Panic::from_join_error(join_error);
        self.handle_panic_with_context(&ctx, &panic).await;
    }

    async fn run_hook_with_lifecycle<F>(
        &self,
        ctx: &Context,
        lifecycle: &mut Lifecycle,
        hook_func: F,
    ) where
        F: Fn(Context) -> PinBoxFutureSendStatic,
    {
        let result: Result<(), JoinError> = tokio::spawn(hook_func(ctx.clone())).await;
        ctx.should_abort(lifecycle).await;
        if let Err(join_error) = result {
            if join_error.is_panic() {
                self.handle_task_panic(&ctx, join_error).await;
            }
        }
    }

    fn init(&self) {
        self.init_panic_hook();
    }

    pub fn run(&self) -> ServerResult {
        self.init();
        sync_block_on(async {
            let tcp_listener: TcpListener = self.create_tcp_listener().await?;
            self.accept_connections(&tcp_listener).await
        })
    }

    async fn create_tcp_listener(&self) -> Result<TcpListener, ServerError> {
        let config: ServerConfig = self.get_read().await.get_config().clone();
        let host: &str = config.get_host();
        let port: usize = *config.get_port();
        let addr: String = Self::format_host_port(host, &port);
        TcpListener::bind(&addr)
            .await
            .map_err(|err| ServerError::TcpBind(err.to_string()))
    }

    async fn accept_connections(&self, tcp_listener: &TcpListener) -> ServerResult {
        while let Ok((stream, _socket_addr)) = tcp_listener.accept().await {
            self.configure_stream(&stream).await;
            let stream: ArcRwLockStream = ArcRwLockStream::from_stream(stream);
            self.spawn_connection_handler(stream).await;
        }
        Ok(())
    }

    async fn configure_stream(&self, stream: &TcpStream) {
        let config: ServerConfig = self.get_read().await.get_config().clone();
        let nodelay_opt: Option<bool> = *config.get_nodelay();
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

    async fn spawn_connection_handler(&self, stream: ArcRwLockStream) {
        let server: Server = self.clone();
        let http_buffer: usize = *self.get_read().await.get_config().get_http_buffer();
        tokio::spawn(async move {
            server.handle_connection(stream, http_buffer).await;
        });
    }

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

    async fn run_pre_upgrade_hook(&self, ctx: &Context, lifecycle: &mut Lifecycle) {
        for func in self.get_read().await.get_pre_upgrade_hook().iter() {
            self.run_hook_with_lifecycle(ctx, lifecycle, move |ctx: Context| func(ctx))
                .await;
        }
    }

    async fn run_connected_hook(&self, ctx: &Context, lifecycle: &mut Lifecycle) {
        for func in self.get_read().await.get_connected_hook().iter() {
            self.run_hook_with_lifecycle(ctx, lifecycle, move |ctx: Context| func(ctx))
                .await;
        }
    }

    async fn run_request_middleware(&self, ctx: &Context, lifecycle: &mut Lifecycle) {
        for func in self.get_read().await.get_request_middleware().iter() {
            self.run_hook_with_lifecycle(ctx, lifecycle, move |ctx: Context| func(ctx))
                .await;
        }
    }

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

    async fn run_response_middleware(&self, ctx: &Context, lifecycle: &mut Lifecycle) {
        for func in self.get_read().await.get_response_middleware().iter() {
            self.run_hook_with_lifecycle(ctx, lifecycle, move |ctx: Context| func(ctx))
                .await;
        }
    }

    async fn request_hook<'a>(&self, state: &HandlerState<'a>, request: &Request) -> bool {
        let route: &str = request.get_path();
        let ctx: &Context = state.ctx;
        self.setup_request_context(ctx, request).await;
        let mut lifecycle: Lifecycle = Lifecycle::Continue(request.is_enable_keep_alive());
        let route_hook: OptionArcFnPinBoxSendSync = self.resolve_route_hook(ctx, route).await;
        if !self.execute_request_middleware(ctx, &mut lifecycle).await {
            return self.extract_keepalive_from_lifecycle(lifecycle);
        }
        if !self
            .execute_route_hook(ctx, &route_hook, &mut lifecycle)
            .await
        {
            return self.extract_keepalive_from_lifecycle(lifecycle);
        }
        if !self.execute_response_middleware(ctx, &mut lifecycle).await {
            return self.extract_keepalive_from_lifecycle(lifecycle);
        }
        self.extract_keepalive_from_lifecycle(lifecycle)
    }

    async fn setup_request_context(&self, ctx: &Context, request: &Request) {
        ctx.set_request(request).await;
    }

    async fn resolve_route_hook(&self, ctx: &Context, route: &str) -> OptionArcFnPinBoxSendSync {
        self.get_read()
            .await
            .get_route_matcher()
            .resolve_route(ctx, route)
            .await
    }

    async fn execute_request_middleware(&self, ctx: &Context, lifecycle: &mut Lifecycle) -> bool {
        self.run_request_middleware(ctx, lifecycle).await;
        !matches!(lifecycle, Lifecycle::Abort(_))
    }

    async fn execute_route_hook(
        &self,
        ctx: &Context,
        route_hook: &OptionArcFnPinBoxSendSync,
        lifecycle: &mut Lifecycle,
    ) -> bool {
        self.run_route_hook(ctx, route_hook, lifecycle).await;
        !matches!(lifecycle, Lifecycle::Abort(_))
    }

    async fn execute_response_middleware(&self, ctx: &Context, lifecycle: &mut Lifecycle) -> bool {
        self.run_response_middleware(ctx, lifecycle).await;
        !matches!(lifecycle, Lifecycle::Abort(_))
    }

    fn extract_keepalive_from_lifecycle(&self, lifecycle: Lifecycle) -> bool {
        match lifecycle {
            Lifecycle::Continue(res) | Lifecycle::Abort(res) => res,
        }
    }

    async fn ws_hook<'a>(&self, state: &HandlerState<'a>, first_request: &mut Request) {
        let route: String = first_request.get_path().clone();
        let ctx: &Context = state.ctx;
        let mut lifecycle: Lifecycle = Lifecycle::Continue(true);
        self.resolve_route_for_ws(ctx, &route).await;
        if !self.execute_pre_upgrade_hooks(ctx, &mut lifecycle).await {
            return;
        }
        if !self.upgrade_connection_to_ws(ctx).await {
            return;
        }
        if !self.execute_connected_hooks(ctx, &mut lifecycle).await {
            return;
        }
        self.handle_ws_requests(state, first_request, &route).await;
    }

    async fn resolve_route_for_ws(&self, ctx: &Context, route: &str) {
        self.get_read()
            .await
            .get_route_matcher()
            .resolve_route(ctx, route)
            .await;
    }

    async fn execute_pre_upgrade_hooks(&self, ctx: &Context, lifecycle: &mut Lifecycle) -> bool {
        self.run_pre_upgrade_hook(ctx, lifecycle).await;
        !lifecycle.is_abort()
    }

    async fn upgrade_connection_to_ws(&self, ctx: &Context) -> bool {
        ctx.upgrade_to_ws().await.is_ok()
    }

    async fn execute_connected_hooks(&self, ctx: &Context, lifecycle: &mut Lifecycle) -> bool {
        self.run_connected_hook(ctx, lifecycle).await;
        !lifecycle.is_abort()
    }

    async fn handle_ws_requests<'a>(
        &self,
        state: &HandlerState<'a>,
        first_request: &mut Request,
        route: &str,
    ) {
        let (disable_ws_hook_contains, buffer) = self.get_ws_config(route).await;
        if disable_ws_hook_contains {
            self.handle_disabled_ws_hook(state, first_request).await;
            return;
        }
        self.process_ws_messages(state, first_request, buffer).await;
    }

    async fn get_ws_config(&self, route: &str) -> (bool, usize) {
        let server_guard: RwLockReadGuardServerInner = self.get_read().await;
        (
            server_guard.get_disable_ws_hook().contains(route),
            *server_guard.get_config().get_ws_buffer(),
        )
    }

    async fn handle_disabled_ws_hook<'a>(
        &self,
        state: &HandlerState<'a>,
        first_request: &mut Request,
    ) {
        while self.request_hook(state, first_request).await {}
    }

    async fn process_ws_messages<'a>(
        &self,
        state: &HandlerState<'a>,
        first_request: &mut Request,
        buffer: usize,
    ) {
        let stream: &ArcRwLockStream = state.stream;
        while let Ok(request) = Request::ws_from_stream(stream, buffer, first_request)
            .await
            .as_ref()
        {
            let _ = self.request_hook(state, request).await;
        }
    }

    async fn http_hook<'a>(&self, state: &HandlerState<'a>, first_request: &Request) {
        let ctx: &Context = state.ctx;
        let mut lifecycle: Lifecycle = Lifecycle::Continue(true);
        if !self
            .execute_connected_hooks_for_http(ctx, &mut lifecycle)
            .await
        {
            return;
        }
        if !self.process_first_http_request(state, first_request).await {
            return;
        }
        self.handle_http_requests(state, first_request).await;
    }

    async fn execute_connected_hooks_for_http(
        &self,
        ctx: &Context,
        lifecycle: &mut Lifecycle,
    ) -> bool {
        self.run_connected_hook(ctx, lifecycle).await;
        !lifecycle.is_abort()
    }

    async fn process_first_http_request<'a>(
        &self,
        state: &HandlerState<'a>,
        first_request: &Request,
    ) -> bool {
        self.request_hook(state, first_request).await
    }

    async fn handle_http_requests<'a>(&self, state: &HandlerState<'a>, first_request: &Request) {
        let route: &String = first_request.get_path();
        let (contains_disable_http_hook, buffer) = self.get_http_config(route).await;
        if contains_disable_http_hook {
            self.handle_disabled_http_hook(state, first_request).await;
            return;
        }
        self.process_http_messages(state, buffer).await;
    }

    async fn get_http_config(&self, route: &str) -> (bool, usize) {
        let server_guard: RwLockReadGuardServerInner = self.get_read().await;
        (
            server_guard.get_disable_http_hook().contains(route),
            *server_guard.get_config().get_http_buffer(),
        )
    }

    async fn handle_disabled_http_hook<'a>(
        &self,
        state: &HandlerState<'a>,
        first_request: &Request,
    ) {
        while self.request_hook(state, first_request).await {}
    }

    async fn process_http_messages<'a>(&self, state: &HandlerState<'a>, buffer: usize) {
        let stream: &ArcRwLockStream = state.stream;
        while let Ok(request) = Request::http_from_stream(stream, buffer).await.as_ref() {
            if !self.request_hook(state, request).await {
                return;
            }
        }
    }
}
