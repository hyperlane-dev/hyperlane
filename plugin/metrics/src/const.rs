/// Default Prometheus scrape endpoint path.
pub const DEFAULT_METRICS_ENDPOINT: &str = "/metrics";

/// Default metric prefix namespace.
pub const DEFAULT_METRICS_NAMESPACE: &str = "hyperlane";

/// Prometheus text exposition format 0.0.4 `Content-Type` header value.
pub const PROMETHEUS_CONTENT_TYPE: &str = "text/plain; version=0.0.4; charset=utf-8";

/// Default latency histogram bucket upper bounds in seconds.
pub const DEFAULT_LATENCY_BUCKETS: [f64; 12] = [
    0.001, 0.005, 0.01, 0.025, 0.05, 0.1, 0.25, 0.5, 1.0, 2.5, 5.0, 10.0,
];

/// Context attribute key storing request start `Instant` for latency observation.
pub(crate) const ATTR_METRICS_STARTED_AT: &str = "hyperlane_plugin_metrics_started_at";

/// `Cache-Control` directive preventing caching of live Prometheus scrapes.
pub(crate) const CACHE_CONTROL_NO_STORE: &str = "no-store";

/// Lowercase `bearer ` prefix for optional `/metrics` scrape authentication.
pub(crate) const PREFIX_BEARER: &str = "bearer ";
