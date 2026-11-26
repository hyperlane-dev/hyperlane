/// ALPN protocol token advertised for HTTP/1.1 TLS connections.
pub(crate) const ALPN_HTTP1_1: &[u8] = b"http/1.1";

/// ALPN protocol token advertised for HTTP/2 TLS connections.
pub(crate) const ALPN_H2: &[u8] = b"h2";

/// Number of TLS client sessions cached in memory for fast TLS 1.3 resumption.
pub(crate) const TLS_CLIENT_SESSION_CACHE_CAP: usize = 1024;

/// Error message returned when PEM root certificate parsing yields no valid certificates.
pub(crate) const INVALID_PEM_CERT_ERROR: &str = "Invalid PEM root certificate";
