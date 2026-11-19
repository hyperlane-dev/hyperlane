use super::*;

/// Split on a multi-byte delimiter, returning each slice. Used to split an
/// HTTP response into status line / headers / body.
/// Matches do not overlap, and an empty delimiter leaves the input unchanged.
///
/// # Arguments
///
/// - `&'a [u8]` - The buffer to split.
/// - `&'a [u8]` - The multi-byte delimiter to split on.
///
/// # Returns
///
/// - `Vec<&'a [u8]>`: The delimiter-separated slices, including any trailing remainder.
pub(crate) fn split_multi_byte<'a>(data: &'a [u8], delimiter: &'a [u8]) -> Vec<&'a [u8]> {
    if delimiter.is_empty() {
        return vec![data];
    }
    let mut result: Vec<&[u8]> = Vec::new();
    let mut start: usize = 0;
    while let Some(offset) = data[start..]
        .windows(delimiter.len())
        .position(|window: &[u8]| window == delimiter)
    {
        let end: usize = start + offset;
        result.push(&data[start..end]);
        start = end + delimiter.len();
    }
    if start < data.len() {
        result.push(&data[start..]);
    }
    result
}

/// Split on whitespace (space or tab).
///
/// # Arguments
///
/// - `&[u8]` - The buffer to split.
///
/// # Returns
///
/// - `Vec<&[u8]>`: The non-empty whitespace-separated slices.
pub(crate) fn split_whitespace(input: &[u8]) -> Vec<&[u8]> {
    let mut parts: Vec<&[u8]> = Vec::new();
    let mut start: usize = 0;
    for (i, &byte) in input.iter().enumerate() {
        if byte == SPACE_U8 || byte == TAB_U8 {
            if i > start {
                parts.push(&input[start..i]);
            }
            start = i + 1;
        }
    }
    if start < input.len() {
        parts.push(&input[start..]);
    }
    parts
}

/// Build the raw HTTP request line + headers + optional body into a single
/// `Vec<u8>` suitable for writing to the wire.
///
/// # Arguments
///
/// - `&str` - The HTTP method token.
/// - `String` - The request path, with any query string already appended.
/// - `Vec<u8>` - The wire-format header block bytes.
/// - `Option<Vec<u8>>` - The wire-format body bytes, or `None` when there is no body.
/// - `String` - The HTTP version token.
///
/// # Returns
///
/// - `Vec<u8>`: The complete request bytes, ready to write to the wire.
pub(crate) fn build_http_request(
    method: &str,
    path: String,
    header_bytes: Vec<u8>,
    body_bytes: Option<Vec<u8>>,
    http_version_str: String,
) -> Vec<u8> {
    let request_line_size: usize = method.len() + 1 + path.len() + 1 + http_version_str.len();
    let body_size: usize = body_bytes.as_ref().map_or(0, |b: &Vec<u8>| b.len());
    let total_size: usize = request_line_size + 2 + header_bytes.len() + 2 + body_size;
    let mut request: Vec<u8> = Vec::with_capacity(total_size);
    request.extend_from_slice(method.as_bytes());
    request.push(b' ');
    request.extend_from_slice(path.as_bytes());
    request.push(b' ');
    request.extend_from_slice(http_version_str.as_bytes());
    request.extend_from_slice(HTTP_BR_BYTES);
    request.extend_from_slice(&header_bytes);
    request.extend_from_slice(HTTP_BR_BYTES);
    if let Some(body) = body_bytes {
        request.extend_from_slice(&body);
    }
    request
}

