# AGENTS.md

Repository instructions for AI coding agents working in the Hyperlane Rust monorepo.

## Workspace Layout

Rust edition `2024`, unified workspace version in root `Cargo.toml`.

| Crate                           | Path                   | Responsibility                                                                                                                                                                                                          |
| ------------------------------- | ---------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `hyperlane`                     | `.` (`src/`)           | Facade crate re-exporting `hyperlane-core` and `hyperlane-macros`                                                                                                                                                       |
| `hyperlane-core`                | `core/`                | Multi-protocol async server (`HTTP/1.x`, `HTTP/2` `h2c`/`h2`, `HTTP/3` QUIC), router, `Context`, `ServerHook`, Linux `io_uring` transport                                                                               |
| `hyperlane-macros`              | `macros/`              | Proc macros (`#[route]`, `#[request_middleware]`, `#[response_middleware]`, `#[hyperlane]`, `#[send]`, `#[host]`, `#[referer]`, etc.)                                                                                   |
| `http-type`                     | `type/`                | Wire parsers/builders, `Request`, `Response`, `Stream`, `Cookie`, `MultipartFormData`, `WebSocket`, `Status`, `RequestConfig`                                                                                           |
| `http-constant`                 | `constant/`            | Shared HTTP header/method/status/symbol constants                                                                                                                                                                       |
| `http-compress`                 | `compress/`            | Bounded gzip, deflate, and brotli compression/decompression (`MAX_DECOMPRESSED_BYTES = 64 MiB`), RFC 9110 `Accept-Encoding` negotiation                                                                                 |
| `http-request`                  | `request/`             | Sync and async HTTP/1.1 + HTTP/2 client with opt-in keep-alive pooling, TLS session resumption, custom PEM roots, retries, form/multipart, and HTTP/SOCKS5 proxies                                                      |
| `hyperlane-cli`                 | `cli/`                 | CLI project scaffolding, `hyperlane fmt` (layered on `crate-cli`), and workflow tooling                                                                                                                                 |
| `hyperlane-performance`         | `performance/`         | Official CPU-pinned Docker and native loadgen / control / alloc-count benchmark harness                                                                                                                                 |
| `hyperlane-plugin-auth`         | `plugin/auth/`         | Pure-Rust HS256 JWT (`kid` rotation), API key, HTTP Basic, RBAC `AuthPrincipal`, extensible `Authenticator`, `AuthHook`                                                                                                 |
| `hyperlane-plugin-cache`        | `plugin/cache/`        | 64-way sharded bounded memory cache, single-flight coalescing, pluggable `CacheStore`, HTTP response caching (`CacheRequestHook`/`CacheResponseHook`)                                                                   |
| `hyperlane-plugin-cors`         | `plugin/cors/`         | Precompiled zero-alloc CORS headers, exact/suffix/predicate origins, credentials safety guard, preflight `OPTIONS`, `Vary` merging                                                                                      |
| `hyperlane-plugin-gateway`      | `plugin/gateway/`      | Reverse proxy & API gateway backed by pooled `http-request` (allocation-free RR/Weighted/LC/P2C balancing, circuit breaker, lock-free GCRA limiter, `GatewayInterceptor`, RFC 9110 hop-by-hop stripping, `GatewayHook`) |
| `hyperlane-plugin-grpc`         | `plugin/grpc/`         | gRPC over HTTP/2 5-byte framing, bounded compression, status codes `0..=16`, `grpc-timeout`, `GrpcInterceptor`, `GrpcHook`                                                                                              |
| `hyperlane-plugin-health`       | `plugin/health/`       | `/healthz`, `/livez`, `/readyz` probes with concurrent async `HealthCheck`, per-probe timeout, snapshot TTL cache, lock-free drain flag, `HealthHook`                                                                   |
| `hyperlane-plugin-logger`       | `plugin/logger/`       | Non-blocking buffered console/file logger, Plain/JSON formats, `LogSink` trait, CRLF/ANSI sanitization, `RequestLogger`/`ResponseLogger`                                                                                |
| `hyperlane-plugin-mcp`          | `plugin/mcp/`          | Model Context Protocol Streamable HTTP server, `XxHash3_64` registries, 16-shard TTL sessions with high-entropy IDs, `Origin` DNS-rebinding guard, `McpHook`                                                            |
| `hyperlane-plugin-metrics`      | `plugin/metrics/`      | Lock-free atomic counters, gauges, latency histograms, Prometheus OpenMetrics 0.0.4 `/metrics` scrape endpoint, `MetricsRequestHook`/`MetricsResponseHook`                                                              |
| `hyperlane-plugin-mq`           | `plugin/mq/`           | Async `MessageBroker` trait and 64-shard `InMemoryBroker` (consumer groups, TTL, retries, $O(\log N)$ deadline heap, visibility timeout, DLQ)                                                                           |
| `hyperlane-plugin-negotiate`    | `plugin/negotiate/`    | `Accept-Encoding` compression and conditional `304 Not Modified` strong/weak ETag middlewares                                                                                                                           |
| `hyperlane-plugin-rate-limit`   | `plugin/rate_limit/`   | 64-way sharded lock-free atomic GCRA & fixed-window limiter, bounded key store, `KeyExtractor`/`RateLimitStore` traits, RFC `RateLimit-*` headers, `RateLimitHook`                                                      |
| `hyperlane-plugin-rpc`          | `plugin/rpc/`          | JSON-RPC 2.0 `XxHash3_64` router, concurrent batch/notification dispatch, payload/batch caps, `RpcInterceptor`, `RpcHook`                                                                                               |
| `hyperlane-plugin-security`     | `plugin/security/`     | Prebuilt HSTS/CSP/COOP/COEP/CORP/Referrer headers, stateless HMAC-SHA256 CSRF protection, host/URI/body/traversal guards, `SecurityRule` trait                                                                          |
| `hyperlane-plugin-session`      | `plugin/session/`      | 64-shard bounded `InMemorySessionStore` + pluggable `SessionStore`, HMAC-SHA256 signed cookies, dirty-tracked zero-write reads, `regenerate_id()` fixation defense                                                      |
| `hyperlane-plugin-shutdown`     | `plugin/shutdown/`     | Graceful shutdown coordinator with `SIGINT`/`SIGTERM` arming, lock-free RAII `ShutdownGuard` draining, LIFO async `ShutdownCallback` hooks                                                                              |
| `hyperlane-plugin-sse`          | `plugin/sse/`          | WHATWG SSE framing, CRLF sanitization, 64-way sharded topic pub/sub, `Last-Event-ID` replay, reusable-buffer heartbeats                                                                                                 |
| `hyperlane-plugin-static-files` | `plugin/static_files/` | Canonicalized traversal-proof static server, `FileExtension` MIME table, 64-shard hot-file cache, `.br`/`.gz` sidecars, `ETag`/`304`, RFC 9110 `Range` (`206`/`416`), SPA fallback                                      |
| `hyperlane-plugin-tracing`      | `plugin/tracing/`      | Lock-free 128-bit W3C Trace Context (`traceparent`/`tracestate`), `X-Request-Id`, `Server-Timing`, pluggable `SpanExporter` trait                                                                                       |
| `hyperlane-plugin-websocket`    | `plugin/websocket/`    | Room-based WebSocket broadcast with lag recovery, lifecycle hooks, and opt-in `WebSocketPlugin`                                                                                                                         |

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

