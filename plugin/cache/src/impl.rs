use super::*;

/// Inherent constructors for [`CachedValue`].
impl CachedValue {
    /// Creates a [`CachedValue`] with status code, headers, and body stamped with current epoch time.
    ///
    /// # Arguments
    ///
    /// - `usize` - HTTP status code.
    /// - `Vec<(String, String)>` - Response headers.
    /// - `Vec<u8>` - Payload bytes.
    ///
    /// # Returns
    ///
    /// - `Self` - Constructed cached value.
    pub fn new(status_code: usize, headers: Vec<(String, String)>, body: Vec<u8>) -> Self {
        Self {
            status_code,
            headers,
            body,
            stored_at_ms: cache_now_ms(),
        }
    }

    /// Creates a raw byte payload cache value with status `200` and no headers.
    ///
    /// # Arguments
    ///
    /// - `Vec<u8>` - Raw bytes to cache.
    ///
    /// # Returns
    ///
    /// - `Self` - Cached byte payload.
    pub fn bytes(body: Vec<u8>) -> Self {
        Self::new(HttpStatus::Ok.code(), Vec::new(), body)
    }

    /// Computes elapsed age in whole seconds since this entry was stored.
    ///
    /// # Returns
    ///
    /// - `u64` - Elapsed age in seconds.
    pub fn age_secs(&self) -> u64 {
        cache_now_ms().saturating_sub(self.get_stored_at_ms()) / 1_000
    }
}

/// Default construction for [`CacheConfig`].
impl Default for CacheConfig {
    /// Constructs a default [`CacheConfig`] (60 s TTL, 16 384 entries, 64 MiB total cap).
    fn default() -> Self {
        Self {
            default_ttl_ms: DEFAULT_CACHE_TTL_MS,
            max_entries: DEFAULT_CACHE_MAX_ENTRIES,
            max_bytes: DEFAULT_CACHE_MAX_BYTES,
            max_entry_bytes: DEFAULT_CACHE_MAX_ENTRY_BYTES,
            bypass_authenticated: true,
            emit_diagnostic_headers: true,
        }
    }
}

/// Inherent fluent builders for [`CacheConfig`].
impl CacheConfig {
    /// Sets the default entry TTL in milliseconds.
    ///
    /// # Arguments
    ///
    /// - `u64` - Default TTL in milliseconds.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated configuration.
    pub fn with_default_ttl_ms(mut self, ttl_ms: u64) -> Self {
        self.set_default_ttl_ms(ttl_ms);
        self
    }

    /// Sets the maximum tracked entry count across all shards.
    ///
    /// # Arguments
    ///
    /// - `usize` - Maximum entry count.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated configuration.
    pub fn with_max_entries(mut self, max_entries: usize) -> Self {
        self.set_max_entries(max_entries.max(1));
        self
    }

    /// Sets the maximum total byte budget across all shards.
    ///
    /// # Arguments
    ///
    /// - `usize` - Byte budget cap.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated configuration.
    pub fn with_max_bytes(mut self, max_bytes: usize) -> Self {
        self.set_max_bytes(max_bytes.max(1));
        self
    }
}

/// Default construction for [`ShardedMemoryCache`].
impl Default for ShardedMemoryCache {
    /// Constructs a default 64-shard [`ShardedMemoryCache`].
    fn default() -> Self {
        Self::new(CacheConfig::default())
    }
}

/// Inherent implementation of [`ShardedMemoryCache`].
impl ShardedMemoryCache {
    /// Creates a 64-way sharded bounded [`ShardedMemoryCache`] governed by `config`.
    ///
    /// # Arguments
    ///
    /// - `CacheConfig` - TTL and memory cap configuration.
    ///
    /// # Returns
    ///
    /// - `Self` - Ready sharded cache.
    pub fn new(config: CacheConfig) -> Self {
        let shards: CacheShardArray =
            Box::new(from_fn(|_: usize| RwLock::new(hash_map_xx_hash3_64())));
        let inflight: InFlightShardArray =
            Box::new(from_fn(|_: usize| Mutex::new(hash_map_xx_hash3_64())));
        Self {
            inner: Arc::new(ShardedMemoryCacheInner {
                config,
                entry_count: AtomicUsize::new(0),
                total_bytes: AtomicUsize::new(0),
                shards,
                inflight,
            }),
        }
    }

