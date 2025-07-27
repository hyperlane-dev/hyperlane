use crate::*;

impl Default for ServerInner {
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

impl<'a> HandlerState<'a> {
    pub(super) fn new(stream: &'a ArcRwLockStream, ctx: &'a Context) -> Self {
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

    pub async fn host<T: ToString>(&self, host: T) -> &Self {
        self.get_write()
            .await
            .get_mut_config()
            .set_host(host.to_string());
        self
    }

    pub async fn port(&self, port: usize) -> &Self {
        self.get_write().await.get_mut_config().set_port(port);
        self
    }

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

    pub async fn set_nodelay(&self, nodelay: bool) -> &Self {
        self.get_write()
            .await
            .get_mut_config()
            .set_nodelay(Some(nodelay));
        self
    }

    pub async fn enable_nodelay(&self) -> &Self {
        self.set_nodelay(true).await
    }

    pub async fn disable_nodelay(&self) -> &Self {
        self.set_nodelay(false).await
    }

    pub async fn set_linger(&self, linger: OptionDuration) -> &Self {
        self.get_write().await.get_mut_config().set_linger(linger);
        self
    }

    pub async fn enable_linger(&self, linger: Duration) -> &Self {
        self.set_linger(Some(linger)).await
    }

    pub async fn disable_linger(&self) -> &Self {
        self.set_linger(None).await
    }

    pub async fn set_ttl(&self, ttl: u32) -> &Self {
        self.get_write().await.get_mut_config().set_ttl(Some(ttl));
        self
    }

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

    pub async fn enable_http_hook<R: ToString>(&self, route: R) -> &Self {
        let route_string: String = route.to_string();
        self.get_write()
            .await
            .get_mut_disable_http_hook()
            .remove(&route_string);
        self
    }

    pub async fn disable_http_hook<R: ToString>(&self, route: R) -> &Self {
        let route_string: String = route.to_string();
        let _ = self
            .get_write()
            .await
            .get_mut_disable_http_hook()
            .add(&route_string, Arc::new(|_: Context| Box::pin(async {})));
        self
    }

    pub async fn enable_ws_hook<R: ToString>(&self, route: R) -> &Self {
        let route_string: String = route.to_string();
        self.get_write()
            .await
            .get_mut_disable_ws_hook()
            .remove(&route_string);
        self
    }

    pub async fn disable_ws_hook<R: ToString>(&self, route: R) -> &Self {
        let route_string: String = route.to_string();
        let _ = self
            .get_write()
            .await
            .get_mut_disable_ws_hook()
            .add(&route_string, Arc::new(|_: Context| Box::pin(async {})));
        self
    }

    async fn contains_disable_http_hook<'a>(&self, route: &'a str) -> bool {
        self.get_read()
            .await
            .get_disable_http_hook()
            .match_route(route)
    }

    async fn contains_disable_ws_hook<'a>(&self, route: &'a str) -> bool {
        self.get_read()
            .await
            .get_disable_ws_hook()
            .match_route(route)
    }

    pub fn format_host_port(host: &str, port: &usize) -> String {
        format!("{}{}{}", host, COLON_SPACE_SYMBOL, port)
    }

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

    async fn handle_panic_with_context(&self, ctx: &Context, panic: &Panic) {
        let panic_clone: Panic = panic.clone();
        let _ = ctx.set_panic(panic_clone).await;
        self.get_read().await.get_panic_hook()(ctx.clone()).await;
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
        let result: ResultJoinError<()> = tokio::spawn(hook_func(ctx.clone())).await;
        ctx.update_lifecycle_status(lifecycle).await;
        if let Err(join_error) = result {
            if join_error.is_panic() {
                self.handle_task_panic(&ctx, join_error).await;
            }
        }
    }

    async fn create_tcp_listener(&self) -> ServerResult<TcpListener> {
        let config: ServerConfig = self.get_read().await.get_config().clone();
        let host: &str = config.get_host();
        let port: usize = *config.get_port();
        let addr: String = Self::format_host_port(host, &port);
        TcpListener::bind(&addr)
            .await
            .map_err(|err| ServerError::TcpBind(err.to_string()))
    }

    async fn accept_connections(&self, tcp_listener: &TcpListener) -> ServerResult<()> {
        while let Ok((stream, _socket_addr)) = tcp_listener.accept().await {
            self.configure_stream(&stream).await;
            let stream: ArcRwLockStream = ArcRwLockStream::from_stream(stream);
            self.spawn_connection_handler(stream).await;
        }
        Ok(())
    }

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

    pub async fn run(&self) -> ServerResult<()> {
        self.init_panic_hook().await;
        let tcp_listener: TcpListener = self.create_tcp_listener().await?;
        self.accept_connections(&tcp_listener).await
    }
}
