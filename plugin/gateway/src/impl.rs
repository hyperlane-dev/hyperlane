use super::*;

/// Inherent implementation of [`RateLimitConfig`].
impl RateLimitConfig {
    /// Creates a [`RateLimitConfig`] with sustained `requests_per_second` and `burst` capacity.
    ///
    /// # Arguments
    ///
    /// - `u64` - Sustained requests per second.
    /// - `u64` - Maximum burst token capacity.
    ///
    /// # Returns
    ///
    /// - `Self` - Constructed rate-limit parameters.
    pub fn new(requests_per_second: u64, burst: u64) -> Self {
        Self {
            requests_per_second,
            burst,
        }
    }
}

/// Inherent implementation of [`UpstreamTarget`].
impl UpstreamTarget {
    /// Creates an [`UpstreamTarget`] with default load-balancing weight `1`.
    ///
    /// # Arguments
    ///
    /// - `U` - Upstream base URL convertible into `String`.
    ///
    /// # Returns
    ///
    /// - `Self` - Upstream target with weight `1`.
    pub fn new<U>(url: U) -> Self
    where
        U: Into<String>,
    {
        Self {
            url: url.into(),
            weight: 1,
        }
    }

    /// Sets a custom positive load-balancing weight (clamped to `>= 1`).
    ///
    /// # Arguments
    ///
    /// - `u32` - Desired relative weight.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated upstream target.
    pub fn with_weight(mut self, weight: u32) -> Self {
        self.set_weight(weight.max(1));
        self
    }
}

/// Inherent implementation of [`GatewayRoute`].
impl GatewayRoute {
    /// Creates a new [`GatewayRoute`] matching `prefix` and forwarding to `upstreams`.
    ///
    /// # Arguments
    ///
    /// - `P` - Path prefix convertible into `String`.
    /// - `Vec<UpstreamTarget>` - Upstream backend pool for this route.
    ///
    /// # Returns
    ///
    /// - `Self` - Route with default round-robin balancing and 10 s timeout.
    pub fn new<P>(prefix: P, upstreams: Vec<UpstreamTarget>) -> Self
    where
        P: Into<String>,
    {
        Self {
            prefix: prefix.into(),
            strip_prefix: false,
            host: None,
            upstreams,
            strategy: LoadBalanceStrategy::RoundRobin,
            timeout_ms: DEFAULT_UPSTREAM_TIMEOUT_MS,
            max_retries: 0,
            rate_limit: None,
            use_http2: false,
        }
    }

    /// Configures whether the matched prefix is stripped before forwarding upstream.
    ///
    /// # Arguments
    ///
    /// - `bool` - `true` to strip prefix.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated route.
    pub fn with_strip_prefix(mut self, strip_prefix: bool) -> Self {
        self.set_strip_prefix(strip_prefix);
        self
    }

    /// Configures the upstream load-balancing strategy.
    ///
    /// # Arguments
    ///
    /// - `LoadBalanceStrategy` - Desired selection algorithm.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated route.
    pub fn with_strategy(mut self, strategy: LoadBalanceStrategy) -> Self {
        self.set_strategy(strategy);
        self
    }

    /// Configures an optional per-route lock-free GCRA rate limiter.
    ///
    /// # Arguments
    ///
    /// - `RateLimitConfig` - Sustained rate and burst parameters.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated route.
    pub fn with_rate_limit(mut self, rate_limit: RateLimitConfig) -> Self {
        self.set_rate_limit(Some(rate_limit));
        self
    }
}

/// Default construction for [`GatewayConfig`].
impl Default for GatewayConfig {
    /// Constructs a default [`GatewayConfig`] value.
    fn default() -> Self {
        Self {
            routes: Vec::new(),
            max_body_size: DEFAULT_GATEWAY_MAX_BODY_SIZE,
            circuit_failure_threshold: DEFAULT_CIRCUIT_FAILURE_THRESHOLD,
            circuit_reset_ms: DEFAULT_CIRCUIT_RESET_MS,
        }
    }
}

