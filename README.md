<p align="center">
  <img src="https://ltpp.vip/github/pages/docs-pages/pages/img/hyperlane.png" alt="Hyperlane logo" height="112">
</p>

# Hyperlane

An asynchronous Rust web framework built on Tokio. Use it to serve HTTP APIs,
WebSocket connections, and server-sent events with explicit control over the
request and response lifecycle.

[![Crates.io](https://img.shields.io/crates/v/hyperlane.svg)][crate]
[![API documentation](https://docs.rs/hyperlane/badge.svg)][api]
[![Rust CI](https://github.com/hyperlane-dev/hyperlane/actions/workflows/rust.yml/badge.svg?branch=master)][ci]
[![MIT license](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

[API documentation][api] · [Quick start](#quick-start) · [Example project][example] · [Benchmarks](#benchmarks)

## Quick start

Create a binary crate with a Rust toolchain that supports edition 2024, then
add the published `hyperlane` crate:

```sh
cargo new hello-hyperlane
cd hello-hyperlane
cargo add hyperlane
```

Replace `src/main.rs` with:

```rust
use hyperlane::{Context, Server, ServerConfig, ServerError, ServerHook, Status, Stream, tokio};

const ADDRESS: &str = "127.0.0.1:3000";
const ROOT_PATH: &str = "/";
const GREETING: &str = "Hello from Hyperlane!\n";

struct Hello;

impl ServerHook for Hello {
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    #[hyperlane::send]
    async fn handle(self, stream: &mut Stream, ctx: &mut Context) -> Status {
        ctx.get_mut_response().set_body(GREETING);
        Status::Reject
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<ServerError>> {
    let mut config = ServerConfig::default();
    config.set_address(ADDRESS);

    let mut server = Server::default();
    server.server_config(config).route::<Hello>(ROOT_PATH);

    let control = server.run().await?;
    println!("Listening on http://{ADDRESS}");
    control.wait().await;
    Ok(())
}
```

Run `cargo run`, then request the route from another terminal:

```sh
curl -fsS http://127.0.0.1:3000/
```

The response is `Hello from Hyperlane!`. `#[hyperlane::send]` sends the
response built by the hook; `Status::Reject` stops the remaining pipeline so
another handler does not send a second response.

## Capabilities

| Area     | Support                                                   |
| -------- | --------------------------------------------------------- |
| HTTP/1   | HTTP/1.0 and HTTP/1.1 over TCP, with optional TLS         |
| HTTP/2   | Cleartext with prior knowledge, or TLS using ALPN         |
| HTTP/3   | QUIC with TLS; a certificate and private key are required |
| Routing  | Route parameters, middleware, and lifecycle hooks         |
| Realtime | WebSocket, server-sent events, and raw TCP handling       |

Select the wire protocol per listener with [`ServerConfig`][config].

## Workspace

| Crates                                                                                                      | Role                                       |
| ----------------------------------------------------------------------------------------------------------- | ------------------------------------------ |
| [`hyperlane`][api], [`hyperlane-core`](core/README.md), [`hyperlane-macros`](macros/README.md)              | Server API, routing, and procedural macros |
| [`http-type`](type/README.md), [`http-constant`](constant/README.md), [`http-compress`](compress/README.md) | HTTP types, constants, and compression     |
| [`http-request`](request/README.md)                                                                         | Synchronous and asynchronous HTTP client   |
| [WebSocket plugin](plugin/websocket/README.md), [Logger plugin](plugin/logger/README.md)                    | Opt-in plugins                             |
| [`hyperlane-cli`](cli/README.md), [`hyperlane-performance`](performance/README.md)                          | Project and benchmark tooling              |

## Protocol notes

These notes describe the current checkout. For the behavior of an installed
version, see its [release notes][releases] and [API documentation][api].

### Conditional requests

`Request::matches_if_none_match(current_etag)` checks `If-None-Match` for an
existing representation without allocating or cloning header values. Call it
explicitly after selecting the representation and its current ETag; requests
that do not call it follow the unchanged request pipeline.

The helper uses weak entity-tag comparison, including repeated header fields,
quoted tags containing commas, and a standalone `*`. Tag contents are
case-sensitive. Missing fields, malformed field values, and invalid current
ETags return `false`, even when an earlier list member matched.

This is a matching predicate, not an automatic cache or response middleware.
The caller handles precondition ordering and chooses `304 Not Modified` for
GET/HEAD or `412 Precondition Failed` for other methods when appropriate. It
must also construct a protocol-correct response; the helper does not clear a
body, change headers, or implement `If-Match` strong comparison.

### HTTP/1.1 request bodies

The server accepts `Transfer-Encoding: chunked`. Handlers receive the decoded
bytes and a `Content-Length` for those bytes, not the wire framing fields.
Chunk extensions and trailers are consumed but not exposed to handlers.
The configured body limit applies to the decoded size.

### Default request errors

When a request cannot be parsed or exceeds a configured limit, the server
sends an empty error response by default. HTTP/1.x closes the connection after
the response; HTTP/2 and HTTP/3 complete the request stream. Registered
request-error hooks retain control of the response instead.

## Benchmarks

The [performance workflow][perf] runs the Docker-based harness against a
baseline and a candidate in the same measurement window. It covers HTTP/1.1,
HTTP/2, and HTTP/3, with separate keep-alive and short-connection cases.
Open a workflow run for its report and raw measurements.

## Contributing

Use [GitHub Issues][issues] for bug reports and feature requests. For protocol
bugs, include the Hyperlane version, the HTTP version, and a minimal
reproduction. Before opening a pull request, run:

```sh
cargo fmt --all --check
cargo test --workspace --all-features
cargo clippy --workspace --all-features --all-targets -- -D warnings
```

## License

Hyperlane is distributed under the [MIT License](LICENSE).

[api]: https://docs.rs/hyperlane/latest/hyperlane/
[ci]: https://github.com/hyperlane-dev/hyperlane/actions/workflows/rust.yml
[config]: https://docs.rs/hyperlane/latest/hyperlane/struct.ServerConfig.html
[crate]: https://crates.io/crates/hyperlane
[example]: https://github.com/hyperlane-dev/hyperlane-quick-start
[issues]: https://github.com/hyperlane-dev/hyperlane/issues
[perf]: https://github.com/hyperlane-dev/hyperlane/actions/workflows/perf.yml
[releases]: https://github.com/hyperlane-dev/hyperlane/releases
