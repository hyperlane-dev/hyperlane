use crate::*;

#[tokio::test]
async fn server_partial_eq() {
    let server1: Server = Server::default();
    let server2: Server = Server::default();
    assert_eq!(server1, server2);
    let server1_clone: Server = server1.clone();
    assert_eq!(server1, server1_clone);
}

#[tokio::test]
async fn server_from_server_config() {
    let mut server_config: ServerConfig = ServerConfig::default();
    server_config.set_nodelay(Some(true));
    let server: Server = server_config.clone().into();
    assert_eq!(server.get_request_config(), &RequestConfig::default());
    assert_eq!(server.get_server_config(), &server_config);
    assert!(server.get_task_panic().is_empty());
    assert!(server.get_request_error().is_empty());
    assert!(server.get_request_middleware().is_empty());
    assert!(server.get_response_middleware().is_empty());
}

#[tokio::test]
async fn server_from_request_config() {
    let mut request_config: RequestConfig = RequestConfig::default();
    request_config.set_buffer_size(KB_1);
    let server: Server = request_config.into();
    assert_eq!(server.get_request_config(), &request_config);
    assert_eq!(server.get_server_config(), &ServerConfig::default());
    assert!(server.get_task_panic().is_empty());
    assert!(server.get_request_error().is_empty());
    assert!(server.get_request_middleware().is_empty());
    assert!(server.get_response_middleware().is_empty());
}

#[tokio::test]
async fn server_inner_partial_eq() {
    let inner1: Server = Server::default();
    let inner2: Server = Server::default();
    assert_eq!(inner1, inner2);
}

struct TestSendRoute;

impl ServerHook for TestSendRoute {
    async fn new(_ctx: &mut Context) -> Self {
        Self
    }

    async fn handle(self, _ctx: &mut Context) {}
}

#[tokio::test]
async fn server_send_sync() {
    fn assert_send<T: Send>() {}
    fn assert_sync<T: Sync>() {}
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send::<Server>();
    assert_sync::<Server>();
    assert_send_sync::<Server>();
}

#[tokio::test]
async fn server_clone_across_threads() {
    let mut server: Server = Server::default();
    server.route::<TestSendRoute>("/test");
    let server_clone: Server = server.clone();
    let handle: JoinHandle<&'static str> = spawn(async move {
        let _server_in_thread: Server = server_clone;
        "success"
    });
    let result: &'static str = handle.await.unwrap();
    assert_eq!(result, "success");
}

#[tokio::test]
async fn server_share_across_threads() {
    let mut server: Server = Server::default();
    server.route::<TestSendRoute>("/test");
    let server: Arc<Server> = Arc::new(server);
    let server1: Arc<Server> = server.clone();
    let server2: Arc<Server> = server.clone();
    let handle1: JoinHandle<&'static str> = spawn(async move {
        let _server_in_thread1: Arc<Server> = server1;
        "thread1"
    });
    let handle2: JoinHandle<&'static str> = spawn(async move {
        let _server_in_thread2: Arc<Server> = server2;
        "thread2"
    });
    let result1: &'static str = handle1.await.unwrap();
    let result2: &'static str = handle2.await.unwrap();
    assert_eq!(result1, "thread1");
    assert_eq!(result2, "thread2");
}

struct TaskPanicHook {
    response_body: String,
    content_type: String,
}

impl ServerHook for TaskPanicHook {
    async fn new(ctx: &mut Context) -> Self {
        let error: PanicData = ctx.try_get_task_panic_data().unwrap_or_default();
        let response_body: String = error.to_string();
        let content_type: String = ContentType::format_content_type_with_charset(TEXT_PLAIN, UTF8);
        Self {
            response_body,
            content_type,
        }
    }

    async fn handle(self, ctx: &mut Context) {
        ctx.get_mut_response()
            .set_version(HttpVersion::Http1_1)
            .set_status_code(500)
            .clear_headers()
            .set_header(SERVER, HYPERLANE)
            .set_header(CONTENT_TYPE, &self.content_type)
            .set_body(&self.response_body);
        if ctx.try_send().await.is_err() {
            ctx.set_aborted(true).set_closed(true);
        }
    }
}

