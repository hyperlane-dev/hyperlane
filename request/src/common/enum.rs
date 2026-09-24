use super::*;

/// Represents the body content of an HTTP request/response.
///
/// Can be text, JSON or binary data.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum Body {
    /// Text content body.
    Text(BodyText),
    /// JSON content body.
    Json(BodyJson),
    /// Binary data body.
    Binary(BodyBinary),
}
