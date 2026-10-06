use super::*;

#[test]
fn http_methods_are_uppercase_tokens() {
    assert_eq!(GET, "GET");
    assert_eq!(POST, "POST");
    assert_eq!(PUT, "PUT");
    assert_eq!(DELETE, "DELETE");
    assert_eq!(HEAD, "HEAD");
    assert_eq!(OPTIONS, "OPTIONS");
    assert_eq!(PATCH, "PATCH");
}

#[test]
fn header_keys_are_lowercase_tokens() {
    assert_eq!(CONTENT_TYPE, "content-type");
    assert_eq!(CONTENT_LENGTH, "content-length");
    assert_eq!(CONTENT_ENCODING, "content-encoding");
    assert_eq!(TRANSFER_ENCODING, "transfer-encoding");
    assert_eq!(ACCEPT_ENCODING, "accept-encoding");
}

#[test]
fn header_keys_contain_no_whitespace() {
    let keys: Vec<&str> = vec![
        CONTENT_TYPE,
        CONTENT_LENGTH,
        CONTENT_ENCODING,
        TRANSFER_ENCODING,
        ACCEPT_ENCODING,
        SERVER,
        HOST,
    ];
    for key in keys {
        assert!(!key.contains(' '), "header key has a space: {key}");
        assert!(!key.contains('\t'), "header key has a tab: {key}");
        assert_eq!(key, key.to_lowercase());
    }
}

#[test]
fn content_type_values_are_lowercase_media_types() {
    assert_eq!(APPLICATION_JSON, "application/json");
    assert_eq!(TEXT_HTML, "text/html");
    assert_eq!(TEXT_PLAIN, "text/plain");
}

#[test]
fn media_types_are_lowercase_and_slash_separated() {
    let types: Vec<&str> = vec![APPLICATION_JSON, TEXT_HTML, TEXT_PLAIN];
    for media in types {
        assert_eq!(media, media.to_lowercase());
        assert!(media.contains('/'), "media type lacks a slash: {media}");
        let parts: Vec<&str> = media.split('/').collect();
        assert_eq!(parts.len(), 2, "media type malformed: {media}");
    }
}

#[test]
fn byte_views_match_their_string_constants() {
    assert_eq!(SPACE_BYTES, SPACE.as_bytes());
    assert_eq!(TAB_BYTES, TAB.as_bytes());
    assert_eq!(BR_BYTES, BR.as_bytes());
    assert_eq!(DOUBLE_BR_BYTES, DOUBLE_BR.as_bytes());
    assert_eq!(COLON_BYTES, COLON.as_bytes());
    assert_eq!(COLON_SPACE_BYTES, COLON_SPACE.as_bytes());
    assert_eq!(EQUAL_BYTES, EQUAL.as_bytes());
    assert_eq!(AND_BYTES, AND.as_bytes());
    assert_eq!(COMMA_BYTES, COMMA.as_bytes());
    assert_eq!(HASH_BYTES, HASH.as_bytes());
    assert_eq!(SEMICOLON_BYTES, SEMICOLON.as_bytes());
    assert_eq!(QUERY_BYTES, QUERY.as_bytes());
    assert_eq!(POINT_BYTES, POINT.as_bytes());
    assert_eq!(ROOT_PATH_BYTES, ROOT_PATH.as_bytes());
    assert_eq!(HTTP_BR_BYTES, HTTP_BR.as_bytes());
    assert_eq!(HTTP_DOUBLE_BR_BYTES, HTTP_DOUBLE_BR.as_bytes());
    assert_eq!(HYPERLANE_BYTES, HYPERLANE.as_bytes());
    assert_eq!(LOCALHOST_BYTES, LOCALHOST.as_bytes());
    assert_eq!(DEFAULT_HTTP_PATH_BYTES, DEFAULT_HTTP_PATH.as_bytes());
}

