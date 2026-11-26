use super::*;

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
    /// Builds a token-bucket rate limiter from `config`.
    ///
    /// # Arguments
    ///
    /// - `RateLimitConfig` - Refill rate and burst capacity.
    ///
    /// # Returns
    ///
    /// - `Self` - Initialized full token bucket.
    pub(crate) fn new(config: RateLimitConfig) -> Self {
        let cap: f64 = config.get_burst().max(1) as f64;
        Self {
            rate_per_sec: config.get_requests_per_second().max(1) as f64,
            capacity: cap,
            state: Mutex::new(TokenBucketState {
                tokens: cap,
                last_refill: Instant::now(),
            }),
        }
    }

    /// Attempts to consume one request token, refilling proportionally to elapsed time.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` when permitted; `false` when rate-limited.
    pub(crate) fn try_acquire(&self) -> bool {
        let Self {
            rate_per_sec,
            capacity,
            state,
        } = self;
        let Ok(mut guard) = state.lock() else {
            return true;
        };
        let now: Instant = Instant::now();
        let elapsed_secs: f64 = now.duration_since(guard.last_refill).as_secs_f64();
        guard.tokens = (guard.tokens + elapsed_secs * *rate_per_sec).min(*capacity);
        guard.last_refill = now;
        if guard.tokens >= 1.0 {
            guard.tokens -= 1.0;
            true
        } else {
            false
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
    /// Selects one healthy upstream node according to this route's [`LoadBalanceStrategy`].
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
        let healthy: Vec<&Arc<UpstreamRuntime>> = upstreams
            .iter()
            .filter(|node: &&Arc<UpstreamRuntime>| node.is_available(now_ms))
            .collect();
        if healthy.is_empty() {
            return None;
        }
        match route.get_strategy() {
            LoadBalanceStrategy::RoundRobin => {
                let idx: usize = cursor.fetch_add(1, Ordering::Relaxed) % healthy.len();
                Some(Arc::clone(healthy[idx]))
            }
            LoadBalanceStrategy::LeastConnections => healthy
                .into_iter()
                .min_by_key(|node: &&Arc<UpstreamRuntime>| {
                    node.active_conns.load(Ordering::Relaxed)
                })
                .cloned(),
            LoadBalanceStrategy::WeightedRoundRobin => {
                let total_weight: usize = healthy
                    .iter()
                    .map(|node: &&Arc<UpstreamRuntime>| node.weight.max(1) as usize)
                    .sum();
                let mut slot: usize = cursor.fetch_add(1, Ordering::Relaxed) % total_weight;
                for node in healthy {
                    let w: usize = node.weight.max(1) as usize;
                    if slot < w {
                        return Some(Arc::clone(node));
                    }
                    slot -= w;
                }
                None
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
        let req_body: Vec<u8> = ctx.get_request().get_body().clone();
        if req_body.len() > self.get_config().get_max_body_size() {
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
        let conn_vals: Vec<String> = ctx
            .get_request()
            .get_headers()
            .iter()
            .filter(|(k, _): &(&String, &RequestHeadersValue)| k.eq_ignore_ascii_case(CONNECTION))
            .flat_map(|(_, vals): (&String, &RequestHeadersValue)| vals.iter().cloned())
            .collect();
        let extra_hop: HashSet<String> = extract_connection_header_tokens(&conn_vals);
        let mut forward_headers: HashMap<String, String> = HashMap::new();
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
        let send_res: Result<HttpResponse, _> = upstream_req.send_async().await;
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
                let resp_extra_hop: HashSet<String> =
                    extract_connection_header_tokens(&resp_conn_vals);
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
    /// - `Status` - `Status::Continue` when unconfigured or no route matches; `Status::Stop` once forwarded.
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
