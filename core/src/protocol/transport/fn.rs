use super::*;

/// Builds the transport pair one protocol request stream runs over.
///
/// # Arguments
///
/// - `RequestConfig` - The parsing limits and read timeout for the stream.
/// - `Option<SocketAddr>` - The local address of the carrying connection.
///
/// # Returns
///
/// - `(Stream, DuplexStream)` - The pipeline's stream and the pump's read end.
pub(crate) fn build_duplex_transport(
    request_config: RequestConfig,
    local: Option<SocketAddr>,
    remote: Option<SocketAddr>,
) -> (Stream, DuplexStream) {
    let (pipe, peer): (DuplexStream, DuplexStream) = tokio::io::duplex(PROTOCOL_STREAM_CAPACITY);
    let transport: DuplexTransport = DuplexTransport {
        pipe,
        local,
        remote,
    };
    (
        Stream::from_transport(transport, request_config, false),
        peer,
    )
}
