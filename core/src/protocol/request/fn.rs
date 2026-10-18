use super::*;

/// Builds a framework request from a protocol request head and body.
///
/// HTTP/2 and HTTP/3 both deliver a decoded header map plus a body stream, and
/// both are converted here into the same request the HTTP/1.1 parser produces,
/// so routing, middleware and hooks above this point cannot tell the protocols
/// apart. The version stamped on the request is the real one, not the default.
///
/// Pseudo headers and hop by hop headers are dropped rather than trusted, and
/// the security limits from the request config are enforced against the decoded
/// values, so an oversized head is rejected exactly like an oversized HTTP/1.1
/// head. A repeated header keeps every value instead of collapsing to the last.
///
/// # Arguments
///
/// - `&RequestConfig` - The parsing limits to enforce.
/// - `HttpVersion` - The real protocol version to stamp on the request.
/// - `http::Method` - The request method decoded by the protocol library.
/// - `&str` - The `value` argument.
/// - `&http::Uri` - The request URI, which carries the authority.
/// - `&ProtocolHeaders` - The protocol request headers.
/// - `Vec<u8>` - The already collected request body.
///
/// # Returns
///
/// - `Result<Request, RequestError>` - The converted request, or the first limit violation.
pub fn build_protocol_request(
    config: &RequestConfig,
    version: HttpVersion,
    method: http::Method,
    target: &str,
    target_uri: &http::Uri,
    headers: &ProtocolHeaders,
    body: Vec<u8>,
) -> Result<Request, RequestError> {
    let max_path_size: usize = config.get_max_path_size();
    if target.len() > max_path_size && max_path_size != DEFAULT_LOW_SECURITY_MAX_PATH_SIZE {
        return Err(RequestError::PathTooLong(HttpStatus::URITooLong));
    }
    let max_body_size: usize = config.get_max_body_size();
    if body.len() > max_body_size && max_body_size != DEFAULT_LOW_SECURITY_MAX_BODY_SIZE {
        return Err(RequestError::ContentLengthTooLarge(
            HttpStatus::PayloadTooLarge,
        ));
    }
    let mut request: Request = Request::default();
    let query_index: Option<usize> = target.find(QUESTION_MARK);
    let hash_index: Option<usize> = target.find(HASH);
    fill_request_querys(
        split_target_query(target, query_index, hash_index),
        request.get_mut_querys(),
    );
    request
        .get_mut_path()
        .push_str(split_target_path(target, query_index, hash_index));
    request.set_method(protocol_method(method));
    request.set_version(version);
    fill_request_headers(headers, config, &mut request)?;
    request.set_host(resolve_request_host(headers, target_uri));
    request.get_mut_body().clear();
    request.get_mut_body().extend_from_slice(&body);
    Ok(request)
}

/// Resolves the authority of a protocol request.
///
/// HTTP/2 and HTTP/3 carry the authority in a pseudo header, while HTTP/1.1
/// carries it in `host`, so both are consulted before falling back to a value
/// that keeps the request usable rather than empty.
///
/// # Arguments
///
/// - `&ProtocolHeaders` - The protocol request headers.
/// - `&http::Uri` - The `uri` argument.
///
/// # Returns
///
/// - `String` - The resolved request host.
pub fn resolve_request_host(headers: &ProtocolHeaders, uri: &http::Uri) -> String {
    if let Some(authority) = uri.authority() {
        return authority.as_str().to_owned();
    }
    for name in [HOST, COLON_AUTHORITY] {
        if let Some(value) = headers.get(name)
            && let Ok(value) = value.to_str()
        {
            return value.to_owned();
        }
    }
    EMPTY_STR.to_owned()
}

/// Extracts the query string from a protocol request target.
///
/// # Arguments
///
/// - `&str` - The request target.
/// - `Option<usize>` - The index of the `?` separator, if present.
///
/// # Returns
///
/// - `&str` - The query portion, or an empty string when the target carries none.
pub fn split_target_query(
    target: &str,
    query_index: Option<usize>,
    hash_index: Option<usize>,
) -> &str {
    query_index.map_or(EMPTY_STR, |query_index: usize| {
        let rest: &str = &target[query_index + 1..];
        match hash_index {
            None => rest,
            Some(hash_index) if hash_index <= query_index => rest,
            Some(hash_index) => &rest[..hash_index - query_index - 1],
        }
    })
}

