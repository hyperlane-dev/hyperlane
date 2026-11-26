use super::*;

/// Populates an existing framework request and target buffer from a protocol
/// request head and body without redundant allocations or copies.
///
/// # Arguments
///
/// - `&RequestConfig` - The parsing limits to enforce.
/// - `ProtocolRequestHead` - The decoded head and the collected body.
/// - `&mut Request` - The recycled request to populate in place.
/// - `&mut String` - The recycled buffer receiving the original wire target.
///
/// # Returns
///
/// - `Result<(), RequestError>` - `Ok(())` on success, or the first limit violation.
pub(crate) fn fill_protocol_request(
    config: &RequestConfig,
    head: ProtocolRequestHead,
    request: &mut Request,
    target: &mut String,
) -> Result<(), RequestError> {
    let ProtocolRequestHead {
        version,
        method,
        parts,
        body,
    } = head;
    let uri: &Uri = &parts.uri;
    write_request_target(uri.path(), uri.query(), target);
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
    check_request_authority(&parts.headers, uri)?;
    request.get_mut_querys().clear();
    let query_index: Option<usize> = target.find(QUESTION_MARK);
    let hash_index: Option<usize> = target.find(HASH);
    fill_request_querys(
        split_target_query(target, query_index, hash_index),
        request.get_mut_querys(),
    );
    let req_path: &mut String = request.get_mut_path();
    req_path.clear();
    req_path.push_str(split_target_path(target, query_index, hash_index));
    request.set_method(protocol_method(method));
    request.set_version(version);
    fill_request_headers(&parts.headers, config, request)?;
    write_request_host(&parts.headers, uri, request.get_mut_host());
    if !body.is_empty() {
        *request.get_mut_body() = body;
    } else {
        request.get_mut_body().clear();
    }
    Ok(())
}

/// Writes the resolved request authority into a caller-owned buffer.
///
/// # Arguments
///
/// - `&ProtocolHeaders` - The protocol request headers.
/// - `&Uri` - The request URI.
/// - `&mut String` - The destination buffer.
pub(crate) fn write_request_host(headers: &ProtocolHeaders, uri: &Uri, out: &mut String) {
    out.clear();
    if let Some(authority) = uri.authority() {
        out.push_str(authority.as_str());
        return;
    }
    for name in [HOST, COLON_AUTHORITY] {
        if let Some(value) = headers.get(name)
            && let Ok(value) = value.to_str()
        {
            out.push_str(value);
            return;
        }
    }
}

/// Writes the wire request target into a caller-owned buffer.
///
/// # Arguments
///
/// - `&str` - The request path.
/// - `Option<&str>` - The request query string, when one is present.
/// - `&mut String` - The destination buffer.
pub(crate) fn write_request_target(path: &str, query: Option<&str>, target: &mut String) {
    target.clear();
    match query {
        Some(query) => {
            target.reserve(path.len() + query.len() + 1);
            target.push_str(path);
            target.push(QUESTION_MARK);
            target.push_str(query);
        }
        None => {
            target.push_str(path);
        }
    }
}

/// Reports whether a raw `:authority` or `host` header value has valid RFC 9110 authority syntax.
///
/// # Arguments
///
/// - `&str` - The candidate authority or host header value.
///
/// # Returns
///
/// - `bool` - `true` when the value is syntactically valid and contains no userinfo or control bytes.
fn is_valid_authority_value(value: &str) -> bool {
    if value.is_empty() {
        return false;
    }
    for byte in value.bytes() {
        let forbidden: bool = match byte {
            b'@' | b'/' | b'\\' | b'?' | b'#' | b',' | b';' => true,
            _ => byte <= 0x20 || byte >= 0x7f,
        };
        if forbidden {
            return false;
        }
    }
    let (host, port): (&str, Option<&str>) = split_authority_port(value);
    if host.is_empty() {
        return false;
    }
    if host.starts_with('[') {
        let Some(inner) = host
            .strip_prefix('[')
            .and_then(|s: &str| s.strip_suffix(']'))
        else {
            return false;
        };
        if inner.is_empty()
            || !inner
                .bytes()
                .all(|b: u8| b.is_ascii_hexdigit() || b == b':' || b == b'.')
        {
            return false;
        }
    } else if host.contains('[') || host.contains(']') || host.contains(':') {
        return false;
    }
    if let Some(port) = port {
        let all_digits: bool = port.bytes().all(|b: u8| b.is_ascii_digit());
        if port.is_empty() || !all_digits {
            return false;
        }
        if port.parse::<u16>().is_err() {
            return false;
        }
    }
    true
}

