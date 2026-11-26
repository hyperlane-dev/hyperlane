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

/// Checks whether the response declares a header matching `target` case-insensitively.
///
/// # Arguments
///
/// - `&Response` - The response to inspect.
/// - `&str` - The header name to search for.
///
/// # Returns
///
/// - `bool` - Whether any response header matches `target` ASCII case-insensitively.
pub(crate) fn has_response_header_ci(response: &Response, target: &str) -> bool {
    response
        .get_headers()
        .keys()
        .any(|key: &String| key.eq_ignore_ascii_case(target))
}

/// Checks whether the response forbids payload transformations via `Cache-Control: no-transform`.
///
/// # Arguments
///
/// - `&Response` - The response to inspect.
///
/// # Returns
///
/// - `bool` - Whether `no-transform` is present in any `Cache-Control` header.
pub(crate) fn has_no_transform(response: &Response) -> bool {
    response
        .get_headers()
        .iter()
        .any(|(key, values): (&String, &ResponseHeadersValue)| {
            key.eq_ignore_ascii_case(CACHE_CONTROL)
                && values.iter().any(|value: &String| {
                    value.split(COMMA).any(|directive: &str| {
                        directive
                            .trim()
                            .eq_ignore_ascii_case(CACHE_CONTROL_NO_TRANSFORM)
                    })
                })
        })
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
        .iter()
        .any(|(key, values): (&String, &ResponseHeadersValue)| {
            key.eq_ignore_ascii_case(CONTENT_TYPE)
                && values.iter().any(|value: &String| {
                    value
                        .split(MEDIA_TYPE_PARAM_DELIMITER)
                        .next()
                        .is_some_and(|media: &str| {
                            media.trim().eq_ignore_ascii_case(TEXT_EVENT_STREAM)
                        })
                })
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
    let mut has_vary: bool = false;
    let mut declared: bool = false;
    for (key, values) in response.get_headers() {
        if !key.eq_ignore_ascii_case(VARY) {
            continue;
        }
        has_vary = true;
        if values.iter().any(|value: &String| {
            value.split(COMMA).any(|token: &str| {
                let trimmed: &str = token.trim();
                trimmed.eq_ignore_ascii_case(VARY_ACCEPT_ENCODING) || trimmed == VARY_WILDCARD
            })
        }) {
            declared = true;
            break;
        }
    }
    if declared {
        return;
    }
    if has_vary {
        response.add_header(VARY, VARY_ACCEPT_ENCODING);
    } else {
        response.set_header(VARY, VARY_ACCEPT_ENCODING);
    }
}
