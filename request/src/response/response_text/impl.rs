use super::*;

/// Implements the `ResponseTrait` trait for `HttpResponseText`.
///
/// This implementation allows `HttpResponseText` to convert between text and binary
/// representations of HTTP responses. It provides methods for parsing raw responses, as well
/// as accessing text and binary formats.
///
/// # Associated Types
/// - `OutputText`: Specifies the text representation of an HTTP response (`HttpResponseText`).
/// - `OutputBinary`: Specifies the binary representation of an HTTP response (`HttpResponseBinary`).
impl ResponseTrait for HttpResponseText {
    type OutputText = HttpResponseText;
    type OutputBinary = HttpResponseBinary;

    /// Creates a new HttpResponseText from raw response bytes.
    ///
    /// # Arguments
    ///
    /// - `&[u8]` - The raw HTTP response bytes.
    ///
    /// # Returns
    ///
    /// - `Self::OutputText` - The parsed HttpResponseText.
    fn from(response: &[u8]) -> Self::OutputText
    where
        Self: Sized,
    {
        <HttpResponseBinary as ResponseTrait>::from(response).text()
    }

    /// Converts the response to text format.
    ///
    /// # Returns
    ///
    /// - `Self::OutputText` - The text representation of the response.
    fn text(&self) -> Self::OutputText {
        self.clone()
    }

    /// Converts the response to binary format.
    ///
    /// # Returns
    ///
    /// - `HttpResponseBinary` - The binary representation of the response.
    fn binary(&self) -> HttpResponseBinary {
        let body: Vec<u8> = self
            .body
            .read()
            .map_or(Vec::new(), |body| body.clone().into_bytes());
        HttpResponseBinary {
            http_version: self.http_version.clone(),
            status_code: self.status_code,
            status_text: self.status_text.clone(),
            headers: self.headers.clone(),
            body: Arc::new(RwLock::new(body)),
        }
    }

    /// Decodes the response body using the specified buffer size.
    ///
    /// # Arguments
    ///
    /// - `usize` - The buffer size for decoding.
    ///
    /// # Returns
    ///
    /// - `HttpResponseBinary` - The decoded binary response.
    fn decode(&self, buffer_size: usize) -> HttpResponseBinary {
        let http_response: HttpResponseText = self.clone();
        let tmp_body: Vec<u8> = self
            .body
            .read()
            .map_or(Vec::new(), |body| body.as_bytes().to_vec())
            .to_vec();
        let headers: HashMapXxHash3_64<String, String> =
            self.headers
                .read()
                .map_or(hash_map_xx_hash3_64(), |headers_ref| {
                    let mut string_headers: HashMapXxHash3_64<String, String> =
                        hash_map_xx_hash3_64();
                    for (key, value_deque) in headers_ref.iter() {
                        if let Some(first_value) = value_deque.front() {
                            string_headers.insert(key.clone(), first_value.clone());
                        }
                    }
                    string_headers
                });
        let body: Vec<u8> = Compress::from(&headers)
            .decode(&tmp_body, buffer_size)
            .into_owned();
        HttpResponseBinary {
            http_version: http_response.http_version,
            status_code: http_response.status_code,
            status_text: http_response.status_text,
            headers: http_response.headers,
            body: Arc::new(RwLock::new(body)),
        }
    }
}

impl HttpResponseText {
    /// Retrieves the HTTP version associated with this response.
    ///
    /// # Returns
    /// - `HttpVersion`: The HTTP version used for the response.
    ///
    /// Gets the HTTP version of the response.
    ///
    /// # Returns
    ///
    /// - `HttpVersion` - The HTTP version.
    pub fn get_http_version(&self) -> HttpVersion {
        if let Ok(http_version) = self.http_version.read() {
            return http_version
                .to_string()
                .parse::<HttpVersion>()
                .unwrap_or_default();
        }
        HttpVersion::default()
    }

    /// Retrieves the HTTP status code associated with this response.
    ///
    /// # Returns
    /// - `ResponseStatusCode`: The HTTP status code as a usize.
    ///
    /// Gets the HTTP status code of the response.
    ///
    /// # Returns
    ///
    /// - `ResponseStatusCode` - The status code.
    pub fn get_status_code(&self) -> ResponseStatusCode {
        self.status_code
    }

    /// Retrieves the status text associated with the HTTP status code.
    ///
    /// # Returns
    /// - `String`: The human-readable status text.
    ///
    /// Gets the HTTP status text of the response.
    ///
    /// # Returns
    ///
    /// - `String` - The status text.
    pub fn get_status_text(&self) -> String {
        if let Ok(status_text) = self.status_text.read() {
            return status_text.to_string();
        }
        HttpStatus::Unknown.to_string()
    }

    /// Retrieves the headers of the HTTP response.
    ///
    /// # Returns
    /// - `ResponseHeaders`: A map of header names and their corresponding values as key-value pairs.
    ///
    /// Gets the HTTP response headers.
    ///
    /// # Returns
    ///
    /// - `ResponseHeaders` - The response headers.
    pub fn get_headers(&self) -> ResponseHeaders {
        if let Ok(headers) = self.headers.read() {
            return headers.clone();
        }
        hash_map_xx_hash3_64()
    }

    /// Retrieves the body content of the HTTP response as a `String`.
    ///
    /// This method attempts to read the body of the response. If the body can be successfully read,
    /// it is converted into a `String` and returned. If reading the body fails, an empty string is returned.
    ///
    /// # Returns
    /// - `RequestBodyString`: The body of the response as a string. If the body could not be read,
    ///   an empty string is returned.
    ///
    /// Gets the HTTP response body.
    ///
    /// # Returns
    ///
    /// - `RequestBodyString` - The response body.
    pub fn get_body(&self) -> RequestBodyString {
        if let Ok(body) = self.body.read() {
            return body.to_string();
        }
        RequestBodyString::new()
    }
}

/// Default implementation for HttpResponseText.
///
/// # Returns
///
/// - `HttpResponseText` - Default initialized HttpResponseText.
impl Default for HttpResponseText {
    #[inline(always)]
    fn default() -> Self {
        Self {
            http_version: Arc::new(RwLock::new(HttpVersion::Unknown(String::new()))),
            status_code: HttpStatus::Unknown.code(),
            status_text: Arc::new(RwLock::new(HttpStatus::Unknown.to_string())),
            headers: Arc::new(RwLock::new(hash_map_xx_hash3_64())),
            body: Arc::new(RwLock::new(String::new())),
        }
    }
}
