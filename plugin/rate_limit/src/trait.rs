use super::*;

/// Extracts the rate-limiting partition key (for example client IP, API key, or user + route).
pub trait KeyExtractor: Send + Sync + 'static {
    /// Computes the bucket key for an incoming request.
    ///
    /// # Arguments
    ///
    /// - `&Stream` - Underlying transport stream (for peer socket address inspection).
    /// - `&Context` - Request and response context.
    ///
    /// # Returns
    ///
    /// - `String` - Partition key identifying the rate-limit bucket.
    fn extract(&self, stream: &Stream, ctx: &Context) -> String;
}

/// Pluggable rate-limit decision backend (in-memory sharded or external distributed store).
pub trait RateLimitStore: Send + Sync + 'static {
    /// Evaluates and records one request against `key`.
    ///
    /// # Arguments
    ///
    /// - `&str` - Bucket partition key.
    ///
    /// # Returns
    ///
    /// - `RateLimitDecision` - Admission decision and remaining quota metadata.
    fn check(&self, key: &str) -> RateLimitDecision;

    /// Evicts stale expired bucket entries to bound memory usage.
    ///
    /// # Returns
    ///
    /// - `usize` - Number of evicted entries.
    fn purge_expired(&self) -> usize {
        0
    }
}