/// Parse a chunked transfer-encoded body into the concatenated raw bytes.
///
/// Walks the `chunk-size CRLF chunk-data CRLF` sequence until it sees a
/// terminating zero-size chunk.
///
/// # Arguments
///
/// - `&[u8]` - The raw chunked body bytes.
///
/// # Returns
///
/// - `Result<Vec<u8>, RequestError>` - The complete decoded payload, or invalid or incomplete framing.
pub(crate) fn parse_chunked_body(body_bytes: &[u8]) -> Result<Vec<u8>, RequestError> {
    let body_end: usize = chunked_body_end(body_bytes)?.ok_or(
        RequestError::HttpRequestPartsInsufficient(HttpStatus::BadRequest),
    )?;
    let mut result: Vec<u8> = Vec::new();
    let mut pos: usize = 0;
    while pos < body_end {
        let line_end: usize = find_crlf(body_bytes, pos).ok_or(
            RequestError::HttpRequestPartsInsufficient(HttpStatus::BadRequest),
        )?;
        let chunk_size: usize = parse_chunk_size(&body_bytes[pos..line_end])?;
        if chunk_size == 0 {
            break;
        }
        let data_start: usize = line_end + HTTP_BR_BYTES.len();
        let data_end: usize = data_start
            .checked_add(chunk_size)
            .ok_or(RequestError::ContentLengthTooLarge(HttpStatus::BadRequest))?;
        result.extend_from_slice(&body_bytes[data_start..data_end]);
        pos = data_end + HTTP_BR_BYTES.len();
    }
    Ok(result)
}

/// Parse a strict hexadecimal chunk size, accepting optional extensions.
///
/// RFC 9112 §7.1.1 permits bad whitespace before the extension delimiter,
/// but not after a chunk size with no extension.
///
/// # Arguments
///
/// - `&[u8]` - The complete chunk-size line without its CRLF.
///
/// # Returns
///
/// - `Result<usize, RequestError>` - The bounded size, or an error for invalid syntax or overflow.
pub(crate) fn parse_chunk_size(line: &[u8]) -> Result<usize, RequestError> {
    if !is_valid_header_value(line) {
        return Err(RequestError::InvalidContentLength(HttpStatus::BadRequest));
    }
    let extension_start: Option<usize> = line
        .iter()
        .position(|byte: &u8| *byte == CHUNK_EXTENSION_SEPARATOR);
    let size_bytes: &[u8] = match extension_start {
        Some(end) => line[..end].trim_ascii_end(),
        None => line,
    };
    if size_bytes.is_empty() || !size_bytes.iter().all(u8::is_ascii_hexdigit) {
        return Err(RequestError::InvalidContentLength(HttpStatus::BadRequest));
    }
    let size: usize = from_utf8(size_bytes)
        .ok()
        .and_then(|value: &str| usize::from_str_radix(value, 16).ok())
        .ok_or(RequestError::ContentLengthTooLarge(HttpStatus::BadRequest))?;
    if size > MAX_RESPONSE_CHUNK_SIZE {
        return Err(RequestError::ContentLengthTooLarge(HttpStatus::BadRequest));
    }
    Ok(size)
}

/// Validate chunk boundaries and locate the complete terminating trailer section.
///
/// # Arguments
///
/// - `&[u8]` - The chunked bytes received so far.
///
/// # Returns
///
/// - `Result<Option<usize>, RequestError>` - The completed body end, incomplete input, or invalid framing.
pub(crate) fn chunked_body_end(body_bytes: &[u8]) -> Result<Option<usize>, RequestError> {
    let mut pos: usize = 0;
    while pos < body_bytes.len() {
        let Some(line_end) = find_crlf(body_bytes, pos) else {
            return Ok(None);
        };
        let chunk_size: usize = parse_chunk_size(&body_bytes[pos..line_end])?;
        let data_start: usize = line_end + HTTP_BR_BYTES.len();
        if chunk_size == 0 {
            if body_bytes[data_start..].starts_with(HTTP_CARRIAGE_RETURN_LINE_FEED) {
                return Ok(Some(data_start + HTTP_BR_BYTES.len()));
            }
            if let Some(trailer_end) = find_double_crlf(body_bytes, data_start) {
                let end: usize = trailer_end + HTTP_BR_BYTES.len() * 2;
                let _: Option<&[u8]> = response_header_value(&body_bytes[data_start..end], &[])?;
                return Ok(Some(end));
            }
            return Ok(None);
        }
        let data_end: usize = data_start
            .checked_add(chunk_size)
            .ok_or(RequestError::ContentLengthTooLarge(HttpStatus::BadRequest))?;
        let next: usize = data_end
            .checked_add(HTTP_BR_BYTES.len())
            .ok_or(RequestError::ContentLengthTooLarge(HttpStatus::BadRequest))?;
        if next > body_bytes.len() {
            return Ok(None);
        }
        if &body_bytes[data_end..next] != HTTP_BR_BYTES {
            return Err(RequestError::Request(INVALID_RESPONSE_FRAMING.to_owned()));
        }
        pos = next;
    }
    Ok(None)
}

