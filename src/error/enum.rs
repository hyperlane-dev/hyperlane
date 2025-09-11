use crate::*;

/// Represents errors that can occur at the server level.
#[derive(CustomDebug, DisplayDebug, PartialEq, Eq, Clone)]
pub enum ServerError {
    /// An error occurred while trying to bind to a TCP socket.
    TcpBind(String),
    /// An unknown or unexpected error occurred.
    Unknown(String),
    /// An error occurred while reading an HTTP request.
    HttpRead(String),
    /// The received HTTP request was invalid or malformed.
    InvalidHttpRequest(Request),
    /// Other error.
    Other(String),
}

/// Represents errors related to route definitions and matching.
#[derive(CustomDebug, DisplayDebug, PartialEq, Eq, Clone)]
pub enum RouteError {
    /// The route pattern cannot be empty.
    EmptyPattern,
    /// A route with the same pattern has already been defined.
    DuplicatePattern(String),
    /// The provided route pattern is not a valid regular expression.
    InvalidRegexPattern(String),
}
