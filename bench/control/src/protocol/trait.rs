use super::*;

/// The transport one protocol variant writes its bytes over.
///
/// The HTTP/1.1 control reads and writes raw bytes, so the plain socket and
/// the TLS session share one implementation through this trait.
pub(crate) trait Transport: AsyncRead + AsyncWrite + Unpin + Send {}

impl<T> Transport for T where T: AsyncRead + AsyncWrite + Unpin + Send {}
