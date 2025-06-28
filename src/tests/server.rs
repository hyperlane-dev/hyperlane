use crate::*;

#[tokio::test]
async fn test_server() {
    async fn async_error_handler(error: String) {
        eprintln!("Async error: {}", error);
        let _ = std::io::Write::flush(&mut std::io::stderr());
    }

    async fn on_ws_connected(ctx: Context) {
        let _ = ctx.set_response_body("connected").await.send_body().await;
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
            .await;
    }

    async fn response_middleware(ctx: Context) {
        let _ = ctx.send().await;
    }

    async fn root_route(ctx: Context) {
        ctx.set_response_status_code(200)
            .await
            .set_response_body("Hello hyperlane => /")
            .await;
    }

    async fn ws_route(ctx: Context) {
        let key: String = ctx.get_request_header(SEC_WEBSOCKET_KEY).await.unwrap();
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
        let server: Server = Server::new();
        server.host("0.0.0.0").await;
        server.port(60000).await;
        server.enable_nodelay().await;
        server.disable_linger().await;
        server.http_buffer_size(4096).await;
        server.ws_buffer_size(4096).await;
        server.error_handler(async_error_handler).await;
        server.on_ws_connected(on_ws_connected).await;
        server.pre_ws_upgrade(request_middleware).await;
        server.request_middleware(request_middleware).await;
        server.response_middleware(response_middleware).await;
        server.route("/", root_route).await;
        server.route("/ws", ws_route).await;
        server.route("/sse", sse_route).await;
        server.route("/dynamic/{routing}", dynamic_route).await;
        server
            .route("/dynamic/routing/{file:^.*$}", dynamic_route)
            .await;
        server.run().await.unwrap();
    }

    let _ = tokio::time::timeout(std::time::Duration::from_secs(60), main()).await;
}