/// Reports whether an HTTP/2 or HTTP/3 request header name/value is a forbidden connection header.
///
/// # Arguments
///
/// - `&str` - Lowercase header name.
/// - `&[u8]` - Raw header value bytes.
///
/// # Returns
///
/// - `bool` - `true` when RFC 9113 §8.2.2 forbids this header on incoming requests.
fn is_forbidden_h2_header(name: &str, value: &[u8]) -> bool {
    match name {
        CONNECTION | KEEP_ALIVE | PROXY_CONNECTION | TRANSFER_ENCODING | UPGRADE => true,
        TE => !value.eq_ignore_ascii_case(HTTP_TE_TRAILERS.as_bytes()),
        _ => false,
    }
}

/// Reports whether an HTTP/2 request carries any forbidden connection-specific header.
///
/// # Arguments
///
/// - `&ProtocolHeaders` - Incoming HTTP/2 request headers.
///
/// # Returns
///
/// - `bool` - `true` when any header violates RFC 9113 §8.2.2.
pub(crate) fn has_forbidden_h2_request_header(headers: &ProtocolHeaders) -> bool {
    headers
        .iter()
        .any(|(name, value): (&HeaderName, &HeaderValue)| {
            is_forbidden_h2_header(name.as_str(), value.as_bytes())
        })
}

/// Checks that the request authority and the host header name the same host.
///
/// HTTP/2 and HTTP/3 carry the authority in a pseudo header, and a client may
/// also send a `host` header beside it. RFC 9113 Section 8.3.1 makes a request
/// that names two different entities malformed, because a proxy that trusts
/// one value and an origin that trusts the other would route the same bytes
/// to different hosts. The pseudo header wins when only one of the two is
/// present.
///
/// The userinfo form is refused in both places, because RFC 9113 Section 8.3.1
/// forbids it in `:authority` and RFC 9110 Section 7.2 forbids it in `host`, and
/// a value that carries it names an entity other than the one it appears to.
///
/// # Arguments
///
/// - `&ProtocolHeaders` - The protocol request headers.
/// - `&Uri` - The request target, carrying the authority.
///
/// # Returns
///
/// - `Result<(), RequestError>` - `Ok(())` when the authority is unambiguous, or
///   `RequestError::InvalidUrlHost` when it is not.
pub fn check_request_authority(headers: &ProtocolHeaders, uri: &Uri) -> Result<(), RequestError> {
    let host_count: usize = headers.get_all(HOST).iter().count();
    if host_count > 1 {
        return Err(RequestError::InvalidUrlHost(HttpStatus::BadRequest));
    }
    let authority: Option<&str> = uri
        .authority()
        .map(|authority: &Authority| authority.as_str());
    let host: Option<&str> = match headers.get(HOST) {
        Some(value) => match value.to_str() {
            Ok(value) => Some(value),
            Err(_) => return Err(RequestError::InvalidUrlHost(HttpStatus::BadRequest)),
        },
        None => match headers.get(COLON_AUTHORITY) {
            Some(value) => match value.to_str() {
                Ok(value) => Some(value),
                Err(_) => return Err(RequestError::InvalidUrlHost(HttpStatus::BadRequest)),
            },
            None => None,
        },
    };
    if let Some(authority) = authority
        && !is_valid_authority_value(authority)
    {
        return Err(RequestError::InvalidUrlHost(HttpStatus::BadRequest));
    }
    if let Some(host) = host
        && !is_valid_authority_value(host)
    {
        return Err(RequestError::InvalidUrlHost(HttpStatus::BadRequest));
    }
    let (Some(authority), Some(host)) = (authority, host) else {
        return Ok(());
    };
    let scheme: Option<&str> = uri.scheme_str();
    if !same_authority(authority, host, scheme) {
        return Err(RequestError::InvalidUrlHost(HttpStatus::BadRequest));
    }
    Ok(())
}