/// Inherent implementation of [`TokenBucket`].
impl TokenBucket {
    /// Builds a lock-free GCRA token-bucket rate limiter from `config`.
    ///
    /// # Arguments
    ///
    /// - `RateLimitConfig` - Refill rate and burst capacity.
    ///
    /// # Returns
    ///
    /// - `Self` - Initialized full lock-free limiter.
    pub(crate) fn new(config: RateLimitConfig) -> Self {
        let rps: u64 = config.get_requests_per_second().max(1);
        let burst: u64 = config.get_burst().max(1);
        let emission_interval_ns: u64 = (NANOS_PER_SECOND / rps).max(1);
        let burst_tolerance_ns: u64 = emission_interval_ns.saturating_mul(burst);
        Self {
            emission_interval_ns,
            burst_tolerance_ns,
            epoch: Instant::now(),
            tat_ns: AtomicU64::new(0),
        }
    }

    /// Attempts to consume one request token via a lock-free atomic CAS loop.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` when permitted; `false` when rate-limited.
    pub(crate) fn try_acquire(&self) -> bool {
        let Self {
            emission_interval_ns,
            burst_tolerance_ns,
            epoch,
            tat_ns,
        } = self;
        let elapsed_ns: u64 = Instant::now()
            .duration_since(*epoch)
            .as_nanos()
            .min(u64::MAX as u128) as u64;
        let max_tat: u64 = elapsed_ns.saturating_add(*burst_tolerance_ns);
        let mut observed: u64 = tat_ns.load(Ordering::Relaxed);
        loop {
            let base: u64 = observed.max(elapsed_ns);
            let next_tat: u64 = base.saturating_add(*emission_interval_ns);
            if next_tat > max_tat {
                return false;
            }
            match tat_ns.compare_exchange_weak(
                observed,
                next_tat,
                Ordering::AcqRel,
                Ordering::Relaxed,
            ) {
                Ok(_) => return true,
                Err(actual) => observed = actual,
            }
        }
    }
}

/// Inherent implementation of [`UpstreamRuntime`].
impl UpstreamRuntime {
    /// Reports whether this upstream's circuit breaker is currently closed or ready to probe.
    ///
    /// # Arguments
    ///
    /// - `u64` - Current Unix timestamp in milliseconds.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` when traffic may be dispatched to this upstream.
    pub(crate) fn is_available(&self, now_ms: u64) -> bool {
        let Self { open_until_ms, .. } = self;
        let open_until: u64 = open_until_ms.load(Ordering::Relaxed);
        open_until == 0 || now_ms >= open_until
    }

    /// Records a successful upstream exchange, resetting failure count and closing the circuit.
    pub(crate) fn record_success(&self) {
        let Self {
            consecutive_failures,
            open_until_ms,
            ..
        } = self;
        consecutive_failures.store(0, Ordering::Relaxed);
        open_until_ms.store(0, Ordering::Relaxed);
    }

    /// Records an upstream failure, opening the circuit when consecutive failures hit `threshold`.
    ///
    /// # Arguments
    ///
    /// - `u32` - Failure threshold required to trip the circuit.
    /// - `u64` - Cooldown duration in milliseconds.
    pub(crate) fn record_failure(&self, threshold: u32, reset_ms: u64) {
        let Self {
            consecutive_failures,
            open_until_ms,
            ..
        } = self;
        let failures: u32 = consecutive_failures.fetch_add(1, Ordering::Relaxed) + 1;
        if threshold > 0 && failures >= threshold {
            let until: u64 = gateway_now_ms().saturating_add(reset_ms);
            open_until_ms.store(until, Ordering::Relaxed);
        }
    }
}

