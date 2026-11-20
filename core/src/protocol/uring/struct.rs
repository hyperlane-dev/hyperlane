use super::*;

/// Cross-thread wakeup channel bridging Tokio timers/channels/shutdown into an `io_uring` worker.
pub(crate) struct UringWakeState {
    /// Non-blocking Linux `eventfd` polled inside the worker's `io_uring` ring.
    pub(crate) eventfd: RawFd,
    /// Slot indices woken asynchronously from outside the `io_uring` loop.
    pub(crate) ready: Mutex<Vec<usize>>,
    /// Coalesces multiple wakes into a single `eventfd` write.
    pub(crate) signaled: AtomicBool,
    /// Set when server shutdown is requested.
    pub(crate) shutdown: AtomicBool,
}

/// Standard `Wake` implementation notifying an `io_uring` worker when a connection slot is ready.
pub(crate) struct UringSlotWaker {
    /// Owning connection slab slot index.
    pub(crate) slot_id: usize,
    /// Worker wakeup state shared with the `io_uring` thread.
    pub(crate) wake_state: Arc<UringWakeState>,
}

/// Fixed-address accept state for one concurrent `opcode::Accept` SQE.
pub(crate) struct UringAcceptSlot {
    /// Kernel-filled peer address storage kept at a stable heap address while accept is in flight.
    pub(crate) addr: Box<sockaddr_storage>,
    /// Kernel-updated peer address length kept at a stable heap address while accept is in flight.
    pub(crate) addr_len: Box<socklen_t>,
    /// Whether an `opcode::Accept` SQE is currently in flight for this slot.
    pub(crate) inflight: bool,
}

/// All fixed-address I/O buffers and task state for one connection owned by an `io_uring` worker.
pub(crate) struct UringConnSlot {
    /// Accepted TCP socket file descriptor.
    pub(crate) fd: RawFd,
    /// Unique connection identity retained independently of the reusable slab index.
    pub(crate) connection_id: u64,
    /// Fixed-address receive buffer passed to `opcode::Recv` (never reallocated while in flight).
    pub(crate) read_buf: Box<[u8]>,
    /// Offset of the first unread byte in `read_buf`.
    pub(crate) read_pos: usize,
    /// Offset one past the last valid byte in `read_buf`.
    pub(crate) read_len: usize,
    /// Fixed-address transmit buffer passed to `opcode::Send` (never reallocated while in flight).
    pub(crate) write_buf: Box<[u8]>,
    /// Offset of the first unsent byte in `write_buf`.
    pub(crate) write_pos: usize,
    /// Offset one past the last valid byte in `write_buf`.
    pub(crate) write_len: usize,
    /// Staging queue for writes arriving while a `Send` SQE is already in flight.
    pub(crate) pending_write: Vec<u8>,
    /// Whether `write_buf[0..write_len]` has staged bytes ready to submit as one `Send` SQE.
    pub(crate) send_staged: bool,
    /// Whether a `Recv` SQE is currently in flight for this slot.
    pub(crate) recv_inflight: bool,
    /// Whether a `Send` SQE is currently in flight for this slot.
    pub(crate) send_inflight: bool,
    /// Whether a `Close` SQE is currently in flight for this slot.
    pub(crate) close_inflight: bool,
    /// Whether a flush caller is waiting for accepted bytes to reach the socket.
    pub(crate) flush_pending: bool,
    /// Whether a receive cancellation CQE must finish before this slot can be reused.
    pub(crate) cancel_inflight: bool,
    /// Whether peer EOF (`Recv` returned `0`) has been observed.
    pub(crate) eof: bool,
    /// Whether `poll_shutdown` has been requested by the connection future.
    pub(crate) write_shutdown: bool,
    /// Whether the connection future has finished and the slot is draining before `Close`.
    pub(crate) closing: bool,
    /// Pending OS read error delivered on the next `poll_read`.
    pub(crate) read_err: Option<i32>,
    /// Pending OS write error delivered on the next `poll_write`.
    pub(crate) write_err: Option<i32>,
    /// Cached `Waker` for polling this connection's future.
    pub(crate) waker: Waker,
    /// Active connection lifecycle future (`None` while currently being polled or closing).
    pub(crate) task: Option<UringTaskFuture>,
    /// Deduplicates insertions into the worker's local ready queue.
    pub(crate) in_local_ready: bool,
}

/// Worker-local Linux `io_uring` reactor owning the ring, connection slab, and buffer pool.
pub(crate) struct UringReactor {
    /// Dedicated per-worker `io_uring` instance (`SINGLE_ISSUER | DEFER_TASKRUN`).
    pub(crate) ring: IoUring,
    /// Complete submission groups retained when the kernel SQ has insufficient room.
    pub(crate) pending_submissions: VecDeque<UringSubmission>,
    /// Connection slab indexed by `slot_id`.
    pub(crate) slots: Vec<Option<UringConnSlot>>,
    /// Free slab indices available for immediate reuse.
    pub(crate) free_slots: Vec<usize>,
    /// Recycled fixed-size 16 KiB buffers eliminating steady-state heap allocations.
    pub(crate) buf_pool: Vec<Box<[u8]>>,
    /// Reusable per-slot `Waker` instances avoiding `Arc` allocation on short connections.
    pub(crate) waker_cache: Vec<Option<Waker>>,
    /// Connection slots scheduled for polling on the current worker tick.
    pub(crate) local_ready: Vec<usize>,
    /// Shared wakeup state bridging Tokio async primitives into this worker.
    pub(crate) wake_state: Arc<UringWakeState>,
    /// Count of currently active (non-None) connection slots for clean shutdown draining.
    pub(crate) active_slots: usize,
    /// Next unique identity; zero means exhaustion and never permits identity reuse.
    pub(crate) next_connection_id: u64,
    /// Cached owning thread identity for constant-time transport ownership checks.
    pub(crate) owner_thread: thread::ThreadId,
}

/// Zero-syscall [`Transport`] handle backed by the worker-local Linux `io_uring` reactor.
#[derive(Debug)]
pub(crate) struct UringTcpTransport {
    /// Slab index inside the owning worker's [`UringReactor`].
    pub(crate) slot_id: usize,
    /// Unique identity of the accepted connection, not just its reusable slab index.
    pub(crate) connection_id: u64,
    /// Owning reactor thread, preventing another worker's slab from being accessed.
    pub(crate) owner_thread: thread::ThreadId,
    /// Real local and remote socket addresses captured at `Accept` time.
    pub(crate) peers: StreamPeers,
}
