use super::*;

/// The decoded head and body of one protocol request.
///
/// HTTP/2 and HTTP/3 both deliver a request as a protocol head plus a body that
/// has already been collected under the connection's limits, and both are turned
/// into the same framework request here. The parts travel as one value because
/// they only mean anything together: a method without the URI it was sent to, or
/// a body without the `content-length` header that declared it, describes no
/// request at all.
///
/// The head parts are taken by value because the request is built once and never
/// read again by the protocol path. This owns the head parts and the collected
/// body; the conversion borrows the URI and the headers out of those owned
/// parts.
#[derive(Debug)]
pub(crate) struct ProtocolRequestHead {
    /// The real protocol version to stamp on the request.
    pub(crate) version: HttpVersion,
    /// The request method decoded by the protocol library.
    pub(crate) method: ProtocolMethod,
    /// The decoded request head, carrying the URI and the headers.
    pub(crate) parts: http::request::Parts,
    /// The already collected request body.
    pub(crate) body: Vec<u8>,
}