/// Reports whether two authority values name the same host.
///
/// RFC 9113 Section 8.3.1 asks for the two values to be normalized before they
/// are compared, and RFC 3986 Section 6.2.3 defines the scheme based
/// normalization that a server must apply: the host is compared without regard
/// to case, and a port that is the default for the scheme is the same as no
/// port at all.
///
/// The `http` crate accepts authority values it cannot interpret, such as a
/// port that is not a number, so the values are split here instead of being
/// compared through the crate's parser.
///
/// # Arguments
///
/// - `&str` - The authority from the pseudo header.
/// - `&str` - The authority from the host header.
/// - `Option<&str>` - The scheme of the request, used to resolve the default port.
///
/// # Returns
///
/// - `bool` - `true` when both values name the same host.
pub fn same_authority(authority: &str, host: &str, scheme: Option<&str>) -> bool {
    if authority.eq_ignore_ascii_case(host) {
        return true;
    }
    let (auth_host, auth_port): (&str, Option<&str>) = split_authority_port(authority);
    let (host_host, host_port): (&str, Option<&str>) = split_authority_port(host);
    if !auth_host.eq_ignore_ascii_case(host_host) {
        return false;
    }
    let default_port: Option<&str> = match scheme {
        Some(HTTP_SCHEME) => Some(DEFAULT_HTTP_PORT_STR),
        Some(HTTPS_SCHEME) => Some(DEFAULT_HTTPS_PORT_STR),
        _ => None,
    };
    let effective_auth_port: Option<&str> = match (auth_port, default_port) {
        (Some(port), Some(default_port)) if port == default_port => None,
        (port, _) => port,
    };
    let effective_host_port: Option<&str> = match (host_port, default_port) {
        (Some(port), Some(default_port)) if port == default_port => None,
        (port, _) => port,
    };
    effective_auth_port == effective_host_port
}

/// Normalizes an authority value for comparison.
///
/// The host is lowercased and a port that is the default for the scheme is
/// dropped, so `EXAMPLE.com` and `example.com:443` name the same host for an
/// `https` request.
///
/// # Arguments
///
/// - `&str` - The authority value.
/// - `Option<&str>` - The scheme of the request, used to resolve the default port.
///
/// # Returns
///
/// - `(String, Option<String>)` - The normalized host, and the normalized port.
#[cfg(test)]
pub fn normalize_authority(authority: &str, scheme: Option<&str>) -> (String, Option<String>) {
    let (host, port): (&str, Option<&str>) = split_authority_port(authority);
    let host: String = host.to_ascii_lowercase();
    let port: Option<String> = port.map(|port: &str| port.to_owned());
    let default_port: Option<&str> = match scheme {
        Some(HTTP_SCHEME) => Some(DEFAULT_HTTP_PORT_STR),
        Some(HTTPS_SCHEME) => Some(DEFAULT_HTTPS_PORT_STR),
        _ => None,
    };
    match (port, default_port) {
        (Some(port), Some(default_port)) if port == default_port => (host, None),
        (port, _) => (host, port),
    }
}

/// Splits an authority value into its host and its optional port.
///
/// # Arguments
///
/// - `&str` - The authority value.
///
/// # Returns
///
/// - `(&str, Option<&str>)` - The host, and the port when the value carries one.
pub fn split_authority_port(authority: &str) -> (&str, Option<&str>) {
    let authority: &str = authority.trim();
    if authority.starts_with('[') {
        return match authority.find(']') {
            Some(end) => match authority[end + 1..].strip_prefix(':') {
                Some(port) => (&authority[..end + 1], Some(port)),
                None => (authority, None),
            },
            None => (authority, None),
        };
    }
    match authority.rsplit_once(':') {
        Some((host, port)) if !host.is_empty() && !host.contains(':') => (host, Some(port)),
        _ => (authority, None),
    }
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
    match (query_index, hash_index) {
        (Some(query_index), Some(hash_index)) if hash_index < query_index => EMPTY_STR,
        (Some(query_index), Some(hash_index)) => &target[query_index + 1..hash_index],
        (Some(query_index), None) => &target[query_index + 1..],
        (None, _) => EMPTY_STR,
    }
}

