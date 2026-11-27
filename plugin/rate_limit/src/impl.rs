use super::*;

/// Default construction for [`RateLimitPolicy`].
impl Default for RateLimitPolicy {
    /// Constructs a default [`RateLimitPolicy`] (100 req / 60 s, GCRA, 65 536 max keys).
    fn default() -> Self {
        Self {
            requests_per_window: DEFAULT_RATE_LIMIT_REQUESTS,
            window_ms: DEFAULT_RATE_LIMIT_WINDOW_MS,
            burst: DEFAULT_RATE_LIMIT_BURST,
            algorithm: RateLimitAlgorithm::Gcra,
            max_keys: DEFAULT_RATE_LIMIT_MAX_KEYS,
            include_headers: true,
            exempt_paths: Vec::new(),
        }
    }
}

/// Inherent fluent builders for [`RateLimitPolicy`].
impl RateLimitPolicy {
    /// Constructs a policy with `requests_per_window` allowed over `window_ms`.
    ///
    /// # Arguments
    ///
    /// - `u64` - Sustained requests per window.
    /// - `u64` - Window duration in milliseconds.
    ///
    /// # Returns
    ///
    /// - `Self` - Constructed rate-limit policy.
    pub fn new(requests_per_window: u64, window_ms: u64) -> Self {
        let count: u64 = requests_per_window.max(1);
        Self {
            requests_per_window: count,
            window_ms: window_ms.max(1),
            burst: count,
            algorithm: RateLimitAlgorithm::Gcra,
            max_keys: DEFAULT_RATE_LIMIT_MAX_KEYS,
            include_headers: true,
            exempt_paths: Vec::new(),
        }
    }

    /// Sets the burst allowance for GCRA enforcement.
    ///
    /// # Arguments
    ///
    /// - `u64` - Maximum instantaneous burst capacity.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated policy.
    pub fn with_burst(mut self, burst: u64) -> Self {
        self.set_burst(burst.max(1));
        self
    }

    /// Sets the rate-limiting algorithm (`Gcra` or `FixedWindow`).
    ///
    /// # Arguments
    ///
    /// - `RateLimitAlgorithm` - Desired enforcement algorithm.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated policy.
    pub fn with_algorithm(mut self, algorithm: RateLimitAlgorithm) -> Self {
        self.set_algorithm(algorithm);
        self
    }

    /// Sets the maximum tracked key capacity across all 64 shards.
    ///
    /// # Arguments
    ///
    /// - `usize` - Maximum number of concurrent tracked keys.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated policy.
    pub fn with_max_keys(mut self, max_keys: usize) -> Self {
        self.set_max_keys(max_keys.max(1));
        self
    }

    /// Enables or disables `RateLimit-*` response headers.
    ///
    /// # Arguments
    ///
    /// - `bool` - Whether to attach headers.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated policy.
    pub fn with_include_headers(mut self, include_headers: bool) -> Self {
        self.set_include_headers(include_headers);
        self
    }

    /// Sets exact request paths exempted from rate limiting.
    ///
    /// # Arguments
    ///
    /// - `I` - Iterator of exact exempt paths.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated policy.
    pub fn with_exempt_paths<I, S>(mut self, paths: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.set_exempt_paths(paths.into_iter().map(Into::into).collect());
        self
    }
}

/// Inherent implementation of [`BucketEntry`].
impl BucketEntry {
    /// Creates a fresh zero-initialized atomic bucket entry.
    ///
    /// # Arguments
    ///
    /// - `u64` - Current timestamp in nanoseconds since limiter epoch.
    ///
    /// # Returns
    ///
    /// - `Self` - Initialized atomic bucket.
    pub(crate) fn new(now_ns: u64) -> Self {
        Self {
            primary: AtomicU64::new(0),
            counter: AtomicU64::new(0),
            last_seen_ns: AtomicU64::new(now_ns),
        }
    }

