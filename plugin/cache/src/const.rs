/// Diagnostic header indicating cache lookup outcome (`HIT`, `MISS`, or `BYPASS`).
pub const HEADER_X_CACHE: &str = "x-cache";

/// `X-Cache` value emitted when served directly from cache.
pub const X_CACHE_HIT: &str = "HIT";

/// `X-Cache` value emitted when computed by the origin handler.
pub const X_CACHE_MISS: &str = "MISS";

/// `X-Cache` value emitted when caching is bypassed by policy or headers.
pub const X_CACHE_BYPASS: &str = "BYPASS";

/// Default response cache TTL in milliseconds (60 seconds).
pub const DEFAULT_CACHE_TTL_MS: u64 = 60_000;

/// Default maximum cached response count across all shards.
pub const DEFAULT_CACHE_MAX_ENTRIES: usize = 16_384;

/// Default maximum total cached byte budget (64 MiB).
pub const DEFAULT_CACHE_MAX_BYTES: usize = 64 * 1024 * 1024;

/// Default maximum single response body eligible for HTTP response caching (1 MiB).
pub const DEFAULT_CACHE_MAX_ENTRY_BYTES: usize = 1024 * 1024;

/// Number of independent cache shards (`64`, power of two).
pub(crate) const CACHE_SHARD_COUNT: usize = 64;

/// Context attribute key passing the computed HTTP cache key from request hook to response hook.
pub(crate) const ATTR_HTTP_CACHE_KEY: &str = "hyperlane_plugin_cache_key";

/// `Cache-Control` directive prohibiting storing a response.
pub(crate) const CC_NO_STORE: &str = "no-store";

/// `Cache-Control` directive prohibiting unvalidated reuse.
pub(crate) const CC_NO_CACHE: &str = "no-cache";

/// `Cache-Control` directive marking a response private to a single user.
pub(crate) const CC_PRIVATE: &str = "private";

/// `Cache-Control` `max-age=` prefix.
pub(crate) const CC_MAX_AGE_PREFIX: &str = "max-age=";

/// `Cache-Control` `s-maxage=` prefix.
pub(crate) const CC_S_MAXAGE_PREFIX: &str = "s-maxage=";