/// Inherent implementation of [`CompiledRoute`].
impl CompiledRoute {
    /// Selects one healthy upstream node without heap allocation according to [`LoadBalanceStrategy`].
    ///
    /// # Arguments
    ///
    /// - `u64` - Current Unix timestamp in milliseconds.
    ///
    /// # Returns
    ///
    /// - `Option<Arc<UpstreamRuntime>>` - Chosen upstream, or `None` when all circuits are open.
    pub(crate) fn select_upstream(&self, now_ms: u64) -> Option<Arc<UpstreamRuntime>> {
        let Self {
            route,
            upstreams,
            cursor,
            ..
        } = self;
        let len: usize = upstreams.len();
        if len == 0 {
            return None;
        }
        match route.get_strategy() {
            LoadBalanceStrategy::RoundRobin => {
                let start: usize = cursor.fetch_add(1, Ordering::Relaxed) % len;
                for offset in 0..len {
                    let candidate: &Arc<UpstreamRuntime> = &upstreams[(start + offset) % len];
                    if candidate.is_available(now_ms) {
                        return Some(Arc::clone(candidate));
                    }
                }
                None
            }
            LoadBalanceStrategy::LeastConnections => upstreams
                .iter()
                .filter(|node: &&Arc<UpstreamRuntime>| node.is_available(now_ms))
                .min_by_key(|node: &&Arc<UpstreamRuntime>| {
                    node.active_conns.load(Ordering::Relaxed)
                })
                .cloned(),
            LoadBalanceStrategy::WeightedRoundRobin => {
                let total_weight: usize = upstreams
                    .iter()
                    .filter(|node: &&Arc<UpstreamRuntime>| node.is_available(now_ms))
                    .map(|node: &Arc<UpstreamRuntime>| node.weight.max(1) as usize)
                    .sum();
                if total_weight == 0 {
                    return None;
                }
                let mut slot: usize = cursor.fetch_add(1, Ordering::Relaxed) % total_weight;
                for node in upstreams {
                    if !node.is_available(now_ms) {
                        continue;
                    }
                    let w: usize = node.weight.max(1) as usize;
                    if slot < w {
                        return Some(Arc::clone(node));
                    }
                    slot -= w;
                }
                None
            }
            LoadBalanceStrategy::PowerOfTwoChoices => {
                if len == 1 {
                    let single: &Arc<UpstreamRuntime> = &upstreams[0];
                    return single.is_available(now_ms).then(|| Arc::clone(single));
                }
                let tick: usize = cursor.fetch_add(1, Ordering::Relaxed);
                let idx_a: usize = tick % len;
                let step: usize = ((tick.wrapping_mul(P2C_MIX_MULTIPLIER) >> 16) % (len - 1)) + 1;
                let idx_b: usize = (idx_a + step) % len;
                let node_a: &Arc<UpstreamRuntime> = &upstreams[idx_a];
                let node_b: &Arc<UpstreamRuntime> = &upstreams[idx_b];
                let avail_a: bool = node_a.is_available(now_ms);
                let avail_b: bool = node_b.is_available(now_ms);
                match (avail_a, avail_b) {
                    (true, true) => {
                        if node_a.active_conns.load(Ordering::Relaxed)
                            <= node_b.active_conns.load(Ordering::Relaxed)
                        {
                            Some(Arc::clone(node_a))
                        } else {
                            Some(Arc::clone(node_b))
                        }
                    }
                    (true, false) => Some(Arc::clone(node_a)),
                    (false, true) => Some(Arc::clone(node_b)),
                    (false, false) => upstreams
                        .iter()
                        .find(|node: &&Arc<UpstreamRuntime>| node.is_available(now_ms))
                        .cloned(),
                }
            }
        }
    }
}

/// Inherent implementation of [`GatewayRuntime`].
impl GatewayRuntime {
    /// Compiles a [`GatewayConfig`] into an immutable longest-prefix-sorted [`GatewayRuntime`].
    ///
    /// # Arguments
    ///
    /// - `GatewayConfig` - Gateway routing and resilience configuration.
    ///
    /// # Returns
    ///
    /// - `Self` - Ready reverse-proxy runtime.
    pub fn new(config: GatewayConfig) -> Self {
        let mut compiled: Vec<CompiledRoute> = config
            .get_routes()
            .iter()
            .map(|route: &GatewayRoute| {
                let normalized_prefix: String = normalize_prefix(route.get_prefix());
                let upstreams: Vec<Arc<UpstreamRuntime>> = route
                    .get_upstreams()
                    .iter()
                    .map(|u: &UpstreamTarget| {
                        Arc::new(UpstreamRuntime {
                            base_url: u.get_url().trim_end_matches('/').to_owned(),
                            weight: u.get_weight().max(1),
                            active_conns: AtomicUsize::new(0),
                            consecutive_failures: AtomicU32::new(0),
                            open_until_ms: AtomicU64::new(0),
                        })
                    })
                    .collect();
                let limiter: Option<TokenBucket> = route.get_rate_limit().map(TokenBucket::new);
                CompiledRoute {
                    route: route.clone(),
                    normalized_prefix,
                    upstreams,
                    cursor: AtomicUsize::new(0),
                    limiter,
                }
            })
            .collect();
        compiled.sort_by(|a: &CompiledRoute, b: &CompiledRoute| {
            b.normalized_prefix
                .len()
                .cmp(&a.normalized_prefix.len())
                .then_with(|| {
                    b.route
                        .get_host()
                        .is_some()
                        .cmp(&a.route.get_host().is_some())
                })
        });
        Self {
            config,
            routes: Arc::new(compiled),
            interceptors: Arc::new(Vec::new()),
        }
    }

