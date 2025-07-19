use crate::*;

impl Default for ServerBuilder {
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
            error_hook: Arc::new(|_ctx: Context| Box::pin(async {})),
        }
    }
}

impl<'a> HandlerState<'a> {
    fn new(stream: &'a ArcRwLockStream, ctx: &'a Context) -> Self {
        Self { stream, ctx }
    }
}

impl ServerBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(self) -> Server {
        let server: ServerInner = ServerInner::default()
            .set_config(self.get_config().clone())
            .set_route_matcher(self.get_route_matcher().clone())
            .set_request_middleware(self.get_request_middleware().clone())
            .set_response_middleware(self.get_response_middleware().clone())
            .set_pre_upgrade_hook(self.get_pre_upgrade_hook().clone())
            .set_connected_hook(self.get_connected_hook().clone())
            .set_disable_http_hook(self.get_disable_http_hook().clone())
            .set_disable_ws_hook(self.get_disable_ws_hook().clone())
            .set_error_hook(self.get_error_hook().clone())
            .clone();
        super::Server(Arc::new(server))
    }

    pub fn host<T: ToString>(mut self, host: T) -> Self {
        self.get_mut_config().set_host(host.to_string());
        self
    }

    pub fn port(mut self, port: usize) -> Self {
        self.get_mut_config().set_port(port);
        self
    }

    pub fn http_buffer(mut self, buffer: usize) -> Self {
        let buffer: usize = if buffer == 0 {
            DEFAULT_BUFFER_SIZE
        } else {
            buffer
        };
        self.get_mut_config().set_http_buffer(buffer);
        self
    }

    pub fn ws_buffer(mut self, buffer: usize) -> Self {
        let buffer: usize = if buffer == 0 {
            DEFAULT_BUFFER_SIZE
        } else {
            buffer
        };
        self.get_mut_config().set_ws_buffer(buffer);
        self
    }

    pub fn error_hook<F, Fut>(mut self, func: F) -> Self
    where
        F: ErrorHandler<Fut>,
        Fut: FutureSendStatic<()>,
    {
        self.set_error_hook(Arc::new(move |ctx: Context| {
            Box::pin(func(ctx)) as PinBoxFutureSendStatic
        }));
        self
    }

    pub fn set_nodelay(mut self, nodelay: bool) -> Self {
        self.get_mut_config().set_nodelay(nodelay);
        self
    }

    pub fn enable_nodelay(self) -> Self {
        self.set_nodelay(true)
    }

    pub fn disable_nodelay(self) -> Self {
        self.set_nodelay(false)
    }

    pub fn set_linger(mut self, linger: OptionDuration) -> Self {
        self.get_mut_config().set_linger(linger);
        self
    }

    pub fn enable_linger(self, linger: Duration) -> Self {
        self.set_linger(Some(linger))
    }

    pub fn disable_linger(self) -> Self {
        self.set_linger(None)
    }

    pub fn set_ttl(mut self, ttl: u32) -> Self {
        self.get_mut_config().set_ttl(Some(ttl));
        self
    }

    pub fn route<R, F, Fut>(mut self, route: R, func: F) -> Self
    where
        R: ToString,
        F: FnSendSyncStatic<Fut>,
        Fut: FutureSendStatic<()>,
    {
        let route_str: String = route.to_string();
        let arc_func = Arc::new(move |ctx: Context| Box::pin(func(ctx)) as PinBoxFutureSendStatic);
        self.get_mut_route_matcher()
            .add(&route_str, arc_func)
            .unwrap_or_else(|err| panic!("{}", err));
        self
    }

    pub fn request_middleware<F, Fut>(mut self, func: F) -> Self
    where
        F: FnSendSyncStatic<Fut>,
        Fut: FutureSendStatic<()>,
    {
        self.get_mut_request_middleware()
            .push(Arc::new(move |ctx: Context| Box::pin(func(ctx))));
        self
    }

    pub fn response_middleware<F, Fut>(mut self, func: F) -> Self
    where
        F: FnSendSyncStatic<Fut>,
        Fut: FutureSendStatic<()>,
    {
        self.get_mut_response_middleware()
            .push(Arc::new(move |ctx: Context| Box::pin(func(ctx))));
        self
    }

    pub fn pre_upgrade_hook<F, Fut>(mut self, func: F) -> Self
    where
        F: FnSendSyncStatic<Fut>,
        Fut: FutureSendStatic<()>,
    {
        self.get_mut_pre_upgrade_hook()
            .push(Arc::new(move |ctx: Context| Box::pin(func(ctx))));
        self
    }

    pub fn connected_hook<F, Fut>(mut self, func: F) -> Self
    where
        F: FnSendSyncStatic<Fut>,
        Fut: FutureSendStatic<()>,
    {
        self.get_mut_connected_hook()
            .push(Arc::new(move |ctx: Context| Box::pin(func(ctx))));
        self
    }

    pub fn enable_http_hook<R: ToString>(mut self, route: R) -> Self {
        let route_string: String = route.to_string();
        self.get_mut_disable_http_hook().remove(&route_string);
        self
    }

    pub fn disable_http_hook<R: ToString>(mut self, route: R) -> Self {
        let route_string: String = route.to_string();
        self.get_mut_disable_http_hook().insert(route_string);
        self
    }

    pub fn enable_ws_hook<R: ToString>(mut self, route: R) -> Self {
        let route_string: String = route.to_string();
        self.get_mut_disable_ws_hook().remove(&route_string);
        self
    }

    pub fn disable_ws_hook<R: ToString>(mut self, route: R) -> Self {
        let route_string: String = route.to_string();
        self.get_mut_disable_ws_hook().insert(route_string);
        self
    }

    pub async fn run(self) -> ServerResult {
        self.build().run().await
    }
}

