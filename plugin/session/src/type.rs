use super::*;

/// Boxed `Send` future returned by async [`SessionStore`] trait methods.
pub type SessionFuture<'a, T> = Pin<Box<dyn Future<Output = T> + Send + 'a>>;

/// Thread-safe shared reference to a pluggable [`SessionStore`].
pub type SharedSessionStore = Arc<dyn SessionStore>;

/// Fixed-size 64-way sharded session storage array.
pub(crate) type SessionShardArray =
    Box<[RwLock<HashMapXxHash3_64<String, StoredSession>>; SESSION_SHARD_COUNT]>;
