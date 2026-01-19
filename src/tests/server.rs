use crate::*;

#[tokio::test]
async fn server_partial_eq() {
    let server1: Server = Server::new();
    let server2: Server = Server::new();
    assert_eq!(server1, server2);
    let server1_clone: Server = server1.clone();
    assert_eq!(server1, server1_clone);
}

#[tokio::test]
async fn server_inner_partial_eq() {
    let inner1: ServerInner = ServerInner::default();
    let inner2: ServerInner = ServerInner::default();
    assert_eq!(inner1, inner2);
}

struct TaskPanicHook {
    response_body: String,
    content_type: String,
}

impl ServerHook for TaskPanicHook {
    async fn new(ctx: &Context) -> Self {
        let error: PanicData = ctx.try_get_task_panic_data().unwrap_or_default();
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
            .set_response_status_code(500)
            .clear_response_headers()
            .set_response_header(SERVER, HYPERLANE)
            .set_response_header(CONTENT_TYPE, &self.content_type)
            .set_response_body(&self.response_body)
            .try_send()
            .await;
        if send_result.is_err() {
            ctx.aborted().closed();
        }
    }
}

struct RequestErrorHook {
    response_status_code: ResponseStatusCode,
    response_body: String,
}

impl ServerHook for RequestErrorHook {
    async fn new(ctx: &Context) -> Self {
        let request_error: RequestError = ctx.try_get_request_error_data().unwrap_or_default();
        Self {
            response_status_code: request_error.get_http_status_code(),
            response_body: request_error.to_string(),
        }
    }

    async fn handle(self, ctx: &Context) {
        let send_result: Result<(), ResponseError> = ctx
            .set_response_version(HttpVersion::Http1_1)
            .set_response_status_code(self.response_status_code)
            .set_response_body(self.response_body)
            .try_send()
            .await;
        if send_result.is_err() {
            ctx.aborted().closed();
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
            .set_response_status_code(200)
            .set_response_header(SERVER, HYPERLANE)
            .set_response_header(CONNECTION, KEEP_ALIVE)
            .set_response_header(CONTENT_TYPE, TEXT_PLAIN)
            .set_response_header(ACCESS_CONTROL_ALLOW_ORIGIN, WILDCARD_ANY)
            .set_response_header("SocketAddr", &self.socket_addr);
    }
}

struct UpgradeMiddleware;

impl ServerHook for UpgradeMiddleware {
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    async fn handle(self, ctx: &Context) {
        if !ctx.get_request().is_ws() {
            return;
        }
        if let Some(key) = &ctx.try_get_request_header_back(SEC_WEBSOCKET_KEY) {
            let accept_key: String = WebSocketFrame::generate_accept_key(key);
            let send_result: Result<(), ResponseError> = ctx
                .set_response_version(HttpVersion::Http1_1)
                .set_response_status_code(101)
                .set_response_header(UPGRADE, WEBSOCKET)
                .set_response_header(CONNECTION, UPGRADE)
                .set_response_header(SEC_WEBSOCKET_ACCEPT, &accept_key)
                .set_response_body(vec![])
                .try_send()
                .await;
            if send_result.is_err() {
                ctx.aborted().closed();
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
        if ctx.get_request().is_ws() {
            return;
        }
        let send_result: Result<(), ResponseError> = ctx.try_send().await;
        if send_result.is_err() {
            ctx.aborted().closed();
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
        let path: RequestPath = ctx.get_request_path();
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
            .add_response_header(SET_COOKIE, &self.cookie2)
            .set_response_body(&self.response_body);
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
            .try_send()
            .await;
        if send_result.is_err() {
            ctx.aborted().closed();
        }
        for i in 0..10 {
            let send_result: Result<(), ResponseError> = ctx
                .set_response_body(format!("data:{}{}", i, HTTP_DOUBLE_BR))
                .try_send_body()
                .await;
            if send_result.is_err() {
                ctx.aborted().closed();
                return;
            }
        }
        ctx.closed();
    }
}

struct WebsocketRoute;

impl WebsocketRoute {
    async fn send_body_hook(&self, ctx: &Context) {
        let send_result: Result<(), ResponseError> = if ctx.get_request().is_ws() {
            let body: ResponseBody = ctx.get_response_body();
            let frame_list: Vec<ResponseBody> = WebSocketFrame::create_frame_list(&body);
            ctx.try_send_body_list_with_data(&frame_list).await
        } else {
            ctx.try_send_body().await
        };
        if send_result.is_err() {
            ctx.aborted().closed();
        }
    }
}

impl ServerHook for WebsocketRoute {
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    async fn handle(self, ctx: &Context) {
        loop {
            match ctx.ws_from_stream(RequestConfig::default()).await {
                Ok(_) => {
                    let request_body: Vec<u8> = ctx.get_request_body();
                    ctx.set_response_body(&request_body);
                    self.send_body_hook(ctx).await;
                    continue;
                }
                Err(error) => {
                    ctx.set_response_body(error.to_string());
                    self.send_body_hook(ctx).await;
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
            params: ctx.get_route_params(),
        }
    }

    async fn handle(mut self, _ctx: &Context) {
        self.params.insert("key".to_owned(), "value".to_owned());
        panic!("Test panic {:?}", self.params);
    }
}

#[tokio::test]
async fn main() {
    let server: Server = Server::new();
    server.task_panic::<TaskPanicHook>();
    server.request_error::<RequestErrorHook>();
    server.request_middleware::<RequestMiddleware>();
    server.request_middleware::<UpgradeMiddleware>();
    server.response_middleware::<ResponseMiddleware>();
    server.route::<RootRoute>("/");
    server.route::<SseRoute>("/sse");
    server.route::<WebsocketRoute>("/websocket");
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
