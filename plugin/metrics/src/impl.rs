use super::*;

/// Inherent implementation of [`Counter`].
impl Counter {
    /// Creates a new zero-initialized [`Counter`].
    ///
    /// # Returns
    ///
    /// - `Self` - Counter initialized to `0`.
    pub fn new() -> Self {
        Self {
            value: AtomicU64::new(0),
        }
    }

    /// Increments the counter by `1`.
    pub fn inc(&self) {
        self.inc_by(1);
    }

    /// Increments the counter by `delta`.
    ///
    /// # Arguments
    ///
    /// - `u64` - Increment amount.
    pub fn inc_by(&self, delta: u64) {
        let Self { value } = self;
        value.fetch_add(delta, Ordering::Relaxed);
    }

    /// Reads the current counter value.
    ///
    /// # Returns
    ///
    /// - `u64` - Cumulative count.
    pub fn get(&self) -> u64 {
        let Self { value } = self;
        value.load(Ordering::Relaxed)
    }
}

/// Inherent implementation of [`Gauge`].
impl Gauge {
    /// Creates a new zero-initialized [`Gauge`].
    ///
    /// # Returns
    ///
    /// - `Self` - Gauge initialized to `0`.
    pub fn new() -> Self {
        Self {
            value: AtomicI64::new(0),
        }
    }

    /// Increments the gauge by `1`.
    pub fn inc(&self) {
        let Self { value } = self;
        value.fetch_add(1, Ordering::Relaxed);
    }

    /// Decrements the gauge by `1`.
    pub fn dec(&self) {
        let Self { value } = self;
        value.fetch_sub(1, Ordering::Relaxed);
    }

    /// Adds signed `delta` to the gauge.
    ///
    /// # Arguments
    ///
    /// - `i64` - Signed delta.
    pub fn add(&self, delta: i64) {
        let Self { value } = self;
        value.fetch_add(delta, Ordering::Relaxed);
    }

    /// Overwrites the gauge value atomically.
    ///
    /// # Arguments
    ///
    /// - `i64` - New gauge value.
    pub fn set(&self, next: i64) {
        let Self { value } = self;
        value.store(next, Ordering::Relaxed);
    }

    /// Reads the current gauge value.
    ///
    /// # Returns
    ///
    /// - `i64` - Current value.
    pub fn get(&self) -> i64 {
        let Self { value } = self;
        value.load(Ordering::Relaxed)
    }
}

/// Inherent implementation of [`Histogram`].
impl Histogram {
    /// Creates a lock-free [`Histogram`] with the provided upper bucket `bounds`.
    ///
    /// # Arguments
    ///
    /// - `&[f64]` - Finite bucket upper bounds (sorted and deduplicated automatically).
    ///
    /// # Returns
    ///
    /// - `Self` - Ready atomic histogram.
    pub fn new(bounds: &[f64]) -> Self {
        let mut sorted: Vec<f64> = bounds
            .iter()
            .copied()
            .filter(|v: &f64| v.is_finite())
            .collect();
        sorted.sort_by(|a: &f64, b: &f64| a.total_cmp(b));
        sorted.dedup();
        let mut bucket_counts: Vec<AtomicU64> = Vec::with_capacity(sorted.len() + 1);
        for _ in 0..=sorted.len() {
            bucket_counts.push(AtomicU64::new(0));
        }
        Self {
            bounds: sorted,
            bucket_counts,
            total_count: AtomicU64::new(0),
            sum_bits: AtomicU64::new(0.0_f64.to_bits()),
        }
    }

    /// Records a single floating-point observation in `O(log B)` without locks.
    ///
    /// # Arguments
    ///
    /// - `f64` - Observed value (for example latency in seconds).
    pub fn observe(&self, sample: f64) {
        if !sample.is_finite() {
            return;
        }
        let Self {
            bounds,
            bucket_counts,
            total_count,
            sum_bits,
        } = self;
        let idx: usize = bounds.partition_point(|bound: &f64| *bound < sample);
        bucket_counts[idx].fetch_add(1, Ordering::Relaxed);
        total_count.fetch_add(1, Ordering::Relaxed);
        atomic_f64_add(sum_bits, sample);
    }

    /// Returns total recorded observations.
    ///
    /// # Returns
    ///
    /// - `u64` - Observation count.
    pub fn count(&self) -> u64 {
        let Self { total_count, .. } = self;
        total_count.load(Ordering::Relaxed)
    }