    /// Evaluates one GCRA request atomically using a lock-free CAS loop.
    ///
    /// # Arguments
    ///
    /// - `u64` - Current elapsed nanoseconds since limiter epoch.
    /// - `u64` - Emission interval in nanoseconds per token.
    /// - `u64` - Burst tolerance in nanoseconds (`emission_interval_ns * burst`).
    /// - `u64` - Effective burst limit.
    ///
    /// # Returns
    ///
    /// - `RateLimitDecision` - Admission decision and remaining tokens.
    pub(crate) fn check_gcra(
        &self,
        now_ns: u64,
        emission_interval_ns: u64,
        burst_tolerance_ns: u64,
        limit: u64,
    ) -> RateLimitDecision {
        let Self {
            primary,
            last_seen_ns,
            ..
        } = self;
        last_seen_ns.store(now_ns, Ordering::Relaxed);
        let max_tat: u64 = now_ns.saturating_add(burst_tolerance_ns);
        let mut observed: u64 = primary.load(Ordering::Relaxed);
        loop {
            let base: u64 = observed.max(now_ns);
            let next_tat: u64 = base.saturating_add(emission_interval_ns);
            if next_tat > max_tat {
                let wait_ns: u64 = next_tat.saturating_sub(max_tat);
                let reset_ns: u64 = observed.saturating_sub(now_ns);
                return RateLimitDecision {
                    allowed: false,
                    limit,
                    remaining: 0,
                    reset_after_secs: ceil_nanos_to_secs(reset_ns),
                    retry_after_secs: ceil_nanos_to_secs(wait_ns).max(1),
                };
            }
            match primary.compare_exchange_weak(
                observed,
                next_tat,
                Ordering::AcqRel,
                Ordering::Relaxed,
            ) {
                Ok(_) => {
                    let headroom_ns: u64 = max_tat.saturating_sub(next_tat);
                    let remaining: u64 = headroom_ns / emission_interval_ns;
                    let reset_ns: u64 = next_tat.saturating_sub(now_ns);
                    return RateLimitDecision {
                        allowed: true,
                        limit,
                        remaining,
                        reset_after_secs: ceil_nanos_to_secs(reset_ns),
                        retry_after_secs: 0,
                    };
                }
                Err(actual) => observed = actual,
            }
        }
    }

    /// Evaluates one fixed-window request atomically without locks.
    ///
    /// # Arguments
    ///
    /// - `u64` - Current elapsed nanoseconds since limiter epoch.
    /// - `u64` - Window size in nanoseconds.
    /// - `u64` - Maximum requests permitted in one window.
    ///
    /// # Returns
    ///
    /// - `RateLimitDecision` - Admission decision and remaining quota.
    pub(crate) fn check_fixed_window(
        &self,
        now_ns: u64,
        window_ns: u64,
        limit: u64,
    ) -> RateLimitDecision {
        let Self {
            primary,
            counter,
            last_seen_ns,
        } = self;
        last_seen_ns.store(now_ns, Ordering::Relaxed);
        let current_win: u64 = (now_ns / window_ns).saturating_add(1);
        let elapsed_in_win: u64 = now_ns % window_ns;
        let reset_ns: u64 = window_ns.saturating_sub(elapsed_in_win);
        let reset_secs: u64 = ceil_nanos_to_secs(reset_ns).max(1);
        let observed_win: u64 = primary.load(Ordering::Acquire);
        if observed_win != current_win
            && primary
                .compare_exchange(
                    observed_win,
                    current_win,
                    Ordering::AcqRel,
                    Ordering::Acquire,
                )
                .is_ok()
        {
            counter.store(1, Ordering::Release);
            return RateLimitDecision {
                allowed: true,
                limit,
                remaining: limit.saturating_sub(1),
                reset_after_secs: reset_secs,
                retry_after_secs: 0,
            };
        }
        let mut cur: u64 = counter.load(Ordering::Relaxed);
        loop {
            if cur >= limit {
                return RateLimitDecision {
                    allowed: false,
                    limit,
                    remaining: 0,
                    reset_after_secs: reset_secs,
                    retry_after_secs: reset_secs,
                };
            }
            match counter.compare_exchange_weak(cur, cur + 1, Ordering::AcqRel, Ordering::Relaxed) {
                Ok(_) => {
                    return RateLimitDecision {
                        allowed: true,
                        limit,
                        remaining: limit.saturating_sub(cur + 1),
                        reset_after_secs: reset_secs,
                        retry_after_secs: 0,
                    };
                }
                Err(actual) => cur = actual,
            }
        }
    }
}

