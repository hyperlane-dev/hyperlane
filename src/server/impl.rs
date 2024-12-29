use super::{
    config::r#type::ServerConfig, controller_data::r#type::ControllerData, error::r#type::Error,
    middleware::r#type::MiddlewareArcLock, r#type::Server, route::r#type::RouterFuncArcLock,
    thread_pool::r#type::ThreadPool, tmp::r#type::Tmp,
};
use http_constant::*;
use http_type::*;
use hyperlane_log::*;
use hyperlane_time::*;
use std_macro_extensions::*;

impl Default for Server {
    fn default() -> Self {
        Self {
            cfg: Arc::new(RwLock::new(ServerConfig::default())),
            router_func: Arc::new(RwLock::new(HashMap::new())),
            middleware: Arc::new(RwLock::new(vec![])),
            tmp: Arc::new(RwLock::new(Tmp::default())),
        }
    }
}

impl Server {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn host(&mut self, host: &'static str) -> &mut Self {
        let _ = self.get_cfg().write().and_then(|mut cfg| {
            cfg.set_host(host);
            Ok(())
        });
        self
    }

    pub fn port(&mut self, port: usize) -> &mut Self {
        let _ = self.get_cfg().write().and_then(|mut cfg| {
            cfg.set_port(port);
            Ok(())
        });
        self
    }

    pub fn thread_pool_size(&mut self, thread_pool_size: usize) -> &mut Self {
        let _ = self.get_cfg().write().and_then(|mut cfg| {
            cfg.set_thread_pool_size(thread_pool_size);
            Ok(())
        });
        self
    }

    pub fn log_dir(&mut self, log_dir: &'static str) -> &mut Self {
        let _ = self.get_cfg().write().and_then(|mut cfg| {
            cfg.set_log_dir(log_dir);
            Ok(())
        });
        let _ = self.get_tmp().write().and_then(|mut tmp| {
            tmp.log.set_path(log_dir);
            Ok(())
        });
        self
    }

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

    pub fn router<F>(&mut self, route: &'static str, func: F) -> &mut Self
    where
        F: 'static + Send + Sync + Fn(&mut ControllerData),
    {
        if let Ok(mut router_func) = self.router_func.write() {
            router_func.insert(route, Box::new(func));
        }
        self
    }

    pub fn middleware<F>(&mut self, func: F) -> &mut Self
    where
        F: 'static + Send + Sync + Fn(&mut ControllerData),
    {
        if let Ok(mut middleware) = self.middleware.write() {
            middleware.push(Box::new(func));
        }
        self
    }

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
        let listener_res: Result<TcpListener, Error> =
            TcpListener::bind(&addr).map_err(|e| Error::TcpBindError(e.to_string()));
        if listener_res.is_err() {
            return self;
        }
        let tcp_listener: TcpListener = listener_res.unwrap();
        let thread_pool: ThreadPool = ThreadPool::new(thread_pool_size);
        for stream_res in tcp_listener.incoming() {
            if stream_res.is_err() {
                continue;
            }
            let stream: TcpStream = stream_res.unwrap();
            let stream_arc: Arc<TcpStream> = Arc::new(stream);
            let middleware_arc: MiddlewareArcLock = Arc::clone(&self.middleware);
            let router_func_arc: RouterFuncArcLock = Arc::clone(&self.router_func);
            let tmp_arc: ArcRwLock<Tmp> = Arc::clone(&self.tmp);
            let thread_pool_func = move || {
                let _ = tmp_arc.write().and_then(|mut tmp| {
                    tmp.add_thread_num();
                    Ok(())
                });
                let log: Log = tmp_arc
                    .read()
                    .and_then(|tmp| Ok(tmp.log.clone()))
                    .unwrap_or_default();
                let thread_result: Result<(), Box<dyn Any + Send>> = catch_unwind(move || {
                    let request_obj_res: Result<Request, Error> =
                        Request::new(&stream_arc.as_ref())
                            .map_err(|err| Error::InvalidHttpRequest(err));
                    if let Ok(request_obj) = request_obj_res {
                        let route =
                            <Cow<'_, str> as Clone>::clone(&request_obj.get_path()).into_owned();
                        let mut controller_data: ControllerData = ControllerData::new();
                        controller_data
                            .set_stream(Some(stream_arc.clone()))
                            .set_response(Some(Response::default()))
                            .set_request(Some(request_obj.clone()))
                            .set_log(log);
                        if let Ok(middleware_guard) = middleware_arc.read() {
                            for middleware in middleware_guard.iter() {
                                middleware(&mut controller_data);
                            }
                        }
                        if let Ok(router_func) = router_func_arc.read() {
                            router_func.get(route.as_str()).and_then(|func| {
                                func(&mut controller_data);
                                Some(())
                            });
                        }
                    }
                });
                let _ = tmp_arc.write().and_then(|mut tmp| {
                    tmp.sub_thread_num();
                    Ok(())
                });
                if let Err(err) = thread_result {
                    let _ = tmp_arc.read().and_then(|tem| {
                        tem.get_log().log_error(format!("{:?}", err), |data| {
                            format!("{}: {}{}", current_time(), data.to_string(), HTTP_BR)
                        });
                        Ok(())
                    });
                }
            };
            thread_pool.execute(thread_pool_func);
        }
        self
    }

    fn init_log(&self) {
        let _ = self.get_tmp().read().and_then(|tmp| {
            log_run(tmp.get_log());
            Ok(())
        });
    }

    fn init(&self) {
        self.init_log();
    }
}
