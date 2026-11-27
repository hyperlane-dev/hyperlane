/// Default combined health endpoint path.
pub const DEFAULT_HEALTH_PATH: &str = "/healthz";

/// Default liveness probe endpoint path.
pub const DEFAULT_LIVE_PATH: &str = "/livez";

/// Default readiness probe endpoint path.
pub const DEFAULT_READY_PATH: &str = "/readyz";

/// Default per-check execution timeout in milliseconds (2 seconds).
pub const DEFAULT_HEALTH_CHECK_TIMEOUT_MS: u64 = 2_000;

/// Default health report cache TTL in milliseconds (`0` disables caching).
pub const DEFAULT_HEALTH_CACHE_TTL_MS: u64 = 0;

/// `Cache-Control` directive preventing intermediaries from caching probe responses.
pub(crate) const CACHE_CONTROL_NO_STORE: &str = "no-store";

/// Detail text recorded when an individual health check exceeds its timeout.
pub(crate) const DETAIL_PROBE_TIMEOUT: &str = "probe timed out";

/// Detail text recorded when a probe task failed to join.
pub(crate) const DETAIL_PROBE_PANICKED: &str = "probe task panicked";
