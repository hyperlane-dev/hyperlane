use crate::*;

impl Default for Server {
    fn default() -> Self {
        Self {
            cfg: arc_rwlock(ServerConfig::default()),
            route: arc_rwlock(hash_map_xxhash3_64()),
            request_middleware: arc_rwlock(vec![]),
            response_middleware: arc_rwlock(vec![]),
            tmp: arc_rwlock(Tmp::default()),
        }
    }
}

impl Server {
    pub fn new() -> Self {
        Self::default()
    }

    pub async fn host(&self, host: &'static str) -> &Self {
        self.get_cfg().write().await.set_host(host);
        self
    }

    pub async fn port(&self, port: usize) -> &Self {
        self.get_cfg().write().await.set_port(port);
        self
    }

    pub async fn log_dir(&self, log_dir: &'static str) -> &Self {
        self.get_cfg().write().await.set_log_dir(log_dir);
        self.get_tmp()
            .write()
            .await
            .get_mut_log()
            .set_path(log_dir.into());
        self
    }

    pub async fn log_size(&self, log_size: usize) -> &Self {
        self.get_cfg().write().await.set_log_size(log_size);
        self.get_tmp()
            .write()
            .await
            .get_mut_log()
            .set_limit_file_size(log_size);
        self
    }

    pub async fn enable_log(&self) -> &Self {
        self.get_cfg()
            .write()
            .await
            .set_log_size(DEFAULT_LOG_FILE_SIZE);
        self.get_tmp()
            .write()
            .await
            .get_mut_log()
            .set_limit_file_size(DEFAULT_LOG_FILE_SIZE);
        self
    }

    pub async fn disable_log(&self) -> &Self {
        self.get_cfg()
            .write()
            .await
            .set_log_size(DISABLE_LOG_FILE_SIZE);
        self.get_tmp()
            .write()
            .await
            .get_mut_log()
            .set_limit_file_size(DISABLE_LOG_FILE_SIZE);
        self
    }

    pub async fn http_line_buffer_size(&self, buffer_size: usize) -> &Self {
        let buffer_size: usize = if buffer_size == 0 {
            DEFAULT_BUFFER_SIZE
        } else {
            buffer_size
        };
        self.get_cfg()
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
        self.get_cfg()
            .write()
            .await
            .set_websocket_buffer_size(buffer_size);
        self
    }

    pub async fn inner_print(&self, print: bool) -> &Self {
        self.get_cfg().write().await.set_inner_print(print);
        self
    }

    pub async fn inner_log(&self, print: bool) -> &Self {
        self.get_cfg().write().await.set_inner_log(print);
        self
    }

    pub async fn enable_inner_print(&self) -> &Self {
        self.inner_print(true).await;
        self
    }

    pub async fn disable_inner_print(&self) -> &Self {
        self.inner_print(false).await;
        self
    }

    pub async fn enable_inner_log(&self) -> &Self {
        self.inner_log(true).await;
        self
    }

    pub async fn disable_inner_log(&self) -> &Self {
        self.inner_log(false).await;
        self
    }

