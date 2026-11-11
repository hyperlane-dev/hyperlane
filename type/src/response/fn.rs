use super::*;

/// The content type a response announces when the application chose none.
///
/// It is built once and then shared, so a response that does not choose a
/// content type does not build the value again for every request.
///
/// # Returns
///
/// - `&'static str` - The default content type.
pub(crate) fn default_content_type() -> &'static str {
    static DEFAULT_CONTENT_TYPE: OnceLock<String> = OnceLock::new();
    DEFAULT_CONTENT_TYPE.get_or_init(|| {
        let mut content_type: String =
            String::with_capacity(TEXT_HTML.len() + SEMICOLON_SPACE.len() + CHARSET_UTF_8.len());
        content_type.push_str(TEXT_HTML);
        content_type.push_str(SEMICOLON_SPACE);
        content_type.push_str(CHARSET_UTF_8);
        content_type
    })
}

/// Renders a body length into the digits of a caller-owned buffer.
///
/// The length is announced on every response, so it is rendered without
/// allocating: the digits are written into the buffer the caller owns and the
/// caller copies them into the header.
///
/// # Arguments
///
/// - `usize` - The length to render.
/// - `&mut [u8]` - The buffer the digits are written into.
///
/// # Returns
///
/// - `&str` - The rendered digits.
pub(crate) fn render_length(length: usize, buffer: &mut [u8]) -> &str {
    let mut cursor: usize = buffer.len();
    let mut remaining: usize = length;
    loop {
        cursor -= 1;
        buffer[cursor] = b'0' + u8::try_from(remaining % B_10).unwrap_or_default();
        remaining /= B_10;
        if remaining == 0 {
            break;
        }
    }
    from_utf8(&buffer[cursor..]).unwrap_or_default()
}
