use super::*;

/// Fixed-size 64-way sharded SSE topic registry array.
pub(crate) type SseShardArray =
    Box<[RwLock<HashMapXxHash3_64<String, Arc<TopicState>>>; SSE_SHARD_COUNT]>;