    pub async fn set_nodelay(&self, nodelay: bool) -> &Self {
        self.get_cfg().write().await.set_nodelay(nodelay);
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

    pub async fn set_linger(&self, linger: Option<Duration>) -> &Self {
        self.get_cfg().write().await.set_linger(linger);
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
        self.get_cfg().write().await.set_ttl(Some(ttl));
        self
    }

    pub async fn route<R, F, Fut>(&self, route: R, func: F) -> &Self
    where
        R: ToString,
        F: FuncWithoutPin<Fut>,
        Fut: Future<Output = ()> + Send + 'static,
    {
        self.get_route()
            .write()
            .await
            .insert(route.to_string(), Box::new(move |ctx| Box::pin(func(ctx))));
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
            .push(Box::new(move |ctx| Box::pin(func(ctx))));

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
            .push(Box::new(move |ctx| Box::pin(func(ctx))));
        self
    }

    pub async fn listen(&self) -> ServerResult {
        self.init().await;
        let cfg: ServerConfig<'_> = self.get_cfg().read().await.clone();
        let log: Log = self.get_tmp().read().await.get_log().clone();
        let host: &str = *cfg.get_host();
        let port: usize = *cfg.get_port();
        let nodelay: bool = *cfg.get_nodelay();
        let linger: Option<Duration> = *cfg.get_linger();
        let ttl_opt: Option<u32> = *cfg.get_ttl();
        let websocket_buffer_size: usize = *cfg.get_websocket_buffer_size();
        let http_line_buffer_size: usize = *cfg.get_http_line_buffer_size();
        let addr: String = Context::format_host_port(host, &port);
        let tcp_listener: TcpListener = TcpListener::bind(&addr)
            .await
            .map_err(|err| ServerError::TcpBindError(err.to_string()))?;
        while let Ok((stream, _socket_addr)) = tcp_listener.accept().await {
            let _ = stream.set_nodelay(nodelay);
            let _ = stream.set_linger(linger);
            if let Some(ttl) = ttl_opt {
                let _ = stream.set_ttl(ttl);
            }
            let log_clone: Log = log.clone();
            let stream: ArcRwLockStream = ArcRwLockStream::from_stream(stream);
            let request_middleware_arc_lock: ArcRwLockMiddlewareFuncBox =
                self.get_request_middleware().clone();
            let response_middleware_arc_lock: ArcRwLockMiddlewareFuncBox =
                self.get_response_middleware().clone();
            let route_func_arc_lock: ArcRwLockHashMapRouteFuncBox = self.get_route().clone();
            tokio::spawn(async move {
                let request_result: RequestReaderHandleResult =
                    Request::http_request_from_stream(&stream, http_line_buffer_size).await;
                if request_result.is_err() {
                    let _ = stream.close().await;
                    return;
                }
                let request: Request = request_result.unwrap_or_default();
                let is_websocket: bool = request.get_upgrade_type().is_websocket();
                match is_websocket {
                    true => {
                        let mut handler: RequestHandlerParams = RequestHandlerParams::new(
                            &stream,
                            request,
                            &log_clone,
                            websocket_buffer_size,
                            &request_middleware_arc_lock,
                            &response_middleware_arc_lock,
                            &route_func_arc_lock,
                        );
                        Self::handle_websocket_connection(&mut handler).await;
                    }
                    false => {
                        let mut handler: RequestHandlerParams = RequestHandlerParams::new(
                            &stream,
                            request,
                            &log_clone,
                            http_line_buffer_size,
                            &request_middleware_arc_lock,
                            &response_middleware_arc_lock,
                            &route_func_arc_lock,
                        );
                        Self::handle_http_connection(&mut handler).await;
                    }
                };
                let _ = stream.close().await;
            });
        }
        Ok(())
    }

    async fn handle_websocket_connection<'a>(handler: &mut RequestHandlerParams<'a>) {
        let stream: &ArcRwLockStream = handler.stream;
        let buffer_size: usize = handler.buffer_size;
        let mut first_request: Request = handler.request.clone();
        let log: &Log = handler.log;
        let ctx: Context = Context::from_stream_request_log(stream, &first_request, log);
        if ctx.handle_websocket().await.is_err() {
            return;
        }
        let request_middleware: &ArcRwLockMiddlewareFuncBox = handler.request_middleware;
        let response_middleware: &ArcRwLockMiddlewareFuncBox = handler.response_middleware;
        let route_func: &ArcRwLockHashMapRouteFuncBox = handler.route_func;
        let route: String = first_request.get_path().clone();
        while let Ok(request) = Request::websocket_request_from_stream(stream, buffer_size).await {
            let body: RequestBody = request.get_body().clone();
            first_request.set_body(body);
            ctx.set_request(first_request.clone()).await;
            for middleware in request_middleware.read().await.iter() {
                middleware(ctx.clone()).await;
            }
            if let Some(route_handler) = route_func.read().await.get(&route) {
                route_handler(ctx.clone()).await;
            }
            for middleware in response_middleware.read().await.iter() {
                middleware(ctx.clone()).await;
            }
            yield_now().await;
        }
    }

    async fn handle_http_common<'a>(handler: &mut RequestHandlerParams<'a>) -> bool {
        let stream: &ArcRwLockStream = handler.stream;
        let request: &Request = &handler.request;
        let log: &Log = handler.log;
        let request_middleware: &ArcRwLockMiddlewareFuncBox = handler.request_middleware;
        let response_middleware: &ArcRwLockMiddlewareFuncBox = handler.response_middleware;
        let route_func: &ArcRwLockHashMapRouteFuncBox = handler.route_func;
        let ctx: Context = Context::from_stream_request_log(stream, request, log);
        let route: String = request.get_path().clone();
        for middleware in request_middleware.read().await.iter() {
            middleware(ctx.clone()).await;
        }
        if let Some(route_handler) = route_func.read().await.get(&route) {
            route_handler(ctx.clone()).await;
        }
        for middleware in response_middleware.read().await.iter() {
            middleware(ctx.clone()).await;
        }
        yield_now().await;
        if request.is_disable_keep_alive() {
            return false;
        }
        return true;
    }

    async fn handle_http_connection<'a>(handler: &mut RequestHandlerParams<'a>) {
        let handle_res: bool = Self::handle_http_common(handler).await;
        if !handle_res {
            return;
        }
        let stream: ArcRwLockStream = handler.stream.clone();
        let buffer_size: usize = handler.buffer_size;
        while let Ok(request) = Request::http_request_from_stream(&stream, buffer_size).await {
            handler.request = request;
            let handle_res: bool = Self::handle_http_common(handler).await;
            if !handle_res {
                return;
            }
        }
    }

    async fn init_panic_hook(&self) {
        let tmp: Tmp = self.get_tmp().read().await.clone();
        let cfg: ServerConfig<'_> = self.get_cfg().read().await.clone();
        let enable_inner_print: bool = *cfg.get_inner_print();
        let enable_inner_log: bool = *cfg.get_inner_log() && tmp.get_log().is_enable();
        set_hook(Box::new(move |err| {
            let err_string: String = err.to_string();
            if enable_inner_print {
                println_error!(err_string);
            }
            if enable_inner_log {
                handle_error(&tmp, &err_string);
            }
        }));
    }

    async fn init(&self) {
        self.init_panic_hook().await;
    }
}

impl<'a> RequestHandlerParams<'a> {
    pub fn new(
        stream: &'a ArcRwLockStream,
        request: Request,
        log: &'a Log,
        buffer_size: usize,
        request_middleware: &'a ArcRwLockMiddlewareFuncBox,
        response_middleware: &'a ArcRwLockMiddlewareFuncBox,
        route_func: &'a ArcRwLockHashMapRouteFuncBox,
    ) -> Self {
        Self {
            stream,
            request,
            log,
            buffer_size,
            request_middleware,
            response_middleware,
            route_func,
        }
    }
}
