/// Default maximum forwarded request body size (10 MiB).
pub const DEFAULT_GATEWAY_MAX_BODY_SIZE: usize = 10 * 1024 * 1024;

/// Default consecutive upstream failure count before opening the circuit breaker.
pub const DEFAULT_CIRCUIT_FAILURE_THRESHOLD: u32 = 5;

/// Default circuit breaker open cooldown window in milliseconds.
pub const DEFAULT_CIRCUIT_RESET_MS: u64 = 15_000;

/// Default upstream request timeout in milliseconds.
pub const DEFAULT_UPSTREAM_TIMEOUT_MS: u64 = 10_000;

/// Standard `X-Forwarded-For` header name.
pub const HEADER_X_FORWARDED_FOR: &str = "x-forwarded-for";

/// Standard `X-Forwarded-Host` header name.
pub const HEADER_X_FORWARDED_HOST: &str = "x-forwarded-host";

/// Standard `X-Forwarded-Proto` header name.
pub const HEADER_X_FORWARDED_PROTO: &str = "x-forwarded-proto";

/// Standard `X-Gateway-Upstream` diagnostic header name.
pub const HEADER_X_GATEWAY_UPSTREAM: &str = "x-gateway-upstream";

/// Nanoseconds per second used by the lock-free GCRA rate limiter.
pub(crate) const NANOS_PER_SECOND: u64 = 1_000_000_000;

/// Golden-ratio odd multiplier used for Power-of-Two-Choices index mixing.
pub(crate) const P2C_MIX_MULTIPLIER: usize = 0x9E37_79B9;

/// Lowercase `http` protocol tag for `X-Forwarded-Proto`.
pub(crate) const PROTO_HTTP: &str = "http";

/// Lowercase `proxy-connection` legacy hop-by-hop header name.
pub(crate) const HEADER_PROXY_CONNECTION: &str = "proxy-connection";

/// Lowercase `trailer` hop-by-hop header name.
pub(crate) const HEADER_TRAILER: &str = "trailer";

/// Lowercase `content-length` header name.
pub(crate) const HEADER_CONTENT_LENGTH: &str = "content-length";