    /// Returns a [`GatewayRuntime`] with an additional pluggable [`GatewayInterceptor`].
    ///
    /// # Arguments
    ///
    /// - `I` - Interceptor implementation.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated runtime carrying the interceptor.
    pub fn with_interceptor<I>(self, interceptor: I) -> Self
    where
        I: GatewayInterceptor,
    {
        let Self {
            config,
            routes,
            interceptors,
        } = self;
        let mut list: Vec<SharedGatewayInterceptor> = (*interceptors).clone();
        list.push(Arc::new(interceptor));
        Self {
            config,
            routes,
            interceptors: Arc::new(list),
        }
    }

    /// Returns a reference to the active [`GatewayConfig`].
    ///
    /// # Returns
    ///
    /// - `&GatewayConfig` - Gateway configuration.
    pub fn get_config(&self) -> &GatewayConfig {
        &self.config
    }

    /// Matches the first compiled route accepting `host` and `path`.
    ///
    /// # Arguments
    ///
    /// - `Option<&str>` - Incoming `Host` header value.
    /// - `&str` - Incoming request path.
    ///
    /// # Returns
    ///
    /// - `Option<&CompiledRoute>` - Best matching route, or `None`.
    pub(crate) fn match_route(&self, host: Option<&str>, path: &str) -> Option<&CompiledRoute> {
        let Self { routes, .. } = self;
        for compiled in routes.iter() {
            if let Some(required_host) = compiled.route.get_host() {
                let matches_host: bool = host.is_some_and(|incoming: &str| {
                    incoming.trim().eq_ignore_ascii_case(&required_host)
                });
                if !matches_host {
                    continue;
                }
            }
            if path_matches_prefix(path, &compiled.normalized_prefix) {
                return Some(compiled);
            }
        }
        None
    }

    /// Selects the next healthy upstream URL for a `(host, path)` pair (useful for inspection/testing).
    ///
    /// # Arguments
    ///
    /// - `Option<&str>` - Optional incoming `Host` header value.
    /// - `&str` - Incoming request path.
    ///
    /// # Returns
    ///
    /// - `Option<String>` - Selected upstream base URL, or `None`.
    pub fn select_target_url(&self, host: Option<&str>, path: &str) -> Option<String> {
        let route: &CompiledRoute = self.match_route(host, path)?;
        route
            .select_upstream(gateway_now_ms())
            .map(|node: Arc<UpstreamRuntime>| node.base_url.clone())
    }

