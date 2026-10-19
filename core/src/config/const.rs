/// ALPN identifier advertised for HTTP/1.1.
pub(crate) const ALPN_HTTP1: &[u8] = b"http/1.1";

/// ALPN identifier advertised for HTTP/2 over TLS.
pub(crate) const ALPN_HTTP2: &[u8] = b"h2";

/// ALPN identifier advertised for HTTP/3 over QUIC.
pub(crate) const ALPN_HTTP3: &[u8] = b"h3";
