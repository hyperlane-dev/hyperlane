/// Pre-encoded HTTP/1.1 200 OK status line and content-length field prefix.
pub(crate) const FAST_HTTP11_200_CL_PREFIX: &[u8] = b"HTTP/1.1 200 OK\r\ncontent-length:";

/// Pre-encoded HTTP/2 200 OK status line and content-length field prefix.
pub(crate) const FAST_HTTP2_200_CL_PREFIX: &[u8] = b"HTTP/2 200 OK\r\ncontent-length:";

/// Pre-encoded HTTP/3 200 OK status line and content-length field prefix.
pub(crate) const FAST_HTTP3_200_CL_PREFIX: &[u8] = b"HTTP/3 200 OK\r\ncontent-length:";

/// Pre-encoded default keep-alive and text/html headers terminating the fast-path head.
pub(crate) const FAST_DEFAULT_HEADERS_SUFFIX: &[u8] =
    b"\r\nconnection:keep-alive\r\ncontent-type:text/html; charset=utf-8\r\n\r\n";

/// Pre-encoded content-length field prefix after a custom status line.
pub(crate) const FAST_CL_PREFIX: &[u8] = b"\r\ncontent-length:";
