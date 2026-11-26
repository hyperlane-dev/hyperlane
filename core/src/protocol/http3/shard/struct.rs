use super::*;

/// Generates cryptographically random 8-byte QUIC Connection IDs whose leading
/// byte routes packets back to `shard_id` even after client IP/port migration.
pub(crate) struct ShardCidGenerator {
    /// Zero-based index of the owning QUIC endpoint shard.
    pub(super) shard_id: u8,
    /// Total number of QUIC endpoint shards sharing the exclusive UDP socket.
    pub(super) shard_count: u8,
}

/// One incoming UDP datagram forwarded by the exclusive-socket dispatcher to a shard.
pub(crate) struct ShardDatagram {
    /// Pooled datagram payload bytes.
    pub(super) data: Vec<u8>,
    /// Receive metadata preserved from the real UDP socket.
    pub(super) meta: RecvMeta,
}

/// Per-shard incoming datagram queue, recycled buffer pool, and driver waker.
pub(crate) struct ShardQueueState {
    /// Queued datagrams awaiting consumption by the shard's `EndpointDriver`.
    pub(super) queue: VecDeque<ShardDatagram>,
    /// Per-shard recycled datagram buffers avoiding cross-shard pool contention.
    pub(super) pool: Vec<Vec<u8>>,
    /// Waker registered by the shard's `poll_recv` call.
    pub(super) waker: Option<Waker>,
    /// Set once the exclusive UDP dispatcher shuts down.
    pub(super) closed: bool,
}

/// Virtual [`AsyncUdpSocket`] for one QUIC endpoint shard sharing a single exclusive UDP socket.
///
/// Outbound sends go directly to the real OS UDP socket (preserving GSO and zero-hop latency),
/// while inbound datagrams are demultiplexed by Connection ID from the single exclusive socket.
pub(crate) struct ShardUdpSocket {
    /// The single exclusively bound OS UDP socket wrapped by Quinn's Tokio runtime.
    pub(super) real_socket: Arc<dyn AsyncUdpSocket>,
    /// Inbound datagram queue and per-shard buffer pool for this shard.
    pub(super) queue: Arc<Mutex<ShardQueueState>>,
    /// Uncontended shard-local recycled buffers flushed to `queue.pool` on the next `poll_recv`.
    pub(super) recycle: Mutex<Vec<Vec<u8>>>,
}

/// Collection of one or more QUIC endpoints plus the optional exclusive-socket dispatcher thread.
pub(crate) struct QuicEndpointSet {
    /// Bound QUIC endpoint(s) served concurrently.
    pub(crate) endpoints: Vec<Endpoint>,
    /// Dedicated OS thread draining the single exclusive OS UDP socket without worker starvation.
    pub(crate) dispatcher: Option<ThreadJoinHandle<()>>,
    /// Signal stopping the UDP receive demultiplexer after every endpoint drains.
    pub(crate) dispatcher_stop: Option<Sender<()>>,
}