/// Extracts the path portion from a protocol request target.
///
/// # Arguments
///
/// - `&str` - The request target.
/// - `Option<usize>` - The index of the `?` separator, if present.
///
/// # Returns
///
/// - `&str` - The path portion without query or fragment.
pub fn split_target_path(
    target: &str,
    query_index: Option<usize>,
    hash_index: Option<usize>,
) -> &str {
    match query_index.or(hash_index) {
        Some(separator_index) => &target[..separator_index],
        None => target,
    }
}

/// Parses a query string into the request's query map.
///
/// A key that repeats keeps every value, because a single valued map would
/// discard all but the last occurrence without reporting anything.
///
/// # Arguments
///
/// - `&str` - The query string.
/// - `&mut RequestQuerys` - The map to fill.
///
pub fn fill_request_querys(query: &str, querys: &mut RequestQuerys) {
    if query.is_empty() {
        return;
    }
    for pair in query.split(AND) {
        if let Some((key, value)) = pair.split_once(EQUAL) {
            if !key.is_empty() {
                querys
                    .entry(key.to_string())
                    .or_default()
                    .push_back(value.to_string());
            }
        } else if !pair.is_empty() {
            querys.entry(pair.to_string()).or_default();
        }
    }
}

/// Decodes a protocol method into the framework method enum.
///
/// # Arguments
///
/// - `http::Method` - The method decoded by the protocol library.
///
/// # Returns
///
/// - `RequestMethod` - The framework method.
pub fn protocol_method(method: http::Method) -> RequestMethod {
    method
        .as_str()
        .parse::<RequestMethod>()
        .unwrap_or(Method::Unknown(method.as_str().to_owned()))
}

/// Copies protocol headers into the request, enforcing the head limits.
///
/// # Arguments
///
/// - `&ProtocolHeaders` - The protocol request headers.
/// - `&RequestConfig` - The parsing limits to enforce.
/// - `&mut Request` - The request to fill.
///
/// # Returns
///
/// - `Result<(), RequestError>` - `Ok(())` when every header was accepted, or
pub fn fill_request_headers(
    headers: &ProtocolHeaders,
    config: &RequestConfig,
    request: &mut Request,
) -> Result<(), RequestError> {
    let mut header_count: usize = 0;
    for (name, value) in headers.iter() {
        let key: &str = name.as_str();
        if key.starts_with(PSEUDO_HEADER_PREFIX) || is_hop_by_hop(key) {
            continue;
        }
        let Ok(value) = value.to_str() else {
            continue;
        };
        header_count += 1;
        if header_count > config.get_max_header_count()
            && config.get_max_header_count() != DEFAULT_LOW_SECURITY_MAX_HEADER_COUNT
        {
            return Err(RequestError::TooManyHeaders(
                HttpStatus::RequestHeaderFieldsTooLarge,
            ));
        }
        if key.len() > config.get_max_header_key_size()
            && config.get_max_header_key_size() != DEFAULT_LOW_SECURITY_MAX_HEADER_KEY_SIZE
        {
            return Err(RequestError::HeaderKeyTooLong(HttpStatus::BadRequest));
        }
        if value.len() > config.get_max_header_value_size()
            && config.get_max_header_value_size() != DEFAULT_LOW_SECURITY_MAX_HEADER_VALUE_SIZE
        {
            return Err(RequestError::HeaderValueTooLong(HttpStatus::BadRequest));
        }
        request
            .get_mut_headers()
            .entry(key.to_owned())
            .or_default()
            .push_back(value.to_owned());
    }
    Ok(())
}

/// Reports whether a header is hop by hop for the HTTP/2 and HTTP/3 framing.
///
/// # Arguments
///
/// - `&str` - The lowercased header name.
///
/// # Returns
///
/// - `bool` - `true` when the header must not cross the protocol boundary.
pub fn is_hop_by_hop(name: &str) -> bool {
    matches!(
        name,
        CONNECTION
            | KEEP_ALIVE
            | PROXY_AUTHENTICATE
            | PROXY_AUTHORIZATION
            | TE
            | TRAILER_HEADER
            | TRANSFER_ENCODING
            | UPGRADE
    )
}

