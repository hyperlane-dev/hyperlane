/// The first byte of the control range, which may not appear in a cookie.
pub(crate) const COOKIE_CTL_FIRST: u8 = 0x00;

/// The last byte of the control range, which may not appear in a cookie.
pub(crate) const COOKIE_CTL_LAST: u8 = 0x1F;

/// The delete byte, which may not appear in a cookie.
pub(crate) const COOKIE_DELETE: u8 = 0x7F;

/// The quote byte, which may not appear in a cookie value.
pub(crate) const COOKIE_VALUE_FORBIDDEN_QUOTE: u8 = 0x22;

/// The comma byte, which may not appear in a cookie value.
pub(crate) const COOKIE_VALUE_FORBIDDEN_COMMA: u8 = 0x2C;

/// The semicolon byte, which may not appear in a cookie value.
pub(crate) const COOKIE_VALUE_FORBIDDEN_SEMICOLON: u8 = 0x3B;

/// The backslash byte, which may not appear in a cookie value.
pub(crate) const COOKIE_VALUE_FORBIDDEN_BACKSLASH: u8 = 0x5C;

/// The punctuation a token may carry beside the letters and the digits.
///
/// RFC 2616 Section 2.2 admits these bytes in a token, and RFC 6265 Section
/// 4.1.1 takes a cookie name from that production.
pub(crate) const COOKIE_TOKEN_PUNCTUATION: &str = "!#$%&'*+-.^_`|~";
