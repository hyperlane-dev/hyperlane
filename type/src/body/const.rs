/// The `Content-Type` value that declares a `multipart/form-data` body.
pub(crate) const MULTIPART_FORM_DATA: &str = "multipart/form-data";

/// The `boundary` parameter name carried by a `multipart/form-data` content type.
pub(crate) const MULTIPART_BOUNDARY_PARAMETER: &str = "boundary";

/// The `name` parameter name carried by a part `Content-Disposition` header.
pub(crate) const MULTIPART_NAME_PARAMETER: &str = "name";

/// The `Content-Disposition` header name carried by every `multipart/form-data` part.
pub(crate) const MULTIPART_CONTENT_DISPOSITION: &str = "content-disposition";

/// The dash pair that opens a boundary line and closes the closing boundary.
pub(crate) const MULTIPART_DELIMITER_DASH: &str = "--";

/// The quote character wrapping a `Content-Disposition` or `boundary` parameter value.
pub(crate) const MULTIPART_PARAMETER_QUOTE: &str = "\"";

/// The maximum number of parts accepted from a single `multipart/form-data` body.
pub(crate) const MULTIPART_MAX_PARTS: usize = 1024;

/// The bare line feed used by lenient parsers that accept `\n` line breaks.
pub(crate) const MULTIPART_LINE_FEED: &[u8] = b"\n";

/// The radix of a hexadecimal digit pair in a percent escape.
pub(crate) const HEX_RADIX: u8 = 16;

/// The offset turning a hexadecimal letter digit into its numeric value.
pub(crate) const HEX_LOWER_OFFSET: u8 = 10;

/// The blank-line separator between a part header block and its payload.
pub(crate) const MULTIPART_HEADER_SEPARATOR: &[u8] = b"\n\n";

/// The byte offset consumed by [`MULTIPART_HEADER_SEPARATOR`].
pub(crate) const MULTIPART_HEADER_SEPARATOR_LEN: usize = 2;

/// The byte a `+` in a URL-encoded form stands in for.
pub(crate) const URL_ENCODED_SPACE: u8 = b' ';

/// The byte that opens a percent escape in a URL-encoded form.
pub(crate) const URL_ENCODED_PERCENT: u8 = b'%';

/// The byte that stands in for a space in a URL-encoded form.
pub(crate) const URL_ENCODED_PLUS: u8 = b'+';

/// The first byte of the decimal digit range in a percent escape.
pub(crate) const HEX_DIGIT_ZERO: u8 = b'0';

/// The last byte of the decimal digit range in a percent escape.
pub(crate) const HEX_DIGIT_NINE: u8 = b'9';

/// The first byte of the lowercase hexadecimal range in a percent escape.
pub(crate) const HEX_DIGIT_LOWER_A: u8 = b'a';

/// The last byte of the lowercase hexadecimal range in a percent escape.
pub(crate) const HEX_DIGIT_LOWER_F: u8 = b'f';

/// The first byte of the uppercase hexadecimal range in a percent escape.
pub(crate) const HEX_DIGIT_UPPER_A: u8 = b'A';

/// The last byte of the uppercase hexadecimal range in a percent escape.
pub(crate) const HEX_DIGIT_UPPER_F: u8 = b'F';
