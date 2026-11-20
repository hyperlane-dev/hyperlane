use super::*;

/// The horizontal tab allowed within a field value.
pub(crate) const HTTP_HEADER_TAB: u8 = b'\t';

/// The quote delimiting an entity-tag opaque value.
pub(crate) const ENTITY_TAG_QUOTE: u8 = b'"';

/// The ASCII delete control byte.
pub(crate) const HTTP_HEADER_DELETE: u8 = 0x7f;

/// The wildcard matching an existing representation.
pub(crate) const ENTITY_TAG_WILDCARD: &[u8] = b"*";

/// The separator between entity-tags in an HTTP list.
pub(crate) const ENTITY_TAG_SEPARATOR: u8 = b',';

/// The line feed terminating a header line.
pub(crate) const HTTP_HEADER_LINE_FEED: char = '\n';

/// The case-sensitive prefix marking a weak entity-tag.
pub(crate) const ENTITY_TAG_WEAK_PREFIX: &[u8] = b"W/";

/// The whitespace and empty list members ignored before an entity-tag.
pub(crate) const ENTITY_TAG_LIST_PADDING: &[u8] = b" \t,";

/// The first byte outside the ASCII control range.
pub(crate) const HTTP_HEADER_CONTROL_LIMIT: u8 = 0x20;

/// The request version that requires a Host field.
pub(crate) const HTTP_HOST_REQUIRED_VERSION: RequestVersion = RequestVersion::Http1_1;

/// The carriage return preceding a line feed.
pub(crate) const HTTP_HEADER_CARRIAGE_RETURN: char = '\r';

/// The punctuation allowed within an HTTP field name.
pub(crate) const HTTP_HEADER_TOKEN_PUNCTUATION: &[u8] = b"!#$%&'*+-.^_`|~";

/// The request transfer coding field.
pub(crate) const HTTP_HEADER_TRANSFER_ENCODING: &str = "transfer-encoding";

/// The field announcing names that might appear as trailers.
pub(super) const HTTP_HEADER_TRAILER: &str = "trailer";

/// The whitespace allowed around an entity-tag list member.
pub(crate) const ENTITY_TAG_OPTIONAL_WHITESPACE: &[u8] = b" \t";

/// The only whitespace allowed around a field value.
pub(crate) const HTTP_HEADER_OPTIONAL_WHITESPACE: [char; 2] = [' ', '\t'];

/// Maximum header name length lowercased on the stack without heap allocation.
pub(crate) const STACK_HEADER_KEY_LIMIT: usize = B_64;

/// Compile-time lookup table of valid RFC 9110 HTTP header token bytes.
pub(crate) const HTTP_HEADER_TOKEN_TABLE: [bool; 256] = {
    let mut table: [bool; 256] = [false; 256];
    let mut idx: usize = 0;
    while idx < 256 {
        let byte: u8 = idx as u8;
        let is_alnum: bool = byte.is_ascii_alphanumeric();
        let is_punct: bool = matches!(
            byte,
            b'!' | b'#'
                | b'$'
                | b'%'
                | b'&'
                | b'\''
                | b'*'
                | b'+'
                | b'-'
                | b'.'
                | b'^'
                | b'_'
                | b'`'
                | b'|'
                | b'~'
        );
        table[idx] = is_alnum || is_punct;
        idx += 1;
    }
    table
};

/// Compile-time lookup table of bytes allowed inside an HTTP header field value.
pub(crate) const HTTP_HEADER_VALUE_VALID_TABLE: [bool; 256] = {
    let mut table: [bool; 256] = [false; 256];
    let mut idx: usize = 0;
    while idx < 256 {
        let byte: u8 = idx as u8;
        let is_invalid: bool = (byte < HTTP_HEADER_CONTROL_LIMIT && byte != HTTP_HEADER_TAB)
            || byte == HTTP_HEADER_DELETE;
        table[idx] = !is_invalid;
        idx += 1;
    }
    table
};
