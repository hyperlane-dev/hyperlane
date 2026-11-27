use super::*;

/// Pluggable asynchronous response/data cache store (in-memory, Redis, Memcached, etc.).
pub trait CacheStore: Send + Sync + 'static {
    /// Looks up an unexpired [`CachedValue`] for `key`.
    ///
    /// # Arguments
    ///
    /// - `&'a self` - Shared reference to self.
    /// - `&'a str` - Cache key.
    ///
    /// # Returns
    ///
    /// - `CacheFuture<'a, Option<CachedValue>>` - Cached value when present and unexpired.
    fn get<'a>(&'a self, key: &'a str) -> CacheFuture<'a, Option<CachedValue>>;

    /// Inserts or replaces `value` under `key` with time-to-live `ttl_ms`.
    ///
    /// # Arguments
    ///
    /// - `String` - Cache key.
    /// - `CachedValue` - Payload and metadata to cache.
    /// - `u64` - Entry TTL in milliseconds.
    ///
    /// # Returns
    ///
    /// - `CacheFuture<'_, ()>` - Completion future.
    fn put(&self, key: String, value: CachedValue, ttl_ms: u64) -> CacheFuture<'_, ()>;

    /// Removes a single key from the cache.
    ///
    /// # Arguments
    ///
    /// - `&'a self` - Shared reference to self.
    /// - `&'a str` - Cache key to invalidate.
    ///
    /// # Returns
    ///
    /// - `CacheFuture<'a, bool>` - `true` when an entry was removed.
    fn remove<'a>(&'a self, key: &'a str) -> CacheFuture<'a, bool>;

    /// Evicts expired entries and returns the count removed.
    ///
    /// # Returns
    ///
    /// - `usize` - Number of expired entries reclaimed.
    fn purge_expired(&self) -> usize {
        0
    }
}