    /// Forwards an incoming Hyperlane [`Context`] to the matched route's healthy upstream.
    ///
    /// # Arguments
    ///
    /// - `&mut Context` - Request and response context.
    /// - `Option<&str>` - Optional client IP address for `X-Forwarded-For`.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` when a route matched and populated `ctx.response`; `false` otherwise.
    pub async fn forward(&self, ctx: &mut Context, peer_ip: Option<&str>) -> bool {
        let Self { interceptors, .. } = self;
        let path: String = ctx.get_request().get_path().clone();
        let host_hdr: Option<String> = ctx.get_request().try_get_header_back(HOST);
        let Some(compiled) = self.match_route(host_hdr.as_deref(), &path) else {
            return false;
        };
        if let Some(limiter) = &compiled.limiter
            && !limiter.try_acquire()
        {
            ctx.get_mut_response()
                .set_status_code(HttpStatus::TooManyRequests.code())
                .set_body(Vec::new());
            return true;
        }
        if ctx.get_request().get_body().len() > self.get_config().get_max_body_size() {
            ctx.get_mut_response()
                .set_status_code(HttpStatus::PayloadTooLarge.code())
                .set_body(Vec::new());
            return true;
        }
        let Some(upstream) = compiled.select_upstream(gateway_now_ms()) else {
            ctx.get_mut_response()
                .set_status_code(HttpStatus::ServiceUnavailable.code())
                .set_body(Vec::new());
            return true;
        };
        let mut query: String = String::new();
        for (k, vals) in ctx.get_request().get_querys() {
            for v in vals {
                if !query.is_empty() {
                    query.push('&');
                }
                query.push_str(k);
                if !v.is_empty() {
                    query.push('=');
                    query.push_str(v);
                }
            }
        }
        let target_url: String = build_upstream_url(
            &upstream.base_url,
            &path,
            &query,
            &compiled.normalized_prefix,
            compiled.route.get_strip_prefix(),
        );
        for interceptor in interceptors.iter() {
            if !interceptor.before_forward(ctx, &target_url) {
                return true;
            }
        }
        let req_body: Vec<u8> = ctx.get_request().get_body().clone();
        let conn_vals: Vec<String> = ctx
            .get_request()
            .get_headers()
            .iter()
            .filter(|(k, _): &(&String, &RequestHeadersValue)| k.eq_ignore_ascii_case(CONNECTION))
            .flat_map(|(_, vals): (&String, &RequestHeadersValue)| vals.iter().cloned())
            .collect();
        let extra_hop: HashSet<String> = if conn_vals.is_empty() {
            HashSet::new()
        } else {
            extract_connection_header_tokens(&conn_vals)
        };
        let mut forward_headers: HashMap<String, String> =
            HashMap::with_capacity(ctx.get_request().get_headers().len() + 3);
        for (name, values) in ctx.get_request().get_headers() {
            if name.eq_ignore_ascii_case(HOST)
                || name.eq_ignore_ascii_case(HEADER_CONTENT_LENGTH)
                || is_hop_by_hop_header(name, &extra_hop)
            {
                continue;
            }
            if let Some(last) = values.back() {
                forward_headers.insert(name.to_ascii_lowercase(), last.clone());
            }
        }
        if let Some(client_ip) = peer_ip
            && !client_ip.is_empty()
        {
            let xff: String = match forward_headers.get(HEADER_X_FORWARDED_FOR) {
                Some(prev) if !prev.is_empty() => format!("{prev}, {client_ip}"),
                _ => client_ip.to_owned(),
            };
            forward_headers.insert(HEADER_X_FORWARDED_FOR.to_owned(), xff);
        }
        if let Some(original_host) = host_hdr {
            forward_headers
                .entry(HEADER_X_FORWARDED_HOST.to_owned())
                .or_insert(original_host);
        }
        forward_headers
            .entry(HEADER_X_FORWARDED_PROTO.to_owned())
            .or_insert_with(|| PROTO_HTTP.to_owned());
        let mut builder: RequestBuilder = RequestBuilder::new();
        builder
            .method(ctx.get_request().get_method().clone())
            .url(target_url)
            .headers(forward_headers)
            .body(req_body)
            .timeout(compiled.route.get_timeout_ms())
            .retry(compiled.route.get_max_retries())
            .keep_alive(true);
        if compiled.route.get_use_http2() {
            builder.http2_only();
        }
        let mut upstream_req: HttpRequest = builder.build();
        upstream.active_conns.fetch_add(1, Ordering::Relaxed);
        let send_res: Result<HttpResponse, RequestError> = upstream_req.send_async().await;
        upstream.active_conns.fetch_sub(1, Ordering::Relaxed);
        match send_res {
            Ok(resp) => {
                if resp.get_status_code() >= 502 && resp.get_status_code() <= 504 {
                    upstream.record_failure(
                        self.get_config().get_circuit_failure_threshold(),
                        self.get_config().get_circuit_reset_ms(),
                    );
                } else {
                    upstream.record_success();
                }
                let resp_conn_vals: Vec<String> = resp
                    .get_headers()
                    .iter()
                    .filter(|(k, _): &(&String, &String)| k.eq_ignore_ascii_case(CONNECTION))
                    .map(|(_, v): (&String, &String)| v.clone())
                    .collect();
                let resp_extra_hop: HashSet<String> = if resp_conn_vals.is_empty() {
                    HashSet::new()
                } else {
                    extract_connection_header_tokens(&resp_conn_vals)
                };
                let out_resp: &mut Response = ctx.get_mut_response();
                out_resp.set_status_code(resp.get_status_code());
                for (name, value) in resp.get_headers() {
                    if name.eq_ignore_ascii_case(HEADER_CONTENT_LENGTH)
                        || is_hop_by_hop_header(name, &resp_extra_hop)
                    {
                        continue;
                    }
                    out_resp.set_header(name, value);
                }
                out_resp
                    .set_header(HEADER_X_GATEWAY_UPSTREAM, &upstream.base_url)
                    .set_body(resp.bytes());
                for interceptor in interceptors.iter() {
                    interceptor.after_forward(ctx, &upstream.base_url);
                }
            }
            Err(_) => {
                upstream.record_failure(
                    self.get_config().get_circuit_failure_threshold(),
                    self.get_config().get_circuit_reset_ms(),
                );
                ctx.get_mut_response()
                    .set_status_code(HttpStatus::BadGateway.code())
                    .set_body(Vec::new());
            }
        }
        true
    }
}

