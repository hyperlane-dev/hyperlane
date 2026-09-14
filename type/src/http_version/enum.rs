use super::*;

/// Represents the HTTP version used in the request or response.
///
/// This enum defines the various HTTP protocol versions supported,
/// including HTTP/0.9, HTTP/1.0, HTTP/1.1, HTTP/2, and HTTP/3.
/// It also includes an `Unknown` variant for unrecognized versions.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub enum HttpVersion {
    /// HTTP version 0.9
    Http0_9,
    /// HTTP version 1.0
    Http1_0,
    /// HTTP version 1.1
    #[default]
    Http1_1,
    /// HTTP version 2.0
    Http2,
    /// HTTP version 3.0
    Http3,
    /// Unknown version
    Unknown(String),
}
