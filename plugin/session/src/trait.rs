use super::*;

/// Pluggable asynchronous session backend (in-memory sharded, Redis, PostgreSQL, etc.).
pub trait SessionStore: Send + Sync + 'static {
    /// Loads the unexpired key-value map for `session_id`, if present.
    ///
    /// # Arguments
    ///
    /// - `&'a self` - Shared reference to self.
    /// - `&'a str` - Session identifier.
    ///
    /// # Returns
    ///
    /// - `SessionFuture<'a, Option<HashMap<String, String>>>` - Session attributes if live.
    fn load<'a>(
        &'a self,
        session_id: &'a str,
    ) -> SessionFuture<'a, Option<HashMap<String, String>>>;

    /// Persists `values` under `session_id` with time-to-live `ttl_secs`.
    ///
    /// # Arguments
    ///
    /// - `String` - Session identifier.
    /// - `HashMap<String, String>` - Session key-value attributes.
    /// - `u64` - Session TTL in seconds.
    ///
    /// # Returns
    ///
    /// - `SessionFuture<'_, ()>` - Completion future.
    fn save(
        &self,
        session_id: String,
        values: HashMap<String, String>,
        ttl_secs: u64,
    ) -> SessionFuture<'_, ()>;

    /// Deletes a session record from the store.
    ///
    /// # Arguments
    ///
    /// - `&'a self` - Shared reference to self.
    /// - `&'a str` - Session identifier to delete.
    ///
    /// # Returns
    ///
    /// - `SessionFuture<'a, bool>` - `true` when an entry existed and was deleted.
    fn delete<'a>(&'a self, session_id: &'a str) -> SessionFuture<'a, bool>;

    /// Evicts expired sessions and returns the number reclaimed.
    ///
    /// # Returns
    ///
    /// - `usize` - Count of expired sessions purged.
    fn purge_expired(&self) -> usize {
        0
    }
}