    /// Returns the number of cached entries across all shards in `O(1)` time without locking.
    ///
    /// # Returns
    ///
    /// - `usize` - Entry count.
    pub fn len(&self) -> usize {
        let Self { inner } = self;
        inner.entry_count.load(Ordering::Relaxed)
    }

    /// Reports whether the cache contains zero entries.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` when empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns the estimated total byte usage across all shards in `O(1)` time.
    ///
    /// # Returns
    ///
    /// - `usize` - Approximate cached byte footprint.
    pub fn total_bytes(&self) -> usize {
        let Self { inner } = self;
        inner.total_bytes.load(Ordering::Relaxed)
    }

    /// Synchronous read-fast lookup returning an unexpired [`CachedValue`].
    ///
    /// # Arguments
    ///
    /// - `&str` - Cache key.
    ///
    /// # Returns
    ///
    /// - `Option<CachedValue>` - Cached value when present and unexpired.
    pub fn get_sync(&self, key: &str) -> Option<CachedValue> {
        let Self { inner } = self;
        let idx: usize = cache_shard_index(key);
        let now_ms: u64 = cache_now_ms();
        let entry: Arc<CacheEntry> = {
            let guard: RwLockReadGuard<'_, HashMapXxHash3_64<String, Arc<CacheEntry>>> =
                inner.shards[idx].read().ok()?;
            Arc::clone(guard.get(key)?)
        };
        if now_ms < entry.expires_at_ms {
            return Some(entry.value.clone());
        }
        if let Ok(mut guard) = inner.shards[idx].write()
            && let Some(removed) = guard.remove(key)
        {
            inner.entry_count.fetch_sub(1, Ordering::Relaxed);
            inner
                .total_bytes
                .fetch_sub(removed.footprint_bytes, Ordering::Relaxed);
        }
        None
    }

    /// Synchronous insert enforcing entry-count and byte-budget caps via expired & oldest eviction.
    ///
    /// # Arguments
    ///
    /// - `String` - Cache key.
    /// - `CachedValue` - Value to store.
    /// - `u64` - TTL in milliseconds.
    pub fn put_sync(&self, key: String, value: CachedValue, ttl_ms: u64) {
        if ttl_ms == 0 {
            return;
        }
        let Self { inner } = self;
        let footprint: usize = estimate_entry_bytes(&key, &value);
        if footprint > inner.config.get_max_bytes() {
            return;
        }
        let now_ms: u64 = cache_now_ms();
        if inner.entry_count.load(Ordering::Relaxed) >= inner.config.get_max_entries()
            || inner.total_bytes.load(Ordering::Relaxed) + footprint > inner.config.get_max_bytes()
        {
            let _: usize = self.purge_expired();
        }
        let idx: usize = cache_shard_index(&key);
        let Ok(mut map): Result<
            RwLockWriteGuard<'_, HashMapXxHash3_64<String, Arc<CacheEntry>>>,
            _,
        > = inner.shards[idx].write() else {
            return;
        };
        if let Some(prev) = map.remove(&key) {
            inner.entry_count.fetch_sub(1, Ordering::Relaxed);
            inner
                .total_bytes
                .fetch_sub(prev.footprint_bytes, Ordering::Relaxed);
        }
        while (inner.entry_count.load(Ordering::Relaxed) >= inner.config.get_max_entries()
            || inner.total_bytes.load(Ordering::Relaxed) + footprint > inner.config.get_max_bytes())
            && !map.is_empty()
        {
            let oldest_key: Option<String> = map
                .iter()
                .min_by_key(|(_, entry): &(&String, &Arc<CacheEntry>)| entry.expires_at_ms)
                .map(|(k, _): (&String, &Arc<CacheEntry>)| k.clone());
            if let Some(evict_key) = oldest_key
                && let Some(evicted) = map.remove(&evict_key)
            {
                inner.entry_count.fetch_sub(1, Ordering::Relaxed);
                inner
                    .total_bytes
                    .fetch_sub(evicted.footprint_bytes, Ordering::Relaxed);
            } else {
                break;
            }
        }
        if inner.entry_count.load(Ordering::Relaxed) >= inner.config.get_max_entries()
            || inner.total_bytes.load(Ordering::Relaxed) + footprint > inner.config.get_max_bytes()
        {
            return;
        }
        let entry: Arc<CacheEntry> = Arc::new(CacheEntry {
            value,
            expires_at_ms: now_ms.saturating_add(ttl_ms),
            footprint_bytes: footprint,
        });
        map.insert(key, entry);
        inner.entry_count.fetch_add(1, Ordering::Relaxed);
        inner.total_bytes.fetch_add(footprint, Ordering::Relaxed);
    }

