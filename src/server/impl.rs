use crate::*;

impl Default for Server {
    #[inline]
    fn default() -> Self {
        Self {
            cfg: Arc::new(RwLock::new(ServerConfig::default())),
            tmp: Arc::new(RwLock::new(Tmp::default())),
            router_func: Arc::new(RwLock::new(hash_map!())),
            middleware: Arc::new(RwLock::new(vec![])),
        }
    }
}

impl Server {
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    #[inline]
    pub async fn host(&mut self, host: &'static str) -> &mut Self {
        {
            let mut cfg: RwLockWriteGuard<'_, ServerConfig<'_>> = self.get_cfg().write().await;
            cfg.set_host(host);
        }
        self
    }

    #[inline]
    pub async fn port(&mut self, port: usize) -> &mut Self {
        {
            let mut cfg: RwLockWriteGuard<'_, ServerConfig<'_>> = self.get_cfg().write().await;
            cfg.set_port(port);
        }
        self
    }

    #[inline]
    pub async fn log_dir(&mut self, log_dir: &'static str) -> &mut Self {
        {
            let mut cfg: RwLockWriteGuard<'_, ServerConfig<'_>> = self.get_cfg().write().await;
            cfg.set_log_dir(log_dir);
            let mut tmp: RwLockWriteGuard<'_, Tmp> = self.get_tmp().write().await;
            tmp.get_mut_log().set_path(log_dir.into());
        }
        self
    }

    #[inline]
    pub async fn log_size(&mut self, log_size: usize) -> &mut Self {
        {
            let mut cfg: RwLockWriteGuard<'_, ServerConfig<'_>> = self.get_cfg().write().await;
            cfg.set_log_size(log_size);
            let mut tmp: RwLockWriteGuard<'_, Tmp> = self.get_tmp().write().await;
            tmp.get_mut_log().set_file_size(log_size);
        }
        self
    }

    #[inline]
    pub async fn print(&mut self, print: bool) -> &mut Self {
        {
            let mut cfg: RwLockWriteGuard<'_, ServerConfig<'_>> = self.get_cfg().write().await;
            cfg.set_print(print);
        }
        self
    }

    #[inline]
    pub async fn enable_print(&mut self) -> &mut Self {
        self.print(true).await;
        self
    }

    #[inline]
    pub async fn disable_print(&mut self) -> &mut Self {
        self.print(false).await;
        self
    }

    #[inline]
    pub async fn open_print(&mut self, print: bool) -> &mut Self {
        {
            let mut cfg: RwLockWriteGuard<'_, ServerConfig<'_>> = self.get_cfg().write().await;
            cfg.set_print(print);
        }
        self
    }

    #[inline]
    pub async fn log_interval_millis(&mut self, interval_millis: usize) -> &mut Self {
        {
            let mut cfg: RwLockWriteGuard<'_, ServerConfig<'_>> = self.get_cfg().write().await;
            cfg.set_interval_millis(interval_millis);
            let mut tmp: RwLockWriteGuard<'_, Tmp> = self.get_tmp().write().await;
            tmp.get_mut_log().set_interval_millis(interval_millis);
        }
        self
    }