    /// Returns the cumulative sum of all recorded observations.
    ///
    /// # Returns
    ///
    /// - `f64` - Observation sum.
    pub fn sum(&self) -> f64 {
        let Self { sum_bits, .. } = self;
        f64::from_bits(sum_bits.load(Ordering::Relaxed))
    }

    /// Appends Prometheus cumulative `_bucket`, `_sum`, and `_count` lines for `name` into `out`.
    ///
    /// # Arguments
    ///
    /// - `&str` - Metric name.
    /// - `&str` - Help text.
    /// - `&mut String` - Destination Prometheus exposition buffer.
    pub(crate) fn write_prometheus(&self, name: &str, help: &str, out: &mut String) {
        let Self {
            bounds,
            bucket_counts,
            ..
        } = self;
        let _: fmt::Result = writeln!(out, "# HELP {name} {help}");
        let _: fmt::Result = writeln!(out, "# TYPE {name} histogram");
        let mut cumulative: u64 = 0;
        for (idx, bound) in bounds.iter().enumerate() {
            cumulative += bucket_counts[idx].load(Ordering::Relaxed);
            let _: fmt::Result = writeln!(out, "{name}_bucket{{le=\"{bound}\"}} {cumulative}");
        }
        let total: u64 = self.count();
        let _: fmt::Result = writeln!(out, "{name}_bucket{{le=\"+Inf\"}} {total}");
        let _: fmt::Result = writeln!(out, "{name}_sum {}", self.sum());
        let _: fmt::Result = writeln!(out, "{name}_count {total}");
    }
}

/// Default construction for [`MetricsConfig`].
impl Default for MetricsConfig {
    /// Constructs a default [`MetricsConfig`] exposing `/metrics` under namespace `"hyperlane"`.
    fn default() -> Self {
        Self {
            endpoint: DEFAULT_METRICS_ENDPOINT.to_owned(),
            namespace: DEFAULT_METRICS_NAMESPACE.to_owned(),
            bearer_token: None,
        }
    }
}

/// Inherent builders for [`MetricsConfig`].
impl MetricsConfig {
    /// Sets the scrape endpoint path.
    ///
    /// # Arguments
    ///
    /// - `E` - Path convertible into `String`.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated configuration.
    pub fn with_endpoint<E>(mut self, endpoint: E) -> Self
    where
        E: Into<String>,
    {
        self.set_endpoint(endpoint.into());
        self
    }

    /// Sets the metric namespace prefix.
    ///
    /// # Arguments
    ///
    /// - `N` - Namespace convertible into `String`.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated configuration.
    pub fn with_namespace<N>(mut self, namespace: N) -> Self
    where
        N: Into<String>,
    {
        self.set_namespace(namespace.into());
        self
    }

    /// Requires `Authorization: Bearer <token>` on `/metrics` scrapes.
    ///
    /// # Arguments
    ///
    /// - `T` - Secret bearer token convertible into `String`.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated configuration.
    pub fn with_bearer_token<T>(mut self, token: T) -> Self
    where
        T: Into<String>,
    {
        self.set_bearer_token(Some(token.into()));
        self
    }
}

/// Default construction for [`MetricsRegistry`].
impl Default for MetricsRegistry {
    /// Constructs a default [`MetricsRegistry`] under namespace `"hyperlane"`.
    fn default() -> Self {
        Self::new(DEFAULT_METRICS_NAMESPACE)
    }
}

/// Inherent implementation of [`MetricsRegistry`].
impl MetricsRegistry {
    /// Creates a new lock-free [`MetricsRegistry`] using `namespace` for built-in HTTP metrics.
    ///
    /// # Arguments
    ///
    /// - `N` - Metric namespace prefix convertible into `String`.
    ///
    /// # Returns
    ///
    /// - `Self` - Ready metrics registry.
    pub fn new<N>(namespace: N) -> Self
    where
        N: Into<String>,
    {
        Self {
            inner: Arc::new(MetricsRegistryInner {
                namespace: namespace.into(),
                status_class_counts: [
                    Counter::new(),
                    Counter::new(),
                    Counter::new(),
                    Counter::new(),
                    Counter::new(),
                    Counter::new(),
                ],
                in_flight: Gauge::new(),
                response_bytes_total: Counter::new(),
                duration_seconds: Histogram::new(&DEFAULT_LATENCY_BUCKETS),
                custom_counters: RwLock::new(hash_map_xx_hash3_64()),
                custom_gauges: RwLock::new(hash_map_xx_hash3_64()),
                custom_histograms: RwLock::new(hash_map_xx_hash3_64()),
            }),
        }
    }

