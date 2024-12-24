use super::{
    config::r#type::ServerConfig, controller_data::r#type::ControllerData, error::r#type::Error,
    r#type::Server,
};
use http_type::*;
use std::{
    collections::HashMap,
    net::{TcpListener, TcpStream},
};

impl<'a> Default for Server<'a> {
    fn default() -> Self {
        Self {
            cfg: ServerConfig::default(),
            router_func: HashMap::new(),
            static_dir: None,
            middleware: vec![],
        }
    }
}

impl<'a> Server<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn host(&mut self, host: &'a str) -> &mut Self {
        self.cfg.host(host);
        self
    }

    pub fn port(&mut self, port: usize) -> &mut Self {
        self.cfg.port(port);
        self
    }

    pub fn buffer_size(&mut self, buffer_size: usize) -> &mut Self {
        self.cfg.buffer_size(buffer_size);
        self
    }

    pub fn router<F>(&mut self, route: &'a str, func: F) -> &mut Self
    where
        F: 'static + Fn(&mut ControllerData),
    {
        self.router_func.insert(route, Box::new(func));
        self
    }

    pub fn middleware<F>(&mut self, func: F) -> &mut Self
    where
        F: 'static + Fn(&mut ControllerData),
    {
        self.middleware.push(Box::new(func));
        self
    }

    pub fn listen(&mut self) -> &mut Self {
        let addr: String = format!("{}:{}", &self.cfg.host, &self.cfg.port);
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
            let mut stream: TcpStream = stream_res.unwrap();
            let request_obj_res: Result<Request<'_>, Error> =
                Request::new(&stream).map_err(|err| Error::InvalidHttpRequest(err));
            let request_obj: Request<'_> = request_obj_res.unwrap();
            let route: String = request_obj.path().into_owned();
            let route_str: &str = route.as_str();
            let mut controller_data: ControllerData<'_> = ControllerData {
                stream: &mut stream,
                response: Response::default(),
                request: request_obj.clone(),
            };
            for middleware in &self.middleware {
                middleware(&mut controller_data);
            }
            self.router_func.get(route_str).and_then(|func| {
                let res: () = func(&mut controller_data);
                Some(res)
            });
        }
        self
    }
}