    #[inline]
    pub async fn router<F, Fut>(&mut self, route: &'static str, func: F) -> &mut Self
    where
        F: FuncWithoutPin<Fut>,
        Fut: Future<Output = ()> + Send + 'static,
    {
        {
            let mut mut_router_func: RwLockWriteGuard<'_, HashMap<&str, Box<dyn Func + Send>>> =
                self.router_func.write().await;
            mut_router_func.insert(
                route,
                Box::new(move |controller_data| Box::pin(func(controller_data))),
            );
        }
        self
    }

    #[inline]
    pub async fn middleware<F, Fut>(&mut self, func: F) -> &mut Self
    where
        F: FuncWithoutPin<Fut>,
        Fut: Future<Output = ()> + Send + 'static,
    {
        {
            let mut mut_async_middleware: RwLockWriteGuard<'_, Vec<Box<dyn Func + Send>>> =
                self.middleware.write().await;
            mut_async_middleware.push(Box::new(move |controller_data| {
                Box::pin(func(controller_data))
            }));
        }
        self
    }

    #[inline]
    pub async fn judge_enable_keep_alive(
        arc_lock_controller_data: &ArcRwLockControllerData,
    ) -> bool {
        let controller_data: RwLockReadGuard<'_, ControllerData> =
            arc_lock_controller_data.get_read_lock().await;
        for tem in controller_data.get_request().get_headers().iter() {
            if tem.0.eq_ignore_ascii_case(CONNECTION) {
                if tem.1.eq_ignore_ascii_case(CONNECTION_KEEP_ALIVE) {
                    return true;
                }
                break;
            }
        }
        return false;
    }

    #[inline]
    pub async fn listen(&mut self) -> &mut Self {
        {
            self.init().await;
            let cfg: RwLockReadGuard<'_, ServerConfig<'_>> = self.get_cfg().read().await;
            let host: &str = *cfg.get_host();
            let port: usize = *cfg.get_port();
            let addr: String = format!("{}{}{}", host, COLON_SPACE_SYMBOL, port);
            let tcp_listener: TcpListener = TcpListener::bind(&addr)
                .await
                .map_err(|e| ServerError::TcpBindError(e.to_string()))
                .unwrap();
            while let Ok((stream, _socket_addr)) = tcp_listener.accept().await {
                let tmp_arc_lock: ArcRwLock<Tmp> = Arc::clone(&self.tmp);
                let stream_arc: ArcRwLockStream = ArcRwLockStream::from_stream(stream);
                let async_middleware_arc_lock: ArcRwLockHashMapMiddlewareFuncBox =
                    Arc::clone(&self.middleware);
                let router_func_arc_lock: ArcRwLockHashMapRouterFuncBox =
                    Arc::clone(&self.router_func);
                let handle_request = move || async move {
                    let log: Log = tmp_arc_lock.read().await.get_log().clone();
                    loop {
                        let mut controller_data: ControllerData = ControllerData::new();
                        let request_obj_result: Result<Request, ServerError> =
                            Request::from_stream(&stream_arc)
                                .await
                                .map_err(|err| ServerError::InvalidHttpRequest(err));
                        if request_obj_result.is_err() {
                            let _ = controller_data.get_mut_response().close(&stream_arc);
                            return;
                        }
                        print_success!(format!("{:?}", request_obj_result));
                        let request_obj: Request = request_obj_result.unwrap_or_default();
                        let route: String = request_obj.get_path().clone();
                        controller_data
                            .set_stream(Some(stream_arc.clone()))
                            .set_request(request_obj)
                            .set_log(log.clone());
                        let arc_lock_controller_data: ArcRwLockControllerData =
                            ArcRwLockControllerData::from_controller_data(controller_data);
                        for middleware in async_middleware_arc_lock.read().await.iter() {
                            middleware(arc_lock_controller_data.clone()).await;
                        }
                        if let Some(async_func) =
                            router_func_arc_lock.read().await.get(route.as_str())
                        {
                            async_func(arc_lock_controller_data.clone()).await;
                        }
                        if !Self::judge_enable_keep_alive(&arc_lock_controller_data).await {
                            let controller_data: RwLockReadControllerData =
                                arc_lock_controller_data.get_read_lock().await;
                            let _ = controller_data.get_response().clone().close(&stream_arc);
                            return;
                        }
                    }
                };
                tokio::spawn(async move { handle_request().await });
            }
        }
        self
    }

    #[inline]
    async fn init_log(&self) {
        let tmp: RwLockReadGuard<'_, Tmp> = self.get_tmp().read().await;
        log_run(tmp.get_log());
    }

    #[inline]
    async fn init_panic_hook(&self) {
        let tmp: Tmp = self.tmp.read().await.clone();
        let print: bool = self.get_cfg().read().await.get_print().clone();
        set_hook(Box::new(move |err| {
            let err_msg: String = format!("{}", err);
            if print {
                println_error!(err_msg);
            }
            handle_error(&tmp, err_msg.clone());
        }));
    }

    #[inline]
    async fn init(&self) {
        self.init_panic_hook().await;
        self.init_log().await;
    }
}
