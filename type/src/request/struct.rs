use super::*;

/// Configuration for HTTP request parsing security limits.
///
/// This struct defines various limits and constraints to prevent
/// denial-of-service attacks and other security vulnerabilities
/// when parsing HTTP requests.
/// Since all fields implement `Copy`, this struct is lightweight
/// and can be easily cloned and shared.
///
/// The fields are crate-internal (§17.18): a consumer reaches them through
/// the generated `get_*` and `set_*` accessors.
#[derive(Clone, Copy, Data, Debug, Deserialize, DisplayDebug, Eq, New, PartialEq, Serialize)]
pub struct RequestConfig {
    /// Buffer size for reading operations.
    #[get(type(copy))]
    pub(crate) buffer_size: usize,
    /// Maximum size for URL path in bytes.
    #[get(type(copy))]
    pub(crate) max_path_size: usize,
    /// Maximum number of headers allowed in a request.
    #[get(type(copy))]
    pub(crate) max_header_count: usize,
    /// Maximum size for a header key in bytes.
    #[get(type(copy))]
    pub(crate) max_header_key_size: usize,
    /// Maximum size for a header value in bytes.
    #[get(type(copy))]
    pub(crate) max_header_value_size: usize,
    /// Maximum size for request body in bytes.
    #[get(type(copy))]
    pub(crate) max_body_size: usize,
    /// Timeout for reading data in milliseconds.
    #[get(type(copy))]
    pub(crate) read_timeout_ms: u64,
}

/// HTTP request representation.
///
/// Contains all components of an HTTP request.
///
/// The fields are crate-internal (§17.18): a consumer reaches them through
/// the generated `get_*` and `set_*` accessors.
#[derive(Clone, Data, Debug, Deserialize, DisplayDebug, Eq, PartialEq, Serialize)]
pub struct Request {
    /// HTTP request method.
    pub(crate) method: RequestMethod,
    /// Request host.
    pub(crate) host: RequestHost,
    /// HTTP protocol version.
    pub(crate) version: RequestVersion,
    /// Request path.
    pub(crate) path: RequestPath,
    /// URL query parameters.
    pub(crate) querys: RequestQuerys,
    /// HTTP headers collection.
    pub(crate) headers: RequestHeaders,
    /// Request body content.
    pub(crate) body: RequestBody,
}