/// Inherent implementation of [`ShardedRateLimiter`].
impl ShardedRateLimiter {
    /// Builds a 64-way sharded lock-free [`ShardedRateLimiter`] governed by `policy`.
    ///
    /// # Arguments
    ///
    /// - `RateLimitPolicy` - Quota, window, burst, algorithm, and memory cap settings.
    ///
    /// # Returns
    ///
    /// - `Self` - Ready rate limiter.
    pub fn new(policy: RateLimitPolicy) -> Self {
        let window_ns: u64 = policy.get_window_ms().max(1).saturating_mul(NANOS_PER_MS);
        let rpw: u64 = policy.get_requests_per_window().max(1);
        let burst: u64 = policy.get_burst().max(1);
        let emission_interval_ns: u64 = (window_ns / rpw).max(1);
        let burst_tolerance_ns: u64 = emission_interval_ns.saturating_mul(burst);
        let shards: RateLimitShardArray =
            Box::new(from_fn(|_: usize| RwLock::new(hash_map_xx_hash3_64())));
        Self {
            inner: Arc::new(ShardedRateLimiterInner {
                policy,
                epoch: Instant::now(),
                emission_interval_ns,
                burst_tolerance_ns,
                window_ns,
                key_count: AtomicUsize::new(0),
                shards,
            }),
        }
    }

    /// Returns the number of currently tracked distinct keys in `O(1)` time without locking.
    ///
    /// # Returns
    ///
    /// - `usize` - Active tracked key count.
    pub fn tracked_keys(&self) -> usize {
        let Self { inner } = self;
        inner.key_count.load(Ordering::Relaxed)
    }

    /// Looks up or creates the [`BucketEntry`] for `key` with read-fast sharding and bounded capacity.
    ///
    /// # Arguments
    ///
    /// - `&str` - Bucket partition key.
    /// - `u64` - Current timestamp in nanoseconds since epoch.
    ///
    /// # Returns
    ///
    /// - `Option<Arc<BucketEntry>>` - Bucket entry, or `None` when table is full of active keys.
    fn get_or_insert_bucket(&self, key: &str, now_ns: u64) -> Option<Arc<BucketEntry>> {
        let Self { inner } = self;
        let idx: usize = rate_limit_shard_index(key);
        if let Ok(read_map) = inner.shards[idx].read()
            && let Some(existing) = read_map.get(key)
        {
            return Some(Arc::clone(existing));
        }
        let max_keys: usize = inner.policy.get_max_keys();
        if inner.key_count.load(Ordering::Relaxed) >= max_keys {
            let _: usize = self.purge_expired();
        }
        let mut write_map: RwLockWriteGuard<'_, HashMapXxHash3_64<String, Arc<BucketEntry>>> =
            inner.shards[idx].write().ok()?;
        if let Some(existing) = write_map.get(key) {
            return Some(Arc::clone(existing));
        }
        if inner.key_count.load(Ordering::Relaxed) >= max_keys {
            let stale_cutoff: u64 = now_ns.saturating_sub(inner.window_ns);
            let before: usize = write_map.len();
            write_map.retain(|_: &String, entry: &mut Arc<BucketEntry>| {
                entry.last_seen_ns.load(Ordering::Relaxed) > stale_cutoff
            });
            let removed: usize = before.saturating_sub(write_map.len());
            if removed > 0 {
                inner.key_count.fetch_sub(removed, Ordering::Relaxed);
            }
            if inner.key_count.load(Ordering::Relaxed) >= max_keys {
                return None;
            }
        }
        let created: Arc<BucketEntry> = Arc::new(BucketEntry::new(now_ns));
        write_map.insert(key.to_owned(), Arc::clone(&created));
        inner.key_count.fetch_add(1, Ordering::Relaxed);
        Some(created)
    }
}

