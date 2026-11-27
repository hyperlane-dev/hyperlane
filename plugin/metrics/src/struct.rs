use super::*;

/// Lock-free monotonically increasing 64-bit counter metric.
#[derive(Debug, Default)]
pub struct Counter {
    /// Atomic counter value.
    pub(crate) value: AtomicU64,
}

/// Lock-free signed 64-bit gauge metric that may increment or decrement.
#[derive(Debug, Default)]
pub struct Gauge {
    /// Atomic gauge value.
    pub(crate) value: AtomicI64,
}

/// Lock-free fixed-bucket latency/size histogram metric.
#[derive(Debug)]
pub struct Histogram {
    /// Sorted finite upper bucket bounds.
    pub(crate) bounds: Vec<f64>,
    /// Per-bucket observation counts (`bounds.len() + 1` including `+Inf`).
    pub(crate) bucket_counts: Vec<AtomicU64>,
    /// Total count of all observations.
    pub(crate) total_count: AtomicU64,
    /// IEEE-754 `f64::to_bits` cumulative sum of observed values.
    pub(crate) sum_bits: AtomicU64,
}

/// Registered custom counter entry with Prometheus help text.
pub(crate) struct RegisteredCounter {
    /// Human-readable `# HELP` string.
    pub(crate) help: String,
    /// Lock-free atomic counter handle.
    pub(crate) metric: Arc<Counter>,
}

/// Registered custom gauge entry with Prometheus help text.
pub(crate) struct RegisteredGauge {
    /// Human-readable `# HELP` string.
    pub(crate) help: String,
    /// Lock-free atomic gauge handle.
    pub(crate) metric: Arc<Gauge>,
}

/// Registered custom histogram entry with Prometheus help text.
pub(crate) struct RegisteredHistogram {
    /// Human-readable `# HELP` string.
    pub(crate) help: String,
    /// Lock-free atomic histogram handle.
    pub(crate) metric: Arc<Histogram>,
}

/// Configuration for [`MetricsPlugin`] and [`MetricsRequestHook`].
#[derive(Clone, Data, Debug, Eq, PartialEq)]
pub struct MetricsConfig {
    /// HTTP path serving Prometheus text format 0.0.4 (default `"/metrics"`).
    pub(crate) endpoint: String,
    /// Metric name prefix namespace (default `"hyperlane"`).
    pub(crate) namespace: String,
    /// Optional bearer token required to scrape the metrics endpoint.
    #[get(type(clone))]
    pub(crate) bearer_token: Option<String>,
}

/// Shared interior state of [`MetricsRegistry`].
pub(crate) struct MetricsRegistryInner {
    /// Namespace prefix applied to built-in HTTP metrics.
    pub(crate) namespace: String,
    /// Lock-free HTTP request counters partitioned by status class (`[1xx, 2xx, 3xx, 4xx, 5xx, other]`).
    pub(crate) status_class_counts: [Counter; 6],
    /// Lock-free gauge tracking active in-flight HTTP requests.
    pub(crate) in_flight: Gauge,
    /// Lock-free counter tracking total response payload bytes emitted.
    pub(crate) response_bytes_total: Counter,
    /// Lock-free HTTP request latency histogram in seconds.
    pub(crate) duration_seconds: Histogram,
    /// Custom user-registered counters.
    pub(crate) custom_counters: RwLock<HashMapXxHash3_64<String, RegisteredCounter>>,
    /// Custom user-registered gauges.
    pub(crate) custom_gauges: RwLock<HashMapXxHash3_64<String, RegisteredGauge>>,
    /// Custom user-registered histograms.
    pub(crate) custom_histograms: RwLock<HashMapXxHash3_64<String, RegisteredHistogram>>,
}

/// Lock-free HTTP and custom application metrics registry with Prometheus 0.0.4 exposition.
#[derive(Clone)]
pub struct MetricsRegistry {
    /// Shared registry state.
    pub(crate) inner: Arc<MetricsRegistryInner>,
}

/// Process-wide [`MetricsPlugin`] singleton storing [`MetricsConfig`] and [`MetricsRegistry`].
#[derive(Clone)]
pub struct MetricsPlugin {
    /// Endpoint and scrape-auth configuration.
    pub(crate) config: MetricsConfig,
    /// Shared lock-free metrics registry.
    pub(crate) registry: MetricsRegistry,
}

/// Opt-in request middleware incrementing in-flight gauges and serving `/metrics` scrapes.
#[derive(Clone, Copy, Debug, Default)]
#[hyperlane::request_middleware(-115)]
pub struct MetricsRequestHook;

/// Opt-in response middleware recording status class, response bytes, and request duration.
#[derive(Clone, Copy, Debug, Default)]
#[hyperlane::response_middleware(95)]
pub struct MetricsResponseHook;
