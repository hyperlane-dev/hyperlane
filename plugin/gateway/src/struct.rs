use super::*;

/// Token-bucket rate-limit parameters attached to a [`GatewayRoute`].
#[derive(Clone, Copy, Data, Debug, Eq, PartialEq)]
pub struct RateLimitConfig {
    /// Sustained refill rate in requests per second.
    #[get(type(copy))]
    pub(crate) requests_per_second: u64,
    /// Maximum burst capacity in tokens.
    #[get(type(copy))]
    pub(crate) burst: u64,
}

/// One upstream backend endpoint participating in a [`GatewayRoute`].
#[derive(Clone, Data, Debug, Eq, PartialEq)]
pub struct UpstreamTarget {
    /// Base upstream URL (for example `"http://127.0.0.1:8081"`).
    pub(crate) url: String,
    /// Relative load-balancing weight (`>= 1`).
    #[get(type(copy))]
    pub(crate) weight: u32,
}

/// Routing, load-balancing, retry, and rate-limit definition for one gateway prefix.
#[derive(Clone, Data, Debug, Eq, PartialEq)]
pub struct GatewayRoute {
    /// Path prefix matched on segment boundaries (for example `"/api"` matches `"/api"` and `"/api/v1"`).
    pub(crate) prefix: String,
    /// Whether to strip `prefix` before forwarding the request path upstream.
    #[get(type(copy))]
    pub(crate) strip_prefix: bool,
    /// Optional exact case-insensitive `Host` matcher.
    #[get(type(clone))]
    pub(crate) host: Option<String>,
    /// Upstream targets eligible to serve matched traffic.
    pub(crate) upstreams: Vec<UpstreamTarget>,
    /// Upstream selection strategy.
    #[get(type(copy))]
    pub(crate) strategy: LoadBalanceStrategy,
    /// Per-attempt upstream timeout in milliseconds.
    #[get(type(copy))]
    pub(crate) timeout_ms: u64,
    /// Maximum automatic retry attempts on idempotent transient upstream errors.
    #[get(type(copy))]
    pub(crate) max_retries: usize,
    /// Optional per-route token-bucket rate limiter.
    #[get(type(copy))]
    pub(crate) rate_limit: Option<RateLimitConfig>,
    /// Whether to forward upstream using multiplexed HTTP/2 (`h2`/`h2c`).
    #[get(type(copy))]
    pub(crate) use_http2: bool,
}

/// Top-level configuration for [`GatewayRuntime`] and [`GatewayPlugin`].
#[derive(Clone, Data, Debug, Eq, PartialEq)]
pub struct GatewayConfig {
    /// Configured reverse-proxy routes (matched longest-prefix first).
    pub(crate) routes: Vec<GatewayRoute>,
    /// Maximum forwarded request body length in bytes.
    #[get(type(copy))]
    pub(crate) max_body_size: usize,
    /// Consecutive failures before opening an upstream's circuit breaker.
    #[get(type(copy))]
    pub(crate) circuit_failure_threshold: u32,
    /// Cooldown duration in milliseconds before an open circuit allows a probe request.
    #[get(type(copy))]
    pub(crate) circuit_reset_ms: u64,
}

/// Live atomic health and concurrency state for one upstream node.
pub(crate) struct UpstreamRuntime {
    /// Normalized upstream base URL (without trailing slash).
    pub(crate) base_url: String,
    /// Load-balancing weight (`>= 1`).
    pub(crate) weight: u32,
    /// Current number of active in-flight requests to this node.
    pub(crate) active_conns: AtomicUsize,
    /// Consecutive failure count since last success.
    pub(crate) consecutive_failures: AtomicU32,
    /// Unix epoch millisecond until which this circuit remains open (`0` = closed).
    pub(crate) open_until_ms: AtomicU64,
}

/// Lock-free GCRA token-bucket rate limiter using a single atomic timestamp.
pub(crate) struct TokenBucket {
    /// Nanoseconds required to regenerate one request token.
    pub(crate) emission_interval_ns: u64,
    /// Burst window in nanoseconds (`emission_interval_ns * burst`).
    pub(crate) burst_tolerance_ns: u64,
    /// Monotonic origin timestamp for nanosecond arithmetic.
    pub(crate) epoch: Instant,
    /// Theoretical Arrival Time (TAT) in nanoseconds since `epoch`.
    pub(crate) tat_ns: AtomicU64,
}

/// Compiled route entry carrying live upstream nodes, selector cursor, and optional limiter.
pub(crate) struct CompiledRoute {
    /// Original route definition.
    pub(crate) route: GatewayRoute,
    /// Normalized prefix (without trailing slash except `"/"`).
    pub(crate) normalized_prefix: String,
    /// Live upstream runtime nodes.
    pub(crate) upstreams: Vec<Arc<UpstreamRuntime>>,
    /// Round-robin / weighted selection cursor.
    pub(crate) cursor: AtomicUsize,
    /// Optional compiled lock-free GCRA limiter.
    pub(crate) limiter: Option<TokenBucket>,
}

/// Compiled thread-safe reverse-proxy engine backing [`GatewayPlugin`] and [`GatewayHook`].
#[derive(Clone)]
pub struct GatewayRuntime {
    /// Top-level limits and circuit-breaker thresholds.
    pub(crate) config: GatewayConfig,
    /// Compiled routes sorted from longest prefix to shortest prefix.
    pub(crate) routes: Arc<Vec<CompiledRoute>>,
    /// Pluggable interceptors invoked around upstream forwarding.
    pub(crate) interceptors: Arc<Vec<SharedGatewayInterceptor>>,
}

/// Process-wide [`GatewayPlugin`] registry storing [`GatewayRuntime`].
#[derive(Clone)]
pub struct GatewayPlugin {
    /// Compiled gateway runtime engine.
    pub(crate) runtime: GatewayRuntime,
}

/// Opt-in [`ServerHook`] forwarding matched requests through [`GatewayPlugin`].
#[derive(Clone, Copy, Debug, Default)]
#[hyperlane::request_middleware(-50)]
pub struct GatewayHook;
