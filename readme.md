<center>

## hyperlane

<img src="./img/logo.png" alt="" height="160">

[![](https://img.shields.io/crates/v/hyperlane.svg)](https://crates.io/crates/hyperlane)
[![](https://docs.rs/hyperlane/badge.svg)](https://docs.rs/hyperlane)
[![](https://github.com/ltpp-universe/hyperlane/workflows/Rust/badge.svg)](https://github.com/ltpp-universe/hyperlane/actions?query=workflow:Rust)
[![](https://img.shields.io/crates/l/hyperlane.svg)](./LICENSE)

</center>

[Official Documentation](https://docs.ltpp.vip/hyperlane/)

[Api Docs](https://docs.rs/hyperlane/latest/hyperlane/)

> hyperlane is a lightweight and high-performance Rust HTTP server library designed to simplify network service development. It supports HTTP request parsing, response building, TCP communication, and redirection features, making it ideal for building modern web services.

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

async fn test_middleware(arc_lock_controller_data: ArcRwLockControllerData) {
    let mut controller_data: RwLockWriteControllerData =
        get_rw_lock_write_controller_data(&arc_lock_controller_data).await;
    let response: &mut Response = controller_data.get_mut_response();
    let socket_addr: String = get_socket_addr(&arc_lock_controller_data)
        .await
        .unwrap_or_default();
    response
        .set_header(SERVER, "hyperlane")
        .set_header(CONNECTION, CONNECTION_KEEP_ALIVE)
        .set_header("SocketAddr", socket_addr);
}

async fn root_router(arc_lock_controller_data: ArcRwLockControllerData) {
    let send_res: ResponseResult =
        send_response(&arc_lock_controller_data, 200, "hello hyperlane => /").await;
    let controller_data: ControllerData = get_controller_data(&arc_lock_controller_data).await;
    controller_data.get_log().info(
        format!("Response result => {:?}", send_res),
        log_debug_format_handler,
    );
}

async fn panic_route(_controller_data: ArcRwLock<ControllerData>) {
    panic!("test panic");
}

async fn run_server() {
    let mut server: Server = Server::new();
    server.host("0.0.0.0").await;
    server.port(60000).await;
    server.log_dir("./logs").await;
    server.log_size(100_024_000).await;
    server.log_interval_millis(1000).await;
    server.middleware(test_middleware).await;
    server.router("/", root_router).await;
    server.router("/panic", panic_route).await;
    let test_string: String = "test".to_owned();
    server
        .router(
            "/test/func",
            async_func!(test_string, |data| {
                println_success!(test_string);
                println_success!(format!("{:?}", data));
            }),
        )
        .await;
    server.listen().await;
}
```

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please open an issue or submit a pull request.

## Contact

For any inquiries, please reach out to the author at [ltpp-universe <root@ltpp.vip>](mailto:root@ltpp.vip).
