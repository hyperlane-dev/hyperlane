use super::*;

/// Cached HTTP response or arbitrary byte payload with headers and timestamp metadata.
#[derive(Clone, Data, Debug, Eq, PartialEq)]
pub struct CachedValue {
    /// Cached HTTP status code (for example `200`).
    #[get(type(copy))]
    pub(crate) status_code: usize,
    /// Cached response headers `(name, value)`.
    pub(crate) headers: Vec<(String, String)>,
    /// Cached payload bytes.
    pub(crate) body: Vec<u8>,
    /// Unix epoch millisecond when this entry was stored.
    #[get(type(copy))]
    pub(crate) stored_at_ms: u64,
}

/// Internal cache record pairing a [`CachedValue`] with expiration and byte footprint.
pub(crate) struct CacheEntry {
    /// Cached payload.
    pub(crate) value: CachedValue,
    /// Unix epoch millisecond at which this entry expires.
    pub(crate) expires_at_ms: u64,
    /// Approximate memory footprint in bytes for budget enforcement.
    pub(crate) footprint_bytes: usize,
}

/// Single-flight coordination slot coalescing concurrent cache misses for one key.
pub(crate) struct InFlightSlot {
    /// Notifier woken once the leader finishes loading.
    pub(crate) notify: Notify,
    /// Completed result shared with all waiting followers.
    pub(crate) result: Mutex<Option<Option<CachedValue>>>,
}

/// Configuration for [`ShardedMemoryCache`] and [`CachePlugin`].
#[derive(Clone, Data, Debug, Eq, PartialEq)]
pub struct CacheConfig {
    /// Default entry TTL in milliseconds when no `Cache-Control: max-age` override is present.
    #[get(type(copy))]
    pub(crate) default_ttl_ms: u64,
    /// Maximum cached entries across all 64 shards.
    #[get(type(copy))]
    pub(crate) max_entries: usize,
    /// Maximum aggregate memory footprint across all 64 shards in bytes.
    #[get(type(copy))]
    pub(crate) max_bytes: usize,
    /// Maximum single response body length eligible for automatic HTTP response caching.
    #[get(type(copy))]
    pub(crate) max_entry_bytes: usize,
    /// Bypasses HTTP response caching when `Authorization` or `Cookie` is present when `true`.
    #[get(type(copy))]
    pub(crate) bypass_authenticated: bool,
    /// Emits diagnostic `X-Cache` and `Age` headers when `true`.
    #[get(type(copy))]
    pub(crate) emit_diagnostic_headers: bool,
}

/// Shared interior state of [`ShardedMemoryCache`].
pub(crate) struct ShardedMemoryCacheInner {
    /// Cache capacity and TTL configuration.
    pub(crate) config: CacheConfig,
    /// Lock-free count of live entries across all shards.
    pub(crate) entry_count: AtomicUsize,
    /// Lock-free estimated total byte usage across all shards.
    pub(crate) total_bytes: AtomicUsize,
    /// 64-way sharded entry tables.
    pub(crate) shards: CacheShardArray,
    /// 64-way sharded single-flight stampede-coalescing map.
    pub(crate) inflight: InFlightShardArray,
}

/// High-concurrency 64-way sharded bounded in-memory [`CacheStore`] with single-flight coalescing.
#[derive(Clone)]
pub struct ShardedMemoryCache {
    /// Shared sharded cache state.
    pub(crate) inner: Arc<ShardedMemoryCacheInner>,
}

/// Process-wide [`CachePlugin`] singleton storing [`CacheConfig`] and a pluggable [`CacheStore`].
#[derive(Clone)]
pub struct CachePlugin {
    /// Active HTTP caching policy.
    pub(crate) config: CacheConfig,
    /// Backing cache store (default [`ShardedMemoryCache`] or custom distributed store).
    pub(crate) store: SharedCacheStore,
}

/// Opt-in request middleware serving cached `GET`/`HEAD` responses with `X-Cache: HIT` and `Age`.
#[derive(Clone, Copy, Debug, Default)]
#[hyperlane::request_middleware(-80)]
pub struct CacheRequestHook;

/// Opt-in response middleware storing eligible `200 OK` responses according to `Cache-Control` and `Vary`.
#[derive(Clone, Copy, Debug, Default)]
#[hyperlane::response_middleware(-65)]
pub struct CacheResponseHook;
