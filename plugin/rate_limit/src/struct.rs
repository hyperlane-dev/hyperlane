use super::*;

/// Result of evaluating one request against a [`RateLimitStore`].
#[derive(Clone, Copy, Data, Debug, Eq, PartialEq)]
pub struct RateLimitDecision {
    /// `true` when the request is admitted; `false` when throttled (`429 Too Many Requests`).
    #[get(type(copy))]
    pub(crate) allowed: bool,
    /// Effective quota limit for the window or burst.
    #[get(type(copy))]
    pub(crate) limit: u64,
    /// Remaining permitted requests right after this evaluation.
    #[get(type(copy))]
    pub(crate) remaining: u64,
    /// Seconds until the bucket fully resets.
    #[get(type(copy))]
    pub(crate) reset_after_secs: u64,
    /// Seconds the caller should wait before retrying (`0` when allowed).
    #[get(type(copy))]
    pub(crate) retry_after_secs: u64,
}

/// Configuration policy governing [`ShardedRateLimiter`] and [`RateLimitPlugin`].
#[derive(Clone, Data, Debug, Eq, PartialEq)]
pub struct RateLimitPolicy {
    /// Sustained requests allowed per `window_ms`.
    #[get(type(copy))]
    pub(crate) requests_per_window: u64,
    /// Window length in milliseconds.
    #[get(type(copy))]
    pub(crate) window_ms: u64,
    /// Maximum burst size for [`RateLimitAlgorithm::Gcra`].
    #[get(type(copy))]
    pub(crate) burst: u64,
    /// Enforcement algorithm (`Gcra` or `FixedWindow`).
    #[get(type(copy))]
    pub(crate) algorithm: RateLimitAlgorithm,
    /// Maximum tracked distinct keys before stale eviction triggers.
    #[get(type(copy))]
    pub(crate) max_keys: usize,
    /// Whether to emit `RateLimit-*` headers on responses.
    #[get(type(copy))]
    pub(crate) include_headers: bool,
    /// Exact paths bypassed by [`RateLimitHook`] (for example `"/healthz"`).
    pub(crate) exempt_paths: Vec<String>,
}

/// Lock-free atomic state for one partition key inside [`ShardedRateLimiter`].
pub(crate) struct BucketEntry {
    /// GCRA theoretical arrival time (ns) or fixed-window bucket index.
    pub(crate) primary: AtomicU64,
    /// Fixed-window request count (unused by GCRA).
    pub(crate) counter: AtomicU64,
    /// Monotonic last-access timestamp in nanoseconds since limiter epoch.
    pub(crate) last_seen_ns: AtomicU64,
}

/// Shared interior state of [`ShardedRateLimiter`].
pub(crate) struct ShardedRateLimiterInner {
    /// Active policy parameters.
    pub(crate) policy: RateLimitPolicy,
    /// Monotonic origin timestamp for nanosecond arithmetic.
    pub(crate) epoch: Instant,
    /// Precomputed GCRA emission interval in nanoseconds (`window_ns / requests_per_window`).
    pub(crate) emission_interval_ns: u64,
    /// Precomputed GCRA burst tolerance in nanoseconds (`emission_interval_ns * burst`).
    pub(crate) burst_tolerance_ns: u64,
    /// Precomputed window duration in nanoseconds.
    pub(crate) window_ns: u64,
    /// Lock-free count of tracked keys across all 64 shards.
    pub(crate) key_count: AtomicUsize,
    /// 64-way lock-striped key tables.
    pub(crate) shards: RateLimitShardArray,
}

/// High-concurrency 64-way sharded lock-free in-memory [`RateLimitStore`].
#[derive(Clone)]
pub struct ShardedRateLimiter {
    /// Shared limiter state.
    pub(crate) inner: Arc<ShardedRateLimiterInner>,
}

/// Built-in [`KeyExtractor`] partitioning traffic by client IP address.
#[derive(Clone, Copy, Data, Debug, Default, Eq, PartialEq)]
pub struct ClientIpKeyExtractor {
    /// When `true`, inspects `X-Forwarded-For` and `X-Real-Ip` before falling back to peer IP.
    #[get(type(copy))]
    pub(crate) trust_proxy_headers: bool,
}

/// Built-in [`KeyExtractor`] partitioning traffic by a specific request header value (for example `X-Api-Key`).
#[derive(Clone, Data, Debug, Eq, PartialEq)]
pub struct HeaderKeyExtractor {
    /// Request header name read as the bucket key.
    pub(crate) header_name: String,
    /// Whether to fall back to client IP when the header is missing.
    #[get(type(copy))]
    pub(crate) fallback_to_ip: bool,
}

/// Built-in [`KeyExtractor`] partitioning traffic by `(request_path, client_ip)`.
#[derive(Clone, Copy, Data, Debug, Default, Eq, PartialEq)]
pub struct PathAndIpKeyExtractor {
    /// Whether forwarding headers are trusted when resolving the client IP.
    #[get(type(copy))]
    pub(crate) trust_proxy_headers: bool,
}

/// Process-wide [`RateLimitPlugin`] registry storing policy, [`RateLimitStore`], and [`KeyExtractor`].
#[derive(Clone)]
pub struct RateLimitPlugin {
    /// Active rate-limiting policy.
    pub(crate) policy: RateLimitPolicy,
    /// Backend store evaluating quota decisions.
    pub(crate) store: SharedRateLimitStore,
    /// Partition key extractor.
    pub(crate) key_extractor: SharedKeyExtractor,
}

/// Opt-in [`ServerHook`] enforcing configured rate limits and attaching RFC `RateLimit-*` headers.
#[derive(Clone, Copy, Debug, Default)]
#[hyperlane::request_middleware(-95)]
pub struct RateLimitHook;
