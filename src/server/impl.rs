use crate::*;

impl Lifecycle {
    pub(crate) fn is_abort(&self) -> bool {
        matches!(self, Lifecycle::Abort(_))
    }
}

impl Default for Server {
    fn default() -> Self {
        Self {
            config: arc_rwlock(ServerConfig::default()),
            route_matcher: arc_rwlock(RouteMatcher::new()),
            request_middleware: arc_rwlock(vec![]),
            response_middleware: arc_rwlock(vec![]),
            before_ws_upgrade: arc_rwlock(vec![]),
            on_ws_connected: arc_rwlock(vec![]),
        }
    }
}

impl Server {
    pub fn new() -> Self {
        Self::default()
    }

    fn format_host_port(host: &str, port: &usize) -> String {
        format!("{}{}{}", host, COLON_SPACE_SYMBOL, port)
    }

    pub async fn host(&self, host: &'static str) -> &Self {
        self.get_config().write().await.set_host(host);
        self
    }

    pub async fn port(&self, port: usize) -> &Self {
        self.get_config().write().await.set_port(port);
        self
    }

    pub async fn http_line_buffer_size(&self, buffer_size: usize) -> &Self {
        let buffer_size: usize = if buffer_size == 0 {
            DEFAULT_BUFFER_SIZE
        } else {
            buffer_size
        };
        self.get_config()
            .write()
            .await
            .set_http_line_buffer_size(buffer_size);
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

    pub async fn error_handler<F>(&self, func: F) -> &Self
    where
        F: ErrorHandle + Send + Sync + 'static,
    {
        self.get_config()
            .write()
            .await
            .set_error_handler(Arc::new(func));
        self
    }

    pub async fn set_nodelay(&self, nodelay: bool) -> &Self {
        self.get_config().write().await.set_nodelay(nodelay);
        self
    }

    pub async fn enable_internal_http_handler<'a, R>(&self, route: R) -> &Self
    where
        R: ToString,
    {
        self.get_config()
            .write()
            .await
            .enable_internal_http_handler(route.to_string())
            .await;
        self
    }

    pub async fn disable_internal_http_handler<'a, R>(&self, route: R) -> &Self
    where
        R: ToString,
    {
        self.get_config()
            .write()
            .await
            .disable_internal_http_handler(route.to_string())
            .await;
        self
    }

    pub async fn enable_internal_ws_handler<'a, R>(&self, route: R) -> &Self
    where
        R: ToString,
    {
        self.get_config()
            .write()
            .await
            .enable_internal_ws_handler(route.to_string())
            .await;
        self
    }

    pub async fn disable_internal_ws_handler<'a, R>(&self, route: R) -> &Self
    where
        R: ToString,
    {
        self.get_config()
            .write()
            .await
            .disable_internal_ws_handler(route.to_string())
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

    pub async fn before_ws_upgrade<F, Fut>(&self, func: F) -> &Self
    where
        F: FuncWithoutPin<Fut>,
        Fut: Future<Output = ()> + Send + 'static,
    {
        self.get_before_ws_upgrade()
            .write()
            .await
            .push(Arc::new(move |ctx: Context| Box::pin(func(ctx))));
        self
    }

    pub async fn on_ws_connected<F, Fut>(&self, func: F) -> &Self
    where
        F: FuncWithoutPin<Fut>,
        Fut: Future<Output = ()> + Send + 'static,
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
        F: FuncWithoutPin<Fut>,
        Fut: Future<Output = ()> + Send + 'static,
    {
        let route_str: String = route.to_string();
        let arc_func = Arc::new(move |ctx: Context| Box::pin(func(ctx)) as PinBoxFutureSend);
        self.route_matcher
            .write()
            .await
            .add(&route_str, arc_func)
            .unwrap_or_else(|err| panic!("{}", err));
        self
    }

    pub async fn request_middleware<F, Fut>(&self, func: F) -> &Self
    where
        F: FuncWithoutPin<Fut>,
        Fut: Future<Output = ()> + Send + 'static,
    {
        self.get_request_middleware()
            .write()
            .await
            .push(Arc::new(move |ctx: Context| Box::pin(func(ctx))));
        self
    }

    pub async fn response_middleware<F, Fut>(&self, func: F) -> &Self
    where
        F: FuncWithoutPin<Fut>,
        Fut: Future<Output = ()> + Send + 'static,
    {
        self.get_response_middleware()
            .write()
            .await
            .push(Arc::new(move |ctx: Context| Box::pin(func(ctx))));
        self
    }

    async fn init_panic_hook(&self) {
        let config: ServerConfig<'_> = self.get_config().read().await.clone();
        let error_handler: ArcErrorHandle = config.get_error_handler().clone();
        set_hook(Box::new(move |err: &'_ PanicHookInfo<'_>| {
            let data: String = err.to_string();
            error_handler(data);
        }));
    }

    async fn init(&self) {
        self.init_panic_hook().await;
    }

    pub async fn run(&self) -> ServerResult {
        self.init().await;
        let config: ServerConfig<'_> = self.get_config().read().await.clone();
        let host: &str = *config.get_host();
        let port: usize = *config.get_port();
        let nodelay: bool = *config.get_nodelay();
        let linger: OptionDuration = *config.get_linger();
        let ttl_opt: OptionU32 = *config.get_ttl();
        let http_line_buffer_size: usize = *config.get_http_line_buffer_size();
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
            let config_clone: ServerConfig<'_> = config.clone();
            let stream: ArcRwLockStream = ArcRwLockStream::from_stream(stream);
            let request_middleware_arc_lock: ArcRwLockVecArcFunc =
                Arc::clone(&self.request_middleware);
            let response_middleware_arc_lock: ArcRwLockVecArcFunc =
                Arc::clone(&self.response_middleware);
            let route_matcher_arc_lock: ArcRwLockRouteMatcher = Arc::clone(&self.route_matcher);
            let before_ws_upgrade_arc_lock: ArcRwLockVecArcFunc =
                Arc::clone(&self.before_ws_upgrade);
            let on_ws_connected_arc_lock: ArcRwLockVecArcFunc = Arc::clone(&self.on_ws_connected);
            tokio::spawn(async move {
                let request_result: RequestReaderHandleResult =
                    Request::http_request_from_stream(&stream, http_line_buffer_size).await;
                if request_result.is_err() {
                    return;
                }
                let mut request: Request = request_result.unwrap_or_default();
                let is_ws: bool = request.is_ws();
                let handler: HandlerState = HandlerState::new(
                    &stream,
                    &config_clone,
                    &request_middleware_arc_lock,
                    &response_middleware_arc_lock,
                    &route_matcher_arc_lock,
                    &before_ws_upgrade_arc_lock,
                    &on_ws_connected_arc_lock,
                );
                match is_ws {
                    true => {
                        Self::handle_ws_connection(&handler, &mut request).await;
                    }
                    false => {
                        Self::handle_http_connection(&handler, &request).await;
                    }
                };
            });
        }
        Ok(())
    }

    async fn execute_before_ws_upgrade<'a>(
        handler: &HandlerState<'a>,
        ctx: &Context,
        lifecycle: &mut Lifecycle,
    ) {
        let middleware_guard: RwLockReadGuard<'_, Vec<Arc<dyn Func>>> =
            handler.before_ws_upgrade.read().await;
        for before_ws_upgrade in middleware_guard.iter() {
            before_ws_upgrade(ctx.clone()).await;
            ctx.should_abort(lifecycle).await;
            if lifecycle.is_abort() {
                return;
            }
        }
    }