    /// Registers or retrieves a custom lock-free [`Counter`] handle.
    ///
    /// # Arguments
    ///
    /// - `N` - Metric name.
    /// - `H` - Prometheus `# HELP` description.
    ///
    /// # Returns
    ///
    /// - `Arc<Counter>` - Lock-free counter handle.
    pub fn register_counter<N, H>(&self, name: N, help: H) -> Arc<Counter>
    where
        N: Into<String>,
        H: Into<String>,
    {
        let Self { inner } = self;
        let key: String = name.into();
        if let Ok(read_map) = inner.custom_counters.read()
            && let Some(entry) = read_map.get(&key)
        {
            return Arc::clone(&entry.metric);
        }
        let metric: Arc<Counter> = Arc::new(Counter::new());
        if let Ok(mut write_map) = inner.custom_counters.write() {
            let entry: &mut RegisteredCounter =
                write_map.entry(key).or_insert_with(|| RegisteredCounter {
                    help: help.into(),
                    metric: Arc::clone(&metric),
                });
            return Arc::clone(&entry.metric);
        }
        metric
    }

    /// Registers or retrieves a custom lock-free [`Gauge`] handle.
    ///
    /// # Arguments
    ///
    /// - `N` - Metric name.
    /// - `H` - Prometheus `# HELP` description.
    ///
    /// # Returns
    ///
    /// - `Arc<Gauge>` - Lock-free gauge handle.
    pub fn register_gauge<N, H>(&self, name: N, help: H) -> Arc<Gauge>
    where
        N: Into<String>,
        H: Into<String>,
    {
        let Self { inner } = self;
        let key: String = name.into();
        if let Ok(read_map) = inner.custom_gauges.read()
            && let Some(entry) = read_map.get(&key)
        {
            return Arc::clone(&entry.metric);
        }
        let metric: Arc<Gauge> = Arc::new(Gauge::new());
        if let Ok(mut write_map) = inner.custom_gauges.write() {
            let entry: &mut RegisteredGauge =
                write_map.entry(key).or_insert_with(|| RegisteredGauge {
                    help: help.into(),
                    metric: Arc::clone(&metric),
                });
            return Arc::clone(&entry.metric);
        }
        metric
    }

    /// Registers or retrieves a custom lock-free [`Histogram`] handle.
    ///
    /// # Arguments
    ///
    /// - `N` - Metric name.
    /// - `H` - Prometheus `# HELP` description.
    /// - `&[f64]` - Bucket upper bounds.
    ///
    /// # Returns
    ///
    /// - `Arc<Histogram>` - Lock-free histogram handle.
    pub fn register_histogram<N, H>(&self, name: N, help: H, bounds: &[f64]) -> Arc<Histogram>
    where
        N: Into<String>,
        H: Into<String>,
    {
        let Self { inner } = self;
        let key: String = name.into();
        if let Ok(read_map) = inner.custom_histograms.read()
            && let Some(entry) = read_map.get(&key)
        {
            return Arc::clone(&entry.metric);
        }
        let metric: Arc<Histogram> = Arc::new(Histogram::new(bounds));
        if let Ok(mut write_map) = inner.custom_histograms.write() {
            let entry: &mut RegisteredHistogram =
                write_map.entry(key).or_insert_with(|| RegisteredHistogram {
                    help: help.into(),
                    metric: Arc::clone(&metric),
                });
            return Arc::clone(&entry.metric);
        }
        metric
    }

    /// Increments the built-in in-flight HTTP request gauge.
    pub fn record_request_start(&self) {
        let Self { inner } = self;
        inner.in_flight.inc();
    }

    /// Records a completed HTTP exchange in lock-free built-in counters and latency histogram.
    ///
    /// # Arguments
    ///
    /// - `usize` - HTTP status code.
    /// - `usize` - Response body length in bytes.
    /// - `f64` - Request duration in seconds.
    pub fn record_request_finish(
        &self,
        status_code: usize,
        response_bytes: usize,
        duration_secs: f64,
    ) {
        let Self { inner } = self;
        inner.in_flight.dec();
        let class_idx: usize = match status_code {
            100..=199 => 0,
            200..=299 => 1,
            300..=399 => 2,
            400..=499 => 3,
            500..=599 => 4,
            _ => 5,
        };
        inner.status_class_counts[class_idx].inc();
        inner.response_bytes_total.inc_by(response_bytes as u64);
        inner.duration_seconds.observe(duration_secs);
    }

