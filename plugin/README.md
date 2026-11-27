# The hyperlane plugin roadmap

A plugin is a crate whose hooks are registered into the framework's inventory.
The framework collects every registered hook when a server is built, so a
plugin becomes active by being a dependency of the application. This document
describes the contract a plugin follows, the standards it has to meet, and the
plugins that are planned, in the order they are worth building.

## The contract

The mechanism already exists; a plugin does not invent one.

- **Registration.** A plugin implements `ServerHook` and registers it with the
  `#[route]`, `#[request_middleware]` and `#[response_middleware]` attributes.
  Each attribute submits a `HookType` to the `inventory`, which is how the
  framework finds it.
- **Installation.** `#[hyperlane(server: Server)]` collects every submitted
  hook, checks that the explicit orders are unique, sorts by order and
  installs them. A plugin therefore needs no registration call of its own.
- **Order.** A hook carries an optional priority. The hooks run in the order
  of their priority, from the smallest to the largest, and a hook without a
  priority runs before every hook that has one. Two hooks of the same kind may
  not claim the same priority.
- **The order of a plugin.** The plugins that answer a request, such as the
  cross origin plugin, keep the negative priorities, and an application that
  gives its own middlewares positive priorities runs after them. A plugin that
  has to mark the answer before it is built, such as the compression plugin,
  therefore runs before the middleware of the application that sends it. A
  plugin that reports the answer, such as the logger plugin, runs after the
  middlewares of the application instead, so that it reports the answer that
  was really sent.
- **Configuration.** The registry is fixed at compile time, so a plugin's
  settings live in a process wide static, initialised before the server
  starts. The pattern is a `OnceLock` behind an `init` and a `get`, and the
  `GetOrInit` trait is the shape the application already uses.
- **Use in the application.** `#[import(path)]` brings the plugin's traits
  into scope without naming them, so an application talks to a plugin through
  its API and not through its internals.
- **Not configured means not in the way.** A hook that is registered but has
  no configuration returns `Status::Continue` and does nothing. A plugin that
  is a dependency but that the application did not configure must not change
  the server's behaviour.

The last point is the rule that shapes every plugin below: being a dependency
is not the same as being enabled.

## The standards a plugin has to meet

The repository's own gate checks every staged `.rs` and `.toml` file, so a
plugin is held to the same rules as the framework.

- **Layout.** A directory holds the keyword files only: `lib.rs`, `mod.rs`,
  `const.rs`, `fn.rs`, `struct.rs`, `enum.rs`, `impl.rs`, `trait.rs`, `type.rs`.
  The entry of a crate is `lib.rs`: the module declarations first, then the
  `pub use` and `pub(crate) use` re-exports, then the imports.
- **Visibility.** In `const.rs` and `static.rs` the `pub` items come before the
  `pub(crate)` ones and the private ones. A `pub` constant needs a reader
  outside its own module, which is why a plugin's settings are reached
  through an accessor rather than through a public field.
- **Strings.** Every literal lives in `const.rs`. The format string of a
  `format!` or a `println!` stays at the call site.
- **No panics.** `panic!`, `.expect()` and `.unwrap()` do not appear in
  production code. Errors travel as `Result`.
- **Types.** Every `let` binding and every closure parameter carries its type.
- **Fields.** A field is read through its accessor, never as `self.field`.
- **Documentation.** Every public item carries a doc comment, and a function
  that takes or returns something documents it under `# Arguments` and
  `# Returns`.
- **No `#[allow]`.**

## Official plugins in this repository

