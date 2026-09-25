use super::*;

impl SharedRequestBuilder {
    /// Constructs an HTTP request byte vector.
    ///
    /// # Arguments
    ///
    /// - `&str` - The HTTP method.
    /// - `String` - The request path.
    /// - `Vec<u8>` - The raw bytes of the request headers.
    /// - `Option<Vec<u8>>` - The optional raw bytes of the request body.
    /// - `String` - The HTTP version string.
    ///
    /// # Returns
    ///
    /// - `Vec<u8>` - The complete HTTP request as a byte vector.
    pub(crate) fn build_http_request(
        method: &str,
        path: String,
        header_bytes: Vec<u8>,
        body_bytes: Option<Vec<u8>>,
        http_version_str: String,
    ) -> Vec<u8> {
        let request_line_size: usize = method.len() + 1 + path.len() + 1 + http_version_str.len();
        let body_size: usize = body_bytes.as_ref().map_or(0, |b| b.len());
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

    /// Constructs an HTTP GET request byte vector.
    ///
    /// # Arguments
    ///
    /// - `String` - The request path.
    /// - `Vec<u8>` - The raw bytes of the request headers.
    /// - `String` - The HTTP version string.
    ///
    /// # Returns
    ///
    /// - `Vec<u8>` - The complete HTTP GET request as a byte vector.
    pub(crate) fn build_get_request(
        path: String,
        header_bytes: Vec<u8>,
        http_version_str: String,
    ) -> Vec<u8> {
        Self::build_http_request("GET", path, header_bytes, None, http_version_str)
    }

    /// Constructs an HTTP POST request byte vector.
    ///
    /// # Arguments
    ///
    /// - `String` - The request path.
    /// - `Vec<u8>` - The raw bytes of the request headers.
    /// - `Vec<u8>` - The raw bytes of the request body.
    /// - `String` - The HTTP version string.
    ///
    /// # Returns
    ///
    /// - `Vec<u8>` - The complete HTTP POST request as a byte vector.
    pub(crate) fn build_post_request(
        path: String,
        header_bytes: Vec<u8>,
        body_bytes: Vec<u8>,
        http_version_str: String,
    ) -> Vec<u8> {
        Self::build_http_request(
            "POST",
            path,
            header_bytes,
            Some(body_bytes),
            http_version_str,
        )
    }
}

impl SharedResponseHandler {
    /// Parses response headers to extract status code, content length, redirect URL, and chunked encoding.
    ///
    /// # Arguments
    ///
    /// - `&[u8]` - The raw bytes of the response headers.
    /// - `&[u8]` - The raw bytes of the HTTP version.
    /// - `&[u8]` - The byte pattern to identify the "Location" header.
    /// - `&mut usize` - A mutable reference to store the content length.
    /// - `&mut Option<Vec<u8>>` - A mutable reference to store the redirect URL if present.
    /// - `&mut bool` - A mutable reference to store whether chunked encoding is used.
    ///
    /// # Returns
    ///
    /// - `Result<(), RequestError>` - Ok if parsing is successful, Err otherwise.
    pub(crate) fn parse_response_headers(
        headers_bytes: &[u8],
        http_version_bytes: &[u8],
        location_sign_key: &[u8],
        content_length: &mut usize,
        redirect_url: &mut Option<Vec<u8>>,
        is_chunked: &mut bool,
    ) -> Result<(), RequestError> {
        if let Some(status_pos) =
            Self::find_pattern_case_insensitive(headers_bytes, http_version_bytes)
        {
            let status_code_start: usize = status_pos + http_version_bytes.len() + 1;
            let status_code_end: usize = status_code_start + 3;
            if status_code_end <= headers_bytes.len() {
                let status_code: usize =
                    Self::parse_status_code(&headers_bytes[status_code_start..status_code_end]);

                if (300..=399).contains(&status_code)
                    && let Some(location_pos) =
                        Self::find_pattern_case_insensitive(headers_bytes, location_sign_key)
                {
                    let start: usize = location_pos + location_sign_key.len();
                    if let Some(end_pos) = Self::find_crlf(headers_bytes, start) {
                        let mut url_vec = Vec::with_capacity(end_pos - start);
                        url_vec.extend_from_slice(&headers_bytes[start..end_pos]);
                        *redirect_url = Some(url_vec);
                    }
                }
            }
        }
        *content_length = Self::get_content_length(headers_bytes);
        *is_chunked = Self::is_chunked_encoding(headers_bytes);
        Ok(())
    }

    /// Finds a pattern within a byte slice, ignoring case.
    ///
    /// # Arguments
    ///
    /// - `&[u8]` - The haystack (the byte slice to search within).
    /// - `&[u8]` - The needle (the byte slice to search for).
    ///
    /// # Returns
    ///
    /// - `Option<usize>` - The starting index of the first occurrence of the needle in the haystack,
    ///   or None if not found.
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

