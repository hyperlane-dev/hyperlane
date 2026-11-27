use super::*;

/// Fixed-size 64-way sharded hot-file cache array.
pub(crate) type StaticCacheShardArray =
    Box<[RwLock<HashMapXxHash3_64<PathBuf, Arc<CachedFile>>>; STATIC_CACHE_SHARDS]>;
