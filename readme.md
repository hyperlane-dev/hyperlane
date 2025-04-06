<center>

## hyperlane

<img src="./img/logo.png" alt="" height="160">

[![](https://img.shields.io/crates/v/hyperlane.svg)](https://crates.io/crates/hyperlane)
[![](https://img.shields.io/crates/d/hyperlane.svg)](https://img.shields.io/crates/d/hyperlane.svg)
[![](https://docs.rs/hyperlane/badge.svg)](https://docs.rs/hyperlane)
[![](https://github.com/ltpp-universe/hyperlane/workflows/Rust/badge.svg)](https://github.com/ltpp-universe/hyperlane/actions?query=workflow:Rust)
[![](https://img.shields.io/crates/l/hyperlane.svg)](./LICENSE)

</center>

[Official Documentation](https://docs.ltpp.vip/hyperlane/)

[Api Docs](https://docs.rs/hyperlane/latest/hyperlane/)

> Hyperlane is a lightweight and high-performance Rust HTTP server library designed to simplify network service development. It supports HTTP request parsing, response building, and TCP communication, making it ideal for building modern web services. Additionally, it provides support for request and response middleware, WebSocket, and Server-Sent Events (SSE), enabling flexible and efficient real-time communication.

## Installation

To use this crate, you can run cmd:

```shell
cargo add hyperlane
```

## Quick start

- [hyperlane-quick-start git](https://github.com/ltpp-universe/hyperlane-quick-start)
- [hyperlane-quick-start docs](https://docs.ltpp.vip/hyperlane/quick-start/)

```sh
git clone https://github.com/ltpp-universe/hyperlane-quick-start.git
```

## Use

```rust
use hyperlane::*;

async fn request_middleware(ctx: Context) {
    let socket_addr: String = ctx.get_socket_addr_or_default_string().await;
    ctx.set_response_header(SERVER, HYPERLANE)
        .await
        .set_response_header(CONNECTION, CONNECTION_KEEP_ALIVE)
        .await
        .set_response_header(CONTENT_TYPE, content_type_charset(TEXT_PLAIN, UTF8))
        .await
        .set_response_header(DATE, current_date_gmt())
        .await
        .set_response_header("SocketAddr", socket_addr)
        .await;
}

async fn response_middleware(ctx: Context) {
    let _ = ctx.send().await;
    let request: String = ctx.get_request_string().await;
    let response: String = ctx.get_response_string().await;
    ctx.log_info(&request, log_handler)
        .await
        .log_info(&response, log_handler)
        .await;
}

async fn root_route(ctx: Context) {
    ctx.set_response_status_code(200)
        .await
        .set_response_body("Hello hyperlane => /")
        .await;
}

async fn websocket_route(ctx: Context) {
    let request_body: Vec<u8> = ctx.get_request_body().await;
    let _ = ctx.send_response_body(request_body).await;
}

#[tokio::main]
async fn main() {
    let server: Server = Server::new();
    server.host("0.0.0.0").await;
    server.port(60000).await;
    server.enable_nodelay().await;
    server.log_dir("./logs").await;
    server.enable_inner_log().await;
    server.enable_inner_print().await;
    server.log_size(100_024_000).await;
    server.http_line_buffer_size(4096).await;
    server.websocket_buffer_size(4096).await;
    server.request_middleware(request_middleware).await;
    server.response_middleware(response_middleware).await;
    server.route("/", root_route).await;
    server.route("/websocket", websocket_route).await;
    let test_string: String = "Hello hyperlane".to_owned();
    server
        .route(
            "/test/:text",
            async_func!(test_string, |ctx| {
                let param: RouteParams = ctx.get_route_params().await;
                print_success!(format!("{:?}", param));
                println_success!(test_string);
                panic!("Test panic\n\ndata: test");
            }),
        )
        .await;
    server.listen().await.unwrap();
}
```

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please open an issue or submit a pull request.

## Contact

For any inquiries, please reach out to the author at [ltpp-universe <root@ltpp.vip>](mailto:root@ltpp.vip).