/// Implements [`RateLimitStore`] for [`ShardedRateLimiter`].
impl RateLimitStore for ShardedRateLimiter {
    /// Evaluates one request against `key` using the configured lock-free algorithm.
    ///
    /// # Arguments
    ///
    /// - `&str` - Bucket partition key.
    ///
    /// # Returns
    ///
    /// - `RateLimitDecision` - Admission decision and remaining quota.
    fn check(&self, key: &str) -> RateLimitDecision {
        let Self { inner } = self;
        let now_ns: u64 = Instant::now()
            .duration_since(inner.epoch)
            .as_nanos()
            .min(u64::MAX as u128) as u64;
        let Some(entry) = self.get_or_insert_bucket(key, now_ns) else {
            let limit: u64 = inner.policy.get_requests_per_window();
            let reset_secs: u64 = ceil_nanos_to_secs(inner.window_ns).max(1);
            return RateLimitDecision {
                allowed: false,
                limit,
                remaining: 0,
                reset_after_secs: reset_secs,
                retry_after_secs: reset_secs,
            };
        };
        match inner.policy.get_algorithm() {
            RateLimitAlgorithm::Gcra => entry.check_gcra(
                now_ns,
                inner.emission_interval_ns,
                inner.burst_tolerance_ns,
                inner.policy.get_burst().max(1),
            ),
            RateLimitAlgorithm::FixedWindow => entry.check_fixed_window(
                now_ns,
                inner.window_ns,
                inner.policy.get_requests_per_window().max(1),
            ),
        }
    }

    /// Evicts entries whose idle age exceeds the configured window duration across all 64 shards.
    ///
    /// # Returns
    ///
    /// - `usize` - Total number of stale keys evicted.
    fn purge_expired(&self) -> usize {
        let Self { inner } = self;
        let now_ns: u64 = Instant::now()
            .duration_since(inner.epoch)
            .as_nanos()
            .min(u64::MAX as u128) as u64;
        let stale_cutoff: u64 =
            now_ns.saturating_sub(inner.window_ns.max(inner.burst_tolerance_ns));
        let mut evicted: usize = 0;
        for shard in inner.shards.iter() {
            let needs_evict: bool = shard
                .read()
                .map(
                    |map: RwLockReadGuard<'_, HashMapXxHash3_64<String, Arc<BucketEntry>>>| {
                        !map.is_empty()
                    },
                )
                .unwrap_or(false);
            if !needs_evict {
                continue;
            }
            if let Ok(mut map) = shard.write() {
                let before: usize = map.len();
                map.retain(|_: &String, entry: &mut Arc<BucketEntry>| {
                    entry.last_seen_ns.load(Ordering::Relaxed) > stale_cutoff
                });
                evicted += before.saturating_sub(map.len());
            }
        }
        if evicted > 0 {
            inner.key_count.fetch_sub(evicted, Ordering::Relaxed);
        }
        evicted
    }
}

/// Inherent constructors for built-in key extractors.
impl ClientIpKeyExtractor {
    /// Creates a [`ClientIpKeyExtractor`] choosing whether to trust `X-Forwarded-For` / `X-Real-Ip`.
    ///
    /// # Arguments
    ///
    /// - `bool` - `true` when behind a trusted reverse proxy.
    ///
    /// # Returns
    ///
    /// - `Self` - Constructed key extractor.
    pub fn new(trust_proxy_headers: bool) -> Self {
        Self {
            trust_proxy_headers,
        }
    }
}

/// Implements [`KeyExtractor`] for [`ClientIpKeyExtractor`].
impl KeyExtractor for ClientIpKeyExtractor {
    /// Extracts the client IP address as the bucket key.
    ///
    /// # Arguments
    ///
    /// - `&Stream` - Transport stream.
    /// - `&Context` - Request context.
    ///
    /// # Returns
    ///
    /// - `String` - Client IP partition key.
    fn extract(&self, stream: &Stream, ctx: &Context) -> String {
        resolve_client_ip(stream, ctx, self.get_trust_proxy_headers())
    }
}

