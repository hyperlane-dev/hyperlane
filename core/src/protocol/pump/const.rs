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

/// Maximum number of query parameters parsed from a single HTTP/2 or HTTP/3 request target.
pub(crate) const PROTOCOL_MAX_QUERY_PARAM_COUNT: usize = 1024;

/// Allowed `te` header token in HTTP/2 and HTTP/3 requests per RFC 9113 §8.2.2.
pub(crate) const HTTP_TE_TRAILERS: &str = "trailers";
