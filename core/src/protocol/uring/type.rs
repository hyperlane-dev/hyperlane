use super::*;

/// Pinned single-threaded connection lifecycle future driven directly by an `io_uring` worker.
pub(crate) type UringTaskFuture = Pin<Box<dyn Future<Output = ()> + 'static>>;