/// Locate `\r\n\r\n` (end of response headers) starting at `start`.
///
/// # Arguments
///
/// - `&[u8]` - The buffer to search.
/// - `usize` - The offset at which the search starts.
///
/// # Returns
///
/// - `Option<usize>`: The absolute offset of the terminator, or `None` when absent.
pub(crate) fn find_double_crlf(data: &[u8], start: usize) -> Option<usize> {
    let search_data: &[u8] = &data[start..];
    for i in 0..search_data.len().saturating_sub(3) {
        if search_data[i] == b'\r'
            && search_data[i + 1] == b'\n'
            && search_data[i + 2] == b'\r'
            && search_data[i + 3] == b'\n'
        {
            return Some(start + i);
        }
    }
    None
}

/// Find a byte-pattern in a haystack, ASCII-case-insensitive.
///
/// # Arguments
///
/// - `&[u8]` - The haystack to search.
/// - `&[u8]` - The byte pattern to look for.
///
/// # Returns
///
/// - `Option<usize>`: The offset of the first match, or `None` when absent.
pub(crate) fn find_pattern_case_insensitive(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    if needle.is_empty() || haystack.len() < needle.len() {
        return None;
    }
    let needle_len: usize = needle.len();
    let search_len: usize = haystack.len() - needle_len + 1;
    let first_needle_lower: u8 = needle[0].to_ascii_lowercase();
    'outer: for i in 0..search_len {
        if haystack[i].to_ascii_lowercase() != first_needle_lower {
            continue;
        }
        for j in 1..needle_len {
            if !haystack[i + j].eq_ignore_ascii_case(&needle[j]) {
                continue 'outer;
            }
        }
        return Some(i);
    }
    None
}

/// Locate the next `\r\n` after `start`.
///
/// # Arguments
///
/// - `&[u8]` - The buffer to search.
/// - `usize` - The offset at which the search starts.
///
/// # Returns
///
/// - `Option<usize>`: The absolute offset of the terminator, or `None` when absent.
pub(crate) fn find_crlf(data: &[u8], start: usize) -> Option<usize> {
    let search_data: &[u8] = &data[start..];
    for i in 0..search_data.len().saturating_sub(1) {
        if search_data[i] == b'\r' && search_data[i + 1] == b'\n' {
            return Some(start + i);
        }
    }
    None
}

/// Extract `Content-Length` from the response fields, distinguishing absence from zero.
///
/// # Arguments
///
/// - `&[u8]` - The raw response header bytes.
///
/// # Returns
///
/// - `Result<Option<usize>, RequestError>` - The declared length, absence, or invalid framing.
pub(crate) fn get_content_length(response_bytes: &[u8]) -> Result<Option<usize>, RequestError> {
    let value: Option<&[u8]> = response_header_value(
        response_bytes,
        &CONTENT_LENGTH_PATTERN[..CONTENT_LENGTH_PATTERN.len() - 1],
    )?;
    match value {
        Some(bytes) => {
            let length: usize = parse_decimal_bytes(bytes)?;
            if length > isize::MAX as usize {
                return Err(RequestError::ContentLengthTooLarge(HttpStatus::BadRequest));
            }
            Ok(Some(length))
        }
        None => Ok(None),
    }
}