| crate                           | path                  | hook order (req / resp) | capabilities                                                                                                                                              |
| ------------------------------- | --------------------- | ----------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `hyperlane-plugin-tracing`      | `plugin/tracing`      | `-130` / `90`           | lock-free 128-bit W3C `traceparent`/`tracestate`, `X-Request-Id`, `Server-Timing`, pluggable `SpanExporter`                                               |
| `hyperlane-plugin-logger`       | `plugin/logger`       | `-120` / `100`          | non-blocking buffered console/file logging, Plain/JSON formats, `LogSink` trait, CRLF/ANSI sanitization                                                   |
| `hyperlane-plugin-metrics`      | `plugin/metrics`      | `-115` / `95`           | lock-free atomic counters, gauges, latency histograms, Prometheus 0.0.4 `/metrics` scrape endpoint                                                        |
| `hyperlane-plugin-health`       | `plugin/health`       | `-110` / —              | `/healthz`, `/livez`, `/readyz` probes with concurrent async `HealthCheck`, timeouts, snapshot cache, drain flag                                          |
| `hyperlane-plugin-security`     | `plugin/security`     | `-105` / `-75`          | prebuilt HSTS/CSP/COOP/COEP/CORP/Referrer headers, stateless HMAC-SHA256 CSRF, host/URI/body/traversal guards, `SecurityRule`                             |
| `hyperlane-plugin-cors`         | `plugin/cors`         | `-100` / `-80`          | precompiled zero-alloc CORS headers, exact/suffix/predicate origin matching, credentials safety guard, `OPTIONS` preflight                                |
| `hyperlane-plugin-rate-limit`   | `plugin/rate_limit`   | `-95` / —               | 64-shard lock-free atomic GCRA & fixed-window limiter, bounded key store, `KeyExtractor`/`RateLimitStore` traits, RFC `RateLimit-*` headers               |
| `hyperlane-plugin-auth`         | `plugin/auth`         | `-90` / —               | pure-Rust HS256 JWT (`kid` rotation), API key, HTTP Basic, RBAC `AuthPrincipal`, extensible async `Authenticator`                                         |
| `hyperlane-plugin-session`      | `plugin/session`      | `-85` / `-70`           | 64-shard bounded `InMemorySessionStore` + pluggable `SessionStore`, HMAC-SHA256 signed cookies, dirty-tracked reads, `regenerate_id()`                    |
| `hyperlane-plugin-cache`        | `plugin/cache`        | `-80` / `-65`           | 64-shard bounded cache, single-flight stampede coalescing, pluggable `CacheStore`, HTTP `Cache-Control`/`Age`/`X-Cache`                                   |
| `hyperlane-plugin-rpc`          | `plugin/rpc`          | `-70` / —               | JSON-RPC 2.0 `XxHash3_64` router, concurrent batch/notification dispatch, DoS caps, `RpcInterceptor`, `RpcHook`                                           |
| `hyperlane-plugin-grpc`         | `plugin/grpc`         | `-65` / —               | gRPC over HTTP/2 5-byte framing, bounded compression, status codes `0..=16`, `grpc-timeout`, `GrpcInterceptor`, `GrpcHook`                                |
| `hyperlane-plugin-mcp`          | `plugin/mcp`          | `-60` / —               | Model Context Protocol Streamable HTTP server, `XxHash3_64` registries, 16-shard TTL sessions, `Origin` guard, `McpHook`                                  |
| `hyperlane-plugin-static-files` | `plugin/static_files` | `-55` / —               | traversal-proof static server, `FileExtension` MIME table, 64-shard hot-file cache, `.br`/`.gz` sidecars, `ETag`/`304`, `Range` `206`/`416`, SPA fallback |
| `hyperlane-plugin-gateway`      | `plugin/gateway`      | `-50` / —               | reverse proxy & API gateway with pooled H1/H2 `http-request`, RR/Weighted/LC/P2C balancing, circuit breaker, lock-free GCRA limiter, `GatewayInterceptor` |
| `hyperlane-plugin-negotiate`    | `plugin/negotiate`    | explicit                | `Accept-Encoding` compression negotiation and conditional `304 Not Modified` strong/weak ETag validation                                                  |
| `hyperlane-plugin-shutdown`     | `plugin/shutdown`     | control hook            | graceful shutdown coordinator with `SIGINT`/`SIGTERM` arming, RAII `ShutdownGuard` draining, LIFO async `ShutdownCallback` hooks                          |
| `hyperlane-plugin-websocket`    | `plugin/websocket`    | route helper            | WebSocket room broadcast with lag recovery, connection lifecycle hooks, and opt-in `WebSocketPlugin`                                                      |
| `hyperlane-plugin-sse`          | `plugin/sse`          | route helper            | WHATWG Server-Sent Events framing, CRLF sanitization, 64-shard topic pub/sub, `Last-Event-ID` replay, heartbeats                                          |
| `hyperlane-plugin-mq`           | `plugin/mq`           | broker                  | pluggable `MessageBroker` trait and 64-shard `InMemoryBroker` with consumer groups, TTL, retries, $O(\log N)$ deadline heap, DLQ                          |

## The plugins, in the order they are worth building

### P0 - the ground every service needs

