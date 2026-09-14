use super::*;

#[derive(Clone, Debug, Deserialize, DisplayDebug, Eq, PartialEq, Serialize)]
pub enum RequestError {
    /// HTTP read error with HTTP status
    HttpRead(HttpStatus),
    /// TCP stream connection error with HTTP status
    GetTcpStream(HttpStatus),
    /// TLS stream connection error with HTTP status
    GetTlsStream(HttpStatus),
    /// Connection read error with HTTP status
    ReadConnection(HttpStatus),
    /// Request was aborted with HTTP status
    RequestAborted(HttpStatus),
    /// TLS stream connection failed with HTTP status
    TlsStreamConnect(HttpStatus),
    /// Redirect functionality needs to be enabled with HTTP status
    NeedOpenRedirect(HttpStatus),
    /// Maximum redirect times exceeded with HTTP status
    MaxRedirectTimes(HttpStatus),
    /// HTTP method not supported with HTTP status
    MethodsNotSupport(HttpStatus),
    /// Redirect URL is invalid with HTTP status
    RedirectInvalidUrl(HttpStatus),
    /// Client disconnected with HTTP status
    ClientDisconnected(HttpStatus),
    /// Redirect URL dead loop detected with HTTP status
    RedirectUrlDeadLoop(HttpStatus),
    /// Client closed connection with HTTP status
    ClientClosedConnection(HttpStatus),
    /// Server closed connection with HTTP status
    ServerClosedConnection(HttpStatus),
    /// Incomplete WebSocket frame with HTTP status
    IncompleteWebSocketFrame(HttpStatus),
    /// Request too long with HTTP status
    RequestTooLong(HttpStatus),
    /// Path too long with HTTP status
    PathTooLong(HttpStatus),
    /// Query too long with HTTP status
    QueryTooLong(HttpStatus),
    /// Header line too long with HTTP status
    HeaderLineTooLong(HttpStatus),
    /// Too many headers with HTTP status
    TooManyHeaders(HttpStatus),
    /// Header key too long with HTTP status
    HeaderKeyTooLong(HttpStatus),
    /// Header value too long with HTTP status
    HeaderValueTooLong(HttpStatus),
    /// Content length too large with HTTP status
    ContentLengthTooLarge(HttpStatus),
    /// Invalid content length with HTTP status
    InvalidContentLength(HttpStatus),
    /// Invalid URL scheme with HTTP status
    InvalidUrlScheme(HttpStatus),
    /// Invalid URL host with HTTP status
    InvalidUrlHost(HttpStatus),
    /// Invalid URL port with HTTP status
    InvalidUrlPort(HttpStatus),
    /// Invalid URL path with HTTP status
    InvalidUrlPath(HttpStatus),
    /// Invalid URL query with HTTP status
    InvalidUrlQuery(HttpStatus),
    /// Invalid URL fragment with HTTP status
    InvalidUrlFragment(HttpStatus),
    /// Read timeout with HTTP status
    ReadTimeout(HttpStatus),
    /// Write timeout with HTTP status
    WriteTimeout(HttpStatus),
    /// TCP connection failed with HTTP status
    TcpConnectionFailed(HttpStatus),
    /// TLS handshake failed with HTTP status
    TlsHandshakeFailed(HttpStatus),
    /// TLS certificate invalid with HTTP status
    TlsCertificateInvalid(HttpStatus),
    /// WebSocket frame too large with HTTP status
    WebSocketFrameTooLarge(HttpStatus),
    /// WebSocket opcode unsupported with HTTP status
    WebSocketOpcodeUnsupported(HttpStatus),
    /// WebSocket mask missing with HTTP status
    WebSocketMaskMissing(HttpStatus),
    /// WebSocket payload corrupted with HTTP status
    WebSocketPayloadCorrupted(HttpStatus),
    /// WebSocket invalid UTF-8 with HTTP status
    WebSocketInvalidUtf8(HttpStatus),
    /// WebSocket invalid close code with HTTP status
    WebSocketInvalidCloseCode(HttpStatus),
    /// WebSocket invalid extension with HTTP status
    WebSocketInvalidExtension(HttpStatus),
    /// HTTP request parts insufficient with HTTP status
    HttpRequestPartsInsufficient(HttpStatus),
    /// TCP stream connection error with HTTP status  
    TcpStreamConnect(HttpStatus),
    /// TLS connector build error with HTTP status
    TlsConnectorBuild(HttpStatus),
    /// Invalid URL error with HTTP status
    InvalidUrl(HttpStatus),
    /// Configuration read error with HTTP status
    ConfigReadError(HttpStatus),
    /// TCP stream connection error with HTTP status
    TcpStreamConnectString(HttpStatus),
    /// TLS connector build error with HTTP status
    TlsConnectorBuildString(HttpStatus),
    /// Request error with custom message
    Request(String),
    /// Unknown error with HTTP status
    Unknown(HttpStatus),
}
