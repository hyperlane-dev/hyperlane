use crate::*;

impl<'a> Default for ServerConfig<'a> {
    fn default() -> Self {
        Self {
            host: DEFAULT_HOST,
            port: DEFAULT_WEB_PORT,
            log_dir: DEFAULT_LOG_DIR,
            log_size: DEFAULT_LOG_FILE_SIZE,
            inner_print: DEFAULT_INNER_PRINT,
            inner_log: DEFAULT_INNER_LOG,
            websocket_buffer_size: DEFAULT_BUFFER_SIZE,
            http_line_buffer_size: DEFAULT_BUFFER_SIZE,
            nodelay: DEFAULT_NODELAY,
            linger: DEFAULT_LINGER,
            ttl: DEFAULT_TTI,
            enable_inner_websocket_handle: arc_rwlock(hash_set_xx_hash3_64()),
            route_matcher: arc_rwlock(RouteMatcher::new()),
        }
    }
}

impl<'a> ServerConfig<'a> {
    pub async fn contains_inner_websocket_handle(&self, route: &'a str) -> bool {
        if self
            .get_enable_inner_websocket_handle()
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

    pub async fn enable_inner_websocket_handle(&self, route: String) -> bool {
        let result: bool = self
            .get_enable_inner_websocket_handle()
            .write()
            .await
            .insert(route.clone());
        let _ = ServerConfig::get_route_matcher(self)
            .write()
            .await
            .add(&route, Arc::new(|_| Box::pin(async move {})));
        result
    }

    pub async fn disable_inner_websocket_handle(&self, route: String) -> bool {
        let result = self
            .get_enable_inner_websocket_handle()
            .write()
            .await
            .remove(&route);
        result
    }
}