#[test]
fn single_byte_views_take_the_first_byte() {
    assert_eq!(SPACE_U8, SPACE_BYTES[0]);
    assert_eq!(TAB_U8, TAB_BYTES[0]);
    assert_eq!(COLON_U8, COLON_BYTES[0]);
    assert_eq!(EQUAL_U8, EQUAL_BYTES[0]);
    assert_eq!(AND_U8, AND_BYTES[0]);
    assert_eq!(COMMA_U8, COMMA_BYTES[0]);
    assert_eq!(HASH_U8, HASH_BYTES[0]);
    assert_eq!(SEMICOLON_U8, SEMICOLON_BYTES[0]);
    assert_eq!(QUERY_U8, QUERY_BYTES[0]);
    assert_eq!(POINT_U8, POINT_BYTES[0]);
    assert_eq!(ZERO_STR_U8, ZERO_STR_BYTES[0]);
}

#[test]
fn delimiter_constants_are_single_ascii_characters() {
    let pairs: Vec<(&str, &str)> = vec![
        (SPACE, " "),
        (TAB, "\t"),
        (COLON, ":"),
        (EQUAL, "="),
        (AND, "&"),
        (COMMA, ","),
        (HASH, "#"),
        (SEMICOLON, ";"),
        (QUERY, "?"),
        (POINT, "."),
    ];
    for (actual, expected) in pairs {
        assert_eq!(actual, expected);
        assert_eq!(actual.len(), 1);
    }
}

#[test]
fn multi_char_delimiters_have_expected_content() {
    assert_eq!(BR, "\n");
    assert_eq!(DOUBLE_BR, "\n\n");
    assert_eq!(HTTP_BR, "\r\n");
    assert_eq!(HTTP_DOUBLE_BR, "\r\n\r\n");
    assert_eq!(COLON_SPACE, ": ");
    assert_eq!(SEMICOLON_SPACE, "; ");
}

#[test]
fn empty_and_zero_constants_agree() {
    assert_eq!(EMPTY_STR, "");
    assert!(EMPTY_STR_BYTES.is_empty());
    assert_eq!(ZERO_STR, "0");
    assert_eq!(ZERO_STR_BYTES, ZERO_STR.as_bytes());
}

#[test]
fn http_version_tokens_are_prefixed() {
    assert_eq!(HTTP_VERSION_1_0, "HTTP/1.0");
    assert_eq!(HTTP_VERSION_1_1, "HTTP/1.1");
    assert_eq!(HTTP_VERSION_2, "HTTP/2");
    for version in [HTTP_VERSION_1_0, HTTP_VERSION_1_1, HTTP_VERSION_2] {
        assert!(version.starts_with("HTTP/"));
    }
}

#[test]
fn default_path_and_root_path_agree() {
    assert_eq!(DEFAULT_HTTP_PATH, "/");
    assert_eq!(ROOT_PATH, "/");
    assert_eq!(DEFAULT_HTTP_PATH_BYTES, ROOT_PATH_BYTES);
}

#[test]
fn security_level_limits_are_ordered() {
    let low_body: usize = DEFAULT_LOW_SECURITY_MAX_BODY_SIZE;
    let high_body: usize = DEFAULT_HIGH_SECURITY_MAX_BODY_SIZE;
    let low_path: usize = DEFAULT_LOW_SECURITY_MAX_PATH_SIZE;
    let high_path: usize = DEFAULT_HIGH_SECURITY_MAX_PATH_SIZE;
    let low_headers: usize = DEFAULT_LOW_SECURITY_MAX_HEADER_COUNT;
    let high_headers: usize = DEFAULT_HIGH_SECURITY_MAX_HEADER_COUNT;
    assert!(low_body >= high_body);
    assert!(low_path >= high_path);
    assert!(low_headers >= high_headers);
    assert!(high_body > 0);
    assert!(high_path > 0);
    assert!(high_headers > 0);
}

#[test]
fn buffer_defaults_are_non_zero() {
    let buffer: usize = DEFAULT_BUFFER_SIZE;
    let request_line: usize = REQUEST_LINE_BUFFER_CAPACITY;
    let header_line: usize = HEADER_LINE_BUFFER_CAPACITY;
    let pooled_count: usize = MAX_POOLED_READ_BUFFERS;
    let pooled_size: usize = MAX_POOLED_READ_BUFFER_SIZE;
    assert!(buffer > 0);
    assert!(request_line > 0);
    assert!(header_line > 0);
    assert!(pooled_count > 0);
    assert!(pooled_size > 0);
}