/// Inherent constructors for [`HeaderKeyExtractor`].
impl HeaderKeyExtractor {
    /// Creates a [`HeaderKeyExtractor`] reading `header_name` with optional client-IP fallback.
    ///
    /// # Arguments
    ///
    /// - `H` - Header name convertible into `String`.
    /// - `bool` - Whether to fall back to client IP when the header is absent.
    ///
    /// # Returns
    ///
    /// - `Self` - Constructed key extractor.
    pub fn new<H>(header_name: H, fallback_to_ip: bool) -> Self
    where
        H: Into<String>,
    {
        Self {
            header_name: header_name.into(),
            fallback_to_ip,
        }
    }
}

/// Implements [`KeyExtractor`] for [`HeaderKeyExtractor`].
impl KeyExtractor for HeaderKeyExtractor {
    /// Extracts the configured request header value (or client IP fallback) as the bucket key.
    ///
    /// # Arguments
    ///
    /// - `&Stream` - Transport stream.
    /// - `&Context` - Request context.
    ///
    /// # Returns
    ///
    /// - `String` - Header-derived partition key.
    fn extract(&self, stream: &Stream, ctx: &Context) -> String {
        if let Some(val) = ctx
            .get_request()
            .try_get_header_back(self.get_header_name())
        {
            let trimmed: &str = val.trim();
            if !trimmed.is_empty() {
                return trimmed.to_owned();
            }
        }
        if self.get_fallback_to_ip() {
            resolve_client_ip(stream, ctx, false)
        } else {
            FALLBACK_CLIENT_KEY.to_owned()
        }
    }
}

/// Inherent constructors for [`PathAndIpKeyExtractor`].
impl PathAndIpKeyExtractor {
    /// Creates a [`PathAndIpKeyExtractor`] combining request path and client IP.
    ///
    /// # Arguments
    ///
    /// - `bool` - `true` when forwarding proxy headers are trusted.
    ///
    /// # Returns
    ///
    /// - `Self` - Constructed key extractor.
    pub fn new(trust_proxy_headers: bool) -> Self {
        Self {
            trust_proxy_headers,
        }
    }
}

/// Implements [`KeyExtractor`] for [`PathAndIpKeyExtractor`].
impl KeyExtractor for PathAndIpKeyExtractor {
    /// Extracts `"<path>|<client_ip>"` as the partition key.
    ///
    /// # Arguments
    ///
    /// - `&Stream` - Transport stream.
    /// - `&Context` - Request context.
    ///
    /// # Returns
    ///
    /// - `String` - Composite path-and-IP partition key.
    fn extract(&self, stream: &Stream, ctx: &Context) -> String {
        let path: &str = ctx.get_request().get_path();
        let ip: String = resolve_client_ip(stream, ctx, self.get_trust_proxy_headers());
        let mut out: String = String::with_capacity(path.len() + ip.len() + 1);
        out.push_str(path);
        out.push('|');
        out.push_str(&ip);
        out
    }
}

