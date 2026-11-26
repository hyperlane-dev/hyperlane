# AGENTS.md

Repository instructions for AI coding agents working in the Hyperlane Rust monorepo.

## Workspace Layout

Rust edition `2024`, unified workspace version in root `Cargo.toml`.

| Crate | Path | Responsibility |
| --- | --- | --- |
| `hyperlane` | `.` (`src/`) | Facade crate re-exporting `hyperlane-core` and `hyperlane-macros` |
| `hyperlane-core` | `core/` | Multi-protocol async server (`HTTP/1.x`, `HTTP/2` `h2c`/`h2`, `HTTP/3` QUIC), router, `Context`, `ServerHook`, Linux `io_uring` transport |
| `hyperlane-macros` | `macros/` | Proc macros (`#[route]`, `#[request_middleware]`, `#[response_middleware]`, `#[hyperlane]`, `#[send]`, `#[host]`, `#[referer]`, etc.) |
| `http-type` | `type/` | Wire parsers/builders, `Request`, `Response`, `Stream`, `Cookie`, `MultipartFormData`, `WebSocket`, `Status`, `RequestConfig` |
| `http-constant` | `constant/` | Shared HTTP header/method/status/symbol constants |
| `http-compress` | `compress/` | Bounded gzip, deflate, and brotli compression/decompression (`MAX_DECOMPRESSED_BYTES = 64 MiB`), RFC 9110 `Accept-Encoding` negotiation |
| `http-request` | `request/` | Sync and async HTTP/1.1 + HTTP/2 client with opt-in keep-alive pooling, TLS session resumption, custom PEM roots, retries, form/multipart, and HTTP/SOCKS5 proxies |
| `hyperlane-cli` | `cli/` | CLI project scaffolding and workflow tooling |
| `hyperlane-performance` | `performance/` | Official CPU-pinned Docker and native loadgen / control / alloc-count benchmark harness |
| `hyperlane-plugin-logger` | `plugin/logger/` | Console and file logger with CRLF/ANSI sanitization |
| `hyperlane-plugin-negotiate` | `plugin/negotiate/` | `Accept-Encoding` compression and conditional `304 Not Modified` ETag middlewares |
| `hyperlane-plugin-websocket` | `plugin/websocket/` | Room-based WebSocket broadcast and lifecycle hooks |
| `hyperlane-plugin-sse` | `plugin/sse/` | WHATWG SSE framing, CRLF sanitization, topic pub/sub, `Last-Event-ID` replay, heartbeats |
| `hyperlane-plugin-mq` | `plugin/mq/` | Async `MessageBroker` trait and `InMemoryBroker` (consumer groups, TTL, retries, visibility timeout, DLQ) |
| `hyperlane-plugin-rpc` | `plugin/rpc/` | JSON-RPC 2.0 router, batch/notification dispatch, payload/batch caps, `RpcHook` |
| `hyperlane-plugin-grpc` | `plugin/grpc/` | gRPC over HTTP/2 5-byte framing, bounded compression, status codes `0..=16`, `grpc-timeout`, `GrpcHook` |
| `hyperlane-plugin-mcp` | `plugin/mcp/` | Model Context Protocol Streamable HTTP server, tools/resources/prompts registries, `Mcp-Session-Id`, `Origin` DNS-rebinding guard, `McpHook` |
| `hyperlane-plugin-gateway` | `plugin/gateway/` | Reverse proxy & API gateway backed by pooled `http-request` (load balancing, circuit breaker, token-bucket rate limit, RFC 9110 hop-by-hop stripping, `GatewayHook`) |

## Core Architectural Invariants

1. **Public API stability**: Never remove or break existing `pub` signatures across workspace crates without an explicit major-version plan. Do not introduce duplicate APIs for equivalent functionality.
2. **`ServerHook` lifecycle**:
   - `ServerHook::new(&mut Stream, &mut Context) -> Self` and `ServerHook::handle(self, &mut Stream, &mut Context) -> Status`.
   - Returning `Status::Continue` advances to the next middleware/route/response hook.
   - Returning `Status::Reject` stops the remaining pipeline immediately (used after writing the response or rejecting an invalid request). Always send via `stream.try_send(&ctx.get_mut_response().build()).await` (or `#[hyperlane::send]`) before returning `Status::Reject` if a reply is expected.
   - Keep-alive connections reset `Context` (request, response, route params, attributes, HTTP version) across every request boundary, including errors and panics.
3. **Plugin contract (`plugin/README.md`)**:
   - Plugins must be opt-in and transparent when unconfigured (`OnceLock` `init`/`get`; unconfigured hooks return `Status::Continue`).
   - Add new publishable plugins to root `Cargo.toml` `[workspace] members` and `.github/workflows/rust.yml` `PACKAGES`.
4. **Security & wire conformance invariants**:
   - Reject HTTP request smuggling (`Content-Length` + `Transfer-Encoding`, duplicate `Content-Length`, obs-fold, bare CR/LF, invalid request-line/HTTP version).
   - Enforce response CRLF/header-value validation at the wire builder boundary and forbid `Content-Length` alongside `Transfer-Encoding`.
   - Cap all unbounded allocations: H1 request line/head/body caps, H2 `SETTINGS_MAX_HEADER_LIST_SIZE` & reset caps, H3 GREASE skip budget & streaming body caps, decompression bomb cap (`64 MiB`), WebSocket control-frame (`125 B`) & reassembled message caps, redirect credential stripping across origins or HTTPS->HTTP downgrades, and MCP `Origin` validation.
   - Preserve exact route anchoring (`{param:regex}` anchored as `^(?:...)$`) and leading-slash normalization.

## Mandatory Rust Coding Standards

All `.rs` and `.toml` changes must pass the workspace style gate:

- **9 keyword filenames only**: `lib.rs`, `mod.rs`, `const.rs`, `static.rs`, `type.rs`, `enum.rs`, `struct.rs`, `trait.rs`, `impl.rs`, `fn.rs`. No code files alongside subdirectories.
- **Centralized imports**: Declare external/std imports once in crate root (`src/lib.rs` / `tests/mod.rs`); submodules use `use super::*;`. No `use ... as ...` aliases.
- **Strict cleanliness**: Zero comments in `mod.rs`, `*.toml`, and `tests/**/*.rs`; no blank lines inside function bodies; explicit types on all `let` bindings and closure parameters; generic bounds in `where` clauses; no `#[allow]`/`#[expect]` in production code; no `.unwrap()`/`.expect()`/`panic!()` in `src/`.
- **Field & string discipline**: Read fields via accessors rather than `self.field` outside accessors/Display/Debug/constructors; store string literals (`>= 4` chars) in `const.rs` with `pub` items ordered before `pub(crate)` and private items; include 4-layer doc comments (`# Arguments`, `# Returns`) on functions.
- **Externalized tests**: Keep unit and integration tests under `tests/`, never inline `#[cfg(test)] mod tests` in `src/`.

## Verification & Benchmarking Workflow

Run locally before opening or updating a PR:

```sh
cargo fmt --all -- --check
cargo clippy --workspace --all-features --all-targets -- -D warnings
cargo test --workspace --all-features
cargo check --release --workspace --all-features
```

For performance changes, validate with real paired measurements (`performance/` harness: `h1`, `h1-tls`, `h2c`, `h2-tls`, `h3`, `errors == 0`, plus deterministic `alloc-count`), never code inspection alone.

## Repo-Local Skills

See [.agents/skills/hyperlane/SKILL.md](.agents/skills/hyperlane/SKILL.md) for task-oriented Hyperlane development, plugin integration, client usage, and benchmarking patterns.
