use super::*;

/// Maximum number of bytes in a chunk size or trailer line, including CRLF.
pub(crate) const MAX_CHUNK_LINE_BYTES: usize = 8 * 1024;

/// Maximum total number of extension bytes across all chunk size lines.
pub(crate) const MAX_CHUNK_EXTENSION_BYTES: usize = 64 * 1024;

/// Maximum number of bytes in the trailer section, including its final CRLF.
pub(crate) const MAX_CHUNK_TRAILER_BYTES: usize = 64 * 1024;

/// Error returned when chunk framing is malformed or incomplete.
pub(crate) const INVALID_CHUNK: RequestError = RequestError::HttpRead(HttpStatus::BadRequest);

/// Error returned when the chunk metadata exceeds its bounds.
pub(crate) const CHUNK_METADATA_TOO_LARGE: RequestError =
    RequestError::HeaderLineTooLong(HttpStatus::RequestHeaderFieldsTooLarge);
