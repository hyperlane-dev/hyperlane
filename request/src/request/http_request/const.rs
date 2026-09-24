/// Byte pattern for matching 'content-length' header in HTTP requests.
///
/// Used for case-sensitive matching of the content-length header.
pub(crate) const CONTENT_LENGTH_PATTERN: &[u8] = b"content-length:";

/// Byte pattern for matching 'transfer-encoding' header in HTTP requests.
///
/// Used for case-sensitive matching of the transfer-encoding header.
pub(crate) const TRANSFER_ENCODING_PATTERN: &[u8] = b"transfer-encoding:";

/// Byte pattern for matching 'chunked' value in HTTP headers.
///
/// Used for case-sensitive matching of the chunked transfer encoding value.
pub(crate) const CHUNKED_PATTERN: &[u8] = b"chunked";
