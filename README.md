<center>

## hyperlane

<img src="https://ltpp.vip/github/pages/docs-pages/pages/img/hyperlane.png" alt="" height="160">

[![](https://img.shields.io/crates/v/hyperlane.svg)](https://crates.io/crates/hyperlane)
[![](https://img.shields.io/crates/d/hyperlane.svg)](https://img.shields.io/crates/d/hyperlane.svg)
[![](https://docs.rs/hyperlane/badge.svg)](https://docs.rs/hyperlane)
[![](https://github.com/hyperlane-dev/hyperlane/workflows/Rust/badge.svg)](https://github.com/hyperlane-dev/hyperlane/actions?query=workflow:Rust)
[![](https://img.shields.io/crates/l/hyperlane.svg)](./LICENSE)

</center>

[Api Docs](https://docs.rs/hyperlane/latest/)

> A lightweight, high-performance, and cross-platform Rust HTTP server library built on Tokio. It simplifies modern web service development by providing built-in support for middleware, WebSocket, Server-Sent Events (SSE), and raw TCP communication. With a unified and ergonomic API across Windows, Linux, and MacOS, it enables developers to build robust, scalable, and event-driven network applications with minimal overhead and maximum flexibility.

## Installation

To use this crate, you can run cmd:

```shell
cargo add hyperlane
```

## Quick start

- [hyperlane-quick-start git](https://github.com/hyperlane-dev/hyperlane-quick-start)

```sh
git clone https://github.com/hyperlane-dev/hyperlane-quick-start.git
```

## Runtime protocol and TLS

The listening protocol and the TLS material are two independent settings. A server
speaks exactly one wire protocol, selected at runtime from the configuration; TLS is
terminated on top of it, or left off for plaintext. There is no automatic protocol
detection, no multi-protocol listener and no fallback between them: an `http1`
listener never answers HTTP/2, and an `http2` listener never answers HTTP/1.1.

```rust
use hyperlane::{
    Server, ServerConfig, ServerControlHook, ServerError, ServerProtocol, TlsConfig,
};

#[tokio::main]
async fn main() -> Result<(), Box<ServerError>> {
    let mut config = ServerConfig::new("127.0.0.1:8443".to_string(), None, None);
    config.set_protocol(ServerProtocol::Http2).set_tls(Some(TlsConfig::new(
        "/etc/hyperlane/cert-chain.pem".to_string(),
        "/etc/hyperlane/privkey.pem".to_string(),
    )));

    let mut server = Server::default();
    server.server_config(config);
    let control: ServerControlHook = server.run().await?;
    control.wait().await;
    Ok(())
}
```

`ServerConfig::new` keeps its three argument signature and always yields plaintext
HTTP/1; choose a different protocol with `set_protocol`. Omitting both `protocol` and
`tls` from a JSON configuration therefore keeps the historical behaviour. The example
above only shows configuration and startup: register routes on `server` before
`run()` if the server is meant to answer anything.

```rust
use hyperlane::{ServerConfig, ServerProtocol, TlsConfig};

fn main() {
    // A config with neither field keeps the historical behaviour.
    let config: ServerConfig = ServerConfig::from_json("{\"address\":\"127.0.0.1:8080\"}")
        .expect("a config without protocol or tls must still load");
    assert_eq!(config.get_protocol(), ServerProtocol::Http1);
    assert!(config.get_tls().is_none(), "an omitted tls field is plaintext");
    assert_eq!(config.get_alpn_protocol(), b"http/1.1" as &[u8]);

    // The three argument constructor cannot select a protocol or tls.
    let legacy: ServerConfig = ServerConfig::new("127.0.0.1:8080".to_string(), None, None);
    assert_eq!(legacy.get_protocol(), ServerProtocol::Http1);
    assert!(legacy.get_tls().is_none());

    // The two settings are independent, in both directions.
    let mut selected: ServerConfig = ServerConfig::default();
    selected.set_protocol(ServerProtocol::Http2);
    assert!(selected.get_tls().is_none(), "choosing a protocol does not enable tls");
    selected.set_tls(Some(TlsConfig::new("chain.pem".to_string(), "key.pem".to_string())));
    assert_eq!(selected.get_protocol(), ServerProtocol::Http2);
    assert_eq!(selected.get_alpn_protocol(), b"h2" as &[u8]);
    selected.set_tls(None);
    assert_eq!(selected.get_protocol(), ServerProtocol::Http2, "clearing tls keeps the protocol");
    assert!(selected.get_tls().is_none());

    // An unknown protocol name is an error, not a silent fallback to http1.
    assert!(ServerConfig::from_json("{\"address\":\"127.0.0.1:8080\",\"protocol\":\"http4\"}").is_err());
}
```

### Protocol and TLS matrix

| `ServerProtocol` | JSON name | Transport | Without `tls` | With `tls` |
| --- | --- | --- | --- | --- |
| `Http1` (default) | `http1` | TCP | HTTP/1.1 over a plain socket, no ALPN | HTTP/1.1 over TLS, ALPN `http/1.1`, TLS 1.2 or 1.3 |
| `Http2` | `http2` | TCP | HTTP/2 with prior knowledge, no ALPN and no TLS | HTTP/2 over TLS, ALPN `h2`, TLS 1.2 or 1.3 |
| `Http3` | `http3` | UDP (QUIC) | refused: `run()` fails with `MissingTlsForHttp3` and the UDP port stays unbound | HTTP/3 over QUIC, ALPN `h3`, TLS 1.3 only |

Notes that the table does not make obvious:

- `ServerProtocol` is a unit-only enum chosen at runtime, not by a compile-time
  feature: the crates declare no `[features]` table, so one binary carries all three
  protocols and the configuration decides which one a given listener speaks.
- HTTP/1 and HTTP/2 listen on TCP; HTTP/3 binds a UDP socket. A firewall rule for HTTP/3
  is a UDP rule, and a port opened for TCP does not serve HTTP/3.
- HTTP/3 carries TLS inside the QUIC handshake, so `tls` is mandatory for `Http3`.
  QUIC admits TLS 1.3 only, so `http3` over TLS never negotiates 1.2; `http1` and
  `http2` over TLS accept either.
- The PEM files are read when the listener is created, not when the configuration is
  built, so a configuration can be deserialised, cloned and compared without touching the
  disk. An unreadable, malformed, truncated, or mismatched certificate chain and key is
  reported by `run()` before the socket is bound.
- An unknown `protocol` value in JSON is an error. It does not resolve to `http1`.

### Negotiating with a server

Exactly one ALPN identifier is advertised, the one belonging to the selected protocol.
A client that offers no ALPN the server accepts is refused rather than downgraded. An
HTTP/2 client needs prior knowledge to talk to a plaintext listener, and an HTTP/3
client must speak QUIC, since an HTTP/3 listener never accepts a TCP connection.

`http1` over TLS still serves a client that negotiates no ALPN at all, because clients
predating ALPN send none and must keep working. `http2` and `http3` require the
negotiated identifier to be their own, and each is refused differently. An `http2`
session that completes the handshake without `h2` is dropped by the listener, because
answering an HTTP/2 request under HTTP/1.1 rules would be worse than refusing the
connection. An `http3` client that does not offer `h3` fails the QUIC handshake itself
(RFC 9001 requires the `no_application_protocol` error), before any request is read.

WebSocket support is unchanged and remains HTTP/1 only: a route upgrades over HTTP/1.
The HTTP/2 extended CONNECT and HTTP/3 CONNECT handshake required by RFC 8441 and
RFC 9220 are not implemented, so do not point a WebSocket client at an `http2` or `http3`
listener.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please open an issue or submit a pull request.

## Contact

For any inquiries, please reach out to the author at [root@ltpp.vip](mailto:root@ltpp.vip).
