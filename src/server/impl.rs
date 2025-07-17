use crate::*;

impl Default for Server {
    fn default() -> Self {
        Self {
            config: arc_rwlock(ServerConfig::default()),
            route_matcher: arc_rwlock(RouteMatcher::new()),
            request_middleware: arc_rwlock(vec![]),
            response_middleware: arc_rwlock(vec![]),
            pre_upgrade_hook: arc_rwlock(vec![]),
            connected_hook: arc_rwlock(vec![]),
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
        Self::default()
    }

    fn format_host_port(host: &str, port: &usize) -> String {
        format!("{}{}{}", host, COLON_SPACE_SYMBOL, port)
    }

    pub async fn host<T: ToString>(&self, host: T) -> &Self {
        self.get_config().write().await.set_host(host.to_string());
        self
    }

    pub async fn port(&self, port: usize) -> &Self {
        self.get_config().write().await.set_port(port);
        self
    }

    pub async fn http_buffer(&self, buffer: usize) -> &Self {
        let buffer: usize = if buffer == 0 {
            DEFAULT_BUFFER_SIZE
        } else {
            buffer
        };
        self.get_config().write().await.set_http_buffer(buffer);
        self
    }

    pub async fn ws_buffer(&self, buffer: usize) -> &Self {
        let buffer: usize = if buffer == 0 {
            DEFAULT_BUFFER_SIZE
        } else {
            buffer
        };
        self.get_config().write().await.set_ws_buffer(buffer);
        self
    }

    pub async fn error_hook<F, Fut>(&self, func: F) -> &Self
    where
        F: ErrorHandler<Fut>,
        Fut: FutureSendStatic,
    {
        self.get_config()
            .write()
            .await
            .set_error_hook(Arc::new(move |ctx: Context| {
                Box::pin(func(ctx)) as PinBoxFutureSendStatic
            }));
        self
    }

    pub async fn set_nodelay(&self, nodelay: bool) -> &Self {
        self.get_config().write().await.set_nodelay(nodelay);
        self
    }

    pub async fn enable_http_hook<'a, R>(&self, route: R) -> &Self
    where
        R: ToString,
    {
        self.get_config()
            .write()
            .await
            .enable_http_hook(route.to_string())
            .await;
        self
    }

    pub async fn disable_http_hook<'a, R>(&self, route: R) -> &Self
    where
        R: ToString,
    {
        self.get_config()
            .write()
            .await
            .disable_http_hook(route.to_string())
            .await;
        self
    }

    pub async fn enable_ws_hook<'a, R>(&self, route: R) -> &Self
    where
        R: ToString,
    {
        self.get_config()
            .write()
            .await
            .enable_ws_hook(route.to_string())
            .await;
        self
    }

    pub async fn disable_ws_hook<'a, R>(&self, route: R) -> &Self
    where
        R: ToString,
    {
        self.get_config()
            .write()
            .await
            .disable_ws_hook(route.to_string())
            .await;
        self
    }

    pub async fn enable_nodelay(&self) -> &Self {
        self.set_nodelay(true).await;
        self
    }

    pub async fn disable_nodelay(&self) -> &Self {
        self.set_nodelay(false).await;
        self
    }

    pub async fn set_linger(&self, linger: OptionDuration) -> &Self {
        self.get_config().write().await.set_linger(linger);
        self
    }

    pub async fn enable_linger(&self, linger: Duration) -> &Self {
        self.set_linger(Some(linger)).await;
        self
    }

    pub async fn disable_linger(&self) -> &Self {
        self.set_linger(None).await;
        self
    }

    pub async fn set_ttl(&self, ttl: u32) -> &Self {
        self.get_config().write().await.set_ttl(Some(ttl));
        self
    }

    pub async fn pre_upgrade_hook<F, Fut>(&self, func: F) -> &Self
    where
        F: FnSendSyncStatic<Fut>,
        Fut: FutureSendStatic,
    {
        self.get_pre_upgrade_hook()
            .write()
            .await
            .push(Arc::new(move |ctx: Context| Box::pin(func(ctx))));
        self
    }

    pub async fn connected_hook<F, Fut>(&self, func: F) -> &Self
    where
        F: FnSendSyncStatic<Fut>,
        Fut: FutureSendStatic,
    {
        self.get_connected_hook()
            .write()
            .await
            .push(Arc::new(move |ctx: Context| Box::pin(func(ctx))));
        self
    }

    pub async fn route<R, F, Fut>(&self, route: R, func: F) -> &Self
    where
        R: ToString,
        F: FnSendSyncStatic<Fut>,
        Fut: FutureSendStatic,
    {
        let route_str: String = route.to_string();
        let arc_func = Arc::new(move |ctx: Context| Box::pin(func(ctx)) as PinBoxFutureSendStatic);
        self.route_matcher
            .write()
            .await
            .add(&route_str, arc_func)
            .unwrap_or_else(|err| panic!("{}", err));
        self
    }

    pub async fn request_middleware<F, Fut>(&self, func: F) -> &Self
    where
        F: FnSendSyncStatic<Fut>,
        Fut: FutureSendStatic,
    {
        self.get_request_middleware()
            .write()
            .await
            .push(Arc::new(move |ctx: Context| Box::pin(func(ctx))));
        self
    }

    pub async fn response_middleware<F, Fut>(&self, func: F) -> &Self
    where
        F: FnSendSyncStatic<Fut>,
        Fut: FutureSendStatic,
    {
        self.get_response_middleware()
            .write()
            .await
            .push(Arc::new(move |ctx: Context| Box::pin(func(ctx))));
        self
    }

    async fn init_panic_hook(&self) {
        let config: RwLockReadGuardServerConfig<'_> = self.get_config().read().await;
        let error_hook: ArcErrorHandlerSendSync = config.get_error_hook().clone();
        let panic_hook: &'static PanicHook = panic_hook();
        panic_hook.set_error_hook(error_hook);
        panic_hook.initialize_once();
    }

    async fn handle_panic_with_context(&self, panic_info: PanicInfo, ctx: Context) {
        let error_hook: ArcErrorHandlerSendSync =
            self.get_config().read().await.get_error_hook().clone();
        tokio::task::spawn(async move {
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

    async fn init(&self) {
        self.init_panic_hook().await;
    }

    pub async fn run(&self) -> ServerResult {
        self.init().await;
        let config: ServerConfig = self.get_config().read().await.clone();
        let host: String = config.get_host().clone();
        let port: usize = *config.get_port();
        let nodelay: bool = *config.get_nodelay();
        let linger: OptionDuration = *config.get_linger();
        let ttl_opt: OptionU32 = *config.get_ttl();
        let http_buffer: usize = *config.get_http_buffer();
        let addr: String = Self::format_host_port(&host, &port);
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
            tokio::task::spawn(async move {
                let request_result: RequestReaderHandleResult =
                    Request::http_from_stream(&stream, http_buffer).await;
                if request_result.is_err() {
                    return;
                }
                let mut request: Request = request_result.unwrap_or_default();
                let is_ws: bool = request.is_ws();
                let ctx: Context = Context::create_context(&stream, &request);
                let handler: HandlerState = HandlerState::new(&stream, &ctx);
                match is_ws {
                    true => {
                        server.ws_hook(&handler, &mut request).await;
                    }
                    false => {
                        server.http_hook(&handler, &request).await;
                    }
                };
            });
        }
        Ok(())
    }

    async fn run_pre_upgrade_hook(&self, ctx: &Context, lifecycle: &mut Lifecycle) {
        let middleware_guard: RwLockReadGuardVecArcFnPinBoxSendSync =
            self.pre_upgrade_hook.read().await;
        for pre_upgrade_hook in middleware_guard.iter() {
            let result: Result<(), JoinError> = task::spawn(pre_upgrade_hook(ctx.clone())).await;
            ctx.should_abort(lifecycle).await;
            if let Err(join_error) = result {
                if join_error.is_panic() {
                    self.handle_task_panic(join_error, ctx.clone()).await;
                }
            }
        }
    }

    async fn run_connected_hook(&self, ctx: &Context, lifecycle: &mut Lifecycle) {
        let middleware_guard: RwLockReadGuardVecArcFnPinBoxSendSync =
            self.connected_hook.read().await;
        for connected_hook in middleware_guard.iter() {
            let result: Result<(), JoinError> = task::spawn(connected_hook(ctx.clone())).await;
            ctx.should_abort(lifecycle).await;
            if let Err(join_error) = result {
                if join_error.is_panic() {
                    self.handle_task_panic(join_error, ctx.clone()).await;
                }
            }
        }
    }

    async fn run_request_middleware(&self, ctx: &Context, lifecycle: &mut Lifecycle) {
        let middleware_guard: RwLockReadGuardVecArcFnPinBoxSendSync =
            self.request_middleware.read().await;
        for middleware in middleware_guard.iter() {
            let ctx_clone: Context = ctx.clone();
            let result: Result<(), JoinError> = task::spawn(middleware(ctx_clone)).await;
            ctx.should_abort(lifecycle).await;
            if let Err(join_error) = result {
                if join_error.is_panic() {
                    self.handle_task_panic(join_error, ctx.clone()).await;
                }
            }
        }
    }

    async fn run_route_hook(
        &self,
        ctx: &Context,
        handler: &OptionArcFnPinBoxSendSync,
        lifecycle: &mut Lifecycle,
    ) {
        if let Some(func) = handler {
            let ctx_clone: Context = ctx.clone();
            let result: Result<(), JoinError> = task::spawn(func(ctx_clone)).await;
            ctx.should_abort(lifecycle).await;
            if let Err(join_error) = result {
                if join_error.is_panic() {
                    self.handle_task_panic(join_error, ctx.clone()).await;
                }
            }
        }
    }

    async fn run_response_middleware(&self, ctx: &Context, lifecycle: &mut Lifecycle) {
        let middleware_guard: RwLockReadGuardVecArcFnPinBoxSendSync =
            self.response_middleware.read().await;
        for middleware in middleware_guard.iter() {
            let ctx_clone: Context = ctx.clone();
            let result: Result<(), JoinError> = task::spawn(middleware(ctx_clone)).await;
            ctx.should_abort(lifecycle).await;
            if let Err(join_error) = result {
                if join_error.is_panic() {
                    self.handle_task_panic(join_error, ctx.clone()).await;
                }
            }
        }
    }

    async fn request_hook<'a>(&self, state: &HandlerState<'a>, request: &Request) -> bool {
        let route: &str = request.get_path();
        let ctx: &Context = &state.ctx;
        ctx.set_request(request).await;
        let mut lifecycle: Lifecycle = Lifecycle::Continue(request.is_enable_keep_alive());
        let route_hook: OptionArcFnPinBoxSendSync = self
            .route_matcher
            .read()
            .await
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
        let route: String = first_request.get_path().to_string();
        let stream: &ArcRwLockStream = state.stream;
        let ctx: &Context = &state.ctx;
        let server: &Server = self;
        let mut lifecycle: Lifecycle = Lifecycle::Continue(true);
        server
            .route_matcher
            .read()
            .await
            .resolve_route(ctx, &route)
            .await;
        with_context(ctx.clone(), async move {
            server.run_pre_upgrade_hook(ctx, &mut lifecycle).await;
            if lifecycle.is_abort() {
                return;
            }
            if ctx.upgrade_to_ws().await.is_err() {
                return;
            }
            server.run_connected_hook(ctx, &mut lifecycle).await;
            if lifecycle.is_abort() {
                return;
            }
            let config: RwLockReadGuardServerConfig<'_> = server.get_config().read().await;
            let buffer: usize = *config.get_ws_buffer();
            let contains_disable_ws_hook: bool = config.contains_disable_ws_hook(&route).await;
            drop(config);
            if contains_disable_ws_hook {
                while server.request_hook(state, first_request).await {}
                return;
            }
            while let Ok(request) = Request::ws_from_stream(stream, buffer, first_request)
                .await
                .as_ref()
            {
                let _ = server.request_hook(state, request).await;
            }
        })
        .await
    }

    async fn http_hook<'a>(&self, state: &HandlerState<'a>, first_request: &Request) {
        let stream: &ArcRwLockStream = state.stream;
        let ctx: &Context = &state.ctx;
        let server: &Server = self;
        with_context(ctx.clone(), async move {
            let mut lifecycle: Lifecycle = Lifecycle::Continue(true);
            server.run_connected_hook(ctx, &mut lifecycle).await;
            if lifecycle.is_abort() {
                return;
            }
            let handle_result: bool = server.request_hook(state, first_request).await;
            if !handle_result {
                return;
            }
            let route: &String = first_request.get_path();
            let config: RwLockReadGuardServerConfig<'_> = server.get_config().read().await;
            let contains_disable_http_hook: bool = config.contains_disable_http_hook(route).await;
            let buffer: usize = *config.get_http_buffer();
            drop(config);
            if contains_disable_http_hook {
                while server.request_hook(state, first_request).await {}
                return;
            }
            while let Ok(request) = Request::http_from_stream(stream, buffer).await.as_ref() {
                let handle_result: bool = server.request_hook(state, request).await;
                if !handle_result {
                    return;
                }
            }
        })
        .await
    }
}
