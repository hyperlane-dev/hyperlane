use super::*;

/// The counter of the allocations the process made.
pub(crate) static ALLOCATIONS: AtomicU64 = AtomicU64::new(0);

/// The counter of the bytes the process allocated.
pub(crate) static ALLOCATED_BYTES: AtomicU64 = AtomicU64::new(0);

/// The counter of the requests the server answered.
pub(crate) static REQUESTS: AtomicU64 = AtomicU64::new(0);

/// The global allocator of the diagnostic build.
#[global_allocator]
pub(crate) static GLOBAL: Counting = Counting;
