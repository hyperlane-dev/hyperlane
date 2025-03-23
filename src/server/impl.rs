use crate::*;

impl Default for Server {
    fn default() -> Self {
        Self {
            cfg: arc_rwlock(ServerConfig::default()),
            route_func: arc_rwlock(hash_map_xxhash3_64()),
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
            .set_file_size(log_size);
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

    pub async fn log_interval_millis(&self, interval_millis: usize) -> &Self {
        self.get_cfg()
            .write()
            .await
            .set_interval_millis(interval_millis);
        self.get_tmp()
            .write()
            .await
            .get_mut_log()
            .set_interval_millis(interval_millis);
        self
    }

    pub async fn route<R, F, Fut>(&self, route: R, func: F) -> &Self
    where
        R: ToString,
        F: FuncWithoutPin<Fut>,
        Fut: Future<Output = ()> + Send + 'static,
    {
        self.get_route_func().write().await.insert(
            route.to_string(),
            Box::new(move |controller_data| Box::pin(func(controller_data))),
        );
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
            .push(Box::new(move |controller_data| {
                Box::pin(func(controller_data))
            }));

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
            .push(Box::new(move |controller_data| {
                Box::pin(func(controller_data))
            }));
        self
    }

    async fn get_request_obj_result(
        stream_arc: &ArcRwLockStream,
        http_line_buffer_size: usize,
        websocket_handshake_finish: bool,
        websocket_buffer_size: usize,
    ) -> RequestNewResult {
        if websocket_handshake_finish {
            Request::websocket_request_from_stream(&stream_arc, websocket_buffer_size).await
        } else {
            Request::http_request_from_stream(&stream_arc, http_line_buffer_size).await
        }
    }

    pub async fn listen(&self) -> &Self {
        {
            self.init().await;
            let cfg: ServerConfig<'_> = self.get_cfg().read().await.clone();
            let host: &str = *cfg.get_host();
            let port: usize = *cfg.get_port();
            let websocket_buffer_size: usize = *cfg.get_websocket_buffer_size();
            let http_line_buffer_size: usize = *cfg.get_http_line_buffer_size();
            let addr: String = format!("{}{}{}", host, COLON_SPACE_SYMBOL, port);
            let tcp_listener: TcpListener = TcpListener::bind(&addr)
                .await
                .map_err(|err| ServerError::TcpBindError(err.to_string()))
                .unwrap();
            while let Ok((stream, _socket_addr)) = tcp_listener.accept().await {
                let tmp_arc_lock: ArcRwLockTmp = self.get_tmp().clone();
                let stream_arc: ArcRwLockStream = ArcRwLockStream::from_stream(stream);
                let request_middleware_arc_lock: ArcRwLockMiddlewareFuncBox =
                    self.get_request_middleware().clone();
                let response_middleware_arc_lock: ArcRwLockMiddlewareFuncBox =
                    self.get_response_middleware().clone();
                let route_func_arc_lock: ArcRwLockHashMapRouteFuncBox =
                    self.get_route_func().clone();
                let handle_request = move || async move {
                    let log: Log = tmp_arc_lock.read().await.get_log().clone();
                    let mut enable_websocket_opt: Option<bool> = None;
                    let mut websocket_handshake_finish: bool = false;
                    let mut history_request: Request = Request::default();
                    loop {
                        let mut inner_controller_data: InnerControllerData =
                            InnerControllerData::default();
                        let request_obj_result: Result<Request, ServerError> =
                            Self::get_request_obj_result(
                                &stream_arc,
                                http_line_buffer_size,
                                websocket_handshake_finish,
                                websocket_buffer_size,
                            )
                            .await
                            .map_err(|err| ServerError::InvalidHttpRequest(err));
                        let init_enable_websocket_opt: bool = enable_websocket_opt.is_some();
                        if request_obj_result.is_err() && !init_enable_websocket_opt {
                            let _ = inner_controller_data
                                .get_mut_response()
                                .close(&stream_arc)
                                .await;
                            return;
                        }
                        let mut request_obj: Request = request_obj_result.unwrap_or_default();
                        if websocket_handshake_finish {
                            history_request.set_body(request_obj.get_body().clone());
                            request_obj = history_request.clone();
                        } else if !init_enable_websocket_opt {
                            history_request = request_obj.clone();
                        }
                        let route: String = request_obj.get_path().clone();
                        inner_controller_data
                            .set_stream(Some(stream_arc.clone()))
                            .set_request(request_obj)
                            .set_log(log.clone());
                        let controller_data: ControllerData =
                            ControllerData::from_controller_data(inner_controller_data);
                        if !init_enable_websocket_opt {
                            enable_websocket_opt =
                                Some(controller_data.judge_enable_websocket().await);
                        }
                        let enable_websocket: bool = enable_websocket_opt.unwrap_or_default();
                        if enable_websocket {
                            let handle_res: ResponseResult = controller_data
                                .handle_websocket(&mut websocket_handshake_finish)
                                .await;
                            if handle_res.is_err() {
                                let _ = controller_data.close().await;
                                return;
                            }
                        }
                        for request_middleware in request_middleware_arc_lock.read().await.iter() {
                            request_middleware(controller_data.clone()).await;
                        }
                        if let Some(route_func) = route_func_arc_lock.read().await.get(&route) {
                            route_func(controller_data.clone()).await;
                        }
                        for response_middleware in response_middleware_arc_lock.read().await.iter()
                        {
                            response_middleware(controller_data.clone()).await;
                        }
                        if controller_data.judge_unenable_keep_alive().await && !enable_websocket {
                            let _ = controller_data.close().await;
                            return;
                        }
                        yield_now().await;
                    }
                };
                tokio::spawn(handle_request());
            }
        }
        self
    }

    async fn init_log(&self) {
        log_run(self.get_tmp().read().await.get_log());
    }

    async fn init_panic_hook(&self) {
        let tmp: Tmp = self.get_tmp().read().await.clone();
        let cfg: ServerConfig<'_> = self.get_cfg().read().await.clone();
        let inner_print: bool = *cfg.get_inner_print();
        let inner_log: bool = *cfg.get_inner_log();
        set_hook(Box::new(move |err| {
            let err_msg: String = format!("{}", err);
            if inner_print {
                println_error!(err_msg);
            }
            if inner_log {
                handle_error(&tmp, err_msg.clone());
            }
        }));
    }

    async fn init(&self) {
        self.init_panic_hook().await;
        self.init_log().await;
    }
}