    /// Single-flight cache loader coalescing concurrent cache misses on `key` into one origin call.
    ///
    /// # Arguments
    ///
    /// - `&str` - Cache key.
    /// - `u64` - Entry TTL in milliseconds when loaded.
    /// - `F` - Async origin loader invoked at most once across concurrent misses.
    ///
    /// # Returns
    ///
    /// - `Option<CachedValue>` - Cached or freshly loaded value.
    pub async fn get_or_load<F, Fut>(
        &self,
        key: &str,
        ttl_ms: u64,
        loader: F,
    ) -> Option<CachedValue>
    where
        F: FnOnce() -> Fut,
        Fut: Future<Output = Option<CachedValue>>,
    {
        if let Some(hit) = self.get_sync(key) {
            return Some(hit);
        }
        let Self { inner } = self;
        let idx: usize = cache_shard_index(key);
        let (slot, is_leader): (Arc<InFlightSlot>, bool) = {
            let Ok(mut guard) = inner.inflight[idx].lock() else {
                return loader().await;
            };
            if let Some(existing) = guard.get(key) {
                (Arc::clone(existing), false)
            } else {
                let created: Arc<InFlightSlot> = Arc::new(InFlightSlot {
                    notify: Notify::new(),
                    result: Mutex::new(None),
                });
                guard.insert(key.to_owned(), Arc::clone(&created));
                (created, true)
            }
        };
        if is_leader {
            let loaded: Option<CachedValue> = loader().await;
            if let Some(ref val) = loaded {
                self.put_sync(key.to_owned(), val.clone(), ttl_ms);
            }
            if let Ok(mut res_guard) = slot.result.lock() {
                *res_guard = Some(loaded.clone());
            }
            if let Ok(mut map_guard) = inner.inflight[idx].lock() {
                map_guard.remove(key);
            }
            slot.notify.notify_waiters();
            loaded
        } else {
            loop {
                let notified: Notified<'_> = slot.notify.notified();
                if let Ok(res_guard) = slot.result.lock()
                    && let Some(ref outcome) = *res_guard
                {
                    return outcome.clone();
                }
                notified.await;
            }
        }
    }
}

/// Implements [`CacheStore`] for [`ShardedMemoryCache`].
impl CacheStore for ShardedMemoryCache {
    /// Looks up an unexpired [`CachedValue`] asynchronously.
    ///
    /// # Arguments
    ///
    /// - `&'a self` - Shared reference to self.
    /// - `&'a str` - Cache key.
    ///
    /// # Returns
    ///
    /// - `CacheFuture<'a, Option<CachedValue>>` - Cached value if present and unexpired.
    fn get<'a>(&'a self, key: &'a str) -> CacheFuture<'a, Option<CachedValue>> {
        let value: Option<CachedValue> = self.get_sync(key);
        Box::pin(async move { value })
    }