/// Inherent implementation of [`RateLimitPlugin`].
impl RateLimitPlugin {
    /// Initializes the global [`RateLimitPlugin`] with the default 64-shard limiter and client-IP extractor.
    ///
    /// # Arguments
    ///
    /// - `RateLimitPolicy` - Quota and algorithm policy.
    ///
    /// # Returns
    ///
    /// - `&'static Self` - Reference to the initialized global plugin.
    pub fn init(policy: RateLimitPolicy) -> &'static Self {
        let store: SharedRateLimitStore = Arc::new(ShardedRateLimiter::new(policy.clone()));
        let key_extractor: SharedKeyExtractor = Arc::new(ClientIpKeyExtractor::new(false));
        Self::init_custom(policy, store, key_extractor)
    }

    /// Initializes the global [`RateLimitPlugin`] with custom [`RateLimitStore`] and [`KeyExtractor`].
    ///
    /// # Arguments
    ///
    /// - `RateLimitPolicy` - Quota header and exempt-path policy.
    /// - `SharedRateLimitStore` - Custom or distributed backend store.
    /// - `SharedKeyExtractor` - Custom partition key extractor.
    ///
    /// # Returns
    ///
    /// - `&'static Self` - Reference to the initialized global plugin.
    pub fn init_custom(
        policy: RateLimitPolicy,
        store: SharedRateLimitStore,
        key_extractor: SharedKeyExtractor,
    ) -> &'static Self {
        RATE_LIMIT_PLUGIN.get_or_init(|| Self {
            policy,
            store,
            key_extractor,
        })
    }

    /// Returns the initialized global [`RateLimitPlugin`], if initialized.
    ///
    /// # Returns
    ///
    /// - `Option<&'static Self>` - Reference to the global plugin, or `None`.
    pub fn get() -> Option<&'static Self> {
        RATE_LIMIT_PLUGIN.get()
    }

    /// Returns the active [`RateLimitPolicy`].
    ///
    /// # Returns
    ///
    /// - `&RateLimitPolicy` - Active policy.
    pub fn get_policy(&self) -> &RateLimitPolicy {
        &self.policy
    }

    /// Returns the active [`SharedRateLimitStore`].
    ///
    /// # Returns
    ///
    /// - `&SharedRateLimitStore` - Active backend store.
    pub fn get_store(&self) -> &SharedRateLimitStore {
        &self.store
    }

    /// Returns the active [`SharedKeyExtractor`].
    ///
    /// # Returns
    ///
    /// - `&SharedKeyExtractor` - Active key extractor.
    pub fn get_key_extractor(&self) -> &SharedKeyExtractor {
        &self.key_extractor
    }

    /// Evaluates one request against `key` and attaches standard `RateLimit-*` response headers.
    ///
    /// # Arguments
    ///
    /// - `&str` - Partition key.
    /// - `&mut Context` - Request and response context.
    ///
    /// # Returns
    ///
    /// - `RateLimitDecision` - Quota decision for this request.
    pub fn evaluate_key(&self, key: &str, ctx: &mut Context) -> RateLimitDecision {
        let decision: RateLimitDecision = self.get_store().check(key);
        let response: &mut Response = ctx.get_mut_response();
        if self.get_policy().get_include_headers() {
            response
                .set_header(HEADER_RATE_LIMIT_LIMIT, decision.get_limit().to_string())
                .set_header(
                    HEADER_RATE_LIMIT_REMAINING,
                    decision.get_remaining().to_string(),
                )
                .set_header(
                    HEADER_RATE_LIMIT_RESET,
                    decision.get_reset_after_secs().to_string(),
                );
        }
        if !decision.get_allowed() {
            response
                .set_status_code(HttpStatus::TooManyRequests.code())
                .set_header(RETRY_AFTER, decision.get_retry_after_secs().to_string())
                .set_body(Vec::new());
        }
        decision
    }
}

/// Implements [`ServerHook`] for [`RateLimitHook`].
impl ServerHook for RateLimitHook {
    /// Constructs a per-request [`RateLimitHook`].
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

    /// Enforces the configured rate limit and rejects excess traffic with `429 Too Many Requests`.
    ///
    /// # Arguments
    ///
    /// - `Self` - Hook instance.
    /// - `&mut Stream` - Connection stream.
    /// - `&mut Context` - Request context.
    ///
    /// # Returns
    ///
    /// - `Status` - `Status::Continue` when unconfigured, exempt, or admitted; `Status::Reject` when throttled.
    async fn handle(self, stream: &mut Stream, ctx: &mut Context) -> Status {
        let Some(plugin) = RateLimitPlugin::get() else {
            return Status::Continue;
        };
        let path: &str = ctx.get_request().get_path();
        if plugin
            .get_policy()
            .get_exempt_paths()
            .iter()
            .any(|exempt: &String| exempt == path)
        {
            return Status::Continue;
        }
        let key: String = plugin.get_key_extractor().extract(stream, ctx);
        let decision: RateLimitDecision = plugin.evaluate_key(&key, ctx);
        if decision.get_allowed() {
            return Status::Continue;
        }
        let wire: ResponseData = ctx.get_mut_response().build();
        let _: Result<(), ResponseError> = stream.try_send(&wire).await;
        Status::Reject
    }
}