struct RequestErrorHook {
    response_status_code: ResponseStatusCode,
    response_body: String,
}

impl ServerHook for RequestErrorHook {
    async fn new(ctx: &mut Context) -> Self {
        let request_error: RequestError = ctx.try_get_request_error_data().unwrap_or_default();
        Self {
            response_status_code: request_error.get_http_status_code(),
            response_body: request_error.to_string(),
        }
    }

    async fn handle(self, ctx: &mut Context) {
        ctx.get_mut_response()
            .set_version(HttpVersion::Http1_1)
            .set_status_code(self.response_status_code)
            .set_body(self.response_body);
        if ctx.try_send().await.is_err() {
            ctx.set_aborted(true).set_closed(true);
        }
    }
}

struct RequestMiddleware {
    socket_addr: String,
}

impl ServerHook for RequestMiddleware {
    async fn new(ctx: &mut Context) -> Self {
        let socket_addr: String = ctx.get_socket_addr_string().await;
        Self { socket_addr }
    }

    async fn handle(self, ctx: &mut Context) {
        ctx.get_mut_response()
            .set_version(HttpVersion::Http1_1)
            .set_status_code(200)
            .set_header(SERVER, HYPERLANE)
            .set_header(CONNECTION, KEEP_ALIVE)
            .set_header(CONTENT_TYPE, TEXT_PLAIN)
            .set_header(ACCESS_CONTROL_ALLOW_ORIGIN, WILDCARD_ANY)
            .set_header("SocketAddr", &self.socket_addr);
    }
}

struct UpgradeMiddleware;

impl ServerHook for UpgradeMiddleware {
    async fn new(_ctx: &mut Context) -> Self {
        Self
    }

    async fn handle(self, ctx: &mut Context) {
        if !ctx.get_request().is_ws_upgrade_type() {
            return;
        }
        if let Some(key) = &ctx.get_request().try_get_header_back(SEC_WEBSOCKET_KEY) {
            let accept_key: String = WebSocketFrame::generate_accept_key(key);
            ctx.get_mut_response()
                .set_version(HttpVersion::Http1_1)
                .set_status_code(101)
                .set_header(UPGRADE, WEBSOCKET)
                .set_header(CONNECTION, UPGRADE)
                .set_header(SEC_WEBSOCKET_ACCEPT, &accept_key)
                .set_body(vec![]);
            if ctx.try_send().await.is_err() {
                ctx.set_aborted(true).set_closed(true);
            }
        }
    }
}

struct ResponseMiddleware;

impl ServerHook for ResponseMiddleware {
    async fn new(_ctx: &mut Context) -> Self {
        Self
    }

    async fn handle(self, ctx: &mut Context) {
        if ctx.get_request().is_ws_upgrade_type() {
            return;
        }
        if ctx.try_send().await.is_err() {
            ctx.set_aborted(true).set_closed(true);
        }
    }
}

struct RootRoute {
    response_body: String,
    cookie1: String,
    cookie2: String,
}

impl ServerHook for RootRoute {
    async fn new(ctx: &mut Context) -> Self {
        let response_body: String = format!("Hello hyperlane => {}", ctx.get_request().get_path());
        let cookie1: String = CookieBuilder::new("key1", "value1").http_only().build();
        let cookie2: String = CookieBuilder::new("key2", "value2").http_only().build();
        Self {
            response_body,
            cookie1,
            cookie2,
        }
    }

    async fn handle(self, ctx: &mut Context) {
        ctx.get_mut_response()
            .add_header(SET_COOKIE, &self.cookie1)
            .add_header(SET_COOKIE, &self.cookie2)
            .set_body(&self.response_body);
    }
}

struct SseRoute;

impl ServerHook for SseRoute {
    async fn new(_ctx: &mut Context) -> Self {
        Self
    }

