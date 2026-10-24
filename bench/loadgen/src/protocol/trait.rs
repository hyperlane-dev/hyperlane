use super::*;

/// The transport a protocol client writes its bytes over.
///
/// The HTTP/1.1 client reads and writes raw bytes, so a plain socket and a
/// TLS session share one implementation through this trait.
pub(crate) trait Transport: AsyncRead + AsyncWrite + Unpin + Send {}

impl<T> Transport for T where T: AsyncRead + AsyncWrite + Unpin + Send {}
