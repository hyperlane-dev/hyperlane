use super::*;

/// Boxed `Send` future returned by async [`MessageBroker`] trait methods.
pub type BrokerFuture<'a, T> = Pin<Box<dyn Future<Output = Result<T, MqError>> + Send + 'a>>;

/// Thread-safe shared reference to any [`MessageBroker`] backend implementation.
pub type SharedBroker = Arc<dyn MessageBroker>;

/// Fixed-size sharded topic lookup array.
pub(crate) type TopicShardArray = Box<[RwLock<HashMap<String, Arc<TopicBroker>>>; MQ_SHARD_COUNT]>;

/// Fixed-size sharded receipt-to-topic lookup array.
pub(crate) type ReceiptShardArray = Box<[RwLock<HashMap<u64, Arc<TopicBroker>>>; MQ_SHARD_COUNT]>;
