/// Standard `RateLimit-Limit` response header name.
pub const HEADER_RATE_LIMIT_LIMIT: &str = "ratelimit-limit";

/// Standard `RateLimit-Remaining` response header name.
pub const HEADER_RATE_LIMIT_REMAINING: &str = "ratelimit-remaining";

/// Standard `RateLimit-Reset` response header name (seconds until window/burst reset).
pub const HEADER_RATE_LIMIT_RESET: &str = "ratelimit-reset";

/// Default maximum requests permitted per window.
pub const DEFAULT_RATE_LIMIT_REQUESTS: u64 = 100;

/// Default rate-limit window duration in milliseconds (60 seconds).
pub const DEFAULT_RATE_LIMIT_WINDOW_MS: u64 = 60_000;

/// Default burst capacity when not overridden.
pub const DEFAULT_RATE_LIMIT_BURST: u64 = 100;

/// Default maximum tracked distinct rate-limit keys across all shards.
pub const DEFAULT_RATE_LIMIT_MAX_KEYS: usize = 65_536;

/// Number of independent lock-striped shards (`64`, power of two).
pub(crate) const RATE_LIMIT_SHARD_COUNT: usize = 64;

/// Nanoseconds per millisecond conversion factor.
pub(crate) const NANOS_PER_MS: u64 = 1_000_000;

/// Nanoseconds per second conversion factor.
pub(crate) const NANOS_PER_SEC: u64 = 1_000_000_000;

/// Fallback key used when no peer address or forwarding header is available.
pub(crate) const FALLBACK_CLIENT_KEY: &str = "anonymous";
