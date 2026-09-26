use super::*;

/// Parsed HTTP response.
///
/// Value type with all fields public — no `Arc<RwLock>` wrapping, no
/// associated-type / boxed-trait indirection. Use the field accessors
/// directly (`response.status_code`, `response.body`, ...) or the
/// convenience helpers (`response.text()`, `response.is_success()`).
///
/// `HttpResponse` is `Clone` and `Debug`. Decoding is a method
/// ([`HttpResponse::decode`]) that returns a fresh `HttpResponse` with
/// the body expanded — original is untouched.
#[derive(Clone, Debug, Getter, Setter)]
pub struct HttpResponse {
    /// HTTP version (`HTTP/1.1`, `HTTP/2`, ...).
    pub version: HttpVersion,
    /// 3-digit numeric status code (`200`, `404`, ...).
    #[get(type(copy))]
    pub status_code: ResponseStatusCode,
    /// Reason phrase (`"OK"`, `"Not Found"`, ...).
    #[set(type(AsRef<str>))]
    pub reason_phrase: String,
    /// Response headers (single-value, lowercase keys).
    pub headers: HttpResponseHeaders,
    /// Raw response body bytes.
    #[set(type(AsRef<[u8]>))]
    pub body: ResponseBody,
}

impl Default for HttpResponse {
    fn default() -> Self {
        Self {
            version: HttpVersion::default(),
            status_code: HttpStatus::Unknown.code(),
            reason_phrase: HttpStatus::Unknown.to_string(),
            headers: new_response_headers(),
            body: ResponseBody::new(),
        }
    }
}

impl HttpResponse {
    /// Construct from raw response bytes (status line + headers + body).
    pub fn from_bytes(response: &[u8]) -> Self {
        let split_lines: Vec<&[u8]> =
            crate::request::parser::r#fn::split_multi_byte(response, HTTP_BR_BYTES);
        let mut lines: IntoIter<&[u8]> = split_lines.into_iter();
        let status_line: &[u8] = lines.next().unwrap_or(&[]);
        let status_parts: Vec<&[u8]> = crate::request::parser::r#fn::split_whitespace(status_line);
        let version: HttpVersion = status_parts
            .first()
            .and_then(|part: &&[u8]| from_utf8(part).ok())
            .and_then(|version_str: &str| version_str.parse::<HttpVersion>().ok())
            .unwrap_or_default();
        let status_code: ResponseStatusCode = status_parts
            .get(1)
            .and_then(|part: &&[u8]| from_utf8(part).ok())
            .and_then(|code_str: &str| code_str.parse().ok())
            .unwrap_or(HttpStatus::Unknown.code());
        let reason_phrase: String = status_parts.get(2..).map_or_else(
            || HttpStatus::Unknown.to_string(),
            |parts: &[&[u8]]| {
                if parts.is_empty() {
                    HttpStatus::Unknown.to_string()
                } else if parts.len() == 1 {
                    String::from_utf8_lossy(parts[0]).into_owned()
                } else {
                    let total_len: usize =
                        parts.iter().map(|p: &&[u8]| p.len()).sum::<usize>() + parts.len() - 1;
                    let mut result: String = String::with_capacity(total_len);
                    for (i, part) in parts.iter().enumerate() {
                        if i > 0 {
                            result.push(' ');
                        }
                        result.push_str(&String::from_utf8_lossy(part));
                    }
                    result
                }
            },
        );
        let mut headers: HttpResponseHeaders = new_response_headers();
        for line in lines.by_ref() {
            if line.is_empty() {
                break;
            }
            let mut colon_pos: Option<usize> = None;
            for (i, &byte) in line.iter().enumerate() {
                if byte == COLON_U8 {
                    colon_pos = Some(i);
                    break;
                }
            }
            if let Some(pos) = colon_pos
                && pos > 0
                && pos + 1 < line.len()
            {
                let key_bytes: &[u8] = &line[..pos];
                let value_start: usize = if line.get(pos + 1) == Some(&SPACE_U8) {
                    pos + 2
                } else {
                    pos + 1
                };
                let value_bytes: &[u8] = &line[value_start..];
                if let (Ok(key_str), Ok(value_str)) = (from_utf8(key_bytes), from_utf8(value_bytes))
                {
                    headers.insert(
                        key_str.trim().to_ascii_lowercase(),
                        value_str.trim().to_owned(),
                    );
                }
            }
        }
        let body: ResponseBody = match lines.len() {
            0 => ResponseBody::new(),
            1 => {
                let line: &[u8] = lines.next().unwrap_or(&[]);
                line.to_vec()
            }
            _ => {
                let lines_slice: &[&[u8]] = lines.as_slice();
                let total_size: usize = lines_slice
                    .iter()
                    .map(|line: &&[u8]| line.len())
                    .sum::<usize>()
                    + lines_slice.len().saturating_sub(1) * BR_BYTES.len();
                let mut body: ResponseBody = ResponseBody::with_capacity(total_size);
                let mut first: bool = true;
                for line in lines {
                    if !first {
                        body.extend_from_slice(BR_BYTES);
                    }
                    body.extend_from_slice(line);
                    first = false;
                }
                body
            }
        };
        HttpResponse {
            version,
            status_code,
            reason_phrase,
            headers,
            body,
        }
    }

    /// Was the response a 2xx success?
    pub fn is_success(&self) -> bool {
        (200..300).contains(&self.status_code)
    }

    /// Was the response a 3xx redirect?
    pub fn is_redirect(&self) -> bool {
        (300..400).contains(&self.status_code)
    }

    /// Look up a single header value (case-insensitive).
    pub fn get_header<K: AsRef<str>>(&self, key: K) -> Option<&str> {
        let normalized = key.as_ref().to_ascii_lowercase();
        self.headers.get(&normalized).map(String::as_str)
    }

    /// Get the response body as a UTF-8 string (lossy on invalid bytes).
    pub fn text(&self) -> String {
        String::from_utf8_lossy(&self.body).into_owned()
    }

    /// Get the response body as raw bytes.
    pub fn bytes(&self) -> &[u8] {
        &self.body
    }

    /// Decode the body using the headers' Content-Encoding (gzip / deflate /
    /// br). Returns a fresh response with the decoded body — the original is
    /// untouched. `buffer_size` controls the chunk size for streaming decoders.
    pub fn decode(&self, buffer_size: usize) -> HttpResponse {
        let flat_headers: HttpResponseHeaders = self.headers.clone();
        let decoded: ResponseBody = Compress::from(&flat_headers)
            .decode(&self.body, buffer_size)
            .into_owned();
        HttpResponse {
            version: self.version.clone(),
            status_code: self.status_code,
            reason_phrase: self.reason_phrase.clone(),
            headers: self.headers.clone(),
            body: decoded,
        }
    }
}
