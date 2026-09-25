<center>

## http-request

[![](https://img.shields.io/crates/v/http-request.svg)](https://crates.io/crates/http-request)
[![](https://img.shields.io/crates/d/http-request.svg)](https://img.shields.io/crates/d/http-request.svg)
[![](https://docs.rs/http-request/badge.svg)](https://docs.rs/http-request)
[![](https://github.com/hyperlane-dev/http-request/workflows/Rust/badge.svg)](https://github.com/hyperlane-dev/http-request/actions?query=workflow:Rust)
[![](https://img.shields.io/crates/l/http-request.svg)](./LICENSE)

</center>

[Api Docs](https://docs.rs/http-request/latest/)

> A lightweight, efficient library for building, sending, and handling HTTP/HTTPS requests in Rust applications. http-request provides a simple and intuitive API, allowing developers to easily interact with web services, whether they use the "HTTP" or "HTTPS" protocol. The library supports various HTTP methods, custom headers, request bodies, timeout, automatic handling of redirects (including detecting redirect loops), and enhanced response body decoding (both automatic and manual), enabling fast and secure communication. Whether working with secure "HTTPS" connections or standard "HTTP" requests, the library is optimized for performance, minimal resource usage, and easy integration into Rust projects.

## Features

- **Support for HTTP/HTTPS**: Supports both HTTP and HTTPS protocols.
- **WebSocket Support**: Full WebSocket support with both synchronous and asynchronous APIs for real-time communication.
- **Lightweight Design**: The `http_request` crate provides a simple and efficient API for building, sending, and handling HTTP requests while minimizing resource consumption.
- **Supports Common HTTP Method**: Supports common HTTP methods such as GET and POST.
- **Flexible Request Building**: Offers rich configuration options through `RequestBuilder` to set request headers, bodies, and URLs.
- **Simple Error Handling**: Utilizes the `Result` type to handle errors in requests and responses, making error handling straightforward.
- **Custom Headers and Request Bodies**: Easily add custom headers and request bodies.
- **Response Handling**: Provides a simple wrapper around HTTP responses, making it easy to access and process response data.
- **Optimized Memory Management**: Implements efficient memory management to minimize unnecessary memory allocations and improve performance.
- **Redirect Handling**: Supports redirect handling, allows setting the maximum number of redirects, and includes redirect loop detection.
- **timeout**: Supports timeout.
- **Automatic and Manual Response Body Decoding**: Supports both automatic and manual decoding of response bodies, allowing for seamless interaction with different content types (e.g., JSON, XML, etc.).
- **Proxy Support**: Comprehensive proxy support including HTTP, HTTPS, and SOCKS5 proxies with authentication for both HTTP requests and WebSocket connections.

## Installation

To use this crate, you can run cmd:

```shell
cargo add http-request
```

## Help

Ensure that CMake is installed on the system

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please open an issue or submit a pull request.

## Contact

For any inquiries, please reach out to the author at [root@ltpp.vip](mailto:root@ltpp.vip).
