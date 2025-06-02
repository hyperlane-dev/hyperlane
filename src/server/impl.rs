use crate::*;

impl Default for Server {
    fn default() -> Self {
        Self {
            config: arc_rwlock(ServerConfig::default()),
            route: arc_rwlock(hash_map_xx_hash3_64()),
            route_matcher: arc_rwlock(RouteMatcher::new()),
            request_middleware: arc_rwlock(vec![]),
            response_middleware: arc_rwlock(vec![]),
        }
    }
}

impl Server {
    pub fn new() -> Self {
        Self::default()
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

    pub async fn websocket_buffer_size(&self, buffer_size: usize) -> &Self {
        let buffer_size: usize = if buffer_size == 0 {
            DEFAULT_BUFFER_SIZE
        } else {
            buffer_size
        };
        self.get_config()
            .write()
            .await
            .set_websocket_buffer_size(buffer_size);
        self
    }

    pub async fn error_handle<F>(&self, func: F) -> &Self
    where
        F: ErrorHandle + Send + Sync + 'static,
    {
        self.get_config()
            .write()
            .await
            .set_error_handle(Arc::new(func));
        self
    }

    pub async fn set_nodelay(&self, nodelay: bool) -> &Self {
        self.get_config().write().await.set_nodelay(nodelay);
        self
    }

    pub async fn enable_inner_http_handle<'a, R>(&self, route: R) -> &Self
    where
        R: ToString,
    {
        self.get_config()
            .write()
            .await
            .enable_inner_http_handle(route.to_string())
            .await;
        self
    }

    pub async fn disable_inner_http_handle<'a, R>(&self, route: R) -> &Self
    where
        R: ToString,
    {
        self.get_config()
            .write()
            .await
            .disable_inner_http_handle(route.to_string())
            .await;
        self
    }

    pub async fn enable_inner_websocket_handle<'a, R>(&self, route: R) -> &Self
    where
        R: ToString,
    {
        self.get_config()
            .write()
            .await
            .enable_inner_websocket_handle(route.to_string())
            .await;
        self
    }

    pub async fn disable_inner_websocket_handle<'a, R>(&self, route: R) -> &Self
    where
        R: ToString,
    {
        self.get_config()
            .write()
            .await
            .disable_inner_websocket_handle(route.to_string())
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
            .add(&route_str, arc_func.clone())
            .unwrap_or_else(|err| panic!("{}", err));
        self.get_route()
            .write()
            .await
            .insert(route_str.clone(), arc_func);
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
        let error_handle: ArcErrorHandle = config.get_error_handle().clone();
        set_hook(Box::new(move |err: &'_ PanicHookInfo<'_>| {
            let data: String = err.to_string();
            error_handle(data);
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
        let addr: String = Context::format_host_port(host, &port);
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
            let request_middleware_arc_lock: ArcRwLockMiddlewareFuncBox =
                self.get_request_middleware().clone();
            let response_middleware_arc_lock: ArcRwLockMiddlewareFuncBox =
                self.get_response_middleware().clone();
            let route_func_arc_lock: ArcRwLockHashMapRouteFuncBox = self.get_route().clone();
            let route_matcher_arc_lock: ArcRwLockRouteMatcher = self.route_matcher.clone();
            tokio::spawn(async move {
                let request_result: RequestReaderHandleResult =
                    Request::http_request_from_stream(&stream, http_line_buffer_size).await;
                if request_result.is_err() {
                    let _ = stream.close().await;
                    return;
                }
                let mut request: Request = request_result.unwrap_or_default();
                let is_websocket: bool = request.upgrade_type_is_websocket();
                let handler: RequestHandlerImmutableParams = RequestHandlerImmutableParams::new(
                    &stream,
                    &config_clone,
                    &request_middleware_arc_lock,
                    &response_middleware_arc_lock,
                    &route_func_arc_lock,
                    &route_matcher_arc_lock,
                );
                match is_websocket {
                    true => {
                        Self::handle_websocket_connection(&handler, &mut request).await;
                    }
                    false => {
                        Self::handle_http_connection(&handler, &request).await;
                    }
                };
                let _ = stream.close().await;
            });
        }
        Ok(())
    }

    async fn handle_request_common<'a>(
        handler: &RequestHandlerImmutableParams<'a>,
        request: &Request,
    ) -> bool {
        let stream: &ArcRwLockStream = handler.stream;
        let route: &String = request.get_path();
        let ctx: Context = Context::from_stream_request(stream, request);
        let return_handle = || async {
            yield_now().await;
            request.is_enable_keep_alive()
        };
        for middleware in handler.request_middleware.read().await.iter() {
            middleware(ctx.clone()).await;
            if ctx.get_aborted().await {
                return return_handle().await;
            }
        }
        if let Some(route_handler) = handler.route_func.read().await.get(route) {
            route_handler(ctx.clone()).await;
        } else if let Some((handler_func, params)) =
            handler.route_matcher.read().await.match_route(route)
        {
            ctx.set_route_params(params).await;
            handler_func(ctx.clone()).await;
        }
        for middleware in handler.response_middleware.read().await.iter() {
            if ctx.get_aborted().await {
                return return_handle().await;
            }
            middleware(ctx.clone()).await;
        }
        return_handle().await
    }

    async fn handle_websocket_connection<'a>(
        handler: &RequestHandlerImmutableParams<'a>,
        first_request: &mut Request,
    ) {
        let stream: &ArcRwLockStream = handler.stream;
        let buffer_size: usize = *handler.config.get_websocket_buffer_size();
        let ctx: Context = Context::from_stream_request(stream, first_request);
        if ctx.handle_websocket().await.is_err() {
            return;
        }
        let route: &String = first_request.get_path();
        let contains_disable_inner_websocket_handle: bool = handler
            .config
            .contains_disable_inner_websocket_handle(route)
            .await;
        if contains_disable_inner_websocket_handle {
            while Self::handle_request_common(handler, first_request).await {}
            return;
        }
        while let Ok(request) =
            Request::websocket_request_from_stream(stream, buffer_size, &first_request).await
        {
            let _ = Self::handle_request_common(handler, &request).await;
        }
    }

    async fn handle_http_connection<'a>(
        handler: &RequestHandlerImmutableParams<'a>,
        first_request: &Request,
    ) {
        let handle_result: bool = Self::handle_request_common(handler, first_request).await;
        if !handle_result {
            return;
        }
        let stream: ArcRwLockStream = handler.stream.clone();
        let route: &String = first_request.get_path();
        let contains_disable_inner_http_handle: bool = handler
            .config
            .contains_disable_inner_http_handle(route)
            .await;
        let buffer_size: usize = *handler.config.get_http_line_buffer_size();
        if contains_disable_inner_http_handle {
            while Self::handle_request_common(handler, first_request).await {}
            return;
        }
        while let Ok(request) = Request::http_request_from_stream(&stream, buffer_size).await {
            let handle_result: bool = Self::handle_request_common(handler, &request).await;
            if !handle_result {
                return;
            }
        }
    }
}

impl<'a> RequestHandlerImmutableParams<'a> {
    pub fn new(
        stream: &'a ArcRwLockStream,
        config: &'a ServerConfig<'a>,
        request_middleware: &'a ArcRwLockMiddlewareFuncBox,
        response_middleware: &'a ArcRwLockMiddlewareFuncBox,
        route_func: &'a ArcRwLockHashMapRouteFuncBox,
        route_matcher: &'a ArcRwLock<RouteMatcher>,
    ) -> Self {
        Self {
            stream,
            config,
            request_middleware,
            response_middleware,
            route_func,
            route_matcher,
        }
    }
}
