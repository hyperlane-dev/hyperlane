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

> A lightweight rust http server with middleware, websocket, sse, and tcp support, built on tokio for cross-platform async networking, hyperlane simplifies modern web service development.

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

async fn connected_hook(ctx: Context) {
    if !ctx.get_request().await.is_ws() {
        return;
    }
    let socket_addr: String = ctx.get_socket_addr_or_default_string().await;
    let _ = ctx.set_response_body(socket_addr).await.send_body().await;
}

async fn request_middleware(ctx: Context) {
    let socket_addr: String = ctx.get_socket_addr_or_default_string().await;
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
        .set_response_header("SocketAddr", socket_addr)
        .await;
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
    ctx.set_response_status_code(200)
        .await
        .set_response_header(SET_COOKIE, cookie1)
        .await
        .set_response_header(SET_COOKIE, cookie2)
        .await
        .set_response_body(response_body)
        .await;
}

async fn ws_route(ctx: Context) {
    let key: RequestHeadersValueItem = ctx
        .get_request_header_back(SEC_WEBSOCKET_KEY)
        .await
        .unwrap_or_default();
    let request_body: Vec<u8> = ctx.get_request_body().await;
    let _ = ctx.set_response_body(key).await.send_body().await;
    let _ = ctx.set_response_body(request_body).await.send_body().await;
}

async fn sse_route(ctx: Context) {
    let _ = ctx
        .replace_response_header(CONTENT_TYPE, TEXT_EVENT_STREAM)
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

#[tokio::main]
async fn main() {
    let server: Server = Server::new();
    server.host("0.0.0.0").await;
    server.port(60000).await;
    server.enable_nodelay().await;
    server.disable_linger().await;
    server.http_buffer(4096).await;
    server.ws_buffer(4096).await;
    server.connected_hook(connected_hook).await;
    server.pre_upgrade_hook(request_middleware).await;
    server.request_middleware(request_middleware).await;
    server.response_middleware(response_middleware).await;
    server.route("/", root_route).await;
    server.route("/ws", ws_route).await;
    server.route("/sse", sse_route).await;
    server.route("/dynamic/{routing}", dynamic_route).await;
    server
        .route("/dynamic/routing/{file:^.*$}", dynamic_route)
        .await;
    let result: ServerResult<()> = server.run().await;
    println!("Server result: {:?}", result);
    let _ = std::io::Write::flush(&mut std::io::stderr());
}
```

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please open an issue or submit a pull request.

## Contact

For any inquiries, please reach out to the author at [root@ltpp.vip](mailto:root@ltpp.vip).
