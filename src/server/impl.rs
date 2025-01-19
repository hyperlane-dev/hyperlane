use crate::*;
use std::{
    future::Future,
    net::{TcpListener, TcpStream},
};

impl Default for Server {
    #[inline]
    fn default() -> Self {
        Self {
            cfg: Arc::new(RwLock::new(ServerConfig::default())),
            router_func: Arc::new(RwLock::new(HashMap::new())),
            middleware: Arc::new(RwLock::new(VecBoxDynFunc::default())),
            tmp: Arc::new(RwLock::new(Tmp::default())),
            async_router_func: Arc::new(tokio::sync::RwLock::new(hash_map!())),
            async_middleware: Arc::new(tokio::sync::RwLock::new(vec![])),
        }
    }
}

impl Server {
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    #[inline]
    pub fn host(&mut self, host: &'static str) -> &mut Self {
        let _ = self.get_cfg().write().and_then(|mut cfg| {
            cfg.set_host(host);
            Ok(())
        });
        self
    }

    #[inline]
    pub fn port(&mut self, port: usize) -> &mut Self {
        let _ = self.get_cfg().write().and_then(|mut cfg| {
            cfg.set_port(port);
            Ok(())
        });
        self
    }

    #[inline]
    pub fn thread_pool_size(&mut self, thread_pool_size: usize) -> &mut Self {
        let _ = self.get_cfg().write().and_then(|mut cfg| {
            cfg.set_thread_pool_size(thread_pool_size);
            Ok(())
        });
        self
    }

    #[inline]
    pub fn log_dir(&mut self, log_dir: &'static str) -> &mut Self {
        let _ = self.get_cfg().write().and_then(|mut cfg| {
            cfg.set_log_dir(log_dir);
            Ok(())
        });
        let _ = self.get_tmp().write().and_then(|mut tmp| {
            tmp.log.set_path(log_dir.into());
            Ok(())
        });
        self
    }

    #[inline]
    pub fn log_size(&mut self, log_size: usize) -> &mut Self {
        let _ = self.get_cfg().write().and_then(|mut cfg| {
            cfg.set_log_size(log_size);
            Ok(())
        });
        let _ = self.get_tmp().write().and_then(|mut tmp| {
            tmp.log.set_file_size(log_size);
            Ok(())
        });
        self
    }

    #[inline]
    pub fn log_interval_millis(&mut self, interval_millis: usize) -> &mut Self {
        let _ = self.get_cfg().write().and_then(|mut cfg| {
            cfg.set_interval_millis(interval_millis);
            Ok(())
        });
        let _ = self.get_tmp().write().and_then(|mut tmp| {
            tmp.log.set_interval_millis(interval_millis);
            Ok(())
        });
        self
    }

