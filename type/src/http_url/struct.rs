use super::*;

/// Represents the components of a parsed HTTP URL.
///
/// This struct holds various parts of a URL, including the protocol, host, port,
/// path, query, and fragment, allowing for structured access to URL information.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct HttpUrlComponents {
    /// The URL scheme, such as "http" or "https".
    pub protocol: String,
    /// The host part of the URL.
    pub host: Option<String>,
    /// The port number in the URL, if specified.
    pub port: Option<u16>,
    /// The path in the URL.
    pub path: Option<String>,
    /// The query string in the URL.
    pub query: Option<String>,
    /// The fragment identifier.
    pub fragment: Option<String>,
}
