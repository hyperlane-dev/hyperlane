use super::*;

/// Configuration controlling queue bounds, visibility timeouts, retries, and DLQ naming.
#[derive(Clone, Data, Debug, Eq, PartialEq)]
pub struct MqConfig {
    /// Maximum pending messages allowed per `(topic, group)` queue.
    #[get(type(copy))]
    pub(crate) max_queue_len: usize,
    /// Visibility timeout in milliseconds before unacked in-flight messages redeliver.
    #[get(type(copy))]
    pub(crate) visibility_timeout_ms: u64,
    /// Default retry budget per message before dead-lettering.
    #[get(type(copy))]
    pub(crate) default_max_retries: u32,
    /// Suffix appended to a topic name when moving messages to its dead-letter queue.
    pub(crate) dlq_suffix: String,
}

/// A message published to and consumed from a [`MessageBroker`].
#[derive(Clone, Data, Debug, Default, Eq, PartialEq)]
pub struct MqMessage {
    /// Monotonic broker-assigned identifier (`0` before publish).
    #[get(type(copy))]
    pub(crate) id: u64,
    /// Target topic name.
    pub(crate) topic: String,
    /// Optional routing or partitioning key.
    #[get(type(clone))]
    pub(crate) key: Option<String>,
    /// Raw payload bytes.
    pub(crate) payload: Vec<u8>,
    /// Optional string metadata headers.
    pub(crate) headers: HashMap<String, String>,
    /// Delivery attempt counter incremented on each dispatch.
    #[get(type(copy))]
    pub(crate) attempts: u32,
    /// Maximum attempts permitted before moving to DLQ (`0` inherits broker default).
    #[get(type(copy))]
    pub(crate) max_retries: u32,
    /// Creation timestamp in Unix epoch milliseconds.
    #[get(type(copy))]
    pub(crate) created_at_ms: u64,
    /// Optional expiration timestamp in Unix epoch milliseconds.
    #[get(type(copy))]
    pub(crate) expires_at_ms: Option<u64>,
}

/// Active delivery leased to a consumer until acknowledged, negatively acknowledged, or timed out.
#[derive(Clone, Data, Debug, Eq, PartialEq)]
pub struct MqDelivery {
    /// Delivered message snapshot.
    pub(crate) message: MqMessage,
    /// Unique lease identifier required for [`MessageBroker::ack`] or [`MessageBroker::nack`].
    #[get(type(copy))]
    pub(crate) receipt_id: u64,
    /// Consumer group that owns this lease.
    pub(crate) group: String,
}

/// Point-in-time counters for one `(topic, group)` queue.
#[derive(Clone, Copy, Data, Debug, Default, Eq, PartialEq)]
pub struct MqStats {
    /// Messages currently waiting in the ready queue.
    #[get(type(copy))]
    pub(crate) queued: usize,
    /// Messages currently leased to consumers awaiting ack/nack.
    #[get(type(copy))]
    pub(crate) in_flight: usize,
    /// Messages currently stored in the topic's dead-letter queue.
    #[get(type(copy))]
    pub(crate) dead_lettered: usize,
    /// Cumulative messages published to the topic.
    #[get(type(copy))]
    pub(crate) published_total: u64,
    /// Cumulative messages acknowledged across this group.
    #[get(type(copy))]
    pub(crate) acked_total: u64,
}

/// One leased message waiting for ack or visibility timeout expiry.
pub(crate) struct InFlightEntry {
    /// Consumer group owning this lease.
    pub(crate) group: String,
    /// Leased message.
    pub(crate) message: MqMessage,
    /// Visibility deadline in Unix epoch milliseconds.
    pub(crate) visible_after_ms: u64,
}

/// Mutable broker state protected by a mutex inside [`InMemoryBroker`].
pub(crate) struct BrokerState {
    /// Registered consumer groups per topic.
    pub(crate) topic_groups: HashMap<String, HashSet<String>>,
    /// Ready FIFO queues keyed by `(topic, group)`.
    pub(crate) queues: HashMap<(String, String), VecDeque<MqMessage>>,
    /// Active leased deliveries keyed by `receipt_id`.
    pub(crate) in_flight: HashMap<u64, InFlightEntry>,
    /// Dead-letter queues keyed by `<topic><dlq_suffix>`.
    pub(crate) dlqs: HashMap<String, VecDeque<MqMessage>>,
    /// Total published messages per topic.
    pub(crate) published_totals: HashMap<String, u64>,
    /// Total acknowledged messages per `(topic, group)`.
    pub(crate) acked_totals: HashMap<(String, String), u64>,
}

/// Shared interior state of [`InMemoryBroker`].
pub(crate) struct InMemoryBrokerInner {
    /// Broker configuration.
    pub(crate) config: MqConfig,
    /// Monotonic message id counter.
    pub(crate) next_msg_id: AtomicU64,
    /// Monotonic delivery receipt counter.
    pub(crate) next_receipt_id: AtomicU64,
    /// Mutable topic, group, in-flight, and DLQ storage.
    pub(crate) state: Mutex<BrokerState>,
}

/// Production-ready in-memory [`MessageBroker`] implementation.
#[derive(Clone)]
pub struct InMemoryBroker {
    /// Shared broker state.
    pub(crate) inner: Arc<InMemoryBrokerInner>,
}

/// Process-wide [`MqPlugin`] registry storing [`MqConfig`] and a shared [`InMemoryBroker`].
#[derive(Clone)]
pub struct MqPlugin {
    /// Active plugin configuration.
    pub(crate) config: MqConfig,
    /// Shared process-wide in-memory broker.
    pub(crate) broker: InMemoryBroker,
}
