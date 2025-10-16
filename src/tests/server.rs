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
    let inner1: ServerInner = ServerInner::default();
    let inner2: ServerInner = ServerInner::default();
    assert_eq!(inner1, inner2);
}

struct SendBodyMiddleware {
    ctx: Context,
}

impl Middleware for SendBodyMiddleware {
    type Prev = DefaultInitialHook;

    async fn new(prev: &Self::Prev) -> Self {
        Self {
            ctx: prev.context.clone(),
        }
    }

    async fn handle(self) {
        let socket_addr: String = self.ctx.get_socket_addr_string().await;
        self.ctx
            .set_response_version(HttpVersion::HTTP1_1)
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
            .set_response_header("SocketAddr", &socket_addr)
            .await;
    }
}

struct UpgradeMiddleware {
    ctx: Context,
}

impl Middleware for UpgradeMiddleware {
    type Prev = DefaultInitialHook;

    async fn new(prev: &Self::Prev) -> Self {
        Self {
            ctx: prev.context.clone(),
        }
    }

    async fn handle(self) {
        if !self.ctx.get_request().await.is_ws() {
            return;
        }
        if let Some(key) = &self
            .ctx
            .try_get_request_header_back(SEC_WEBSOCKET_KEY)
            .await
        {
            let accept_key: String = WebSocketFrame::generate_accept_key(key);
            self.ctx
                .set_response_status_code(101)
                .await
                .set_response_header(UPGRADE, WEBSOCKET)
                .await
                .set_response_header(CONNECTION, UPGRADE)
                .await
                .set_response_header(SEC_WEBSOCKET_ACCEPT, &accept_key)
                .await
                .set_response_body(&vec![])
                .await
                .send()
                .await
                .unwrap();
        }
    }
}

struct ResponseMiddleware {
    ctx: Context,
}

impl Middleware for ResponseMiddleware {
    type Prev = DefaultInitialHook;

    async fn new(prev: &Self::Prev) -> Self {
        Self {
            ctx: prev.context.clone(),
        }
    }

    async fn handle(self) {
        if self.ctx.get_request().await.is_ws() {
            return;
        }
        let _ = self.ctx.send().await;
    }
}

struct RootRoute {
    ctx: Context,
}

impl Route for RootRoute {
    type Prev = DefaultInitialHook;

    async fn new(prev: &Self::Prev) -> Self {
        Self {
            ctx: prev.context.clone(),
        }
    }

    async fn handle(self) {
        let path: RequestPath = self.ctx.get_request_path().await;
        let response_body: String = format!("Hello hyperlane => {}", path);
        let cookie1: String = CookieBuilder::new("key1", "value1").http_only().build();
        let cookie2: String = CookieBuilder::new("key2", "value2").http_only().build();
        self.ctx
            .add_response_header(SET_COOKIE, &cookie1)
            .await
            .add_response_header(SET_COOKIE, &cookie2)
            .await
            .set_response_body(&response_body)
            .await;
    }
}

struct WsRoute {
    ctx: Context,
}

impl WsRoute {
    async fn send_body_hook(&self) {
        let body: ResponseBody = self.ctx.get_response_body().await;
        if self.ctx.get_request().await.is_ws() {
            let frame_list: Vec<ResponseBody> = WebSocketFrame::create_frame_list(&body);
            self.ctx
                .send_body_list_with_data(&frame_list)
                .await
                .unwrap();
        } else {
            self.ctx.send_body().await.unwrap();
        }
    }
}

impl Route for WsRoute {
    type Prev = DefaultInitialHook;

    async fn new(prev: &Self::Prev) -> Self {
        Self {
            ctx: prev.context.clone(),
        }
    }

    async fn handle(self) {
        while self.ctx.ws_from_stream(4096).await.is_ok() {
            let request_body: Vec<u8> = self.ctx.get_request_body().await;
            self.ctx.set_response_body(&request_body).await;
            self.send_body_hook().await;
        }
    }
}

struct SseRoute {
    ctx: Context,
}

impl Route for SseRoute {
    type Prev = DefaultInitialHook;

    async fn new(prev: &Self::Prev) -> Self {
        Self {
            ctx: prev.context.clone(),
        }
    }

    async fn handle(self) {
        let _ = self
            .ctx
            .set_response_header(CONTENT_TYPE, TEXT_EVENT_STREAM)
            .await
            .send()
            .await;
        for i in 0..10 {
            let _ = self
                .ctx
                .set_response_body(&format!("data:{}{}", i, HTTP_DOUBLE_BR))
                .await
                .send_body()
                .await;
        }
        let _ = self.ctx.closed().await;
    }
}

struct DynamicRoute {
    ctx: Context,
}

impl Route for DynamicRoute {
    type Prev = DefaultInitialHook;

    async fn new(prev: &Self::Prev) -> Self {
        Self {
            ctx: prev.context.clone(),
        }
    }

    async fn handle(self) {
        let param: RouteParams = self.ctx.get_route_params().await;
        panic!("Test panic {:?}", param);
    }
}

struct PanicHooks {
    ctx: Context,
}

impl PanicHook for PanicHooks {
    type Prev = DefaultInitialHook;

    async fn new(prev: &Self::Prev) -> Self {
        Self {
            ctx: prev.context.clone(),
        }
    }

    async fn handle(self) {
        let error: Panic = self.ctx.try_get_panic().await.unwrap_or_default();
        let response_body: String = error.to_string();
        let content_type: String = ContentType::format_content_type_with_charset(TEXT_PLAIN, UTF8);
        let _ = self
            .ctx
            .set_response_status_code(500)
            .await
            .clear_response_headers()
            .await
            .set_response_header(SERVER, HYPERLANE)
            .await
            .set_response_header(CONTENT_TYPE, &content_type)
            .await
            .set_response_body(&response_body)
            .await
            .send()
            .await;
    }
}

#[tokio::test]

async fn main() {
    let config: ServerConfig = ServerConfig::new().await;
    config.host("0.0.0.0").await;
    config.port(60000).await;
    config.buffer(4096).await;
    config.disable_linger().await;
    config.disable_nodelay().await;
    let server: Server = Server::from(config).await;
    server.request_middleware::<SendBodyMiddleware>().await;
    server.request_middleware::<UpgradeMiddleware>().await;
    server.response_middleware::<ResponseMiddleware>().await;
    server.panic_hook::<PanicHooks>().await;
    server.route::<RootRoute>("/").await;
    server.route::<WsRoute>("/ws").await;
    server.route::<SseRoute>("/sse").await;
    server.route::<DynamicRoute>("/dynamic/{routing}").await;
    server.route::<DynamicRoute>("/regex/{file:^.*$}").await;
    let server_hook: ServerHook = server.run().await.unwrap_or_default();
    let server_hook_clone: ServerHook = server_hook.clone();
    tokio::spawn(async move {
        tokio::time::sleep(std::time::Duration::from_secs(60)).await;
        server_hook.shutdown().await;
    });
    server_hook_clone.wait().await;
}
