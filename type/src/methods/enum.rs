use super::*;

/// Defines the `Method` enum, representing HTTP request methods.
///
/// This enum provides a comprehensive list of standard HTTP methods,
/// such as GET, POST, PUT, DELETE, PATCH, HEAD, OPTIONS, CONNECT, and TRACE.
/// It also includes an `UNKNOWN` variant for unrecognized methods.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum Method {
    /// Represents the HTTP `GET` method.
    Get,
    /// Represents the HTTP `POST` method.
    Post,
    /// Represents the HTTP `PUT` method.
    Put,
    /// Represents the HTTP `DELETE` method.
    Delete,
    /// Represents the HTTP `PATCH` method.
    Patch,
    /// Represents the HTTP `HEAD` method.
    Head,
    /// Represents the HTTP `OPTIONS` method.
    Options,
    /// Represents the HTTP `CONNECT` method.
    Connect,
    /// Represents the HTTP `TRACE` method.
    Trace,
    /// Unknown
    Unknown(String),
}
