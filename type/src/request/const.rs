use super::*;

/// The horizontal tab allowed within a field value.
pub(crate) const HTTP_HEADER_TAB: u8 = b'\t';

/// The ASCII delete control byte.
pub(crate) const HTTP_HEADER_DELETE: u8 = 0x7f;

/// The line feed terminating a header line.
pub(crate) const HTTP_HEADER_LINE_FEED: char = '\n';

/// The first byte outside the ASCII control range.
pub(crate) const HTTP_HEADER_CONTROL_LIMIT: u8 = 0x20;

/// The request version that requires a Host field.
pub(crate) const HTTP_HOST_REQUIRED_VERSION: RequestVersion = RequestVersion::Http1_1;

/// The carriage return preceding a line feed.
pub(crate) const HTTP_HEADER_CARRIAGE_RETURN: char = '\r';

/// The punctuation allowed within an HTTP field name.
pub(crate) const HTTP_HEADER_TOKEN_PUNCTUATION: &[u8] = b"!#$%&'*+-.^_`|~";

/// The unsupported request transfer coding field.
pub(crate) const HTTP_HEADER_TRANSFER_ENCODING: &str = "transfer-encoding";

/// The only whitespace allowed around a field value.
pub(crate) const HTTP_HEADER_OPTIONAL_WHITESPACE: [char; 2] = [' ', '\t'];