    /// Inserts or replaces a [`CachedValue`] asynchronously.
    ///
    /// # Arguments
    ///
    /// - `String` - Cache key.
    /// - `CachedValue` - Value to cache.
    /// - `u64` - Entry TTL in milliseconds.
    ///
    /// # Returns
    ///
    /// - `CacheFuture<'_, ()>` - Completion future.
    fn put(&self, key: String, value: CachedValue, ttl_ms: u64) -> CacheFuture<'_, ()> {
        self.put_sync(key, value, ttl_ms);
        Box::pin(async {})
    }

    /// Removes a cached key asynchronously.
    ///
    /// # Arguments
    ///
    /// - `&'a self` - Shared reference to self.
    /// - `&'a str` - Cache key to remove.
    ///
    /// # Returns
    ///
    /// - `CacheFuture<'a, bool>` - `true` if removed.
    fn remove<'a>(&'a self, key: &'a str) -> CacheFuture<'a, bool> {
        let Self { inner } = self;
        let idx: usize = cache_shard_index(key);
        let removed: bool = inner.shards[idx]
            .write()
            .ok()
            .and_then(
                |mut map: RwLockWriteGuard<'_, HashMapXxHash3_64<String, Arc<CacheEntry>>>| {
                    map.remove(key)
                },
            )
            .map(|entry: Arc<CacheEntry>| {
                inner.entry_count.fetch_sub(1, Ordering::Relaxed);
                inner
                    .total_bytes
                    .fetch_sub(entry.footprint_bytes, Ordering::Relaxed);
                true
            })
            .unwrap_or(false);
        Box::pin(async move { removed })
    }

    /// Evicts expired entries across all 64 shards.
    ///
    /// # Returns
    ///
    /// - `usize` - Number of expired entries removed.
    fn purge_expired(&self) -> usize {
        let Self { inner } = self;
        let now_ms: u64 = cache_now_ms();
        let mut evicted: usize = 0;
        let mut freed_bytes: usize = 0;
        for shard in inner.shards.iter() {
            if let Ok(mut map) = shard.write() {
                map.retain(|_: &String, entry: &mut Arc<CacheEntry>| {
                    if now_ms >= entry.expires_at_ms {
                        evicted += 1;
                        freed_bytes += entry.footprint_bytes;
                        false
                    } else {
                        true
                    }
                });
            }
        }
        if evicted > 0 {
            inner.entry_count.fetch_sub(evicted, Ordering::Relaxed);
            inner.total_bytes.fetch_sub(freed_bytes, Ordering::Relaxed);
        }
        evicted
    }
}

/// Inherent implementation of [`CachePlugin`].
impl CachePlugin {
    /// Initializes the global [`CachePlugin`] once with `config` and the default [`ShardedMemoryCache`].
    ///
    /// # Arguments
    ///
    /// - `CacheConfig` - HTTP cache policy and memory limits.
    ///
    /// # Returns
    ///
    /// - `&'static Self` - Reference to the initialized global plugin.
    pub fn init(config: CacheConfig) -> &'static Self {
        let store: SharedCacheStore = Arc::new(ShardedMemoryCache::new(config.clone()));
        Self::init_with_store(config, store)
    }

    /// Initializes the global [`CachePlugin`] once with `config` and a custom [`CacheStore`].
    ///
    /// # Arguments
    ///
    /// - `CacheConfig` - HTTP cache policy.
    /// - `SharedCacheStore` - Pluggable cache backend.
    ///
    /// # Returns
    ///
    /// - `&'static Self` - Reference to the initialized global plugin.
    pub fn init_with_store(config: CacheConfig, store: SharedCacheStore) -> &'static Self {
        CACHE_PLUGIN.get_or_init(|| Self { config, store })
    }

    /// Returns the initialized global [`CachePlugin`], if initialized.
    ///
    /// # Returns
    ///
    /// - `Option<&'static Self>` - Reference to the global plugin, or `None`.
    pub fn get() -> Option<&'static Self> {
        CACHE_PLUGIN.get()
    }

    /// Returns the active [`CacheConfig`].
    ///
    /// # Returns
    ///
    /// - `&CacheConfig` - Active configuration.
    pub fn get_config(&self) -> &CacheConfig {
        &self.config
    }

    /// Returns the active [`SharedCacheStore`].
    ///
    /// # Returns
    ///
    /// - `&SharedCacheStore` - Active backing store.
    pub fn get_store(&self) -> &SharedCacheStore {
        &self.store
    }
}

/// Implements [`ServerHook`] for [`CacheRequestHook`].
impl ServerHook for CacheRequestHook {
    /// Constructs a per-request [`CacheRequestHook`].
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

