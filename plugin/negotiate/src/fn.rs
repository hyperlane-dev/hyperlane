use super::*;

/// Checks whether a status carries no body a content coding could apply to.
///
/// # Arguments
///
/// - `ResponseStatusCode` - The status code of the response.
///
/// # Returns
///
/// - `bool` - Whether the status is bodyless.
pub(crate) fn has_no_body(status_code: ResponseStatusCode) -> bool {
    matches!(status_code, 100..=199)
        || status_code == HttpStatus::NoContent.code()
        || status_code == HttpStatus::ResetContent.code()
        || status_code == HttpStatus::NotModified.code()
}

/// Checks whether the response declares an event stream.
///
/// An event stream is delivered incrementally, so a coded stream would hold the
/// bytes back until the whole stream was compressed, which defeats the point.
///
/// # Arguments
///
/// - `&Response` - The response to read the content type of.
///
/// # Returns
///
/// - `bool` - Whether the response is an event stream.
pub(crate) fn is_event_stream(response: &Response) -> bool {
    response
        .get_headers()
        .get(CONTENT_TYPE)
        .is_some_and(|values: &ResponseHeadersValue| {
            values
                .iter()
                .any(|value: &String| value.eq_ignore_ascii_case(TEXT_EVENT_STREAM))
        })
}

/// Declares that the response varies on the accepted content coding.
///
/// A response that is coded for one client must not be reused for another that
/// asked for a different coding, so the variance is declared on the response.
/// A variance the response already declares is extended rather than replaced,
/// because the field is a list and every entry of it is load bearing.
///
/// # Arguments
///
/// - `&mut Response` - The response to declare the variance on.
pub(crate) fn vary_accept_encoding(response: &mut Response) {
    let declared: bool =
        response
            .get_headers()
            .get(VARY)
            .is_some_and(|values: &ResponseHeadersValue| {
                values.iter().any(|value: &String| {
                    value
                        .split(COMMA)
                        .any(|token: &str| token.trim().eq_ignore_ascii_case(VARY_ACCEPT_ENCODING))
                })
            });
    if declared {
        return;
    }
    if response.has_header(VARY) {
        response.add_header(VARY, VARY_ACCEPT_ENCODING);
    } else {
        response.set_header(VARY, VARY_ACCEPT_ENCODING);
    }
}
