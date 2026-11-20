/// Number of submission queue entries allocated per Linux `io_uring` worker ring.
pub(crate) const URING_RING_ENTRIES: u32 = 2048;

/// Number of concurrent `Accept` SQEs armed on a single-worker `io_uring` thread.
pub(crate) const URING_SINGLE_WORKER_ACCEPT_SLOTS: usize = 8;

/// Number of concurrent `Accept` SQEs armed per worker on multi-worker `io_uring` servers
/// so incoming bursts balance evenly across workers without single-thread accept hoarding.
pub(crate) const URING_MULTI_WORKER_ACCEPT_SLOTS: usize = 2;

/// Fixed heap buffer size for each connection's `Recv` and `Send` `io_uring` operations.
pub(crate) const URING_IO_BUF_SIZE: usize = 16 * 1024;

/// Maximum response tail size eligible for an in-kernel linked send and close.
pub(crate) const URING_LINKED_SEND_LIMIT: usize = 4 * 1024;

/// Maximum buffered outbound bytes queued while a `Send` SQE is already in flight.
pub(crate) const URING_PENDING_WRITE_LIMIT: usize = 64 * 1024;

/// Maximum recycled 16 KiB `Box<[u8]>` buffers retained per `io_uring` worker thread.
pub(crate) const URING_MAX_POOLED_BUFS: usize = 512;

/// Bit shift separating the 8-bit operation tag from the 56-bit slot index in `user_data`.
pub(crate) const URING_TAG_SHIFT: u32 = 56;

/// Bit mask extracting the 56-bit slot index from an `io_uring` `user_data` value.
pub(crate) const URING_INDEX_MASK: u64 = (1u64 << URING_TAG_SHIFT) - 1;

/// `user_data` tag for `opcode::Accept` operations.
pub(crate) const URING_TAG_ACCEPT: u64 = 1;

/// `user_data` tag for `eventfd` wakeup `opcode::PollAdd` operations.
pub(crate) const URING_TAG_EVENTFD: u64 = 2;

/// `user_data` tag for `opcode::Recv` operations.
pub(crate) const URING_TAG_RECV: u64 = 3;

/// `user_data` tag for `opcode::Send` operations.
pub(crate) const URING_TAG_SEND: u64 = 4;

/// `user_data` tag for `opcode::Close` operations.
pub(crate) const URING_TAG_CLOSE: u64 = 5;

/// `user_data` tag for `opcode::AsyncCancel` operations.
pub(crate) const URING_TAG_CANCEL: u64 = 6;

/// Thread name prefix for dedicated Linux `io_uring` worker threads.
pub(crate) const URING_THREAD_NAME_PREFIX: &str = "hyperlane-uring-";

/// Error message returned when a transport is polled outside its owning `io_uring` thread.
pub(crate) const URING_REACTOR_UNAVAILABLE: &str = "io_uring reactor unavailable";