## Unified Layered Formatting (`hyperlane fmt`)

`hyperlane-cli` integrates `crate-cli` unified Prettier-aligned formatting as the mandatory first pass before Hyperlane attribute normalization and `cargo fmt --all`:

```sh
cargo run -p hyperlane-cli -- fmt
cargo run -p hyperlane-cli -- fmt
cargo run -p hyperlane-cli -- fmt --check
cargo fmt --all -- --check
```

Always run `hyperlane fmt` locally before pushing so `.github/workflows/*.yml` stay pre-formatted (CI `publish` uses `GITHUB_TOKEN`, which cannot push workflow file diffs).

## Unified Documentation & Skill Maintenance Policy

| Change Trigger                                                | Target Document(s)                                                                                        | Mandatory Update Procedure                                                                                                                                                                                          |
| :------------------------------------------------------------ | :-------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| Add, rename, or remove a workspace crate or plugin            | Root `README.md`, `<crate>/README.md`, `plugin/README.md`, root `AGENTS.md`, `.github/workflows/rust.yml` | Update the 29-crate tables, workspace members, CI `PACKAGES`, and crate README in the same commit.                                                                                                                  |
| Change wire/protocol behavior, `ServerHook`, or security caps | Root `README.md`, `core/README.md`, `type/README.md`, root `AGENTS.md`                                    | Document wire guarantees and verify with real wire-byte tests.                                                                                                                                                      |
| Add or change CLI subcommands or formatting behavior          | `cli/README.md`, root `README.md`, root `AGENTS.md`, `.agents/skills/hyperlane/SKILL.md`                  | Keep CLI commands, `hyperlane fmt` layering, and verification gates synchronized.                                                                                                                                   |
| Upgrade upstream `ctares` dependencies or cut a release       | Root `Cargo.toml`, downstream Tier 3/4 repos                                                              | Wait until `ctares` is live on `crates.io`, bump version, run `hyperlane fmt` + full gates, verify green CI + `crates.io` publish, then cascade to `hyperlane-quick-start`, `commerce-suite`, and `web-frameworks`. |

- Run `cargo run -p hyperlane-cli -- fmt --check` after any documentation edit.
- Keep `.agents/skills/hyperlane/SKILL.md` synchronized with `~/.agents/skills/hyperlane/SKILL.md` (standards, requirements, and workflows only; `<= 160` lines; validated via `audit_skills.py`).

## Repo-Local Skills

See [.agents/skills/hyperlane/SKILL.md](.agents/skills/hyperlane/SKILL.md) for task-oriented Hyperlane development, plugin integration, client usage, and benchmarking patterns.