    /// Returns total HTTP requests observed across all status classes.
    ///
    /// # Returns
    ///
    /// - `u64` - Cumulative request count.
    pub fn total_http_requests(&self) -> u64 {
        let Self { inner } = self;
        inner.status_class_counts.iter().map(Counter::get).sum()
    }

    /// Renders all built-in and custom metrics into Prometheus text exposition format 0.0.4.
    ///
    /// # Returns
    ///
    /// - `String` - Prometheus scrape body.
    pub fn render_prometheus(&self) -> String {
        let Self { inner } = self;
        let ns: &str = &inner.namespace;
        let mut out: String = String::with_capacity(2048);
        let _: fmt::Result = writeln!(
            out,
            "# HELP {ns}_http_requests_total Total HTTP requests partitioned by status class."
        );
        let _: fmt::Result = writeln!(out, "# TYPE {ns}_http_requests_total counter");
        let labels: [&str; 6] = ["1xx", "2xx", "3xx", "4xx", "5xx", "other"];
        for (idx, label) in labels.iter().enumerate() {
            let count: u64 = inner.status_class_counts[idx].get();
            let _: fmt::Result = writeln!(
                out,
                "{ns}_http_requests_total{{status_class=\"{label}\"}} {count}"
            );
        }
        let _: fmt::Result = writeln!(
            out,
            "# HELP {ns}_http_requests_in_flight Current number of active in-flight HTTP requests."
        );
        let _: fmt::Result = writeln!(out, "# TYPE {ns}_http_requests_in_flight gauge");
        let _: fmt::Result = writeln!(
            out,
            "{ns}_http_requests_in_flight {}",
            inner.in_flight.get()
        );
        let _: fmt::Result = writeln!(
            out,
            "# HELP {ns}_http_response_bytes_total Cumulative HTTP response body bytes sent."
        );
        let _: fmt::Result = writeln!(out, "# TYPE {ns}_http_response_bytes_total counter");
        let _: fmt::Result = writeln!(
            out,
            "{ns}_http_response_bytes_total {}",
            inner.response_bytes_total.get()
        );
        let dur_name: String = format!("{ns}_http_request_duration_seconds");
        inner.duration_seconds.write_prometheus(
            &dur_name,
            "HTTP request latency distribution in seconds.",
            &mut out,
        );
        if let Ok(counters) = inner.custom_counters.read() {
            for (name, entry) in counters.iter() {
                let _: fmt::Result = writeln!(out, "# HELP {name} {}", entry.help);
                let _: fmt::Result = writeln!(out, "# TYPE {name} counter");
                let _: fmt::Result = writeln!(out, "{name} {}", entry.metric.get());
            }
        }
        if let Ok(gauges) = inner.custom_gauges.read() {
            for (name, entry) in gauges.iter() {
                let _: fmt::Result = writeln!(out, "# HELP {name} {}", entry.help);
                let _: fmt::Result = writeln!(out, "# TYPE {name} gauge");
                let _: fmt::Result = writeln!(out, "{name} {}", entry.metric.get());
            }
        }
        if let Ok(histograms) = inner.custom_histograms.read() {
            for (name, entry) in histograms.iter() {
                entry.metric.write_prometheus(name, &entry.help, &mut out);
            }
        }
        out
    }
}

/// Inherent implementation of [`MetricsPlugin`].
impl MetricsPlugin {
    /// Initializes the global [`MetricsPlugin`] once with `config` and an automatically created registry.
    ///
    /// # Arguments
    ///
    /// - `MetricsConfig` - Scrape endpoint, namespace, and optional auth settings.
    ///
    /// # Returns
    ///
    /// - `&'static Self` - Reference to the initialized global plugin.
    pub fn init(config: MetricsConfig) -> &'static Self {
        let registry: MetricsRegistry = MetricsRegistry::new(config.get_namespace().clone());
        Self::init_with_registry(config, registry)
    }

    /// Initializes the global [`MetricsPlugin`] once with `config` and a pre-created [`MetricsRegistry`].
    ///
    /// # Arguments
    ///
    /// - `MetricsConfig` - Scrape endpoint and auth settings.
    /// - `MetricsRegistry` - Shared metrics registry.
    ///
    /// # Returns
    ///
    /// - `&'static Self` - Reference to the initialized global plugin.
    pub fn init_with_registry(config: MetricsConfig, registry: MetricsRegistry) -> &'static Self {
        METRICS_PLUGIN.get_or_init(|| Self { config, registry })
    }

    /// Returns the initialized global [`MetricsPlugin`], if initialized.
    ///
    /// # Returns
    ///
    /// - `Option<&'static Self>` - Reference to the global plugin, or `None`.
    pub fn get() -> Option<&'static Self> {
        METRICS_PLUGIN.get()
    }

    /// Returns the active [`MetricsConfig`].
    ///
    /// # Returns
    ///
    /// - `&MetricsConfig` - Active metrics configuration.
    pub fn get_config(&self) -> &MetricsConfig {
        &self.config
    }

    /// Returns the shared lock-free [`MetricsRegistry`].
    ///
    /// # Returns
    ///
    /// - `&MetricsRegistry` - Active metrics registry.
    pub fn get_registry(&self) -> &MetricsRegistry {
        &self.registry
    }
}

