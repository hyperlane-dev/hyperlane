/// Maximum idle HTTP/1.1 connections cached per `(origin, proxy)` pool key.
pub(crate) const POOL_MAX_IDLE_PER_HOST: usize = 32;

/// Maximum total idle HTTP/1.1 connections retained across all origins.
pub(crate) const POOL_MAX_IDLE_TOTAL: usize = 256;

/// Idle keep-alive timeout before a pooled connection is discarded (seconds).
pub(crate) const POOL_IDLE_TTL_SECS: u64 = 60;

/// Maximum number of reusable byte buffers kept in each worker's thread-local pool.
pub(crate) const CLIENT_MAX_POOLED_BUFFERS: usize = 32;

/// Largest byte buffer capacity retained in the worker-local buffer pool.
pub(crate) const CLIENT_MAX_POOLED_BUFFER_CAP: usize = 64 * 1024;
