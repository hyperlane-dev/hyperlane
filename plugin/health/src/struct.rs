use super::*;

/// Result returned by a single [`HealthCheck::check`] invocation.
#[derive(Clone, Data, Debug, Eq, PartialEq)]
pub struct CheckOutcome {
    /// Health state reported by the probe.
    #[get(type(copy))]
    pub(crate) status: HealthStatus,
    /// Optional human-readable diagnostic detail.
    #[get(type(clone))]
    pub(crate) detail: Option<String>,
}

/// Serialized report for one executed probe in a [`HealthReport`].
#[derive(Clone, Data, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct CheckReport {
    /// Registered probe name.
    pub(crate) name: String,
    /// Outcome status of this probe.
    #[get(type(copy))]
    pub(crate) status: HealthStatus,
    /// Optional diagnostic detail.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[get(type(clone))]
    pub(crate) detail: Option<String>,
    /// Execution time in milliseconds.
    #[get(type(copy))]
    pub(crate) latency_ms: u64,
}

/// Top-level JSON health report emitted on `/healthz`, `/livez`, and `/readyz`.
#[derive(Clone, Data, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct HealthReport {
    /// Overall status derived from draining flag and individual checks.
    #[get(type(copy))]
    pub(crate) status: HealthStatus,
    /// `true` when the server has been marked draining for graceful shutdown.
    #[get(type(copy))]
    pub(crate) draining: bool,
    /// Individual probe results.
    pub(crate) checks: Vec<CheckReport>,
}

/// Endpoint paths, timeout, and snapshot cache configuration for [`HealthPlugin`].
#[derive(Clone, Data, Debug, Eq, PartialEq)]
pub struct HealthConfig {
    /// Combined health endpoint path (default `"/healthz"`).
    pub(crate) health_path: String,
    /// Process liveness endpoint path (default `"/livez"`).
    pub(crate) live_path: String,
    /// Traffic readiness endpoint path (default `"/readyz"`).
    pub(crate) ready_path: String,
    /// Per-probe execution timeout in milliseconds.
    #[get(type(copy))]
    pub(crate) check_timeout_ms: u64,
    /// Optional cached report TTL in milliseconds (`0` disables caching).
    #[get(type(copy))]
    pub(crate) cache_ttl_ms: u64,
}

/// Thread-safe registry of async [`HealthCheck`] probes with lock-free drain state and snapshot cache.
#[derive(Clone)]
pub struct HealthRegistry {
    /// Registered dependency health checks executed concurrently.
    pub(crate) checks: Arc<Vec<SharedHealthCheck>>,
    /// Lock-free flag flipped during graceful shutdown to fail readiness probes immediately.
    pub(crate) draining: Arc<AtomicBool>,
    /// Cached readiness/health report paired with its capture timestamp.
    pub(crate) cache: Arc<RwLock<Option<(Instant, HealthReport)>>>,
}

/// Process-wide [`HealthPlugin`] singleton storing [`HealthConfig`] and [`HealthRegistry`].
#[derive(Clone)]
pub struct HealthPlugin {
    /// Active probe endpoint and caching configuration.
    pub(crate) config: HealthConfig,
    /// Registered probes and drain state.
    pub(crate) registry: HealthRegistry,
}

/// Opt-in [`ServerHook`] answering `/healthz`, `/livez`, and `/readyz` probes.
#[derive(Clone, Copy, Debug, Default)]
#[hyperlane::request_middleware(-110)]
pub struct HealthHook;
