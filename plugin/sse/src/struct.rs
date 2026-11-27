use super::*;

/// One Server-Sent Event frame with optional id, event type, retry hint, comment, and data payload.
#[derive(Clone, Data, Debug, Default, Eq, PartialEq)]
pub struct SseEvent {
    /// Optional event identifier emitted as `id: <value>`.
    #[get(type(clone))]
    pub(crate) id: Option<String>,
    /// Optional event name emitted as `event: <value>`.
    #[get(type(clone))]
    pub(crate) event: Option<String>,
    /// Event payload emitted as one or more `data: <line>` lines.
    pub(crate) data: String,
    /// Optional client reconnection delay in milliseconds (`retry: <ms>`).
    #[get(type(copy))]
    pub(crate) retry: Option<u64>,
    /// Optional SSE comment line emitted as `: <comment>`.
    #[get(type(clone))]
    pub(crate) comment: Option<String>,
}

/// Configuration for [`SseHub`] and [`SseStream`].
#[derive(Clone, Data, Debug, Eq, PartialEq)]
pub struct SseConfig {
    /// Heartbeat comment interval in milliseconds (`0` disables heartbeats).
    #[get(type(copy))]
    pub(crate) heartbeat_ms: u64,
    /// Reconnection hint sent to clients in milliseconds (`0` omits `retry:`).
    #[get(type(copy))]
    pub(crate) retry_ms: u64,
    /// Maximum number of recent events retained per topic for `Last-Event-ID` replay.
    #[get(type(copy))]
    pub(crate) replay_capacity: usize,
    /// Broadcast channel buffer capacity per topic.
    #[get(type(copy))]
    pub(crate) channel_capacity: usize,
    /// Maximum concurrent distinct topics tracked by an [`SseHub`].
    #[get(type(copy))]
    pub(crate) max_topics: usize,
}

/// Per-topic broadcast channel and independently locked bounded replay ring.
pub(crate) struct TopicState {
    /// Live broadcast sender for this topic (lock-free fan-out).
    pub(crate) sender: Sender<SseEvent>,
    /// Bounded FIFO replay buffer storing `(sequence_id, event)`.
    pub(crate) replay: Mutex<VecDeque<(u64, SseEvent)>>,
}

/// Shared sharded interior state of an [`SseHub`].
pub(crate) struct SseHubInner {
    /// Configuration limits governing this hub.
    pub(crate) config: SseConfig,
    /// Monotonic event sequence generator across all topics.
    pub(crate) next_seq: AtomicU64,
    /// Lock-free counter tracking total active topics across all shards.
    pub(crate) active_topics: AtomicUsize,
    /// 64-way lock-striped topic registry indexed by `XxHash3_64`.
    pub(crate) shards: SseShardArray,
}

/// Thread-safe sharded multi-topic Server-Sent Events pub/sub hub with `Last-Event-ID` replay.
#[derive(Clone)]
pub struct SseHub {
    /// Shared hub state.
    pub(crate) inner: Arc<SseHubInner>,
}

/// Active subscriber handle returned by [`SseHub::subscribe`].
pub struct SseSubscription {
    /// Topic name owned by this subscription for cleanup on drop.
    pub(crate) topic: String,
    /// Reference to the parent hub for automatic empty-topic eviction.
    pub(crate) hub: Arc<SseHubInner>,
    /// Buffered replay events captured on subscribe when `Last-Event-ID` matched.
    pub(crate) replayed: VecDeque<SseEvent>,
    /// Live broadcast receiver for subsequent events.
    pub(crate) receiver: Receiver<SseEvent>,
}

/// Helper for configuring SSE response headers and streaming events over a Hyperlane [`Context`].
pub struct SseStream;

/// Process-wide [`SsePlugin`] registry storing an [`SseConfig`] and shared [`SseHub`].
#[derive(Clone)]
pub struct SsePlugin {
    /// Active plugin configuration.
    pub(crate) config: SseConfig,
    /// Shared process-wide SSE pub/sub hub.
    pub(crate) hub: SseHub,
}