/// Reads the body length a protocol request declares.
///
/// A request is not required to declare a length, so an absent header is
/// reported as none and the collected body is accepted on its own terms. A
/// declared length larger than the configured cap is refused before the body is
/// read, and an unparseable length is refused rather than treated as zero, so a
/// request that claims a body it does not send cannot reach a handler.
///
/// # Arguments
///
/// - `&ProtocolHeaders` - The protocol request headers.
/// - `usize` - The maximum number of body bytes to accept.
///
/// # Returns
///
/// - `Result<Option<usize>, RequestError>` - The declared length when one is
pub fn declared_content_length(
    headers: &ProtocolHeaders,
    max_body_size: usize,
) -> Result<Option<usize>, RequestError> {
    let Some(value) = headers.get(CONTENT_LENGTH) else {
        return Ok(None);
    };
    let Ok(value) = value.to_str() else {
        return Err(RequestError::InvalidContentLength(HttpStatus::BadRequest));
    };
    let declared: usize = parse_ascii_length(value)?;
    if declared > max_body_size && max_body_size != DEFAULT_LOW_SECURITY_MAX_BODY_SIZE {
        return Err(RequestError::ContentLengthTooLarge(
            HttpStatus::PayloadTooLarge,
        ));
    }
    Ok(Some(declared))
}

/// Parses a declared body length from one header field value.
///
/// The length is a run of ASCII digits and nothing else. `str::parse` accepts
/// a leading `+`, so a header announcing `+1` would otherwise be read as `1` —
/// a value the sender could not have meant, and one that makes the declared
/// length disagree with the bytes that follow. An empty value is refused for
/// the same reason: it is not a length at all, and treating it as zero would
/// turn a malformed head into a truncated-but-successful request.
///
/// # Arguments
///
/// - `&str` - The raw header field value.
///
/// # Returns
///
/// - `Result<usize, RequestError>` - The declared length, or the refusal.
pub(crate) fn parse_ascii_length(value: &str) -> Result<usize, RequestError> {
    let digits: &str = value.trim();
    if digits.is_empty() || !digits.bytes().all(|byte: u8| byte.is_ascii_digit()) {
        return Err(RequestError::InvalidContentLength(HttpStatus::BadRequest));
    }
    digits
        .parse::<usize>()
        .map_err(|_: ParseIntError| RequestError::InvalidContentLength(HttpStatus::BadRequest))
}

/// Reports whether a status code forbids a response body.
///
/// A `HEAD` response carries the length the matching `GET` would have but no
/// bytes, and the no-content statuses carry none either.
///
/// # Arguments
///
/// - `usize` - The response status code.
/// - `bool` - Whether the request method was `HEAD`.
///
/// # Returns
///
/// - `bool` - `true` when no body bytes may be emitted.
pub fn is_bodyless_status(status_code: usize, is_head: bool) -> bool {
    is_head
        || status_code < 200
        || status_code == HttpStatus::NoContent.code()
        || status_code == HttpStatus::NotModified.code()
}

/// Collects a protocol request body into a bounded buffer.
///
/// The framework request carries its body as bytes, so the stream is drained
/// into a single buffer. The cap is enforced as the body grows rather than
/// after it is complete, so an oversized upload is rejected while it is being
/// read instead of after it has already been buffered.
///
/// # Arguments
///
/// - `usize` - The maximum number of body bytes to accept.
/// - `&mut Vec<u8>` - The buffer to append into.
/// - `Bytes` - The `value` argument.
///
/// # Returns
///
/// - `Result<(), RequestError>` - `Ok(())` while the body stays within the cap,
pub fn push_bounded_body(
    limit: usize,
    body: &mut Vec<u8>,
    chunk: Bytes,
) -> Result<(), RequestError> {
    if body.len() + chunk.len() > limit && limit != DEFAULT_LOW_SECURITY_MAX_BODY_SIZE {
        return Err(RequestError::ContentLengthTooLarge(
            HttpStatus::PayloadTooLarge,
        ));
    }
    body.extend_from_slice(chunk.as_ref());
    Ok(())
}

/// Joins a request path and its query into the target carried on the wire.
///
/// # Arguments
///
/// - `&str` - The request path.
/// - `Option<&str>` - The request query string, when one is present.
///
/// # Returns
///
/// - `String` - The target including its query string and query separator.
pub fn request_target(path: &str, query: Option<&str>) -> String {
    match query {
        Some(query) if !query.is_empty() => {
            let mut target: String = String::with_capacity(path.len() + query.len() + 1);
            target.push_str(path);
            target.push(QUESTION_MARK);
            target.push_str(query);
            target
        }
        _ => path.to_owned(),
    }
}
