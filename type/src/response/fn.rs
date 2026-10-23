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