    /// Serves cached `GET`/`HEAD` responses directly on cache hit, or records cache key for [`CacheResponseHook`].
    ///
    /// # Arguments
    ///
    /// - `Self` - Hook instance.
    /// - `&mut Stream` - Connection stream.
    /// - `&mut Context` - Request context.
    ///
    /// # Returns
    ///
    /// - `Status` - `Status::Reject` on cache hit; `Status::Continue` on miss or bypass.
    async fn handle(self, stream: &mut Stream, ctx: &mut Context) -> Status {
        let Some(plugin) = CachePlugin::get() else {
            return Status::Continue;
        };
        let is_get: bool = ctx.get_request().get_method().is_get();
        let is_head: bool = ctx.get_request().get_method().is_head();
        if !is_get && !is_head {
            return Status::Continue;
        }
        let config: &CacheConfig = plugin.get_config();
        let req_cc: Option<String> = ctx.get_request().try_get_header_back(CACHE_CONTROL);
        let (req_cacheable, _): (bool, Option<u64>) = parse_cache_control(req_cc.as_deref());
        let has_auth: bool = config.get_bypass_authenticated()
            && (ctx
                .get_request()
                .try_get_header_back(AUTHORIZATION)
                .is_some()
                || ctx.get_request().try_get_header_back(COOKIE).is_some());
        if !req_cacheable || has_auth {
            if config.get_emit_diagnostic_headers() {
                ctx.get_mut_response()
                    .set_header(HEADER_X_CACHE, X_CACHE_BYPASS);
            }
            return Status::Continue;
        }
        let key: String = build_http_cache_key(ctx);
        if let Some(hit) = plugin.get_store().get(&key).await {
            let age: String = hit.age_secs().to_string();
            let resp: &mut Response = ctx.get_mut_response();
            resp.set_status_code(hit.get_status_code());
            for (name, val) in hit.get_headers() {
                resp.set_header(name, val);
            }
            if config.get_emit_diagnostic_headers() {
                resp.set_header(HEADER_X_CACHE, X_CACHE_HIT)
                    .set_header(AGE, age);
            }
            if is_head {
                resp.set_body(Vec::new());
            } else {
                resp.set_body(hit.get_body().clone());
            }
            let wire: ResponseData = ctx.get_mut_response().build();
            let _: Result<(), ResponseError> = stream.try_send(&wire).await;
            return Status::Reject;
        }
        ctx.set_attribute(ATTR_HTTP_CACHE_KEY, key);
        Status::Continue
    }
}

/// Implements [`ServerHook`] for [`CacheResponseHook`].
impl ServerHook for CacheResponseHook {
    /// Constructs a per-response [`CacheResponseHook`].
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

    /// Stores eligible `200 OK` responses into [`CachePlugin`]'s backing store and emits `X-Cache: MISS`.
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
        let Some(plugin) = CachePlugin::get() else {
            return Status::Continue;
        };
        let Some(key) = ctx.try_get_attribute::<String, _>(ATTR_HTTP_CACHE_KEY) else {
            return Status::Continue;
        };
        let config: &CacheConfig = plugin.get_config();
        let status: usize = ctx.get_response().get_status_code();
        let body_len: usize = ctx.get_response().get_body().len();
        let resp_cc: Option<String> = ctx.get_response().try_get_header_back(CACHE_CONTROL);
        let (cc_ok, ttl_override): (bool, Option<u64>) = parse_cache_control(resp_cc.as_deref());
        let vary_star: bool = ctx
            .get_response()
            .try_get_header_back(VARY)
            .is_some_and(|val: String| val.trim() == "*");
        if status == HttpStatus::Ok.code()
            && cc_ok
            && !vary_star
            && body_len <= config.get_max_entry_bytes()
        {
            let ttl_ms: u64 = ttl_override.unwrap_or(config.get_default_ttl_ms());
            if ttl_ms > 0 {
                let mut headers: Vec<(String, String)> = Vec::new();
                for (name, values) in ctx.get_response().get_headers() {
                    if !name.eq_ignore_ascii_case(HEADER_X_CACHE)
                        && !name.eq_ignore_ascii_case(AGE)
                        && let Some(last) = values.back()
                    {
                        headers.push((name.clone(), last.clone()));
                    }
                }
                let cached: CachedValue =
                    CachedValue::new(status, headers, ctx.get_response().get_body().clone());
                plugin.get_store().put(key, cached, ttl_ms).await;
            }
        }
        if config.get_emit_diagnostic_headers()
            && ctx
                .get_response()
                .try_get_header_back(HEADER_X_CACHE)
                .is_none()
        {
            ctx.get_mut_response()
                .set_header(HEADER_X_CACHE, X_CACHE_MISS);
        }
        Status::Continue
    }
}