/// Implements [`ServerHook`] for [`MetricsRequestHook`].
impl ServerHook for MetricsRequestHook {
    /// Constructs a per-request [`MetricsRequestHook`].
    ///
    /// # Arguments
    ///
    /// - `&mut Stream` - Connection stream.
    /// - `&mut Context` - Request context.
    ///
    /// # Returns
    ///
    /// - `Self` - Hook instance.
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    /// Answers `/metrics` scrapes or starts request timing and increments in-flight gauge.
    ///
    /// # Arguments
    ///
    /// - `Self` - Hook instance.
    /// - `&mut Stream` - Connection stream.
    /// - `&mut Context` - Request context.
    ///
    /// # Returns
    ///
    /// - `Status` - `Status::Reject` after answering `/metrics`; `Status::Continue` for normal requests.
    async fn handle(self, stream: &mut Stream, ctx: &mut Context) -> Status {
        let Some(plugin) = MetricsPlugin::get() else {
            return Status::Continue;
        };
        let config: &MetricsConfig = plugin.get_config();
        if ctx.get_request().get_path() == config.get_endpoint() {
            if !is_scrape_authorized(ctx, config.try_get_bearer_token().as_deref()) {
                ctx.get_mut_response()
                    .set_status_code(HttpStatus::Unauthorized.code())
                    .set_body(Vec::new());
            } else {
                let body: Vec<u8> = if ctx.get_request().get_method().is_head() {
                    Vec::new()
                } else {
                    plugin.get_registry().render_prometheus().into_bytes()
                };
                ctx.get_mut_response()
                    .set_status_code(HttpStatus::Ok.code())
                    .set_header(CONTENT_TYPE, PROMETHEUS_CONTENT_TYPE)
                    .set_header(CACHE_CONTROL, CACHE_CONTROL_NO_STORE)
                    .set_body(body);
            }
            let wire: ResponseData = ctx.get_mut_response().build();
            let _: Result<(), ResponseError> = stream.try_send(&wire).await;
            return Status::Reject;
        }
        plugin.get_registry().record_request_start();
        ctx.set_attribute(ATTR_METRICS_STARTED_AT, Instant::now());
        Status::Continue
    }
}

/// Implements [`ServerHook`] for [`MetricsResponseHook`].
impl ServerHook for MetricsResponseHook {
    /// Constructs a per-response [`MetricsResponseHook`].
    ///
    /// # Arguments
    ///
    /// - `&mut Stream` - Connection stream.
    /// - `&mut Context` - Request context.
    ///
    /// # Returns
    ///
    /// - `Self` - Hook instance.
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    /// Records HTTP status class, response payload bytes, and latency histogram observation.
    ///
    /// # Arguments
    ///
    /// - `Self` - Hook instance.
    /// - `&mut Stream` - Connection stream.
    /// - `&mut Context` - Request context.
    ///
    /// # Returns
    ///
    /// - `Status` - Always `Status::Continue`.
    async fn handle(self, _: &mut Stream, ctx: &mut Context) -> Status {
        let Some(plugin) = MetricsPlugin::get() else {
            return Status::Continue;
        };
        let Some(started_at) = ctx.try_get_attribute::<Instant, _>(ATTR_METRICS_STARTED_AT) else {
            return Status::Continue;
        };
        let status: usize = ctx.get_response().get_status_code();
        let bytes: usize = ctx.get_response().get_body().len();
        let duration_secs: f64 = started_at.elapsed().as_secs_f64();
        plugin
            .get_registry()
            .record_request_finish(status, bytes, duration_secs);
        Status::Continue
    }
}