    #[inline]
    pub fn router<F>(&mut self, route: &'static str, func: F) -> &mut Self
    where
        F: Func,
    {
        if let Ok(mut router_func) = self.router_func.write() {
            router_func.insert(route, Box::new(func));
        }
        self
    }

    #[inline]
    pub fn middleware<F>(&mut self, func: F) -> &mut Self
    where
        F: Func,
    {
        if let Ok(mut middleware) = self.middleware.write() {
            middleware.push(Box::new(func));
        }
        self
    }

    #[inline]
    pub async fn async_router<F, Fut>(&mut self, route: &'static str, func: F) -> &mut Self
    where
        F: AsyncFuncWithoutPin<Fut>,
        Fut: Future<Output = ()> + Send + Sync + 'static,
    {
        {
            let has_route: bool = match self.router_func.read() {
                Ok(router_func_map) => router_func_map.contains_key(route),
                Err(_) => false,
            };
            if !has_route {
                let mut mut_async_router_func: tokio::sync::RwLockWriteGuard<
                    '_,
                    HashMap<&str, Box<dyn AsyncFunc>>,
                > = self.async_router_func.write().await;
                mut_async_router_func.insert(
                    route,
                    Box::new(move |controller_data| Box::pin(func(controller_data))),
                );
            }
        }
        self
    }

    #[inline]
    pub async fn async_middleware<F, Fut>(&mut self, func: F) -> &mut Self
    where
        F: AsyncFuncWithoutPin<Fut>,
        Fut: Future<Output = ()> + Send + Sync + 'static,
    {
        {
            let mut mut_async_middleware: tokio::sync::RwLockWriteGuard<
                '_,
                Vec<Box<dyn AsyncFunc>>,
            > = self.async_middleware.write().await;
            mut_async_middleware.push(Box::new(move |controller_data| {
                Box::pin(func(controller_data))
            }));
        }
        self
    }

    #[inline]
    fn common_log(data: &String) -> String {
        format!("{}: {}{}", current_time(), data.to_string(), HTTP_BR)
    }

    #[inline]
    pub fn listen(&mut self) -> &mut Self {
        self.init();
        let mut host: &str = EMPTY_STR;
        let mut port: usize = usize::default();
        let mut thread_pool_size: usize = usize::default();
        let _ = self.get_cfg().read().and_then(|cfg| {
            host = cfg.get_host();
            port = *cfg.get_port();
            thread_pool_size = *cfg.get_thread_pool_size();
            Ok(())
        });
        let addr: String = format!("{}{}{}", host, COLON_SPACE_SYMBOL, port);
        let listener_res: Result<TcpListener, ServerError> =
            TcpListener::bind(&addr).map_err(|e| ServerError::TcpBindError(e.to_string()));
        if listener_res.is_err() {
            let _ = self.get_tmp().write().and_then(|tmp| {
                tmp.get_log().error(
                    format!("{}", listener_res.err().unwrap_or(ServerError::Unknown)),
                    Self::common_log,
                );
                Ok(())
            });
            return self;
        }
        let tcp_listener: TcpListener = listener_res.unwrap();
        let thread_pool: ThreadPool = ThreadPool::new(thread_pool_size);

        for stream_res in tcp_listener.incoming() {
            if stream_res.is_err() {
                let tmp_arc_lock: ArcRwLock<Tmp> = Arc::clone(&self.tmp);
                let _ = run_function(move || {
                    if let Ok(tem) = tmp_arc_lock.read() {
                        tem.get_log()
                            .error(stream_res.err().unwrap().to_string(), Self::common_log);
                    }
                });
                continue;
            }
            let stream: TcpStream = stream_res.unwrap();
            let stream_arc: Arc<TcpStream> = Arc::new(stream);
            let middleware_arc_lock: MiddlewareArcLock = Arc::clone(&self.middleware);
            let async_middleware_arc_lock: AsyncArcRwLockHashMapMiddlewareFuncBox =
                Arc::clone(&self.async_middleware);
            let router_func_arc_lock: RouterFuncArcLock = Arc::clone(&self.router_func);
            let async_router_func_arc_lock: AsyncArcRwLockHashMapRouterFuncBox =
                Arc::clone(&self.async_router_func);
            let tmp_arc_lock: ArcRwLock<Tmp> = Arc::clone(&self.tmp);
            let error_handle_tmp_arc_lock: ArcRwLock<Tmp> = Arc::clone(&self.tmp);
            let thread_pool_func = move || async move {
                let log: Log = tmp_arc_lock
                    .read()
                    .and_then(|tmp| Ok(tmp.log.clone()))
                    .unwrap_or_default();
                let request_obj: Request = Request::new(&stream_arc.as_ref())
                    .map_err(|err| ServerError::InvalidHttpRequest(err))
                    .unwrap();
                let route: &String = &request_obj.get_path().clone();
                let mut controller_data: ControllerData = ControllerData::new();
                controller_data
                    .set_stream(Some(stream_arc))
                    .set_request(request_obj)
                    .set_log(log);
                let arc_lock_controller_data: ArcRwLockControllerData =
                    Arc::new(RwLock::new(controller_data));
                if let Ok(middleware_guard) = middleware_arc_lock.read() {
                    for middleware in middleware_guard.iter() {
                        middleware(arc_lock_controller_data.clone());
                    }
                }
                for async_middleware in async_middleware_arc_lock.read().await.iter() {
                    async_middleware(arc_lock_controller_data.clone()).await;
                }
                if let Ok(router_func) = router_func_arc_lock.read() {
                    if let Some(func) = router_func.get(route.as_str()) {
                        func(arc_lock_controller_data.clone());
                    }
                }
                if let Some(async_func) =
                    async_router_func_arc_lock.read().await.get(route.as_str())
                {
                    async_func(arc_lock_controller_data.clone()).await;
                }
            };
            let handle_error_func = move |err_string: Arc<String>| async move {
                if let Ok(tem) = error_handle_tmp_arc_lock.read() {
                    tem.get_log()
                        .error(err_string.to_string(), Self::common_log);
                }
            };
            let _ = thread_pool.async_execute_with_catch(thread_pool_func, handle_error_func);
        }
        self
    }

    #[inline]
    fn init_log(&self) {
        let _ = self.get_tmp().read().and_then(|tmp| {
            log_run(tmp.get_log());
            Ok(())
        });
    }

    #[inline]
    fn init(&self) {
        self.init_log();
    }
}
