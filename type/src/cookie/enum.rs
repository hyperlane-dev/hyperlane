use super::*;

/// Represents the reasons a cookie cannot be built or parsed.
///
/// The variants name the part of the cookie that was refused, so a caller can
/// tell a malformed name apart from a malformed value.
#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub enum CookieError {
    /// The cookie name is not a token.
    InvalidName,
    /// The cookie value carries a character that may not appear in one.
    InvalidValue,
    /// The cookie attribute carries a character that may not appear in one.
    InvalidAttribute,
    /// Unknown error
    #[default]
    Unknown,
}