/// Extracts the path portion from a protocol request target.
///
/// # Arguments
///
/// - `&str` - The request target.
/// - `Option<usize>` - The index of the `?` separator, if present.
/// - `Option<usize>` - The index of the `#` separator, if present.
///
/// # Returns
///
/// - `&str` - The path portion without query or fragment.
pub fn split_target_path(
    target: &str,
    query_index: Option<usize>,
    hash_index: Option<usize>,
) -> &str {
    match (query_index, hash_index) {
        (Some(query_index), Some(hash_index)) => &target[..query_index.min(hash_index)],
        (Some(query_index), None) => &target[..query_index],
        (None, Some(hash_index)) => &target[..hash_index],
        (None, None) => target,
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
pub fn fill_request_querys(query: &str, querys: &mut RequestQuerys) {
    if query.is_empty() {
        return;
    }
    let mut param_count: usize = 0;
    for pair in query.split(AND) {
        if param_count >= PROTOCOL_MAX_QUERY_PARAM_COUNT {
            break;
        }
        if let Some((key, value)) = pair.split_once(EQUAL) {
            if !key.is_empty() {
                param_count += 1;
                if let Some(existing) = querys.get_mut(key) {
                    existing.push_back(value.to_owned());
                } else {
                    let mut values: VecDeque<String> = VecDeque::with_capacity(1);
                    values.push_back(value.to_owned());
                    querys.insert(key.to_owned(), values);
                }
            }
        } else if !pair.is_empty() {
            param_count += 1;
            if !querys.contains_key(pair) {
                querys.insert(pair.to_owned(), VecDeque::new());
            }
        }
    }
}

/// Decodes a protocol method into the framework method enum.
///
/// # Arguments
///
/// - `ProtocolMethod` - The method decoded by the protocol library.
///
/// # Returns
///
/// - `RequestMethod` - The framework method.
pub fn protocol_method(method: ProtocolMethod) -> RequestMethod {
    if method == ProtocolMethod::GET {
        return Method::Get;
    }
    if method == ProtocolMethod::POST {
        return Method::Post;
    }
    if method == ProtocolMethod::PUT {
        return Method::Put;
    }
    if method == ProtocolMethod::DELETE {
        return Method::Delete;
    }
    if method == ProtocolMethod::PATCH {
        return Method::Patch;
    }
    if method == ProtocolMethod::HEAD {
        return Method::Head;
    }
    if method == ProtocolMethod::OPTIONS {
        return Method::Options;
    }
    if method == ProtocolMethod::CONNECT {
        return Method::Connect;
    }
    if method == ProtocolMethod::TRACE {
        return Method::Trace;
    }
    Method::Unknown(method.as_str().to_owned())
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
    let max_header_count: usize = config.get_max_header_count();
    let max_header_key_size: usize = config.get_max_header_key_size();
    let max_header_value_size: usize = config.get_max_header_value_size();
    let target_headers: &mut RequestHeaders = request.get_mut_headers();
    let mut header_count: usize = 0;
    PROTOCOL_HEADER_POOL.with(|pool: &RefCell<Vec<(String, VecDeque<String>)>>| {
        let mut pool_guard: RefMut<'_, Vec<(String, VecDeque<String>)>> = pool.borrow_mut();
        for (name, value) in headers.iter() {
            let key: &str = name.as_str();
            if key.starts_with(PSEUDO_HEADER_PREFIX) {
                continue;
            }
            if is_forbidden_h2_header(key, value.as_bytes()) {
                return Err(RequestError::HttpRead(HttpStatus::BadRequest));
            }
            let Ok(value) = value.to_str() else {
                return Err(RequestError::HttpRead(HttpStatus::BadRequest));
            };
            header_count += 1;
            if header_count > max_header_count
                && max_header_count != DEFAULT_LOW_SECURITY_MAX_HEADER_COUNT
            {
                return Err(RequestError::TooManyHeaders(
                    HttpStatus::RequestHeaderFieldsTooLarge,
                ));
            }
            if key.len() > max_header_key_size
                && max_header_key_size != DEFAULT_LOW_SECURITY_MAX_HEADER_KEY_SIZE
            {
                return Err(RequestError::HeaderKeyTooLong(HttpStatus::BadRequest));
            }
            if value.len() > max_header_value_size
                && max_header_value_size != DEFAULT_LOW_SECURITY_MAX_HEADER_VALUE_SIZE
            {
                return Err(RequestError::HeaderValueTooLong(HttpStatus::BadRequest));
            }
            if let Some(existing) = target_headers.get_mut(key) {
                existing.push_back(value.to_owned());
                continue;
            }
            let (mut pooled_key, mut pooled_values): (String, VecDeque<String>) = pool_guard
                .pop()
                .unwrap_or_else(|| (String::new(), VecDeque::new()));
            pooled_key.clear();
            pooled_key.push_str(key);
            while pooled_values.len() > 1 {
                pooled_values.pop_back();
            }
            if let Some(slot) = pooled_values.front_mut() {
                slot.clear();
                slot.push_str(value);
            } else {
                pooled_values.push_back(value.to_owned());
            }
            target_headers.insert(pooled_key, pooled_values);
        }
        Ok(())
    })
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
    let mut declared: Option<usize> = None;
    for value in headers.get_all(CONTENT_LENGTH).iter() {
        if declared.is_some() {
            return Err(RequestError::InvalidContentLength(HttpStatus::BadRequest));
        }
        let Ok(value) = value.to_str() else {
            return Err(RequestError::InvalidContentLength(HttpStatus::BadRequest));
        };
        let parsed: usize = parse_ascii_length(value)?;
        if parsed > max_body_size && max_body_size != DEFAULT_LOW_SECURITY_MAX_BODY_SIZE {
            return Err(RequestError::ContentLengthTooLarge(
                HttpStatus::PayloadTooLarge,
            ));
        }
        declared = Some(parsed);
    }
    Ok(declared)
}

/// Reports whether any Expect field requests a 100 Continue response.
///
/// # Arguments
///
/// - `&ProtocolHeaders` - The request headers.
///
/// # Returns
///
/// - `bool` - Whether an unparameterized continue expectation is present.
#[cold]
pub(crate) fn has_continue_expectation(headers: &ProtocolHeaders) -> bool {
    headers
        .get_all(EXPECT)
        .iter()
        .any(|value: &HeaderValue| contains_continue_member(value.as_bytes()))
}

/// Matches a bare list member without splitting quoted extension values.
///
/// # Arguments
///
/// - `&[u8]` - One Expect field value.
///
/// # Returns
///
/// - `bool` - Whether the field contains the continue token.
pub(crate) fn contains_continue_member(value: &[u8]) -> bool {
    let token: &[u8] = CONTINUE_EXPECT.as_bytes();
    let mut start: usize = 0;
    let mut quoted: bool = false;
    let mut escaped: bool = false;
    for (offset, byte) in value.iter().copied().enumerate() {
        match byte {
            _ if escaped => escaped = false,
            HTTP_QUOTED_FIELD_ESCAPE if quoted => escaped = true,
            HTTP_QUOTED_FIELD_QUOTE => quoted = !quoted,
            COMMA_U8 if !quoted => {
                if value[start..offset]
                    .trim_ascii()
                    .eq_ignore_ascii_case(token)
                {
                    return true;
                }
                start = offset + 1;
            }
            _ => {}
        }
    }
    value[start..].trim_ascii().eq_ignore_ascii_case(token)
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
        || status_code == HttpStatus::ResetContent.code()
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
/// - `Bytes` - The incoming body chunk.
///
/// # Returns
///
/// - `Result<(), RequestError>` - `Ok(())` while the body stays within the cap, or payload-too-large error.
pub fn push_bounded_body(
    limit: usize,
    body: &mut Vec<u8>,
    chunk: Bytes,
) -> Result<(), RequestError> {
    if limit != DEFAULT_LOW_SECURITY_MAX_BODY_SIZE && chunk.len() > limit.saturating_sub(body.len())
    {
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
#[cfg(test)]
pub fn request_target(path: &str, query: Option<&str>) -> String {
    match query {
        Some(query) => {
            let mut target: String = String::with_capacity(path.len() + query.len() + 1);
            target.push_str(path);
            target.push(QUESTION_MARK);
            target.push_str(query);
            target
        }
        None => path.to_owned(),
    }
}
