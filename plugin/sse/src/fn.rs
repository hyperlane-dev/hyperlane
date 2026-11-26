/// Sanitizes a single-line SSE header field value by stripping `\r`, `\n`, and NUL bytes.
///
/// # Arguments
///
/// - `&str` - Raw field text.
///
/// # Returns
///
/// - `String` - Safe single-line value that cannot inject additional SSE lines or frames.
pub(crate) fn sanitize_sse_line(raw: &str) -> String {
    let mut out: String = String::with_capacity(raw.len());
    for ch in raw.chars() {
        if ch != '\r' && ch != '\n' && ch != '\0' {
            out.push(ch);
        }
    }
    out
}

/// Parses a `Last-Event-ID` string into a numeric sequence id when valid.
///
/// # Arguments
///
/// - `Option<&str>` - Raw `Last-Event-ID` header value.
///
/// # Returns
///
/// - `Option<u64>` - Parsed sequence id, or `None` when absent or non-numeric.
pub(crate) fn parse_last_event_id(raw: Option<&str>) -> Option<u64> {
    raw.and_then(|text: &str| text.trim().parse::<u64>().ok())
}
