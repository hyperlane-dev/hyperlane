use super::*;

/// Represents different types of errors that can occur when handling HTTP URLs.
///
/// This enum defines various error types specifically related to parsing or
/// otherwise processing HTTP URLs.
#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub enum HttpUrlError {
    /// Indicates that the provided URL is invalid.
    InvalidUrl,
    /// Unknown error
    #[default]
    Unknown,
}