/// Determine whether the response has exactly one supported transfer coding.
///
/// # Arguments
///
/// - `&[u8]` - The raw response headers, optionally followed by a body.
///
/// # Returns
///
/// - `Result<bool, RequestError>` - Whether chunked framing is present, or an error for unsupported coding.
pub(crate) fn is_chunked_encoding(headers_bytes: &[u8]) -> Result<bool, RequestError> {
    let name: &[u8] = &TRANSFER_ENCODING_PATTERN[..TRANSFER_ENCODING_PATTERN.len() - 1];
    match response_header_value(headers_bytes, name)? {
        Some(value) if value.eq_ignore_ascii_case(CHUNKED_PATTERN) => Ok(true),
        Some(_) => Err(RequestError::Request(INVALID_RESPONSE_FRAMING.to_owned())),
        None => Ok(false),
    }
}

/// Parse a nonempty sequence of decimal digits without truncation or overflow.
///
/// # Arguments
///
/// - `&[u8]` - The ASCII decimal digits, without surrounding whitespace.
///
/// # Returns
///
/// - `Result<usize, RequestError>` - The exact length, or an error for invalid digits or overflow.
pub(crate) fn parse_decimal_bytes(bytes: &[u8]) -> Result<usize, RequestError> {
    if bytes.is_empty()
        || !bytes
            .iter()
            .all(|byte: &u8| (ASCII_ZERO..=ASCII_NINE).contains(byte))
    {
        return Err(RequestError::InvalidContentLength(HttpStatus::BadRequest));
    }
    let mut result: usize = 0;
    for &byte in bytes {
        result = result
            .checked_mul(10)
            .and_then(|value: usize| value.checked_add((byte - ASCII_ZERO) as usize))
            .ok_or(RequestError::ContentLengthTooLarge(HttpStatus::BadRequest))?;
    }
    Ok(result)
}

/// Check an HTTP field name against the nonempty RFC token grammar.
///
/// # Arguments
///
/// - `&[u8]` - The field name bytes.
///
/// # Returns
///
/// - `bool` - Whether every byte is an allowed token character.
pub(crate) fn is_valid_header_name(name: &[u8]) -> bool {
    !name.is_empty()
        && name
            .iter()
            .all(|byte: &u8| byte.is_ascii_alphanumeric() || FIELD_NAME_PUNCTUATION.contains(byte))
}

/// Check a field value for forbidden control bytes, permitting horizontal tabs.
///
/// # Arguments
///
/// - `&[u8]` - The field value bytes.
///
/// # Returns
///
/// - `bool` - Whether the value can be serialized as one header line.
pub(crate) fn is_valid_header_value(value: &[u8]) -> bool {
    value
        .iter()
        .all(|byte: &u8| !byte.is_ascii_control() || *byte == TAB_U8)
}

/// Validate every outgoing field before opening a stream or serializing headers.
///
/// # Arguments
///
/// - `&HashMap<String, String>` - The caller-provided header map, with arbitrary key casing.
///
/// # Returns
///
/// - `Result<(), RequestError>` - Success for valid fields, or an error before bytes can be sent.
pub(crate) fn validate_request_headers(
    headers: &HashMap<String, String>,
) -> Result<(), RequestError> {
    for (name, value) in headers {
        if !is_valid_header_name(name.as_bytes()) || !is_valid_header_value(value.as_bytes()) {
            return Err(RequestError::Request(INVALID_REQUEST_HEADER.to_owned()));
        }
    }
    Ok(())
}

