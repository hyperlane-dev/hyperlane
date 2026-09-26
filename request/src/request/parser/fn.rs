use super::*;

/// Split on a multi-byte delimiter, returning each slice. Used to split an
/// HTTP response into status line / headers / body.
pub(crate) fn split_multi_byte<'a>(data: &'a [u8], delimiter: &'a [u8]) -> Vec<&'a [u8]> {
    let mut result: Vec<&[u8]> = Vec::new();
    let mut start: usize = 0;
    for i in 0..=data.len() {
        if data[i..].starts_with(delimiter) {
            result.push(&data[start..i]);
            start = i + delimiter.len();
        }
    }
    if start < data.len() {
        result.push(&data[start..]);
    }
    result
}

/// Split on whitespace (space or tab).
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
pub(crate) fn parse_chunked_body(body_bytes: &[u8]) -> Vec<u8> {
    let mut result: Vec<u8> = Vec::new();
    let mut pos: usize = 0;
    while pos < body_bytes.len() {
        let chunk_size_end: usize = match body_bytes[pos..]
            .windows(2)
            .position(|window: &[u8]| window == b"\r\n")
        {
            Some(p) => pos + p,
            None => break,
        };
        let chunk_size_str: &[u8] = &body_bytes[pos..chunk_size_end];
        let chunk_size_str: &[u8] = match chunk_size_str.iter().position(|&b: &u8| b == b';') {
            Some(p) => &chunk_size_str[..p],
            None => chunk_size_str,
        };
        let chunk_size: usize = match std::str::from_utf8(chunk_size_str) {
            Ok(s) => match usize::from_str_radix(s.trim(), 16) {
                Ok(n) => n,
                Err(_) => break,
            },
            Err(_) => break,
        };
        if chunk_size == 0 {
            break;
        }
        let chunk_data_start: usize = chunk_size_end + 2;
        let chunk_data_end: usize = chunk_data_start + chunk_size;
        if chunk_data_end > body_bytes.len() {
            break;
        }
        result.extend_from_slice(&body_bytes[chunk_data_start..chunk_data_end]);
        pos = chunk_data_end + 2;
    }
    result
}

/// Locate `\r\n\r\n` (end of response headers) starting at `start`.
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
pub(crate) fn find_crlf(data: &[u8], start: usize) -> Option<usize> {
    let search_data: &[u8] = &data[start..];
    for i in 0..search_data.len().saturating_sub(1) {
        if search_data[i] == b'\r' && search_data[i + 1] == b'\n' {
            return Some(start + i);
        }
    }
    None
}

/// Extract `Content-Length` value from response bytes (0 if missing).
pub(crate) fn get_content_length(response_bytes: &[u8]) -> usize {
    if let Some(pos) = find_pattern_case_insensitive(response_bytes, CONTENT_LENGTH_PATTERN) {
        let value_start: usize = pos + CONTENT_LENGTH_PATTERN.len();
        let value_start: usize = if response_bytes.get(value_start) == Some(&b' ') {
            value_start + 1
        } else {
            value_start
        };
        if let Some(end_pos) = find_crlf(response_bytes, value_start) {
            let value_bytes: &[u8] = &response_bytes[value_start..end_pos];
            return parse_decimal_bytes(value_bytes);
        }
    }
    0
}

/// Does the response use `Transfer-Encoding: chunked`?
pub(crate) fn is_chunked_encoding(headers_bytes: &[u8]) -> bool {
    if let Some(pos) = find_pattern_case_insensitive(headers_bytes, TRANSFER_ENCODING_PATTERN) {
        let value_start: usize = pos + TRANSFER_ENCODING_PATTERN.len();
        let value_start: usize = if headers_bytes.get(value_start) == Some(&b' ') {
            value_start + 1
        } else {
            value_start
        };
        if let Some(end_pos) = find_crlf(headers_bytes, value_start) {
            let value_bytes: &[u8] = &headers_bytes[value_start..end_pos];
            return find_pattern_case_insensitive(value_bytes, CHUNKED_PATTERN).is_some();
        }
    }
    false
}

/// Parse a byte slice as a decimal `usize` (skipping leading whitespace).
pub(crate) fn parse_decimal_bytes(bytes: &[u8]) -> usize {
    let mut result: usize = 0;
    let mut started: bool = false;
    for &byte in bytes {
        match byte {
            b'0'..=b'9' => {
                started = true;
                result = result * 10 + (byte - b'0') as usize;
            }
            b' ' | b'\t' if !started => continue,
            _ => break,
        }
    }
    result
}

/// Parse the 3-byte ASCII status code from the response status line.
pub(crate) fn parse_status_code(status_bytes: &[u8]) -> usize {
    if status_bytes.len() != 3 {
        return 0;
    }
    let mut result: usize = 0;
    for &byte in status_bytes {
        if byte.is_ascii_digit() {
            result = result * 10 + (byte - b'0') as usize;
        } else {
            return 0;
        }
    }
    result
}

/// Calculate the next buffer capacity for growing the response buffer.
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

/// Parse the response status line + headers to extract the 3-digit status code,
/// the `Content-Length`, the redirect URL (if any), and whether the body is
/// chunked. All three out-params are populated; only `redirect_url` may be
/// `None`.
pub(crate) fn parse_response_headers(
    headers_bytes: &[u8],
    http_version_bytes: &[u8],
    location_sign_key: &[u8],
    content_length: &mut usize,
    redirect_url: &mut Option<Vec<u8>>,
    is_chunked: &mut bool,
) -> Result<(), RequestError> {
    if let Some(status_pos) = find_pattern_case_insensitive(headers_bytes, http_version_bytes) {
        let status_code_start: usize = status_pos + http_version_bytes.len() + 1;
        let status_code_end: usize = status_code_start + 3;
        if status_code_end <= headers_bytes.len() {
            let status_code: usize =
                parse_status_code(&headers_bytes[status_code_start..status_code_end]);

            if (300..=399).contains(&status_code)
                && let Some(location_pos) =
                    find_pattern_case_insensitive(headers_bytes, location_sign_key)
            {
                let start: usize = location_pos + location_sign_key.len();
                if let Some(end_pos) = find_crlf(headers_bytes, start) {
                    let mut url_vec = Vec::with_capacity(end_pos - start);
                    url_vec.extend_from_slice(&headers_bytes[start..end_pos]);
                    *redirect_url = Some(url_vec);
                }
            }
        }
    }
    *content_length = get_content_length(headers_bytes);
    *is_chunked = is_chunked_encoding(headers_bytes);
    Ok(())
}
