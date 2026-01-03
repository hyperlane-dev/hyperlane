<center>

## hyperlane

<img src="https://docs.ltpp.vip/img/hyperlane.png" alt="" height="160">

[![](https://img.shields.io/crates/v/hyperlane.svg)](https://crates.io/crates/hyperlane)
[![](https://img.shields.io/crates/d/hyperlane.svg)](https://img.shields.io/crates/d/hyperlane.svg)
[![](https://docs.rs/hyperlane/badge.svg)](https://docs.rs/hyperlane)
[![](https://github.com/hyperlane-dev/hyperlane/workflows/Rust/badge.svg)](https://github.com/hyperlane-dev/hyperlane/actions?query=workflow:Rust)
[![](https://img.shields.io/crates/l/hyperlane.svg)](./LICENSE)

</center>

[Official Documentation](https://docs.ltpp.vip/hyperlane/)

[Api Docs](https://docs.rs/hyperlane/latest/hyperlane/)

> A lightweight, high-performance, and cross-platform Rust HTTP server library built on Tokio. It simplifies modern web service development by providing built-in support for middleware, WebSocket, Server-Sent Events (SSE), and raw TCP communication. With a unified and ergonomic API across Windows, Linux, and MacOS, it enables developers to build robust, scalable, and event-driven network applications with minimal overhead and maximum flexibility.

## Installation

To use this crate, you can run cmd:

```shell
cargo add hyperlane
```

## Quick start

- [hyperlane-quick-start git](https://github.com/hyperlane-dev/hyperlane-quick-start)
- [hyperlane-quick-start docs](https://docs.ltpp.vip/hyperlane/quick-start/)

```sh
git clone https://github.com/hyperlane-dev/hyperlane-quick-start.git
```

## Use

```rust
use hyperlane::*;

struct ServerPanic {
    response_body: String,
    content_type: String,
}

impl ServerHook for ServerPanic {
    async fn new(ctx: &Context) -> Self {
        let error: PanicData = ctx.try_get_panic_data().await.unwrap_or_default();
        let response_body: String = error.to_string();
        let content_type: String = ContentType::format_content_type_with_charset(TEXT_PLAIN, UTF8);
        Self {
            response_body,
            content_type,
        }
    }

    async fn handle(self, ctx: &Context) {
        ctx.set_response_version(HttpVersion::Http1_1)
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
            .send()
            .await;
    }
}

struct ServerRequestError {
    response_status_code: ResponseStatusCode,
    response_body: String,
}

impl ServerHook for ServerRequestError {
    async fn new(ctx: &Context) -> Self {
        let request_error: RequestError =
            ctx.try_get_request_error_data().await.unwrap_or_default();
        Self {
            response_status_code: request_error.get_http_status_code(),
            response_body: request_error.to_string(),
        }
    }

    async fn handle(self, ctx: &Context) {
        ctx.set_response_version(HttpVersion::Http1_1)
            .await
            .set_response_status_code(self.response_status_code)
            .await
            .set_response_body(self.response_body)
            .await
            .send()
            .await;
    }
}

struct SendBodyMiddleware {
    socket_addr: String,
}

impl ServerHook for SendBodyMiddleware {
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
        if !ctx.get_request().await.is_ws() {
            return;
        }
        if let Some(key) = &ctx.try_get_request_header_back(SEC_WEBSOCKET_KEY).await {
            let accept_key: String = WebSocketFrame::generate_accept_key(key);
            ctx.set_response_version(HttpVersion::Http1_1)
                .await
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
                .await;
        }
    }
}

struct ResponseMiddleware;

impl ServerHook for ResponseMiddleware {
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    async fn handle(self, ctx: &Context) {
        if ctx.get_request().await.is_ws() {
            return;
        }
        ctx.send().await;
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

struct WebsocketRoute;

impl WebsocketRoute {
    async fn send_body_hook(&self, ctx: &Context) {
        if ctx.get_request().await.is_ws() {
            let body: ResponseBody = ctx.get_response_body().await;
            let frame_list: Vec<ResponseBody> = WebSocketFrame::create_frame_list(&body);
            ctx.send_body_list_with_data(&frame_list).await;
        } else {
            ctx.send_body().await;
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
                    let request_body: Vec<u8> = ctx.get_request_body().await;
                    ctx.set_response_body(&request_body).await;
                    self.send_body_hook(ctx).await;
                    continue;
                }
                Err(error) => {
                    ctx.set_response_body(&error.to_string()).await;
                    self.send_body_hook(ctx).await;
                    return;
                }
            }
        }
    }
}

struct SseRoute;

impl ServerHook for SseRoute {
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    async fn handle(self, ctx: &Context) {
        ctx.set_response_header(CONTENT_TYPE, TEXT_EVENT_STREAM)
            .await
            .send()
            .await;
        for i in 0..10 {
            ctx.set_response_body(&format!("data:{}{}", i, HTTP_DOUBLE_BR))
                .await
                .send_body()
                .await;
        }
        ctx.closed().await;
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

#[tokio::main]
async fn main() {
    let server: Server = Server::new().await;
    server.panic::<ServerPanic>().await;
    server.request_error::<ServerRequestError>().await;
    server.request_middleware::<SendBodyMiddleware>().await;
    server.request_middleware::<UpgradeMiddleware>().await;
    server.response_middleware::<ResponseMiddleware>().await;
    server.route::<RootRoute>("/").await;
    server.route::<WebsocketRoute>("/websocket").await;
    server.route::<SseRoute>("/sse").await;
    server.route::<DynamicRoute>("/dynamic/{routing}").await;
    server.route::<DynamicRoute>("/regex/{file:^.*$}").await;
    let server_control_hook: ServerControlHook = server.run().await.unwrap_or_default();
    server_control_hook.wait().await;
}
```

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please open an issue or submit a pull request.

## Contact

For any inquiries, please reach out to the author at [root@ltpp.vip](mailto:root@ltpp.vip).
