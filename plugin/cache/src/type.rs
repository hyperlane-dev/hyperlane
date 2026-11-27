use super::*;

/// Boxed `Send` future returned by async [`CacheStore`] trait methods.
pub type CacheFuture<'a, T> = Pin<Box<dyn Future<Output = T> + Send + 'a>>;

/// Thread-safe shared reference to a pluggable [`CacheStore`].
pub type SharedCacheStore = Arc<dyn CacheStore>;

/// Fixed-size 64-way sharded cache entry map array.
pub(crate) type CacheShardArray =
    Box<[RwLock<HashMapXxHash3_64<String, Arc<CacheEntry>>>; CACHE_SHARD_COUNT]>;

/// Fixed-size 64-way sharded single-flight inflight table.
pub(crate) type InFlightShardArray =
    Box<[Mutex<HashMapXxHash3_64<String, Arc<InFlightSlot>>>; CACHE_SHARD_COUNT]>;
