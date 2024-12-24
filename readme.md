## hyperlane

[![](https://img.shields.io/crates/v/hyperlane.svg)](https://crates.io/crates/hyperlane)
[![](https://docs.rs/hyperlane/badge.svg)](https://docs.rs/hyperlane)
[![](https://img.shields.io/crates/l/hyperlane.svg)](./LICENSE)
[![](https://github.com/ltpp-universe/hyperlane/workflows/Rust/badge.svg)](https://github.com/ltpp-universe/hyperlane/actions?query=workflow:Rust)

[Official Documentation](https://docs.ltpp.vip/hyperlane/)

[Api Docs](https://docs.rs/hyperlane/latest/hyperlane/)

> Hyperlane is a lightweight and high-performance Rust HTTP server library designed to simplify network service development. It supports HTTP request parsing, response building, TCP communication, and redirection features, making it ideal for building modern web services.

## Features

## Installation

To use this crate, you can run cmd:

```shell
cargo add hyperlane
```

## Use

```rust
use hyperlane::*;
Server::new()
    .host("0.0.0.0")
    .port(80)
    .router("/", |controller_data| {
        let mut response: Response<'_> = controller_data.response.clone();
        let body: Vec<u8> = "404 Not Found".as_bytes().to_vec();
        let stream: &std::net::TcpStream = controller_data.stream();
        let res: Result<(), ResponseError> = response
            .body(body)
            .status_code(404)
            .header("server", "hyperlane")
            .send(stream);
        println!("{:?}", res);
    })
    .router("/hello", |controller_data| {
        let mut response: Response<'_> = controller_data.response.clone();
        let body: Vec<u8> = "hello world!".as_bytes().to_vec();
        let stream: &std::net::TcpStream = controller_data.stream();
        let res: Result<(), ResponseError> = response
            .body(body)
            .status_code(200)
            .header("server", "hyperlane")
            .send(stream);
        println!("{:?}", res);
    })
    .listen();
```

```rust
use hyperlane::*;
let mut server: Server<'_> = Server::new();
server.host("0.0.0.0");
server.port(80);
server.router("/", |controller_data| {
    let mut response: Response<'_> = controller_data.response.clone();
    let body: Vec<u8> = "404 Not Found".as_bytes().to_vec();
    let stream: &std::net::TcpStream = controller_data.stream();
    let res: Result<(), ResponseError> = response
        .body(body)
        .status_code(404)
        .header("server", "hyperlane")
        .send(stream);
    println!("{:?}", res);
});
server.router("/hello", |controller_data| {
    let mut response: Response<'_> = controller_data.response.clone();
    let body: Vec<u8> = "hello world!".as_bytes().to_vec();
    let stream: &std::net::TcpStream = controller_data.stream();
    let res: Result<(), ResponseError> = response
        .body(body)
        .status_code(200)
        .header("server", "hyperlane")
        .send(stream);
    println!("{:?}", res);
});
server.listen();
```

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please open an issue or submit a pull request.

## Contact

For any inquiries, please reach out to the author at [ltpp-universe <root@ltpp.vip>](mailto:root@ltpp.vip).
