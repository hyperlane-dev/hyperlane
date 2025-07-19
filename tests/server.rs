use hyperlane::*;

#[tokio::test]
async fn test_server() {
    async fn connected_hook(ctx: Context) {
        if !ctx.get_request().await.is_ws() {
            return;
        }
        let socket_addr: String = ctx.get_socket_addr_or_default_string().await;
        let _ = ctx.set_response_body(socket_addr).await.send_body().await;
    }

    async fn request_middleware(ctx: Context) {
        let socket_addr: String = ctx.get_socket_addr_or_default_string().await;
        ctx.set_response_header(SERVER, HYPERLANE)
            .await
            .set_response_header(CONNECTION, KEEP_ALIVE)
            .await
            .set_response_header(CONTENT_TYPE, TEXT_PLAIN)
            .await
            .set_response_header("SocketAddr", socket_addr)
            .await
            .set_response_version(HttpVersion::HTTP1_1)
            .await;
    }

    async fn response_middleware(ctx: Context) {
        let _ = ctx.send().await;
    }

    async fn root_route(ctx: Context) {
        let cookie1: String = CookieBuilder::new("key1", "key2").http_only().build();
        let cookie2: String = CookieBuilder::new("key2", "key2").http_only().build();
        ctx.set_response_status_code(200)
            .await
            .set_response_header(SET_COOKIE, cookie1)
            .await
            .set_response_header(SET_COOKIE, cookie2)
            .await
            .set_response_body("Hello hyperlane => /")
            .await;
    }

    async fn ws_route(ctx: Context) {
        let key: RequestHeadersValueItem = ctx
            .get_request_header_back(SEC_WEBSOCKET_KEY)
            .await
            .unwrap();
        let request_body: Vec<u8> = ctx.get_request_body().await;
        let _ = ctx.set_response_body(key).await.send_body().await;
        let _ = ctx.set_response_body(request_body).await.send_body().await;
    }

    async fn sse_route(ctx: Context) {
        let _ = ctx
            .set_response_header(CONTENT_TYPE, TEXT_EVENT_STREAM)
            .await
            .set_response_status_code(200)
            .await
            .send()
            .await;
        for i in 0..10 {
            let _ = ctx
                .set_response_body(format!("data:{}{}", i, HTTP_DOUBLE_BR))
                .await
                .send_body()
                .await;
        }
        let _ = ctx.closed().await;
    }

    async fn dynamic_route(ctx: Context) {
        let param: RouteParams = ctx.get_route_params().await;
        panic!("Test panic {:?}", param);
    }

    async fn main() {
        Server::new()
            .host("0.0.0.0")
            .port(60000)
            .enable_nodelay()
            .disable_linger()
            .http_buffer(4096)
            .ws_buffer(4096)
            .connected_hook(connected_hook)
            .pre_upgrade_hook(request_middleware)
            .request_middleware(request_middleware)
            .response_middleware(response_middleware)
            .route("/", root_route)
            .route("/ws", ws_route)
            .route("/sse", sse_route)
            .route("/dynamic/{routing}", dynamic_route)
            .route("/dynamic/routing/{file:^.*$}", dynamic_route)
            .run()
            .await
            .unwrap();
    }

    let _ = tokio::time::timeout(std::time::Duration::from_secs(60), main()).await;
}