    async fn execute_on_ws_connected<'a>(
        handler: &HandlerState<'a>,
        ctx: &Context,
        lifecycle: &mut Lifecycle,
    ) {
        let middleware_guard: RwLockReadGuard<'_, Vec<Arc<dyn Func>>> =
            handler.on_ws_connected.read().await;
        for on_ws_connected in middleware_guard.iter() {
            on_ws_connected(ctx.clone()).await;
            ctx.should_abort(lifecycle).await;
            if lifecycle.is_abort() {
                return;
            }
        }
    }

    async fn execute_request_middleware<'a>(
        ctx: &Context,
        handler: &HandlerState<'a>,
        lifecycle: &mut Lifecycle,
    ) {
        let middleware_guard: RwLockReadGuard<'_, Vec<Arc<dyn Func>>> =
            handler.request_middleware.read().await;
        for middleware in middleware_guard.iter() {
            middleware(ctx.clone()).await;
            ctx.should_abort(lifecycle).await;
            if lifecycle.is_abort() {
                return;
            }
        }
    }

    async fn execute_route_handler<'a>(
        ctx: &Context,
        route_handler: &OptionArcFunc,
        lifecycle: &mut Lifecycle,
    ) {
        if let Some(func) = route_handler {
            func(ctx.clone()).await;
            ctx.should_abort(lifecycle).await;
            if lifecycle.is_abort() {
                return;
            }
        }
    }

    async fn execute_response_middleware<'a>(
        ctx: &Context,
        handler: &HandlerState<'a>,
        lifecycle: &mut Lifecycle,
    ) {
        let middleware_guard: RwLockReadGuard<'_, Vec<Arc<dyn Func>>> =
            handler.response_middleware.read().await;
        for middleware in middleware_guard.iter() {
            middleware(ctx.clone()).await;
            ctx.should_abort(lifecycle).await;
            if lifecycle.is_abort() {
                return;
            }
        }
    }

    async fn handle_request_common<'a>(handler: &HandlerState<'a>, request: &Request) -> bool {
        let stream: &ArcRwLockStream = handler.stream;
        let route: &str = request.get_path();
        let ctx: Context = Context::from_stream_request(stream, request);
        let mut lifecycle: Lifecycle = Lifecycle::Continue(request.is_enable_keep_alive());
        let route_handler: OptionArcFunc = handler
            .route_matcher
            .read()
            .await
            .resolve_route(&ctx, route)
            .await;
        Self::execute_request_middleware(&ctx, handler, &mut lifecycle).await;
        if let Lifecycle::Abort(request_keepalive) = lifecycle {
            return request_keepalive;
        }
        Self::execute_route_handler(&ctx, &route_handler, &mut lifecycle).await;
        if let Lifecycle::Abort(request_keepalive) = lifecycle {
            return request_keepalive;
        }
        Self::execute_response_middleware(&ctx, handler, &mut lifecycle).await;
        if let Lifecycle::Abort(request_keepalive) = lifecycle {
            return request_keepalive;
        }
        yield_now().await;
        match lifecycle {
            Lifecycle::Continue(res) | Lifecycle::Abort(res) => res,
        }
    }

    async fn handle_ws_connection<'a>(handler: &HandlerState<'a>, first_request: &mut Request) {
        let route: &String = first_request.get_path();
        let stream: &ArcRwLockStream = handler.stream;
        let ctx: Context = Context::from_stream_request(stream, first_request);
        let mut lifecycle: Lifecycle = Lifecycle::Continue(true);
        handler
            .route_matcher
            .read()
            .await
            .resolve_route(&ctx, route)
            .await;
        Self::execute_before_ws_upgrade(handler, &ctx, &mut lifecycle).await;
        if lifecycle.is_abort() {
            return;
        }
        if ctx.upgrade_to_ws().await.is_err() {
            return;
        }
        Self::execute_on_ws_connected(handler, &ctx, &mut lifecycle).await;
        if lifecycle.is_abort() {
            return;
        }
        let route: &String = first_request.get_path();
        let buffer_size: usize = *handler.config.get_ws_buffer_size();
        let contains_disable_internal_ws_handler: bool = handler
            .config
            .contains_disable_internal_ws_handler(route)
            .await;
        if contains_disable_internal_ws_handler {
            while Self::handle_request_common(handler, first_request).await {}
            return;
        }
        while let Ok(request) =
            Request::ws_request_from_stream(stream, buffer_size, first_request).await
        {
            let _ = Self::handle_request_common(handler, &request).await;
        }
    }

    async fn handle_http_connection<'a>(handler: &HandlerState<'a>, first_request: &Request) {
        let handle_result: bool = Self::handle_request_common(handler, first_request).await;
        if !handle_result {
            return;
        }
        let stream: &ArcRwLockStream = handler.stream;
        let route: &String = first_request.get_path();
        let contains_disable_internal_http_handler: bool = handler
            .config
            .contains_disable_internal_http_handler(route)
            .await;
        let buffer_size: usize = *handler.config.get_http_line_buffer_size();
        if contains_disable_internal_http_handler {
            while Self::handle_request_common(handler, first_request).await {}
            return;
        }
        while let Ok(request) = Request::http_request_from_stream(stream, buffer_size).await {
            let handle_result: bool = Self::handle_request_common(handler, &request).await;
            if !handle_result {
                return;
            }
        }
    }
}

impl<'a> HandlerState<'a> {
    fn new(
        stream: &'a ArcRwLockStream,
        config: &'a ServerConfig<'a>,
        request_middleware: &'a ArcRwLockVecArcFunc,
        response_middleware: &'a ArcRwLockVecArcFunc,
        route_matcher: &'a ArcRwLock<RouteMatcher>,
        before_ws_upgrade: &'a ArcRwLockVecArcFunc,
        on_ws_connected: &'a ArcRwLockVecArcFunc,
    ) -> Self {
        Self {
            stream,
            config,
            request_middleware,
            response_middleware,
            route_matcher,
            before_ws_upgrade,
            on_ws_connected,
        }
    }
}