impl Server {
    pub fn format_host_port(host: &str, port: &usize) -> String {
        format!("{}{}{}", host, COLON_SPACE_SYMBOL, port)
    }

    fn get(&self) -> &ServerInner {
        &self.0
    }

    fn init_panic_hook(&self) {
        let error_hook: ArcErrorHandlerSendSync = self.get().get_error_hook().clone();
        let panic_hook: &'static PanicHook = panic_hook();
        panic_hook.set_error_hook(error_hook);
        panic_hook.initialize_once();
    }

    async fn handle_panic_with_context(&self, panic_info: PanicInfo, ctx: Context) {
        let error_hook: ArcErrorHandlerSendSync = self.get().get_error_hook().clone();
        tokio::spawn(async move {
            let _ = ctx.set_panic_info(panic_info).await;
            error_hook(ctx).await;
        });
    }

    async fn handle_task_panic(&self, join_error: JoinError, ctx: Context) {
        let panic_payload: BoxAnySend = join_error.into_panic();
        let message: String = if let Some(msg) = panic_payload.downcast_ref::<&str>() {
            msg.to_string()
        } else if let Some(msg) = panic_payload.downcast_ref::<String>() {
            msg.clone()
        } else {
            EMPTY_STR.to_string()
        };
        let panic_info: PanicInfo = PanicInfo::new(message, None, EMPTY_STR.to_string());
        self.handle_panic_with_context(panic_info, ctx).await;
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
                self.handle_task_panic(join_error, ctx.clone()).await;
            }
        }
    }

    fn init(&self) {
        self.init_panic_hook();
    }

    pub async fn run(&self) -> ServerResult {
        self.init();
        let config: &ServerConfig = self.get().get_config();
        let host: &str = config.get_host();
        let port: usize = *config.get_port();
        let nodelay: bool = *config.get_nodelay();
        let linger: OptionDuration = *config.get_linger();
        let ttl_opt: OptionU32 = *config.get_ttl();
        let http_buffer: usize = *config.get_http_buffer();
        let addr: String = Self::format_host_port(host, &port);
        let tcp_listener: TcpListener = TcpListener::bind(&addr)
            .await
            .map_err(|err| ServerError::TcpBind(err.to_string()))?;
        while let Ok((stream, _socket_addr)) = tcp_listener.accept().await {
            let _ = stream.set_nodelay(nodelay);
            let _ = stream.set_linger(linger);
            if let Some(ttl) = ttl_opt {
                let _ = stream.set_ttl(ttl);
            }
            let stream: ArcRwLockStream = ArcRwLockStream::from_stream(stream);
            let server: Server = self.clone();
            tokio::spawn(async move {
                let request_result: RequestReaderHandleResult =
                    Request::http_from_stream(&stream, http_buffer).await;
                if let Ok(mut request) = request_result {
                    let ctx: Context = Context::create_context(&stream, &request);
                    let handler: HandlerState = HandlerState::new(&stream, &ctx);
                    if request.is_ws() {
                        server.ws_hook(&handler, &mut request).await;
                    } else {
                        server.http_hook(&handler, &request).await;
                    }
                }
            });
        }
        Ok(())
    }

    async fn run_pre_upgrade_hook(&self, ctx: &Context, lifecycle: &mut Lifecycle) {
        for pre_upgrade in self.get().get_pre_upgrade_hook().iter() {
            self.run_hook_with_lifecycle(ctx, lifecycle, move |ctx: Context| pre_upgrade(ctx))
                .await;
        }
    }

    async fn run_connected_hook(&self, ctx: &Context, lifecycle: &mut Lifecycle) {
        for connected in self.get().get_connected_hook().iter() {
            self.run_hook_with_lifecycle(ctx, lifecycle, move |ctx: Context| connected(ctx))
                .await;
        }
    }

    async fn run_request_middleware(&self, ctx: &Context, lifecycle: &mut Lifecycle) {
        for middleware in self.get().get_request_middleware().iter() {
            self.run_hook_with_lifecycle(ctx, lifecycle, move |ctx: Context| middleware(ctx))
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
        for middleware in self.get().get_response_middleware().iter() {
            self.run_hook_with_lifecycle(ctx, lifecycle, move |ctx: Context| middleware(ctx))
                .await;
        }
    }

    async fn request_hook<'a>(&self, state: &HandlerState<'a>, request: &Request) -> bool {
        let route: &str = request.get_path();
        let ctx: &Context = state.ctx;
        ctx.set_request(request).await;
        let mut lifecycle: Lifecycle = Lifecycle::Continue(request.is_enable_keep_alive());
        let route_hook: OptionArcFnPinBoxSendSync = self
            .get()
            .get_route_matcher()
            .resolve_route(&ctx, route)
            .await;
        self.run_request_middleware(&ctx, &mut lifecycle).await;
        if let Lifecycle::Abort(request_keepalive) = lifecycle {
            return request_keepalive;
        }
        self.run_route_hook(&ctx, &route_hook, &mut lifecycle).await;
        if let Lifecycle::Abort(request_keepalive) = lifecycle {
            return request_keepalive;
        }
        self.run_response_middleware(&ctx, &mut lifecycle).await;
        if let Lifecycle::Abort(request_keepalive) = lifecycle {
            return request_keepalive;
        }
        match lifecycle {
            Lifecycle::Continue(res) | Lifecycle::Abort(res) => res,
        }
    }

    async fn ws_hook<'a>(&self, state: &HandlerState<'a>, first_request: &mut Request) {
        let route: &String = first_request.get_path();
        let ctx: &Context = state.ctx;
        let mut lifecycle: Lifecycle = Lifecycle::Continue(true);
        self.get()
            .get_route_matcher()
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
        if self.get().get_disable_ws_hook().contains(route) {
            while self.request_hook(state, first_request).await {}
            return;
        }
        let stream: &ArcRwLockStream = state.stream;
        let buffer: usize = *self.get().get_config().get_ws_buffer();
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
        self.run_connected_hook(ctx, &mut lifecycle).await;
        if lifecycle.is_abort() {
            return;
        }
        if !self.request_hook(state, first_request).await {
            return;
        }
        let route: &String = first_request.get_path();
        let contains_disable_http_hook: bool = self.get().get_disable_http_hook().contains(route);
        let buffer: usize = *self.get().get_config().get_http_buffer();
        if contains_disable_http_hook {
            while self.request_hook(state, first_request).await {}
            return;
        }
        let stream: &ArcRwLockStream = state.stream;
        while let Ok(request) = Request::http_from_stream(stream, buffer).await.as_ref() {
            if !self.request_hook(state, request).await {
                return;
            }
        }
    }
}