    async fn handle(self, ctx: &mut Context) {
        ctx.get_mut_response()
            .set_header(CONTENT_TYPE, TEXT_EVENT_STREAM)
            .set_body(vec![]);
        if ctx.try_send().await.is_err() {
            ctx.set_aborted(true).set_closed(true);
            return;
        }
        for i in 0..10 {
            ctx.get_mut_response()
                .set_body(format!("data:{i}{HTTP_DOUBLE_BR}"));
            if ctx.try_send_body().await.is_err() {
                ctx.set_aborted(true).set_closed(true);
                return;
            }
        }
        ctx.set_aborted(true).set_closed(true);
    }
}

struct WebsocketRoute;

impl WebsocketRoute {
    async fn try_send_body_hook(&self, ctx: &mut Context) -> Result<(), ResponseError> {
        let send_result: Result<(), ResponseError> = if ctx.get_request().is_ws_upgrade_type() {
            let body: &ResponseBody = ctx.get_response().get_body();
            let frame_list: Vec<ResponseBody> = WebSocketFrame::create_frame_list(body);
            ctx.try_send_body_list_with_data(&frame_list).await
        } else {
            ctx.try_send_body().await
        };
        if send_result.is_err() {
            ctx.set_aborted(true).set_closed(true);
        }
        send_result
    }
}

impl ServerHook for WebsocketRoute {
    async fn new(_ctx: &mut Context) -> Self {
        Self
    }

    async fn handle(self, ctx: &mut Context) {
        loop {
            match ctx.ws_from_stream().await {
                Ok(_) => {
                    let body: Vec<u8> = ctx.get_request().get_body().clone();
                    ctx.get_mut_response().set_body(body);
                    if self.try_send_body_hook(ctx).await.is_err() {
                        return;
                    }
                }
                Err(error) => {
                    ctx.get_mut_response().set_body(error.to_string());
                    let _ = self.try_send_body_hook(ctx).await;
                    return;
                }
            }
        }
    }
}

struct DynamicRoute {
    params: RouteParams,
}

impl ServerHook for DynamicRoute {
    async fn new(ctx: &mut Context) -> Self {
        Self {
            params: ctx.get_route_params().clone(),
        }
    }

    async fn handle(mut self, _ctx: &mut Context) {
        self.params.insert("key".to_owned(), "value".to_owned());
        panic!("Test panic {:?}", self.params);
    }
}

struct GetAllRoutes;

impl ServerHook for GetAllRoutes {
    async fn new(_ctx: &mut Context) -> Self {
        Self
    }

    async fn handle(self, ctx: &mut Context) {
        let route_matcher: RouteMatcher = ctx.get_server().get_route_matcher().clone();
        let mut response_body: String = String::new();
        for key in route_matcher.get_static_route().keys() {
            response_body.push_str(&format!("Static route: {key}\n"));
        }
        for value in route_matcher.get_dynamic_route().values() {
            for (route_pattern, _) in value {
                response_body.push_str(&format!("Dynamic route: {route_pattern}\n"));
            }
        }
        for value in route_matcher.get_regex_route().values() {
            for (route_pattern, _) in value {
                response_body.push_str(&format!("Regex route: {route_pattern}\n"));
            }
        }
        ctx.get_mut_response().set_body(&response_body);
    }
}

#[tokio::test]
async fn main() {
    let mut server: Server = Server::default();
    server.task_panic::<TaskPanicHook>();
    server.request_error::<RequestErrorHook>();
    server.request_middleware::<RequestMiddleware>();
    server.request_middleware::<UpgradeMiddleware>();
    server.response_middleware::<ResponseMiddleware>();
    server.route::<RootRoute>("/");
    server.route::<SseRoute>("/sse");
    server.route::<WebsocketRoute>("/websocket");
    server.route::<GetAllRoutes>("/get/all/routes");
    server.route::<DynamicRoute>("/dynamic/{routing}");
    server.route::<DynamicRoute>("/regex/{file:^.*$}");
    let server_control_hook_1: ServerControlHook = server.run().await.unwrap_or_default();
    let server_control_hook_2: ServerControlHook = server_control_hook_1.clone();
    tokio::spawn(async move {
        tokio::time::sleep(std::time::Duration::from_secs(60)).await;
        server_control_hook_2.shutdown().await;
    });
    server_control_hook_1.wait().await;
}
