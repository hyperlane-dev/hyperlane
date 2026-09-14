use super::*;

/// Configuration for HTTP request parsing security limits.
///
/// This struct defines various limits and constraints to prevent
/// denial-of-service attacks and other security vulnerabilities
/// when parsing HTTP requests.
/// Since all fields implement `Copy`, this struct is lightweight
/// and can be easily cloned and shared.
#[derive(Clone, Copy, Data, Debug, Deserialize, DisplayDebug, Eq, New, PartialEq, Serialize)]
pub struct RequestConfig {
    /// Buffer size for reading operations.
    #[get(type(copy))]
    #[set]
    pub buffer_size: usize,
    /// Maximum size for URL path in bytes.
    #[get(type(copy))]
    #[set]
    pub max_path_size: usize,
    /// Maximum number of headers allowed in a request.
    #[get(type(copy))]
    #[set]
    pub max_header_count: usize,
    /// Maximum size for a header key in bytes.
    #[get(type(copy))]
    #[set]
    pub max_header_key_size: usize,
    /// Maximum size for a header value in bytes.
    #[get(type(copy))]
    #[set]
    pub max_header_value_size: usize,
    /// Maximum size for request body in bytes.
    #[get(type(copy))]
    #[set]
    pub max_body_size: usize,
    /// Timeout for reading data in milliseconds.
    #[get(type(copy))]
    #[set]
    pub read_timeout_ms: u64,
}

/// HTTP request representation.
///
/// Contains all components of an HTTP request.
#[derive(
    Clone, Debug, Deserialize, DisplayDebug, Eq, Getter, GetterMut, PartialEq, Serialize, Setter,
)]
pub struct Request {
    /// HTTP request method.
    pub method: RequestMethod,
    /// Request host.
    pub host: RequestHost,
    /// HTTP protocol version.
    pub version: RequestVersion,
    /// Request path.
    pub path: RequestPath,
    /// URL query parameters.
    pub querys: RequestQuerys,
    /// HTTP headers collection.
    pub headers: RequestHeaders,
    /// Request body content.
    pub body: RequestBody,
}
