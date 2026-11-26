---
name: hyperlane
description: Use when building, debugging, optimizing, or reviewing Hyperlane Rust HTTP/1.x, HTTP/2, HTTP/3 services, http-request clients, or Hyperlane plugins (logger, negotiate, websocket, sse, mq, rpc, grpc, mcp, gateway).
---

# Hyperlane Development Skill

## Quick Orientation

- **Facade crate**: `hyperlane` re-exports `hyperlane-core` (`Server`, `ServerConfig`, `Context`, `ServerHook`, `Status`, `Stream`, `Request`, `Response`, `HttpStatus`, `tokio`) and `hyperlane-macros` (`#[route]`, `#[request_middleware]`, `#[response_middleware]`, `#[hyperlane]`, `#[send]`, `#[host]`, `#[referer]`, etc.).
- **Protocols**: `HTTP/1.0`, `HTTP/1.1` (TCP & TLS), `HTTP/2` (`h2c` prior knowledge & TLS ALPN `h2`), `HTTP/3` (QUIC + TLS), WebSocket, and Server-Sent Events (SSE).
- **Client crate**: `http-request` (`RequestBuilder`, `HttpRequest`, `HttpResponse`, `Proxy`) supporting sync/async H1, multiplexed async H2 (`h2`/`h2c`), opt-in connection pooling (`.keep_alive(true)`), TLS session resumption, custom PEM root CAs (`.root_cert_pem()`), idempotent retries (`.retry(n)`), form/multipart bodies, auth helpers, and HTTP/HTTPS/SOCKS5 proxies.

## ServerHook Pattern

```rust
use hyperlane::{Context, ServerHook, Status, Stream};

pub struct HealthRoute;

impl ServerHook for HealthRoute {
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    #[hyperlane::send]
    async fn handle(self, stream: &mut Stream, ctx: &mut Context) -> Status {
        ctx.get_mut_response().set_body("ok");
        Status::Reject
    }
}
```

- Return `Status::Continue` to pass execution to the next middleware/route/response hook.
- Return `Status::Reject` to terminate the request pipeline immediately (ensure the response is sent before rejecting if the client expects a reply).
- Route dynamic segments use `{name}` or anchored regex `{name:pattern}` (automatically anchored as `^(?:pattern)$`).

## HTTP Client (`http-request`) Pattern

```rust
use http_request::{HttpRequest, HttpResponse, RequestBuilder};

let mut builder: RequestBuilder = RequestBuilder::new();
let mut req: HttpRequest = builder
    .get("https://api.example.com/v1/items")
    .query("limit", "20")
    .bearer_auth("token")
    .keep_alive(true)
    .connect_timeout(2000)
    .timeout(5000)
    .retry(2)
    .decode()
    .build();
let resp: HttpResponse = req.send_async().await?;
```

- Use `.keep_alive(true)` for high-concurrency pooling (reuses persistent H1 connections and multiplexed H2 senders).
- Use `.http2_only()` for async `h2` (ALPN over TLS) or cleartext `h2c`.
- Automatic retries apply strictly to RFC 9110 idempotent methods (`GET`, `HEAD`, `PUT`, `DELETE`, `OPTIONS`) on transient transport errors or `429`/`502`/`503`/`504`.

## Official Plugin Ecosystem

All plugins live under `plugin/` and follow the opt-in `OnceLock` (`init`/`get`) contract — unconfigured hooks return `Status::Continue`:

| Plugin | Key Types | Purpose |
| --- | --- | --- |
| `hyperlane-plugin-logger` | `Logger`, `LoggerConfig`, `LoggerHook` | Console/file logging with CRLF/ANSI sanitization |
| `hyperlane-plugin-negotiate` | `CompressMiddleware`, `ConditionalMiddleware`, `EntityTag` | `Accept-Encoding` compression + `304 Not Modified` |
| `hyperlane-plugin-websocket` | `WebSocketHook`, room broadcast | WebSocket upgrade, rooms, connection lifecycle |
| `hyperlane-plugin-sse` | `SseEvent`, `SseHub`, `SseStream`, `SsePlugin` | CRLF-safe SSE framing, topic fan-out, `Last-Event-ID` replay, heartbeats |
| `hyperlane-plugin-mq` | `MessageBroker`, `InMemoryBroker`, `MqMessage`, `MqPlugin` | Consumer groups, bounded queues, TTL, visibility timeout, retries, DLQ |
| `hyperlane-plugin-rpc` | `RpcRouter`, `RpcRequest`, `RpcResponse`, `RpcHook`, `RpcPlugin` | JSON-RPC 2.0 single/batch/notification dispatch with DoS caps |
| `hyperlane-plugin-grpc` | `GrpcFrame`, `GrpcRouter`, `GrpcStatus`, `GrpcHook`, `GrpcPlugin` | 5-byte gRPC framing, bounded compression, `grpc-timeout` enforcement |
| `hyperlane-plugin-mcp` | `McpRegistry`, `McpTool`, `McpResource`, `McpPrompt`, `McpHook` | MCP Streamable HTTP server, `Mcp-Session-Id`, `Origin` DNS-rebinding guard |
| `hyperlane-plugin-gateway` | `GatewayRuntime`, `GatewayRoute`, `GatewayHook`, `GatewayPlugin` | Reverse proxy with pooled H1/H2, LB strategies, circuit breaker, token-bucket rate limit, hop-by-hop stripping |

## Verification & Performance Rules

1. Run `cargo fmt --all -- --check`, `cargo clippy --workspace --all-features --all-targets -- -D warnings`, and `cargo test --workspace --all-features`.
2. Note two long-running integration tests (`core/tests/server/fn.rs::main` and `plugin/websocket/tests/websocket/fn.rs::main`) sleep 60 s; skip only during tight iteration (`-- --skip "::main"`), and run the full suite before committing.
3. Validate throughput/latency changes with paired runs in `performance/` (`h1`, `h1-tls`, `h2c`, `h2-tls`, `h3`, `errors == 0`) and deterministic `alloc-count` before claiming improvements.
4. On Linux targets, verify `#[cfg(target_os = "linux")]` `io_uring` code builds and passes tests cleanly.
