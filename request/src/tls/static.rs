use super::*;

/// Cached default TLS configuration for HTTP/1.1 requests with session resumption enabled.
pub(crate) static DEFAULT_H1_TLS_CONFIG: OnceLock<Arc<ClientConfig>> = OnceLock::new();

/// Cached default TLS configuration advertising HTTP/2 ALPN with session resumption enabled.
pub(crate) static DEFAULT_H2_TLS_CONFIG: OnceLock<Arc<ClientConfig>> = OnceLock::new();