/// Look up one complete field name, stopping at the header/body boundary.
///
/// # Arguments
///
/// - `&'a [u8]` - The response header block, optionally followed by body bytes.
/// - `&[u8]` - The exact field name without its colon.
///
/// # Returns
///
/// - `Result<Option<&'a [u8]>, RequestError>` - The trimmed value, absence, or invalid or duplicate fields.
pub(crate) fn response_header_value<'a>(
    headers_bytes: &'a [u8],
    field_name: &[u8],
) -> Result<Option<&'a [u8]>, RequestError> {
    let mut value: Option<&[u8]> = None;
    let mut pos: usize = 0;
    while pos < headers_bytes.len() {
        let end: usize = find_crlf(headers_bytes, pos).unwrap_or(headers_bytes.len());
        let line: &[u8] = &headers_bytes[pos..end];
        if line.is_empty() {
            break;
        }
        if !(pos == 0 && line.starts_with(HTTP_STATUS_PREFIX)) {
            let separator: usize = line
                .iter()
                .position(|byte: &u8| *byte == COLON_U8)
                .ok_or_else(|| RequestError::Request(INVALID_RESPONSE_FRAMING.to_owned()))?;
            let name: &[u8] = &line[..separator];
            let raw_value: &[u8] = &line[separator + 1..];
            if !is_valid_header_name(name) || !is_valid_header_value(raw_value) {
                return Err(RequestError::Request(INVALID_RESPONSE_FRAMING.to_owned()));
            }
            if name.eq_ignore_ascii_case(field_name) {
                if value.is_some() {
                    return Err(RequestError::Request(INVALID_RESPONSE_FRAMING.to_owned()));
                }
                value = Some(raw_value.trim_ascii());
            }
        }
        if end == headers_bytes.len() {
            break;
        }
        pos = end + HTTP_BR_BYTES.len();
    }
    Ok(value)
}

/// Parse the 3-byte ASCII status code from the response status line.
///
/// # Arguments
///
/// - `&[u8]` - The status-line bytes following the HTTP version.
///
/// # Returns
///
/// - `usize`: The status code, or `0` when the slice is not three digits.
pub(crate) fn parse_status_code(status_bytes: &[u8]) -> usize {
    if status_bytes.len() != 3 {
        return 0;
    }
    let mut result: usize = 0;
    for &byte in status_bytes {
        if byte.is_ascii_digit() {
            result = result * 10 + (byte - ASCII_ZERO) as usize;
        } else {
            return 0;
        }
    }
    result
}

/// Calculate the next buffer capacity for growing the response buffer.
///
/// # Arguments
///
/// - `&[u8]` - The response bytes accumulated so far.
/// - `usize` - The number of bytes just read.
/// - `usize` - The current capacity of the response buffer.
///
/// # Returns
///
/// - `usize`: The new capacity to reserve, or `0` when no growth is needed.
pub(crate) fn calculate_buffer_capacity(
    response_bytes: &[u8],
    n: usize,
    current_capacity: usize,
) -> usize {
    if response_bytes.len() + n <= current_capacity {
        return 0;
    }
    let needed_cap: usize = response_bytes.len() + n;
    if current_capacity == 0 {
        needed_cap.max(1024)
    } else if needed_cap <= current_capacity * 2 {
        current_capacity * 2
    } else {
        (needed_cap * 3) / 2
    }
}

/// Resolve HTTP/1 response framing from a complete field section.
///
/// For HEAD, framing fields are representation metadata (RFC 9112 §6.3).
///
/// # Arguments
///
/// - `&[u8]` - The raw response header bytes.
/// - `bool` - Whether the response answers a HEAD request.
///
/// # Returns
///
/// - `Result<Option<ResponseHeadFields>, RequestError>` - The final framing and redirect, or `None` for an interim response.
pub(crate) fn parse_response_headers(
    headers_bytes: &[u8],
    is_head: bool,
) -> Result<Option<ResponseHeadFields>, RequestError> {
    let status_start: usize = HTTP_STATUS_CODE_OFFSET;
    let status_code: usize = headers_bytes
        .get(status_start..status_start + 3)
        .map_or(0, parse_status_code);
    if headers_bytes
        .get(..HTTP_STATUS_PREFIX.len())
        .is_none_or(|prefix: &[u8]| {
            find_pattern_case_insensitive(prefix, HTTP_STATUS_PREFIX).is_none()
        })
        || headers_bytes.get(status_start - 1) != Some(&SPACE_U8)
        || !(100..=599).contains(&status_code)
        || !matches!(headers_bytes.get(status_start + 3), Some(b' ' | b'\r'))
    {
        return Err(RequestError::Request(INVALID_RESPONSE_FRAMING.to_owned()));
    }
    let framing: ResponseFraming = if is_head {
        let _: Option<&[u8]> = response_header_value(headers_bytes, &[])?;
        ResponseFraming::Fixed(0)
    } else {
        let length: Option<usize> = get_content_length(headers_bytes)?;
        let chunked: bool = is_chunked_encoding(headers_bytes)?;
        match (length, chunked) {
            (Some(_), true) => {
                return Err(RequestError::Request(INVALID_RESPONSE_FRAMING.to_owned()));
            }
            (None, true) => ResponseFraming::Chunked,
            (Some(length), false) => ResponseFraming::Fixed(length),
            (None, false) => ResponseFraming::Close,
        }
    };
    if status_code == HttpStatus::SwitchingProtocols.code()
        || status_code == HttpStatus::NoContent.code()
        || status_code == HttpStatus::NotModified.code()
    {
        return Ok(Some((ResponseFraming::Fixed(0), None)));
    }
    if status_code < HttpStatus::Ok.code() {
        return Ok(None);
    }

    let redirect: Option<Vec<u8>> = if (300..=399).contains(&status_code) {
        response_header_value(headers_bytes, LOCATION.as_bytes())?.map(<[u8]>::to_vec)
    } else {
        None
    };
    Ok(Some((framing, redirect)))
}

