/// Default maximum pending messages allowed per topic consumer group.
pub const DEFAULT_MQ_MAX_QUEUE_LEN: usize = 10_000;

/// Default unacknowledged delivery visibility timeout in milliseconds.
pub const DEFAULT_MQ_VISIBILITY_TIMEOUT_MS: u64 = 30_000;

/// Default maximum delivery attempts before routing a message to the dead-letter queue.
pub const DEFAULT_MQ_MAX_RETRIES: u32 = 3;

/// Default consumer group name used when no explicit groups are subscribed.
pub const DEFAULT_MQ_GROUP: &str = "default";

/// Default suffix appended to a topic name for its dead-letter queue.
pub const DEFAULT_MQ_DLQ_SUFFIX: &str = ".dlq";

/// Error text returned when a consumer group queue reaches capacity.
pub(crate) const ERR_MQ_QUEUE_FULL: &str = "Message queue capacity exceeded";

/// Error text returned when an ack/nack references an unknown receipt id.
pub(crate) const ERR_MQ_UNKNOWN_DELIVERY: &str = "Unknown message delivery receipt";

/// Error text returned when the broker mutex is poisoned or closed.
pub(crate) const ERR_MQ_BROKER_CLOSED: &str = "Message broker is unavailable";

/// Error text returned when publishing an already-expired message.
pub(crate) const ERR_MQ_EXPIRED: &str = "Message already expired";
