use super::{
    config::r#type::ServerConfig, controller_data::r#type::ControllerData, error::r#type::Error,
    middleware::r#type::MiddlewareArcLock, r#type::Server, route::r#type::RouterFuncArcLock,
    tmp::r#type::Tmp,
};
use http_constant::*;
use http_type::*;
use std::{
    collections::HashMap,
    net::{TcpListener, TcpStream},
    sync::{Arc, RwLock},
    thread::spawn,
};

impl Default for Server {
    fn default() -> Self {
        Self {
            cfg: ServerConfig::default(),
            router_func: Arc::new(RwLock::new(HashMap::new())),
            middleware: Arc::new(RwLock::new(vec![])),
            tmp: Tmp::default(),
        }
    }
}

impl Server {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn host(&mut self, host: &'static str) -> &mut Self {
        self.cfg.host(host);
        self
    }

    pub fn port(&mut self, port: usize) -> &mut Self {
        self.cfg.port(port);
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
        let addr: String = format!("{}{}{}", &self.cfg.host, COLON_SPACE_SYMBOL, &self.cfg.port);
        let listener_res: Result<TcpListener, Error> =
            TcpListener::bind(&addr).map_err(|e| Error::TcpBindError(e.to_string()));
        if listener_res.is_err() {
            return self;
        }
        let tcp_listener: TcpListener = listener_res.unwrap();
        for stream_res in tcp_listener.incoming() {
            if stream_res.is_err() {
                continue;
            }
            let stream: TcpStream = stream_res.unwrap();
            let stream_arc: Arc<TcpStream> = Arc::new(stream);
            let request_obj_res: Result<Request, Error> =
                Request::new(&stream_arc.as_ref()).map_err(|err| Error::InvalidHttpRequest(err));
            let request_obj: Request = request_obj_res.unwrap();
            let route: String = request_obj.path().into_owned();
            let mut controller_data: ControllerData = ControllerData {
                stream: stream_arc,
                response: Response::default(),
                request: request_obj.clone(),
            };
            let middleware_arc: MiddlewareArcLock = Arc::clone(&self.middleware);
            let router_func_arc: RouterFuncArcLock = Arc::clone(&self.router_func);
            spawn(move || {
                if let Ok(middleware_guard) = middleware_arc.read() {
                    for middleware in middleware_guard.iter() {
                        middleware(&mut controller_data);
                    }
                }
                if let Ok(router_func) = router_func_arc.read() {
                    router_func.get(route.as_str()).and_then(|func| {
                        let res: () = func(&mut controller_data);
                        Some(res)
                    });
                }
            });
        }
        self
    }
}
