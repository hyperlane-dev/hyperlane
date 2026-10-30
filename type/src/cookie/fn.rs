use super::*;

/// Reports whether a cookie name may appear in a `Set-Cookie` header.
///
/// RFC 6265 Section 4.1.1 takes the name from the token production of
/// RFC 2616 Section 2.2, which admits the letters, the digits and a fixed
/// set of punctuation, and refuses the controls, the whitespace and the
/// separators. A name that carries a separator would end the name early
/// and turn the rest of it into an attribute.
///
/// # Arguments
///
/// - `&str` - The cookie name.
///
/// # Returns
///
/// - `bool` - `true` when the name is a token.
pub fn is_cookie_name_valid(name: &str) -> bool {
    !name.is_empty() && name.bytes().all(is_token_byte)
}

/// Reports whether a cookie value may appear in a `Set-Cookie` header.
///
/// The value sits between the name and the end of the cookie pair, so it may
/// not carry a byte that would end the pair or the header line: a control
/// byte would end the header line and turn the rest of the cookie into a
/// second header, a semicolon would start another attribute, a comma would
/// make the header ambiguous when several cookies are folded into one field
/// as RFC 6265 Section 3 warns, and the double quote and the backslash
/// would make the value ambiguous against the quoted form of the grammar in
/// RFC 6265 Section 4.1.1.
///
/// # Arguments
///
/// - `&str` - The cookie value.
///
/// # Returns
///
/// - `bool` - `true` when the value may appear in a header.
pub fn is_cookie_value_valid(value: &str) -> bool {
    value
        .bytes()
        .all(|byte: u8| !is_control_byte(byte) && !is_value_terminator(byte))
}

/// Reports whether a cookie attribute value may appear in a `Set-Cookie`
/// header.
///
/// An attribute carries a date, a domain, a path or a policy, and a date
/// legitimately carries a comma and spaces, so the only bytes refused are
/// the ones that would end the header line or start another attribute.
///
/// # Arguments
///
/// - `&str` - The attribute value.
///
/// # Returns
///
/// - `bool` - `true` when the attribute value may appear in a header.
pub fn is_cookie_attribute_valid(attribute: &str) -> bool {
    attribute
        .bytes()
        .all(|byte: u8| !is_control_byte(byte) && byte != SEMICOLON_U8)
}

/// Reports whether a byte is a control byte.
///
/// # Arguments
///
/// - `u8` - The byte to check.
///
/// # Returns
///
/// - `bool` - `true` when the byte is a control byte.
pub fn is_control_byte(byte: u8) -> bool {
    (COOKIE_CTL_FIRST..=COOKIE_CTL_LAST).contains(&byte) || byte == COOKIE_DELETE
}

/// Reports whether a byte may appear in a token.
///
/// # Arguments
///
/// - `u8` - The byte to check.
///
/// # Returns
///
/// - `bool` - `true` when the byte may appear in a token.
pub fn is_token_byte(byte: u8) -> bool {
    byte.is_ascii_alphanumeric() || COOKIE_TOKEN_PUNCTUATION.contains(byte as char)
}

/// Reports whether a byte would end the cookie pair or make it ambiguous.
///
/// # Arguments
///
/// - `u8` - The byte to check.
///
/// # Returns
///
/// - `bool` - `true` when the byte may not appear in a cookie value.
pub fn is_value_terminator(byte: u8) -> bool {
    byte == COOKIE_VALUE_FORBIDDEN_SEMICOLON
        || byte == COOKIE_VALUE_FORBIDDEN_COMMA
        || byte == COOKIE_VALUE_FORBIDDEN_QUOTE
        || byte == COOKIE_VALUE_FORBIDDEN_BACKSLASH
}