/// Inherent implementation of [`GatewayPlugin`].
impl GatewayPlugin {
    /// Initializes the global [`GatewayPlugin`] once with `config`.
    ///
    /// # Arguments
    ///
    /// - `GatewayConfig` - Reverse-proxy and resilience configuration.
    ///
    /// # Returns
    ///
    /// - `&'static Self` - Reference to the initialized global plugin.
    pub fn init(config: GatewayConfig) -> &'static Self {
        GATEWAY_PLUGIN.get_or_init(|| Self {
            runtime: GatewayRuntime::new(config),
        })
    }

    /// Initializes the global [`GatewayPlugin`] once with a prebuilt [`GatewayRuntime`].
    ///
    /// # Arguments
    ///
    /// - `GatewayRuntime` - Compiled gateway runtime (including any attached interceptors).
    ///
    /// # Returns
    ///
    /// - `&'static Self` - Reference to the initialized global plugin.
    pub fn init_runtime(runtime: GatewayRuntime) -> &'static Self {
        GATEWAY_PLUGIN.get_or_init(|| Self { runtime })
    }

    /// Returns the initialized global [`GatewayPlugin`], if [`GatewayPlugin::init`] was called.
    ///
    /// # Returns
    ///
    /// - `Option<&'static Self>` - Reference to the global plugin, or `None`.
    pub fn get() -> Option<&'static Self> {
        GATEWAY_PLUGIN.get()
    }

    /// Returns the compiled [`GatewayRuntime`].
    ///
    /// # Returns
    ///
    /// - `&GatewayRuntime` - Active reverse-proxy runtime.
    pub fn get_runtime(&self) -> &GatewayRuntime {
        &self.runtime
    }
}

/// Implements [`ServerHook`] for [`GatewayHook`].
impl ServerHook for GatewayHook {
    /// Constructs a per-request [`GatewayHook`].
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

    /// Forwards matched requests to configured upstreams via [`GatewayPlugin`].
    ///
    /// # Arguments
    ///
    /// - `Self` - Hook instance.
    /// - `&mut Stream` - Connection stream.
    /// - `&mut Context` - Request context.
    ///
    /// # Returns
    ///
    /// - `Status` - `Status::Continue` when unconfigured or no route matches; `Status::Reject` once forwarded.
    async fn handle(self, stream: &mut Stream, ctx: &mut Context) -> Status {
        let Some(plugin) = GatewayPlugin::get() else {
            return Status::Continue;
        };
        let peer_ip: Option<String> = stream
            .get_peer_addr()
            .ok()
            .map(|addr: SocketAddr| addr.ip().to_string());
        if plugin.get_runtime().forward(ctx, peer_ip.as_deref()).await {
            let wire: ResponseData = ctx.get_mut_response().build();
            let _: Result<(), ResponseError> = stream.try_send(&wire).await;
            return Status::Reject;
        }
        Status::Continue
    }
}
