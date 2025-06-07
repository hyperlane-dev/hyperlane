use crate::*;

impl<'a> Default for ServerConfig<'a> {
    fn default() -> Self {
        Self {
            host: DEFAULT_HOST,
            port: DEFAULT_WEB_PORT,
            ws_buffer_size: DEFAULT_BUFFER_SIZE,
            http_line_buffer_size: DEFAULT_BUFFER_SIZE,
            nodelay: DEFAULT_NODELAY,
            linger: DEFAULT_LINGER,
            ttl: DEFAULT_TTI,
            disable_internal_http_handler: arc_rwlock(hash_set_xx_hash3_64()),
            disable_internal_ws_handler: arc_rwlock(hash_set_xx_hash3_64()),
            route_matcher: arc_rwlock(RouteMatcher::new()),
            error_handler: Arc::new(print_error_handler),
        }
    }
}

impl<'a> ServerConfig<'a> {
    pub async fn contains_disable_internal_http_handler(&self, route: &'a str) -> bool {
        if self
            .get_disable_internal_http_handler()
            .read()
            .await
            .contains(route)
        {
            return true;
        }
        if let Some(_) = self.get_route_matcher().read().await.match_route(route) {
            return true;
        }
        false
    }

    pub async fn disable_internal_http_handler(&self, route: String) -> bool {
        ServerConfig::get_route_matcher(self)
            .write()
            .await
            .add(&route, Arc::new(|_| Box::pin(async move {})))
            .unwrap_or_else(|err| panic!("{}", err));
        let result: bool = self
            .get_disable_internal_http_handler()
            .write()
            .await
            .insert(route.clone());
        result
    }

    pub async fn enable_internal_http_handler(&self, route: String) -> bool {
        let result: bool = self
            .get_disable_internal_http_handler()
            .write()
            .await
            .remove(&route);
        result
    }

    pub async fn contains_disable_internal_ws_handler(&self, route: &'a str) -> bool {
        if self
            .get_disable_internal_ws_handler()
            .read()
            .await
            .contains(route)
        {
            return true;
        }
        if let Some(_) = self.get_route_matcher().read().await.match_route(route) {
            return true;
        }
        false
    }

    pub async fn disable_internal_ws_handler(&self, route: String) -> bool {
        ServerConfig::get_route_matcher(self)
            .write()
            .await
            .add(&route, Arc::new(|_| Box::pin(async move {})))
            .unwrap_or_else(|err| panic!("{}", err));
        let result: bool = self
            .get_disable_internal_ws_handler()
            .write()
            .await
            .insert(route.clone());
        result
    }

    pub async fn enable_internal_ws_handler(&self, route: String) -> bool {
        let result: bool = self
            .get_disable_internal_ws_handler()
            .write()
            .await
            .remove(&route);
        result
    }
}
