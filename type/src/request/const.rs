use super::*;

/// Wire bytes for the GET request method.
pub(crate) const WIRE_METHOD_GET: &[u8] = GET.as_bytes();

/// Wire bytes for the POST request method.
pub(crate) const WIRE_METHOD_POST: &[u8] = POST.as_bytes();

/// Wire bytes for the PUT request method.
pub(crate) const WIRE_METHOD_PUT: &[u8] = PUT.as_bytes();

/// Wire bytes for the DELETE request method.
pub(crate) const WIRE_METHOD_DELETE: &[u8] = DELETE.as_bytes();

/// Wire bytes for the HEAD request method.
pub(crate) const WIRE_METHOD_HEAD: &[u8] = HEAD.as_bytes();

/// Wire bytes for the OPTIONS request method.
pub(crate) const WIRE_METHOD_OPTIONS: &[u8] = OPTIONS.as_bytes();

/// Wire bytes for the PATCH request method.
pub(crate) const WIRE_METHOD_PATCH: &[u8] = PATCH.as_bytes();

/// Wire bytes for the CONNECT request method.
pub(crate) const WIRE_METHOD_CONNECT: &[u8] = CONNECT.as_bytes();

/// Wire bytes for the TRACE request method.
pub(crate) const WIRE_METHOD_TRACE: &[u8] = TRACE.as_bytes();

/// Wire bytes for the HTTP version 1.1.
pub(crate) const WIRE_VERSION_1_1: &[u8] = HTTP_VERSION_1_1.as_bytes();

/// Wire bytes for the HTTP version 1.0.
pub(crate) const WIRE_VERSION_1_0: &[u8] = HTTP_VERSION_1_0.as_bytes();

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

/// Additional byte budget beyond `max_path_size` for the HTTP request line framing.
pub(crate) const HTTP_REQUEST_LINE_OVERHEAD: usize = 1024;

/// Additional byte budget beyond key and value limits for one HTTP header line.
pub(crate) const HTTP_HEADER_LINE_OVERHEAD: usize = 1024;

/// Maximum chunk size pre-allocated per incremental body read.
pub(crate) const HTTP_BODY_READ_CHUNK_SIZE: usize = 64 * 1024;

/// Maximum number of query parameters accepted per request unless limits are disabled.
pub(crate) const HTTP_MAX_QUERY_PARAM_COUNT: usize = 1024;

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