    /// Finds the position of the Carriage Return Line Feed (CRLF) sequence in a byte slice.
    ///
    /// Searches for `\r\n` starting from the specified index.
    ///
    /// # Arguments
    ///
    /// - `&[u8]` - The data to search within.
    /// - `usize` - The starting index for the search.
    ///
    /// # Returns
    ///
    /// - `Option<usize>` - The index where the CRLF sequence starts, or None if not found.
    pub(crate) fn find_crlf(data: &[u8], start: usize) -> Option<usize> {
        let search_data: &[u8] = &data[start..];
        for i in 0..search_data.len().saturating_sub(1) {
            if search_data[i] == b'\r' && search_data[i + 1] == b'\n' {
                return Some(start + i);
            }
        }
        None
    }

    /// Finds the position of the double Carriage Return Line Feed (CRLF) sequence in a byte slice.
    ///
    /// Searches for `\r\n\r\n` starting from the specified index.
    ///
    /// # Arguments
    ///
    /// - `&[u8]` - The data to search within.
    /// - `usize` - The starting index for the search.
    ///
    /// # Returns
    ///
    /// - `Option<usize>` - The index where the double CRLF sequence starts, or None if not found.
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

    /// Extracts the Content-Length value from response bytes.
    ///
    /// Searches for the "Content-Length" header and parses its value.
    ///
    /// # Arguments
    ///
    /// - `&[u8]` - The raw bytes of the HTTP response.
    ///
    /// # Returns
    ///
    /// - `usize` - The content length value, or 0 if not found or parsing fails.
    pub(crate) fn get_content_length(response_bytes: &[u8]) -> usize {
        if let Some(pos) =
            Self::find_pattern_case_insensitive(response_bytes, CONTENT_LENGTH_PATTERN)
        {
            let value_start: usize = pos + CONTENT_LENGTH_PATTERN.len();
            let value_start: usize = if response_bytes.get(value_start) == Some(&b' ') {
                value_start + 1
            } else {
                value_start
            };
            if let Some(end_pos) = Self::find_crlf(response_bytes, value_start) {
                let value_bytes: &[u8] = &response_bytes[value_start..end_pos];
                return Self::parse_decimal_bytes(value_bytes);
            }
        }
        0
    }

    /// Checks if the response uses chunked transfer encoding.
    ///
    /// Searches for the "Transfer-Encoding: chunked" header.
    ///
    /// # Arguments
    ///
    /// - `&[u8]` - The raw bytes of the HTTP response headers.
    ///
    /// # Returns
    ///
    /// - `bool` - True if chunked transfer encoding is used, false otherwise.
    pub(crate) fn is_chunked_encoding(headers_bytes: &[u8]) -> bool {
        if let Some(pos) =
            Self::find_pattern_case_insensitive(headers_bytes, TRANSFER_ENCODING_PATTERN)
        {
            let value_start: usize = pos + TRANSFER_ENCODING_PATTERN.len();
            let value_start: usize = if headers_bytes.get(value_start) == Some(&b' ') {
                value_start + 1
            } else {
                value_start
            };
            if let Some(end_pos) = Self::find_crlf(headers_bytes, value_start) {
                let value_bytes: &[u8] = &headers_bytes[value_start..end_pos];
                return Self::find_pattern_case_insensitive(value_bytes, CHUNKED_PATTERN).is_some();
            }
        }
        false
    }

    /// Parses a chunked transfer encoded body.
    ///
    /// Decodes the chunked encoding and returns the decoded body bytes.
    ///
    /// # Arguments
    ///
    /// - `&[u8]` - The raw bytes of the chunked body (starting after headers).
    ///
    /// # Returns
    ///
    /// - `Vec<u8>` - The decoded body bytes.
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
            let chunk_size_str: &[u8] = match chunk_size_str.iter().position(|&b| b == b';') {
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

    /// Parses a byte slice representing a decimal number into a `usize`.
    ///
    /// Skips leading whitespace and stops at the first non-digit character.
    ///
    /// # Arguments
    ///
    /// - `&[u8]` - The byte slice containing the decimal number.
    ///
    /// # Returns
    ///
    /// - `usize` - The parsed decimal value.
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

    /// Parses a byte slice representing an HTTP status code into a `usize`.
    ///
    /// Expects a 3-digit status code.
    ///
    /// # Arguments
    ///
    /// - `&[u8]` - The byte slice containing the status code.
    ///
    /// # Returns
    ///
    /// - `usize` - The parsed status code, or 0 if parsing fails or the input is invalid.
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

    /// Calculates a new buffer capacity based on current capacity and needed size.
    ///
    /// This function determines an appropriate buffer size, typically doubling the current
    /// capacity or increasing it by 50% of the needed capacity, ensuring efficient memory allocation.
    ///
    /// # Arguments
    ///
    /// - `&[u8]` - The current response bytes.
    /// - `usize` - The number of additional bytes needed.
    /// - `usize` - The current buffer capacity.
    ///
    /// # Returns
    ///
    /// - `usize` - The recommended new buffer capacity. Returns 0 if no increase is needed.
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
}
