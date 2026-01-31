use crate::*;

#[tokio::test]
async fn server_partial_eq() {
    let server1: Server = Server::new().await;
    let server2: Server = Server::new().await;
    assert_eq!(server1, server2);
    let server1_clone: Server = server1.clone();
    assert_eq!(server1, server1_clone);
}

#[tokio::test]
async fn server_inner_partial_eq() {
    let inner1: ServerData = ServerData::default();
    let inner2: ServerData = ServerData::default();
    assert_eq!(inner1, inner2);
}

struct TestSendRoute;

impl ServerHook for TestSendRoute {
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    async fn handle(self, _ctx: &Context) {}
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
    let server: Server = Server::new()
        .await
        .route::<TestSendRoute>("/test")
        .await
        .clone();
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
    let server: Arc<Server> = Arc::new(
        Server::new()
            .await
            .route::<TestSendRoute>("/test")
            .await
            .clone(),
    );
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
    async fn new(ctx: &Context) -> Self {
        let error: PanicData = ctx.try_get_task_panic_data().await.unwrap_or_default();
        let response_body: String = error.to_string();
        let content_type: String = ContentType::format_content_type_with_charset(TEXT_PLAIN, UTF8);
        Self {
            response_body,
            content_type,
        }
    }

    async fn handle(self, ctx: &Context) {
        let send_result: Result<(), ResponseError> = ctx
            .set_response_version(HttpVersion::Http1_1)
            .await
            .set_response_status_code(500)
            .await
            .clear_response_headers()
            .await
            .set_response_header(SERVER, HYPERLANE)
            .await
            .set_response_header(CONTENT_TYPE, &self.content_type)
            .await
            .set_response_body(&self.response_body)
            .await
            .try_send()
            .await;
        if send_result.is_err() {
            ctx.aborted().await.closed().await;
        }
    }
}

struct RequestErrorHook {
    response_status_code: ResponseStatusCode,
    response_body: String,
}

impl ServerHook for RequestErrorHook {
    async fn new(ctx: &Context) -> Self {
        let request_error: RequestError =
            ctx.try_get_request_error_data().await.unwrap_or_default();
        Self {
            response_status_code: request_error.get_http_status_code(),
            response_body: request_error.to_string(),
        }
    }

    async fn handle(self, ctx: &Context) {
        let send_result: Result<(), ResponseError> = ctx
            .set_response_version(HttpVersion::Http1_1)
            .await
            .set_response_status_code(self.response_status_code)
            .await
            .set_response_body(self.response_body)
            .await
            .try_send()
            .await;
        if send_result.is_err() {
            ctx.aborted().await.closed().await;
        }
    }
}

struct RequestMiddleware {
    socket_addr: String,
}

impl ServerHook for RequestMiddleware {
    async fn new(ctx: &Context) -> Self {
        let socket_addr: String = ctx.get_socket_addr_string().await;
        Self { socket_addr }
    }

    async fn handle(self, ctx: &Context) {
        ctx.set_response_version(HttpVersion::Http1_1)
            .await
            .set_response_status_code(200)
            .await
            .set_response_header(SERVER, HYPERLANE)
            .await
            .set_response_header(CONNECTION, KEEP_ALIVE)
            .await
            .set_response_header(CONTENT_TYPE, TEXT_PLAIN)
            .await
            .set_response_header(ACCESS_CONTROL_ALLOW_ORIGIN, WILDCARD_ANY)
            .await
            .set_response_header("SocketAddr", &self.socket_addr)
            .await;
    }
}

struct UpgradeMiddleware;

impl ServerHook for UpgradeMiddleware {
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    async fn handle(self, ctx: &Context) {
        if !ctx.get_request_is_ws_upgrade_type().await {
            return;
        }
        if let Some(key) = &ctx.try_get_request_header_back(SEC_WEBSOCKET_KEY).await {
            let accept_key: String = WebSocketFrame::generate_accept_key(key);
            let send_result: Result<(), ResponseError> = ctx
                .set_response_version(HttpVersion::Http1_1)
                .await
                .set_response_status_code(101)
                .await
                .set_response_header(UPGRADE, WEBSOCKET_LOWERCASE)
                .await
                .set_response_header(CONNECTION, UPGRADE)
                .await
                .set_response_header(SEC_WEBSOCKET_ACCEPT, &accept_key)
                .await
                .set_response_body(&vec![])
                .await
                .try_send()
                .await;
            if send_result.is_err() {
                ctx.aborted().await.closed().await;
            }
        }
    }
}

