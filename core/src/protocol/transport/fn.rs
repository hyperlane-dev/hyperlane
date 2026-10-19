use super::*;

/// Builds the transport pair one protocol request stream runs over.
///
/// The peers are carried as one value because the transport reports both ends
/// of the same connection.
///
/// # Arguments
///
/// - `RequestConfig` - The parsing limits and read timeout for the stream.
/// - `StreamPeers` - The addresses of the carrying connection.
///
/// # Returns
///
/// - `(Stream, DuplexStream)` - The pipeline's stream and the pump's read end.
pub(crate) fn build_duplex_transport(
    request_config: RequestConfig,
    peers: StreamPeers,
) -> (Stream, DuplexStream) {
    let (pipe, peer): (DuplexStream, DuplexStream) = tokio::io::duplex(PROTOCOL_STREAM_CAPACITY);
    let transport: DuplexTransport = DuplexTransport {
        pipe,
        local: Some(peers.local),
        remote: Some(peers.remote),
    };
    (
        Stream::from_transport(transport, request_config, false),
        peer,
    )
}