| plugin                      | what it gives                                                                                                               |
| --------------------------- | --------------------------------------------------------------------------------------------------------------------------- |
| `hyperlane-plugin-shutdown` | graceful shutdown: the framework already has the control hook, the plugin turns it into "configure it once and it is armed" |
| `hyperlane-plugin-logger`   | logging, with a file backend                                                                                                |

Both are small, and the application already carries a working version of each,
so upstreaming them is a move rather than a rewrite. A service that cannot shut
down cleanly or cannot say what it is doing is not a service yet.

The logger is in the tree now, and it is published. The shutdown plugin is
not: the signal of the operating system is not framework state, so the plugin
that joined it to the control hook is written again when the need is proven.

### P1 - what a web service is asked for every day

| plugin                          | what it gives                                                            |
| ------------------------------- | ------------------------------------------------------------------------ |
| `hyperlane-plugin-cors`         | cross origin headers and preflight                                       |
| `hyperlane-plugin-compression`  | gzip and brotli on the response                                          |
| `hyperlane-plugin-static-files` | serving files, with the content type table the framework already carries |

None of the three is in the tree. Their rules are not framework state either,
so they are written where they are needed, and a server that is not Hyperlane
can use the same rules.

The compression plugin is the cheapest of the three: `http-compress` is
already a dependency and the response builder already honours a
`Content-Encoding` header, so the plugin only has to set the header when the
client asked for it and the body is worth compressing.

The static file plugin registers a route and no middleware, because the path
the files are served under belongs to the application. The application
registers the route with the pattern the plugin builds, and the answer of a
file is built and sent by the middlewares of the application, like the answer
of any other route.

### P2 - the data layer

| plugin                        | what it gives                                                       |
| ----------------------------- | ------------------------------------------------------------------- |
| `hyperlane-plugin-database`   | the connection and auto creation traits, with no driver behind them |
| `hyperlane-plugin-postgresql` | the PostgreSQL driver                                               |
| `hyperlane-plugin-mysql`      | the MySQL driver                                                    |
| `hyperlane-plugin-redis`      | the Redis client and its pool                                       |

The application already has this shape, down to the trait names. The base
crate carries no driver, so it stays small; the driver crates carry theirs,
so they stay large and are published separately.

### P3 - running a service in production

| plugin                        | what it gives                              |
| ----------------------------- | ------------------------------------------ |
| `hyperlane-plugin-env`        | configuration loaded from the environment  |
| `hyperlane-plugin-metrics`    | counters and timers in a scrapeable format |
| `hyperlane-plugin-rate-limit` | a limit per client or per route            |

### P4 - the rest, when the need is proven

`hyperlane-plugin-message-queue`, `hyperlane-plugin-jwt`,
`hyperlane-plugin-tracing` and `hyperlane-plugin-multipart`.

## The shape of a plugin

```
plugin/<name>/
  Cargo.toml
  README.md
  src/
    lib.rs      the entry: module declarations, re-exports, the imports
    const.rs    every string literal
    struct.rs   the types
    enum.rs     the enumerations
    trait.rs    the traits the application talks to
    impl.rs     the implementations
    fn.rs       the free functions
    mod.rs      a submodule, when the plugin has more than one
  tests/
    mod.rs      the test entry: the modules and the crate under test
```

The entry of a crate is `lib.rs`. A `mod.rs` holds a submodule instead, and
carries the module declarations first, then the `pub use` and `pub(crate) use`
re-exports. Every keyword file below the entry starts with `use super::*;` and
reaches the crate's imports through it, so an import is written once, in the
entry, and never again.

A plugin declares its dependency on the framework and on nothing else, unless
it carries a driver. Its `Cargo.toml` carries `publish = false` until it is
ready to be published, and it is added to the workspace members, to the
workspace dependencies and to the release job's package list.

Its public surface is small on purpose: an `init` that takes the
configuration, a `get` that hands the configuration back, and the traits the
application calls. Everything else is private.

## What is still to decide

- **Publishing.** The release job names the plugins of this tree, and the
  plugins that hold no framework state are published from the crates-dev family
  instead, so a server that is not Hyperlane can use the same rules.
- **How much configuration is enough.** Every plugin that needs a setting
  needs it set before the server starts, and that is a second thing the
  application has to remember. A single configuration entry point would be
  better than one per plugin, and it is a decision, not an implementation.