struct ResponseMiddleware;

impl ServerHook for ResponseMiddleware {
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    async fn handle(self, ctx: &Context) {
        if ctx.get_request_is_ws_upgrade_type().await {
            return;
        }
        let send_result: Result<(), ResponseError> = ctx.try_send().await;
        if send_result.is_err() {
            ctx.aborted().await.closed().await;
        }
    }
}

struct RootRoute {
    response_body: String,
    cookie1: String,
    cookie2: String,
}

impl ServerHook for RootRoute {
    async fn new(ctx: &Context) -> Self {
        let path: RequestPath = ctx.get_request_path().await;
        let response_body: String = format!("Hello hyperlane => {}", path);
        let cookie1: String = CookieBuilder::new("key1", "value1").http_only().build();
        let cookie2: String = CookieBuilder::new("key2", "value2").http_only().build();
        Self {
            response_body,
            cookie1,
            cookie2,
        }
    }

    async fn handle(self, ctx: &Context) {
        ctx.add_response_header(SET_COOKIE, &self.cookie1)
            .await
            .add_response_header(SET_COOKIE, &self.cookie2)
            .await
            .set_response_body(&self.response_body)
            .await;
    }
}

struct SseRoute;

impl ServerHook for SseRoute {
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    async fn handle(self, ctx: &Context) {
        let send_result: Result<(), ResponseError> = ctx
            .set_response_header(CONTENT_TYPE, TEXT_EVENT_STREAM)
            .await
            .try_send()
            .await;
        if send_result.is_err() {
            ctx.aborted().await.closed().await;
            return;
        }
        for i in 0..10 {
            let send_result: Result<(), ResponseError> = ctx
                .set_response_body(&format!("data:{}{}", i, HTTP_DOUBLE_BR))
                .await
                .try_send_body()
                .await;
            if send_result.is_err() {
                ctx.aborted().await.closed().await;
                return;
            }
        }
        ctx.closed().await;
    }
}

struct WebsocketRoute;

impl WebsocketRoute {
    async fn try_send_body_hook(&self, ctx: &Context) -> Result<(), ResponseError> {
        let send_result: Result<(), ResponseError> = if ctx.get_request_is_ws_upgrade_type().await {
            let body: ResponseBody = ctx.get_response_body().await;
            let frame_list: Vec<ResponseBody> = WebSocketFrame::create_frame_list(&body);
            ctx.try_send_body_list_with_data(&frame_list).await
        } else {
            ctx.try_send_body().await
        };
        if send_result.is_err() {
            ctx.aborted().await.closed().await;
        }
        send_result
    }
}

impl ServerHook for WebsocketRoute {
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    async fn handle(self, ctx: &Context) {
        loop {
            match ctx.ws_from_stream(&RequestConfigData::default()).await {
                Ok(_) => {
                    let request_body: Vec<u8> = ctx.get_request_body().await;
                    ctx.set_response_body(&request_body).await;
                    if self.try_send_body_hook(ctx).await.is_err() {
                        return;
                    }
                }
                Err(error) => {
                    ctx.set_response_body(&error.to_string()).await;
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
    async fn new(ctx: &Context) -> Self {
        Self {
            params: ctx.get_route_params().await,
        }
    }

    async fn handle(mut self, _ctx: &Context) {
        self.params.insert("key".to_owned(), "value".to_owned());
        panic!("Test panic {:?}", self.params);
    }
}

#[tokio::test]
async fn main() {
    let server: Server = Server::new().await;
    server.task_panic::<TaskPanicHook>().await;
    server.request_error::<RequestErrorHook>().await;
    server.request_middleware::<RequestMiddleware>().await;
    server.request_middleware::<UpgradeMiddleware>().await;
    server.response_middleware::<ResponseMiddleware>().await;
    server.route::<RootRoute>("/").await;
    server.route::<SseRoute>("/sse").await;
    server.route::<WebsocketRoute>("/websocket").await;
    server.route::<DynamicRoute>("/dynamic/{routing}").await;
    server.route::<DynamicRoute>("/regex/{file:^.*$}").await;
    let server_control_hook_1: ServerControlHook = server.run().await.unwrap_or_default();
    let server_control_hook_2: ServerControlHook = server_control_hook_1.clone();
    tokio::spawn(async move {
        tokio::time::sleep(std::time::Duration::from_secs(60)).await;
        server_control_hook_2.shutdown().await;
    });
    server_control_hook_1.wait().await;
}
