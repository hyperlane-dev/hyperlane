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

fn sync_middleware(arc_lock_controller_data: ArcRwLockControllerData) {
    let mut controller_data: RwLockWriteControllerData =
        get_rw_lock_write_controller_data(&arc_lock_controller_data);
    let response: &mut Response = controller_data.get_mut_response();
    response.set_header("server", "hyperlane");
}

async fn test_async_middleware(arc_lock_controller_data: ArcRwLockControllerData) {
    sync_middleware(arc_lock_controller_data);
}

fn sync_root_router(arc_lock_controller_data: ArcRwLockControllerData) {
    let send_res: ResponseResult =
        send_response(&arc_lock_controller_data, 200, "hello hyperlane => /index");
    let controller_data: ControllerData = get_controller_data(&arc_lock_controller_data);
    controller_data.get_log().info(
        format!("Response result => {:?}", send_res),
        log_debug_format_handler,
    );
}

fn sync_hello_router(arc_lock_controller_data: ArcRwLockControllerData) {
    let controller_data: ControllerData = get_controller_data(&arc_lock_controller_data);
    controller_data
        .get_log()
        .info("visit path /hello", log_handler);
    let mut response: Response = controller_data.get_response().clone();
    let body: &str = "hello world!";
    let stream: ArcTcpStream = controller_data.get_stream().clone().unwrap();
    let res: ResponseResult = response
        .set_body(body)
        .set_status_code(200)
        .set_header("server", "hyperlane")
        .send(&stream);
    controller_data
        .get_log()
        .info(format!("Response result => {:?}", res), log_handler);
}

fn sync_panic_route(_controller_data: ArcRwLock<ControllerData>) {
    panic!("test panic");
}

async fn async_test_router(arc_lock_controller_data: ArcRwLockControllerData) {
    let controller_data: ControllerData = get_controller_data(&arc_lock_controller_data);
    controller_data.get_log().info("visit path /", log_handler);
    let mut response: Response = controller_data.get_response().clone();
    let body: &str = "Async";
    let stream: ArcTcpStream = controller_data.get_stream().clone().unwrap();
    let res: ResponseResult = response
        .set_body(body)
        .set_status_code(200)
        .set_header("server", "hyperlane")
        .send(&stream);
    controller_data
        .get_log()
        .info(format!("Response result => {:?}", res), log_handler);
}

async fn run_server() {
    let mut server: Server = Server::new();
    server.host("0.0.0.0");
    server.port(60000);
    server.log_dir("./logs");
    server.log_size(100_024_000);
    server.log_interval_millis(1000);
    server.middleware(sync_middleware);
    server.async_middleware(test_async_middleware).await;
    server.router("/", sync_root_router);
    server.router("/hello", sync_hello_router);
    server.router("/panic", sync_panic_route);
    server.async_router("/async/test", async_test_router).await;
    let test_string: String = "test".to_owned();
    server
        .async_router(
            "/test/async_func",
            async_func!(test_string, |data| {
                println_success!(test_string);
                println_success!(format!("{:?}", data));
            }),
        )
        .await;
    server.listen();
}
```

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please open an issue or submit a pull request.

## Contact

For any inquiries, please reach out to the author at [ltpp-universe <root@ltpp.vip>](mailto:root@ltpp.vip).
