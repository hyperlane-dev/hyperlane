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

#[tokio::test]
async fn server() {
    async fn send_body_hook(ctx: Context) {
        let body: ResponseBody = ctx.get_request_body().await;
        if ctx.get_request().await.is_ws() {
            let body_list: Vec<ResponseBody> = WebSocketFrame::create_frame_list(body);
            ctx.send_body_list_with_data(&body_list).await.unwrap();
        } else {
            ctx.send_body().await.unwrap();
        }
    }

    async fn request_middleware(ctx: Context) {
        ctx.set_send_body_hook(send_body_hook).await;
        let socket_addr: String = ctx.get_socket_addr_string().await;
        ctx.set_response_version(HttpVersion::HTTP1_1)
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

    async fn upgrade_hook(ctx: Context) {
        if !ctx.get_request().await.is_ws() {
            return;
        }
        if let Some(key) = &ctx.try_get_request_header_back(SEC_WEBSOCKET_KEY).await {
            let accept_key: String = WebSocketFrame::generate_accept_key(key);
            ctx.set_response_status_code(101)
                .await
                .set_response_header(UPGRADE, WEBSOCKET)
                .await
                .set_response_header(CONNECTION, UPGRADE)
                .await
                .set_response_header(SEC_WEBSOCKET_ACCEPT, accept_key)
                .await
                .send()
                .await
                .unwrap();
        }
    }

    async fn response_middleware(ctx: Context) {
        if ctx.get_request().await.is_ws() {
            return;
        }
        let _ = ctx.send().await;
    }

    async fn root_route(ctx: Context) {
        let path: RequestPath = ctx.get_request_path().await;
        let response_body: String = format!("Hello hyperlane => {}", path);
        let cookie1: String = CookieBuilder::new("key1", "value1").http_only().build();
        let cookie2: String = CookieBuilder::new("key2", "value2").http_only().build();
        ctx.add_response_header(SET_COOKIE, &cookie1)
            .await
            .add_response_header(SET_COOKIE, &cookie2)
            .await
            .set_response_body(&response_body)
            .await;
    }

    async fn ws_route(ctx: Context) {
        if let Some(send_body_hook) = ctx.try_get_send_body_hook().await {
            while ctx.ws_from_stream(1024).await.is_ok() {
                let request_body: Vec<u8> = ctx.get_request_body().await;
                ctx.set_response_body(&request_body).await;
                send_body_hook(ctx.clone()).await;
            }
        }
    }

    async fn sse_route(ctx: Context) {
        let _ = ctx
            .set_response_header(CONTENT_TYPE, TEXT_EVENT_STREAM)
            .await
            .send()
            .await;
        for i in 0..10 {
            let _ = ctx
                .set_response_body(&format!("data:{}{}", i, HTTP_DOUBLE_BR))
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

    async fn panic_hook(ctx: Context) {
        let error: Panic = ctx.try_get_panic().await.unwrap_or_default();
        let response_body: String = error.to_string();
        let content_type: String = ContentType::format_content_type_with_charset(TEXT_PLAIN, UTF8);
        let _ = ctx
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

    async fn main() {
        let config: ServerConfig = ServerConfig::new().await;
        config.host("0.0.0.0").await;
        config.port(60000).await;
        config.buffer(4096).await;
        config.enable_nodelay().await;
        let server: Server = Server::from(config).await;
        server.panic_hook(panic_hook).await;
        server.request_middleware(request_middleware).await;
        server.request_middleware(upgrade_hook).await;
        server.response_middleware(response_middleware).await;
        server.route("/", root_route).await;
        server.route("/ws", ws_route).await;
        server.route("/sse", sse_route).await;
        server.route("/dynamic/{routing}", dynamic_route).await;
        server.route("/regex/{file:^.*$}", dynamic_route).await;
        let server_hook: ServerHook = server.run().await.unwrap_or_default();
        let server_hook_clone: ServerHook = server_hook.clone();
        tokio::spawn(async move {
            tokio::time::sleep(std::time::Duration::from_secs(60)).await;
            server_hook.shutdown().await;
        });
        server_hook_clone.wait().await;
    }

    main().await;
}