#[test]
fn redirect_limit_is_bounded() {
    let limit: usize = DEFAULT_MAX_REDIRECT_TIMES;
    assert!(limit > 0);
    assert!(limit <= 10);
}

#[test]
fn hyperlane_name_variants_are_consistent() {
    assert_eq!(HYPERLANE, "hyperlane");
    assert_eq!(HYPERLANE_PASCAL_CASE, "Hyperlane");
    assert_eq!(HYPERLANE_UPPERCASE, "HYPERLANE");
    assert_eq!(
        HYPERLANE_PASCAL_CASE_BYTES,
        HYPERLANE_PASCAL_CASE.as_bytes()
    );
    assert_eq!(HYPERLANE_UPPERCASE_BYTES, HYPERLANE_UPPERCASE.as_bytes());
    assert_eq!(HYPERLANE_BYTES, HYPERLANE.as_bytes());
}

#[test]
fn log_level_names_have_expected_values() {
    assert_eq!(ERROR, "error");
    assert_eq!(WARNING, "warning");
    assert_eq!(INFO, "info");
    assert_eq!(DEBUG, "debug");
    assert_eq!(TRACE, "TRACE");
    assert_eq!(SUCCESS, "success");
    assert_eq!(FAIL, "fail");
}

#[test]
fn log_level_byte_views_match() {
    assert_eq!(WARNING_BYTES, WARNING.as_bytes());
    assert_eq!(SUCCESS_BYTES, SUCCESS.as_bytes());
    assert_eq!(FAIL_BYTES, FAIL.as_bytes());
    assert_eq!(ERROR_BYTES, ERROR.as_bytes());
    assert_eq!(INFO_BYTES, INFO.as_bytes());
    assert_eq!(DEBUG_BYTES, DEBUG.as_bytes());
    assert_eq!(PLAIN_BYTES, PLAIN.as_bytes());
    assert_eq!(BINARY_BYTES, BINARY.as_bytes());
}

#[test]
fn bracket_pairs_are_balanced() {
    assert_eq!(LEFT_BRACKET, "{");
    assert_eq!(RIGHT_BRACKET, "}");
    assert_eq!(LEFT_SQUARE_BRACKET, "[");
    assert_eq!(RIGHT_SQUARE_BRACKET, "]");
    assert_eq!(LEFT_PAREN, "(");
    assert_eq!(RIGHT_PAREN, ")");
    assert_eq!(LEFT_BRACKET_BYTES, LEFT_BRACKET.as_bytes());
    assert_eq!(RIGHT_BRACKET_BYTES, RIGHT_BRACKET.as_bytes());
    assert_eq!(LEFT_PAREN_BYTES, LEFT_PAREN.as_bytes());
    assert_eq!(RIGHT_PAREN_BYTES, RIGHT_PAREN.as_bytes());
    assert_eq!(LEFT_SQUARE_BRACKET_BYTES, LEFT_SQUARE_BRACKET.as_bytes());
    assert_eq!(RIGHT_SQUARE_BRACKET_BYTES, RIGHT_SQUARE_BRACKET.as_bytes());
}

#[test]
fn default_ports_are_standard() {
    assert_eq!(DEFAULT_HTTP_PORT, 80);
    assert_eq!(DEFAULT_HTTPS_PORT, 443);
    assert_eq!(DEFAULT_HTTP_PORT_STR, "80");
    assert_eq!(DEFAULT_HTTPS_PORT_STR, "443");
}

#[test]
fn loopback_addresses_are_local() {
    assert_eq!(LOCALHOST, "localhost");
    assert_eq!(LOOPBACK, "127.0.0.1");
    assert_eq!(DEFAULT_HOST_IPV4_ADDR.to_string(), "0.0.0.0");
    assert_eq!(DEFAULT_IPV4_ADDR.to_string(), "0.0.0.0");
}
