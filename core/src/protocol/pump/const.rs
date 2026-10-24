/// The number of buffers the pool keeps for reuse.
pub(crate) const MAX_POOLED_BUFFERS: usize = 64;

/// The largest buffer the pool keeps for reuse.
///
/// A buffer that grew past this size is dropped rather than pooled, so one
/// large response does not make every worker hold a large buffer.
pub(crate) const MAX_POOLED_BUFFER_CAPACITY: usize = 64 * 1024;
