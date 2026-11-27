/// Rate-limiting algorithm used by [`crate::ShardedRateLimiter`].
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum RateLimitAlgorithm {
    /// Generic Cell Rate Algorithm (smooth leaky-bucket with exact burst allowance, lock-free CAS).
    #[default]
    Gcra,
    /// Lock-free atomic fixed-window counter.
    FixedWindow,
}
