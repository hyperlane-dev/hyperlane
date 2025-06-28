use crate::*;

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            host: DEFAULT_HOST.to_owned(),
            port: DEFAULT_WEB_PORT,
            ws_buffer_size: DEFAULT_BUFFER_SIZE,
            http_buffer_size: DEFAULT_BUFFER_SIZE,
            nodelay: DEFAULT_NODELAY,
            linger: DEFAULT_LINGER,
            ttl: DEFAULT_TTI,
            disable_http_handler: hash_set_xx_hash3_64(),
            disable_ws_handler: hash_set_xx_hash3_64(),
            route_matcher: RouteMatcher::new(),
            error_handler: Arc::new(print_error_handler),
        }
    }
}

impl ServerConfig {
    pub(crate) async fn contains_disable_http_handler<'a>(&self, route: &'a str) -> bool {
        if self.get_disable_http_handler().contains(route) {
            return true;
        }
        self.get_route_matcher().match_route(route)
    }

    pub(crate) async fn disable_http_handler(&mut self, route: String) -> bool {
        ServerConfig::get_mut_route_matcher(self)
            .add(&route, Arc::new(|_| Box::pin(async move {})))
            .unwrap_or_else(|err| panic!("{}", err));
        let result: bool = self.get_mut_disable_http_handler().insert(route.clone());
        result
    }

    pub(crate) async fn enable_http_handler(&mut self, route: String) -> bool {
        let result: bool = self.get_mut_disable_http_handler().remove(&route);
        result
    }

    pub(crate) async fn contains_disable_ws_handler<'a>(&self, route: &'a str) -> bool {
        if self.get_disable_ws_handler().contains(route) {
            return true;
        }
        self.get_route_matcher().match_route(route)
    }

    pub(crate) async fn disable_ws_handler(&mut self, route: String) -> bool {
        ServerConfig::get_mut_route_matcher(self)
            .add(&route, Arc::new(|_| Box::pin(async move {})))
            .unwrap_or_else(|err| panic!("{}", err));
        let result: bool = self.get_mut_disable_ws_handler().insert(route.clone());
        result
    }

    pub(crate) async fn enable_ws_handler(&mut self, route: String) -> bool {
        let result: bool = self.get_mut_disable_ws_handler().remove(&route);
        result
    }
}
