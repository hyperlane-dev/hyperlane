use super::*;

/// Thread-safe shared reference to a pluggable [`KeyExtractor`].
pub type SharedKeyExtractor = Arc<dyn KeyExtractor>;

/// Thread-safe shared reference to a pluggable [`RateLimitStore`].
pub type SharedRateLimitStore = Arc<dyn RateLimitStore>;

/// Fixed-size 64-way sharded rate-limiter entry map.
pub(crate) type RateLimitShardArray =
    Box<[RwLock<HashMapXxHash3_64<String, Arc<BucketEntry>>>; RATE_LIMIT_SHARD_COUNT]>;
