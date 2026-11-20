/// The largest buffer the pool keeps for reuse.
///
/// A buffer that grew past this size is dropped rather than pooled, so one
/// large response does not make every worker hold a large buffer.
pub(crate) const MAX_POOLED_BUFFER_CAPACITY: usize = 64 * 1024;

/// The number of contexts retained per worker thread for reuse.
pub(crate) const MAX_POOLED_CONTEXTS: usize = 64;

/// The number of recycled protocol header entries retained per worker thread.
pub(crate) const MAX_POOLED_PROTOCOL_HEADERS: usize = 256;

/// The largest string capacity retained in protocol header or target pools.
pub(crate) const MAX_POOLED_STRING_CAPACITY: usize = 4 * 1024;