/// Finds the final response head and removes any preceding informational heads.
///
/// # Arguments
///
/// - `&mut Vec<u8>` - The accumulated response bytes.
/// - `usize` - The earliest position where a new header terminator could start.
/// - `bool` - Whether the response answers a HEAD request.
///
/// # Returns
///
/// - `Result<Option<FinalResponseHead>, RequestError>` - The framing of the final response, or `None` when more bytes are needed.
pub(crate) fn find_final_response_head(
    bytes: &mut Vec<u8>,
    mut search_start: usize,
    is_head: bool,
) -> Result<Option<FinalResponseHead>, RequestError> {
    let mut start: usize = 0;
    while let Some(position) = find_double_crlf(bytes, search_start) {
        let end: usize = position + HTTP_BR_BYTES.len() * 2;
        if let Some((framing, redirect)) = parse_response_headers(&bytes[start..end], is_head)? {
            if start != 0 {
                drop(bytes.drain(..start));
            }
            return Ok(Some((end - start, framing, redirect)));
        }
        start = end;
        search_start = end;
    }
    if start != 0 {
        drop(bytes.drain(..start));
    }
    Ok(None)
}

/// Reports whether the final response body is complete without waiting for EOF.
///
/// # Arguments
///
/// - `ResponseFraming` - The final body framing.
/// - `&[u8]` - The bytes following the response head.
///
/// # Returns
///
/// - `Result<bool, RequestError>` - Whether the body is complete, or a framing error.
#[inline]
pub(crate) fn response_body_is_complete(
    framing: ResponseFraming,
    bytes: &[u8],
) -> Result<bool, RequestError> {
    match framing {
        ResponseFraming::Fixed(length) => Ok(bytes.len() >= length),
        ResponseFraming::Chunked => Ok(chunked_body_end(bytes)?.is_some()),
        ResponseFraming::Close => Ok(false),
    }
}

/// Separates response fields from the decoded framing and preserves the body bytes.
///
/// # Arguments
///
/// - `&[u8]` - The accumulated final response of a completed read.
/// - `usize` - The end of the final response head.
/// - `ResponseFraming` - The final body framing.
///
/// # Returns
///
/// - `Result<HttpResponse, RequestError>` - The response, or an incomplete or invalid body error.
pub(crate) fn finish_framed_response(
    bytes: &[u8],
    head_end: usize,
    framing: ResponseFraming,
) -> Result<HttpResponse, RequestError> {
    let payload: &[u8] = &bytes[head_end..];
    let body: ResponseBody = match framing {
        ResponseFraming::Fixed(length) => payload
            .get(..length)
            .ok_or(RequestError::InvalidContentLength(HttpStatus::BadRequest))?
            .to_vec(),
        ResponseFraming::Chunked => parse_chunked_body(payload)?,
        ResponseFraming::Close => payload.to_vec(),
    };
    let mut response: HttpResponse = HttpResponse::from_bytes(&bytes[..head_end]);
    response.body = body;
    Ok(response)
}
