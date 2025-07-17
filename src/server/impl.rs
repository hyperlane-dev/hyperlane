use crate::*;

impl Default for Server {
    fn default() -> Self {
        Self {
            config: arc_rwlock(ServerConfig::default()),
            route_matcher: arc_rwlock(RouteMatcher::new()),
            request_middleware: arc_rwlock(vec![]),
            response_middleware: arc_rwlock(vec![]),
            pre_ws_upgrade: arc_rwlock(vec![]),
            on_ws_connected: arc_rwlock(vec![]),
        }
    }
}

impl<'a> HandlerState<'a> {
    fn new(stream: &'a ArcRwLockStream) -> Self {
        Self { stream }
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

    pub async fn http_buffer_size(&self, buffer_size: usize) -> &Self {
        let buffer_size: usize = if buffer_size == 0 {
            DEFAULT_BUFFER_SIZE
        } else {
            buffer_size
        };
        self.get_config()
            .write()
            .await
            .set_http_buffer_size(buffer_size);
        self
    }

    pub async fn ws_buffer_size(&self, buffer_size: usize) -> &Self {
        let buffer_size: usize = if buffer_size == 0 {
            DEFAULT_BUFFER_SIZE
        } else {
            buffer_size
        };
        self.get_config()
            .write()
            .await
            .set_ws_buffer_size(buffer_size);
        self
    }

    pub async fn error_hook<F, Fut>(&self, func: F) -> &Self
    where
        F: ErrorHandler<Fut>,
        Fut: FutureSendStatic,
    {
        self.get_config().write().await.set_error_hook(Arc::new(
            move |ctx: Context, error: PanicInfo| {
                Box::pin(func(ctx, error)) as PinBoxFutureSendStatic
            },
        ));
        self
    }

    pub async fn set_nodelay(&self, nodelay: bool) -> &Self {
        self.get_config().write().await.set_nodelay(nodelay);
        self
    }

    pub async fn enable_http_handler<'a, R>(&self, route: R) -> &Self
    where
        R: ToString,
    {
        self.get_config()
            .write()
            .await
            .enable_http_handler(route.to_string())
            .await;
        self
    }

    pub async fn disable_http_handler<'a, R>(&self, route: R) -> &Self
    where
        R: ToString,
    {
        self.get_config()
            .write()
            .await
            .disable_http_handler(route.to_string())
            .await;
        self
    }

    pub async fn enable_ws_handler<'a, R>(&self, route: R) -> &Self
    where
        R: ToString,
    {
        self.get_config()
            .write()
            .await
            .enable_ws_handler(route.to_string())
            .await;
        self
    }

    pub async fn disable_ws_handler<'a, R>(&self, route: R) -> &Self
    where
        R: ToString,
    {
        self.get_config()
            .write()
            .await
            .disable_ws_handler(route.to_string())
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

    pub async fn pre_ws_upgrade<F, Fut>(&self, func: F) -> &Self
    where
        F: FnSendSyncStatic<Fut>,
        Fut: FutureSendStatic,
    {
        self.get_pre_ws_upgrade()
            .write()
            .await
            .push(Arc::new(move |ctx: Context| Box::pin(func(ctx))));
        self
    }

    pub async fn on_ws_connected<F, Fut>(&self, func: F) -> &Self
    where
        F: FnSendSyncStatic<Fut>,
        Fut: FutureSendStatic,
    {
        self.get_on_ws_connected()
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
        tokio::spawn(async move {
            let handler_func = error_hook.as_ref();
            handler_func(ctx, panic_info).await;
        });
    }

    async fn handle_task_panic(&self, join_error: tokio::task::JoinError, ctx: Context) {
        let panic_payload = join_error.into_panic();
        let message: String = if let Some(s) = panic_payload.downcast_ref::<&str>() {
            s.to_string()
        } else if let Some(s) = panic_payload.downcast_ref::<String>() {
            s.clone()
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
        let http_buffer_size: usize = *config.get_http_buffer_size();
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
            tokio::spawn(async move {
                let request_result: RequestReaderHandleResult =
                    Request::http_from_stream(&stream, http_buffer_size).await;
                if request_result.is_err() {
                    return;
                }
                let mut request: Request = request_result.unwrap_or_default();
                let is_ws: bool = request.is_ws();
                let handler: HandlerState = HandlerState::new(&stream);
                match is_ws {
                    true => {
                        server.ws_handler(&handler, &mut request).await;
                    }
                    false => {
                        server.http_handler(&handler, &request).await;
                    }
                };
            });
        }
        Ok(())
    }

    async fn run_pre_ws_upgrade(&self, ctx: &Context, lifecycle: &mut Lifecycle) {
        let middleware_guard: RwLockReadGuardVecArcFnPinBoxSendSync =
            self.pre_ws_upgrade.read().await;
        for pre_ws_upgrade in middleware_guard.iter() {
            pre_ws_upgrade(ctx.clone()).await;
            ctx.should_abort(lifecycle).await;
            if lifecycle.is_abort() {
                return;
            }
        }
    }

    async fn run_on_ws_connected(&self, ctx: &Context, lifecycle: &mut Lifecycle) {
        let middleware_guard: RwLockReadGuardVecArcFnPinBoxSendSync =
            self.on_ws_connected.read().await;
        for on_ws_connected in middleware_guard.iter() {
            on_ws_connected(ctx.clone()).await;
            ctx.should_abort(lifecycle).await;
            if lifecycle.is_abort() {
                return;
            }
        }
    }

    async fn run_request_middleware(&self, ctx: &Context, lifecycle: &mut Lifecycle) {
        let middleware_guard: RwLockReadGuardVecArcFnPinBoxSendSync =
            self.request_middleware.read().await;
        for middleware in middleware_guard.iter() {
            let ctx_clone: Context = ctx.clone();
            let middleware_future = middleware(ctx_clone);
            let result = tokio::task::spawn(middleware_future).await;
            if let Err(join_error) = result {
                if join_error.is_panic() {
                    self.handle_task_panic(join_error, ctx.clone()).await;
                    return;
                }
            }
            ctx.should_abort(lifecycle).await;
            if lifecycle.is_abort() {
                return;
            }
        }
    }

    async fn run_route_handler(
        &self,
        ctx: &Context,
        handler: &OptionArcFnPinBoxSendSync,
        lifecycle: &mut Lifecycle,
    ) {
        if let Some(func) = handler {
            let ctx_clone: Context = ctx.clone();
            let handler_future = func(ctx_clone);
            let result = tokio::task::spawn(handler_future).await;
            if let Err(join_error) = result {
                if join_error.is_panic() {
                    self.handle_task_panic(join_error, ctx.clone()).await;
                    return;
                }
            }
            ctx.should_abort(lifecycle).await;
            if lifecycle.is_abort() {
                return;
            }
        }
    }

    async fn run_response_middleware(&self, ctx: &Context, lifecycle: &mut Lifecycle) {
        let middleware_guard: RwLockReadGuardVecArcFnPinBoxSendSync =
            self.response_middleware.read().await;
        for middleware in middleware_guard.iter() {
            let ctx_clone: Context = ctx.clone();
            let middleware_future = middleware(ctx_clone);
            let result = tokio::task::spawn(middleware_future).await;
            if let Err(join_error) = result {
                if join_error.is_panic() {
                    self.handle_task_panic(join_error, ctx.clone()).await;
                    return;
                }
            }
            ctx.should_abort(lifecycle).await;
            if lifecycle.is_abort() {
                return;
            }
        }
    }

    async fn request_handler<'a>(&self, state: &HandlerState<'a>, request: &Request) -> bool {
        let stream: &ArcRwLockStream = state.stream;
        let route: &str = request.get_path();
        let ctx: Context = Context::create_context(stream, request);
        with_context(ctx.clone(), async {
            let mut lifecycle: Lifecycle = Lifecycle::Continue(request.is_enable_keep_alive());
            let route_handler: OptionArcFnPinBoxSendSync = self
                .route_matcher
                .read()
                .await
                .resolve_route(&ctx, route)
                .await;
            self.run_request_middleware(&ctx, &mut lifecycle).await;
            if let Lifecycle::Abort(request_keepalive) = lifecycle {
                return request_keepalive;
            }
            self.run_route_handler(&ctx, &route_handler, &mut lifecycle)
                .await;
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
        })
        .await
    }

    async fn ws_handler<'a>(&self, state: &HandlerState<'a>, first_request: &mut Request) {
        let route: String = first_request.get_path().to_string();
        let stream: &ArcRwLockStream = state.stream;
        let ctx: Context = Context::create_context(stream, first_request);
        let route_clone: String = route.clone();
        let self_ref = self;
        with_context(ctx.clone(), async move {
            let mut lifecycle: Lifecycle = Lifecycle::Continue(true);
            self_ref
                .route_matcher
                .read()
                .await
                .resolve_route(&ctx, &route_clone)
                .await;
            self_ref.run_pre_ws_upgrade(&ctx, &mut lifecycle).await;
            if lifecycle.is_abort() {
                return;
            }
            if ctx.upgrade_to_ws().await.is_err() {
                return;
            }
            self_ref.run_on_ws_connected(&ctx, &mut lifecycle).await;
            if lifecycle.is_abort() {
                return;
            }
            let config: RwLockReadGuardServerConfig<'_> = self_ref.get_config().read().await;
            let buffer_size: usize = *config.get_ws_buffer_size();
            let contains_disable_ws_handler: bool =
                config.contains_disable_ws_handler(&route_clone).await;
            drop(config);
            if contains_disable_ws_handler {
                while self_ref.request_handler(state, first_request).await {}
                return;
            }
            while let Ok(request) =
                Request::ws_from_stream(stream, buffer_size, first_request).await
            {
                let _ = self_ref.request_handler(state, &request).await;
            }
        })
        .await
    }

    async fn http_handler<'a>(&self, state: &HandlerState<'a>, first_request: &Request) {
        let handle_result: bool = self.request_handler(state, first_request).await;
        if !handle_result {
            return;
        }
        let stream: &ArcRwLockStream = state.stream;
        let route: &String = first_request.get_path();
        let config: RwLockReadGuardServerConfig<'_> = self.get_config().read().await;
        let contains_disable_http_handler: bool = config.contains_disable_http_handler(route).await;
        let buffer_size: usize = *config.get_http_buffer_size();
        if contains_disable_http_handler {
            while self.request_handler(state, first_request).await {}
            return;
        }
        while let Ok(request) = Request::http_from_stream(stream, buffer_size).await {
            let handle_result: bool = self.request_handler(state, &request).await;
            if !handle_result {
                return;
            }
        }
    }
}
