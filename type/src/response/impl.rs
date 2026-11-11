use super::*;

/// Implements the `std::error::Error` trait for `ResponseError`.
/// This allows `ResponseError` to be treated as a standard Rust error type.
impl Error for ResponseError {}

/// Converts an I/O error to a `ResponseError`.
///
/// Maps I/O errors to `Send` variant with the error message.
impl From<io::Error> for ResponseError {
    /// Converts an I/O error to a `ResponseError`.
    ///
    /// # Arguments
    ///
    /// - `io::Error` - The I/O error to convert.
    ///
    /// # Returns
    ///
    /// - `ResponseError` - The corresponding response error as `Send`.
    #[inline(always)]
    fn from(error: io::Error) -> Self {
        ResponseError::Send(error.to_string())
    }
}

/// Implements the `Display` trait for `ResponseError`.
/// This allows `ResponseError` variants to be formatted into human-readable strings.
impl Display for ResponseError {
    /// Formats the `ResponseError` variant into a human-readable string.
    ///
    /// # Arguments
    ///
    /// - `&mut Formatter<'_>` - The formatter used for writing the formatted string.
    ///
    /// # Returns
    ///
    /// A `fmt::Result` indicating whether the formatting was successful.
    #[inline(always)]
    fn fmt(&self, data: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotFoundStream => {
                write!(data, "Not found stream")
            }
            Self::ConnectionClosed => {
                write!(data, "Connection has been closed")
            }
            Self::Terminated => {
                write!(data, "Current processing has been terminated")
            }
            Self::Send(error) => write!(data, "Send error{COLON_SPACE}{error}"),
            Self::FlushError(error) => write!(data, "Flush error{COLON_SPACE}{error}"),
            Self::Unknown => write!(data, "Unknown error"),
        }
    }
}

/// Provides a default value for `Response`.
///
/// Returns a new `Response` instance with all fields initialized to their default values.
impl Default for Response {
    /// Builds the default `Response` with all fields set to their defaults.
    #[inline(always)]
    fn default() -> Self {
        let http_status: HttpStatus = HttpStatus::default();
        Self {
            version: HttpVersion::Http1_1,
            status_code: http_status.code(),
            reason_phrase: http_status.to_string(),
            headers: hash_map_xx_hash3_64(),
            body: Vec::new(),
        }
    }
}

/// Inherent implementation of [`Response`].
impl Response {
    /// Pushes a header with a key and value as_ref the response bytes.
    ///
    /// # Arguments
    ///
    /// - `&mut Vec<u8>` - A mutable reference to the bytes where the header will be added.
    /// - `&str` - The header key as a string slice (`&str`).
    /// - `&str` - The header value as a string slice (`&str`).
    #[inline(always)]
    fn push_header(response_bytes: &mut Vec<u8>, key: &str, value: &str) {
        response_bytes.extend_from_slice(key.as_bytes());
        response_bytes.extend_from_slice(COLON.as_bytes());
        response_bytes.extend_from_slice(value.as_bytes());
        response_bytes.extend_from_slice(HTTP_BR.as_bytes());
    }

    /// Tries to retrieve the value of a response header by its key.
    ///
    /// # Arguments
    ///
    /// - `K` - The header's key, which must implement `AsRef<str>`.
    ///
    /// # Returns
    ///
    /// - `Option<ResponseHeadersValue>` - The optional header values.
    #[inline(always)]
    pub fn try_get_header<K>(&self, key: K) -> Option<ResponseHeadersValue>
    where
        K: AsRef<str>,
    {
        self.headers.get(key.as_ref()).cloned()
    }

    /// Retrieves the value of a response header by its key.
    ///
    /// # Arguments
    ///
    /// - `K` - The header's key, which must implement `AsRef<str>`.
    ///
    /// # Returns
    ///
    /// - `ResponseHeadersValue` - The optional header values.
    ///
    /// # Panics
    ///
    /// This function will panic if the header key is not found.
    #[inline(always)]
    pub fn get_header<K>(&self, key: K) -> ResponseHeadersValue
    where
        K: AsRef<str>,
    {
        self.try_get_header(key).unwrap()
    }

    /// Tries to retrieve the first value of a response header by its key.
    ///
    /// # Arguments
    ///
    /// - `K` - The header's key, which must implement `AsRef<str>`.
    ///
    /// # Returns
    ///
    /// - `Option<ResponseHeadersValueItem>` - The first header value if exists.
    #[inline(always)]
    pub fn try_get_header_front<K>(&self, key: K) -> Option<ResponseHeadersValueItem>
    where
        K: AsRef<str>,
    {
        self.headers
            .get(key.as_ref())
            .and_then(|data: &VecDeque<String>| data.front().cloned())
    }

    /// Retrieves the first value of a response header by its key.
    ///
    /// # Arguments
    ///
    /// - `K` - The header's key, which must implement `AsRef<str>`.
    ///
    /// # Returns
    ///
    /// - `ResponseHeadersValueItem` - The first header value if exists.
    ///
    /// # Panics
    ///
    /// This function will panic if the header key is not found.
    #[inline(always)]
    pub fn get_header_front<K>(&self, key: K) -> ResponseHeadersValueItem
    where
        K: AsRef<str>,
    {
        self.try_get_header_front(key).unwrap()
    }

    /// Tries to retrieve the last value of a response header by its key.
    ///
    /// # Arguments
    ///
    /// - `K` - The header's key, which must implement `AsRef<str>`.
    ///
    /// # Returns
    ///
    /// - `Option<ResponseHeadersValueItem>` - The last header value if exists.
    #[inline(always)]
    pub fn try_get_header_back<K>(&self, key: K) -> Option<ResponseHeadersValueItem>
    where
        K: AsRef<str>,
    {
        self.headers
            .get(key.as_ref())
            .and_then(|data: &VecDeque<String>| data.back().cloned())
    }

    /// Retrieves the last value of a response header by its key.
    ///
    /// # Arguments
    ///
    /// - `K` - The header's key, which must implement `AsRef<str>`.
    ///
    /// # Returns
    ///
    /// - `ResponseHeadersValueItem` - The last header value if exists.
    ///
    /// # Panics
    ///
    /// This function will panic if the header key is not found.
    #[inline(always)]
    pub fn get_header_back<K>(&self, key: K) -> ResponseHeadersValueItem
    where
        K: AsRef<str>,
    {
        self.try_get_header_back(key).unwrap()
    }

    /// Checks if a header exists in the response.
    ///
    /// # Arguments
    ///
    /// - `K` - The header key to check, which must implement `AsRef<str>`.
    ///
    /// # Returns
    ///
    /// - `bool` - Whether the header exists.
    #[inline(always)]
    pub fn has_header<K>(&self, key: K) -> bool
    where
        K: AsRef<str>,
    {
        self.get_headers().contains_key(key.as_ref())
    }

    /// Checks if a header contains a specific value.
    ///
    /// # Arguments
    ///
    /// - `K` - The header key to check, which must implement `AsRef<str>`.
    /// - `V` - The value to search for, which must implement `AsRef<str>`.
    ///
    /// # Returns
    ///
    /// - `bool` - Whether the header contains the value.
    #[inline(always)]
    pub fn has_header_value<K, V>(&self, key: K, value: V) -> bool
    where
        K: AsRef<str>,
        V: AsRef<str>,
    {
        if let Some(values) = self.get_headers().get(key.as_ref()) {
            values.contains(&value.as_ref().to_owned())
        } else {
            false
        }
    }

    /// Gets the number of headers in the response.
    ///
    /// # Returns
    ///
    /// - `usize` - The count of unique header keys.
    #[inline(always)]
    pub fn get_headers_size(&self) -> usize {
        self.headers.len()
    }

    /// Tries to get the number of values for a specific header key.
    ///
    /// # Arguments
    ///
    /// - `K` - The header key to count, which must implement `AsRef<str>`.
    ///
    /// # Returns
    ///
    /// - `Option<usize>` - The count of values for the header.
    #[inline(always)]
    pub fn try_get_header_size<K>(&self, key: K) -> Option<usize>
    where
        K: AsRef<str>,
    {
        self.headers
            .get(key.as_ref())
            .map(|data: &VecDeque<String>| data.len())
    }

    /// Gets the number of values for a specific header key.
    ///
    /// # Arguments
    ///
    /// - `K` - The header key to count, which must implement `AsRef<str>`.
    ///
    /// # Returns
    ///
    /// - `usize` - The count of values for the header.
    ///
    /// # Panics
    ///
    /// This function will panic if the header key is not found.
    #[inline(always)]
    pub fn get_header_size<K>(&self, key: K) -> usize
    where
        K: AsRef<str>,
    {
        self.try_get_header_size(key).unwrap()
    }

    /// Gets the total number of header values in the response.
    ///
    /// This counts all values across all headers, so a header with multiple values
    /// will contribute more than one to the total count.
    ///
    /// # Returns
    ///
    /// - `usize` - The total count of all header values.
    #[inline(always)]
    pub fn get_headers_values_size(&self) -> usize {
        self.headers
            .values()
            .map(|data: &VecDeque<String>| data.len())
            .sum()
    }

    /// Retrieves the body content of the response as a UTF-8 encoded string.
    ///
    /// This method uses `String::from_utf8_lossy` to convert the byte slice returned by `self.get_body()` as_ref a string.
    /// If the byte slice contains invalid UTF-8 sequences, they will be replaced with the Unicode replacement character ().
    ///
    /// # Returns
    ///
    /// - `String` - The body content as a string.
    #[inline(always)]
    pub fn get_body_string(&self) -> String {
        String::from_utf8_lossy(self.get_body()).into_owned()
    }

    /// Deserializes the body content of the response as_ref a specified type `T`.
    ///
    /// This method first retrieves the body content as a byte slice using `self.get_body()`.
    /// It then attempts to deserialize the byte slice as_ref the specified type `T` using `json_from_slice`.
    ///
    /// The target type `T` is a type parameter of this method and must implement `DeserializeOwned`.
    ///
    /// # Returns
    ///
    /// - `Result<T, serde_json::Error>` - The deserialization result.
    pub fn try_get_body_json<T>(&self) -> Result<T, serde_json::Error>
    where
        T: DeserializeOwned,
    {
        serde_json::from_slice(self.get_body())
    }

    /// Deserializes the body content of the response as_ref a specified type `T`.
    ///
    /// This method first retrieves the body content as a byte slice using `self.get_body()`.
    /// It then attempts to deserialize the byte slice as_ref the specified type `T` using `json_from_slice`.
    ///
    /// The target type `T` is a type parameter of this method and must implement `DeserializeOwned`.
    ///
    /// # Returns
    ///
    /// - `T` - The deserialized body content.
    ///
    /// # Panics
    ///
    /// This function will panic if the deserialization fails.
    pub fn get_body_json<T>(&self) -> T
    where
        T: DeserializeOwned,
    {
        self.try_get_body_json().unwrap()
    }

    /// Determines whether the header should be skipped during setting.
    ///
    /// # Arguments
    ///
    /// - `&str` - The header key to inspect.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` if the header is empty or not allowed, `false` if the header can be set.
    #[inline(always)]
    fn should_skip_header(&self, key: &str) -> bool {
        key.trim().is_empty() || key == CONTENT_LENGTH
    }

    /// Sets a header in the response, replacing any existing values.
    ///
    /// This function replaces all existing values for a header with a single new value.
    ///
    /// # Arguments
    ///
    /// - `K` - The header key, which must implement `AsRef<str>`.
    /// - `V` - The header value, which must implement `AsRef<String>`.
    ///
    /// # Returns
    ///
    /// - `&mut Self` - A mutable reference to self for chaining.
    #[inline(always)]
    fn set_header_without_check<K, V>(&mut self, key: K, value: V) -> &mut Self
    where
        K: AsRef<str>,
        V: AsRef<str>,
    {
        let key: &str = key.as_ref();
        let value: &str = value.as_ref();
        if let Some(values) = self.get_mut_headers().get_mut(key) {
            values.truncate(1);
            if let Some(slot) = values.front_mut() {
                slot.clear();
                slot.push_str(value);
            } else {
                values.push_back(value.to_owned());
            }
            return self;
        }
        // A recycled entry carries the buffers of the name and of the value,
        // so a response that sets the same headers over and over stops
        // allocating for them.
        let (mut name, mut values): (String, VecDeque<String>) =
            take_header_entry().unwrap_or_else(|| (String::new(), VecDeque::new()));
        name.clear();
        name.push_str(key);
        // A recycled entry may carry the values of a header that was sent
        // more than once. Those values belong to that header, so only the
        // first one is kept and the rest are dropped.
        while values.len() > 1 {
            values.pop_back();
        }
        if values.is_empty() {
            values.push_back(String::new());
        }
        if let Some(slot) = values.front_mut() {
            slot.clear();
            slot.push_str(value.as_ref());
        }
        self.get_mut_headers().insert(name, values);
        self
    }

    /// Sets a header in the response, replacing any existing values.
    ///
    /// This function replaces all existing values for a header with a single new value.
    ///
    /// # Arguments
    ///
    /// - `K` - The header key, which must implement `AsRef<str>`.
    /// - `V` - The header value, which must implement `AsRef<String>`.
    ///
    /// # Returns
    ///
    /// - `&mut Self` - A mutable reference to self for chaining.
    #[inline(always)]
    pub fn set_header<K, V>(&mut self, key: K, value: V) -> &mut Self
    where
        K: AsRef<str>,
        V: AsRef<str>,
    {
        let key: &str = key.as_ref();
        if self.should_skip_header(key) {
            return self;
        }
        self.set_header_without_check(key, value)
    }

    /// Adds a header to the response.
    ///
    /// This function appends a value to the response headers.
    /// If the header already exists, the new value will be added to the existing values.
    ///
    /// # Arguments
    ///
    /// - `K` - The header key, which must implement `AsRef<str>`.
    /// - `V` - The header value, which must implement `AsRef<String>`.
    ///
    /// # Returns
    ///
    /// - `&mut Self` - A mutable reference to self for chaining.
    #[inline(always)]
    pub fn add_header<K, V>(&mut self, key: K, value: V) -> &mut Self
    where
        K: AsRef<str>,
        V: AsRef<str>,
    {
        let key: ResponseHeadersKey = key.as_ref().to_owned();
        if self.should_skip_header(&key) {
            return self;
        }
        self.get_mut_headers()
            .entry(key)
            .or_default()
            .push_back(value.as_ref().to_owned());
        self
    }

    /// Removes a header from the response.
    ///
    /// This function removes all values for the specified header key.
    ///
    /// # Arguments
    ///
    /// - `K` - The header key to remove, which must implement `AsRef<str>`.
    ///
    /// # Returns
    ///
    /// - `&mut Self` - A mutable reference to self for chaining.
    #[inline(always)]
    pub fn remove_header<K>(&mut self, key: K) -> &mut Self
    where
        K: AsRef<str>,
    {
        let _: bool = self.get_mut_headers().remove(key.as_ref()).is_some();
        self
    }

    /// Removes a specific value from a header in the response.
    ///
    /// This function removes only the specified value from the header.
    /// If the header has multiple values, only the matching value is removed.
    /// If this was the last value for the header, the entire header is removed.
    ///
    /// # Arguments
    ///
    /// - `K` - The header key, which must implement `AsRef<str>`.
    /// - `V` - The value to remove, which must implement `AsRef<String>`.
    ///
    /// # Returns
    ///
    /// - `&mut Self` - A mutable reference to self for chaining.
    #[inline(always)]
    pub fn remove_header_value<K, V>(&mut self, key: K, value: V) -> &mut Self
    where
        K: AsRef<str>,
        V: AsRef<str>,
    {
        let key: ResponseHeadersKey = key.as_ref().to_owned();
        if let Some(values) = self.get_mut_headers().get_mut(&key) {
            values.retain(|data: &String| data != &value.as_ref().to_owned());
            if values.is_empty() {
                self.get_mut_headers().remove(&key);
            }
        }
        self
    }

    /// Clears all headers from the response.
    ///
    /// This function removes all headers, leaving the headers map empty.
    ///
    /// # Returns
    ///
    /// - `&mut Self` - A mutable reference to self for chaining.
    #[inline(always)]
    pub fn clear_headers(&mut self) -> &mut Self {
        self.get_mut_headers().clear();
        self
    }

    /// Resets the response to its default state while retaining allocated capacity.
    ///
    /// This keeps the header map and body allocations so persistent
    /// (keep-alive) connections avoid repeated allocation per request.
    ///
    /// # Returns
    ///
    /// - `&mut Self` - A mutable reference to self for chaining.
    pub fn reset(&mut self) -> &mut Self {
        let http_status: HttpStatus = HttpStatus::default();
        self.set_status_code(http_status.code());
        // The phrase is written into the existing buffer rather than
        // replaced, so a response that is reset over and over stops
        // allocating for it.
        let stored: &mut ResponseReasonPhrase = self.get_mut_reason_phrase();
        stored.clear();
        stored.push_str(HttpStatus::phrase_ref(http_status.code()));
        self.recycle_headers();
        self.get_mut_body().clear();
        self
    }

    /// Hands the header entries to the pool and empties the header map.
    ///
    /// The map ends up empty, exactly as clearing it would, but the name and
    /// the value buffers of every header survive in the pool. The next
    /// response reuses them instead of allocating new ones.
    ///
    /// # Returns
    ///
    /// - `()` - The headers have been recycled.
    fn recycle_headers(&mut self) {
        let headers: &mut ResponseHeaders = self.get_mut_headers();
        if headers.is_empty() {
            return;
        }
        for (key, values) in headers.drain() {
            return_header_entry((key, values));
        }
    }

    /// Tries to parse cookies from the `Set-Cookie` header.
    ///
    /// This method retrieves the last `Set-Cookie` header value and parses it
    /// into a collection of key-value pairs representing the cookies.
    ///
    /// # Returns
    ///
    /// - `Option<Cookies>` - The parsed cookies if the `Set-Cookie` header exists, otherwise `None`.
    #[inline(always)]
    pub fn try_get_cookies(&self) -> Option<Cookies> {
        self.try_get_header_back(SET_COOKIE)
            .map(|cookie_header: String| Cookie::parse(cookie_header))
    }

    /// Parses cookies from the `Set-Cookie` headers.
    ///
    /// This method retrieves all `Set-Cookie` header values and parses each one
    /// into a collection of key-value pairs representing the cookies.
    ///
    /// # Returns
    ///
    /// - `Cookies` - The parsed cookies.
    ///
    /// # Panics
    ///
    /// This function will panic if the `Set-Cookie` header is not found.
    #[inline(always)]
    pub fn get_cookies(&self) -> Cookies {
        self.try_get_cookies().unwrap()
    }

    /// Tries to get a cookie value by its key from `Set-Cookie` headers.
    ///
    /// This method parses the cookies from all `Set-Cookie` headers,
    /// then attempts to retrieve the value for the specified key.
    ///
    /// # Arguments
    ///
    /// - `K` - The cookie key, which implements `AsRef<str>`.
    ///
    /// # Returns
    ///
    /// - `Option<CookieValue>` - The cookie value if exists.
    #[inline(always)]
    pub fn try_get_cookie<K>(&self, key: K) -> Option<CookieValue>
    where
        K: AsRef<str>,
    {
        self.try_get_cookies()
            .and_then(|cookies: Cookies| cookies.get(key.as_ref()).cloned())
    }

    /// Gets a cookie value by its key from `Set-Cookie` headers.
    ///
    /// This method parses the cookies from all `Set-Cookie` headers,
    /// then retrieves the value for the specified key.
    ///
    /// # Arguments
    ///
    /// - `K` - The cookie key, which implements `AsRef<str>`.
    ///
    /// # Returns
    ///
    /// - `CookieValue` - The cookie value.
    ///
    /// # Panics
    ///
    /// This function will panic if the `Set-Cookie` header is not found
    /// or the cookie key does not exist.
    #[inline(always)]
    pub fn get_cookie<K>(&self, key: K) -> CookieValue
    where
        K: AsRef<str>,
    {
        self.try_get_cookie(key).unwrap()
    }

    /// Builds the full HTTP response as a byte vector.
    ///
    /// This method constructs the complete HTTP response, including the status line,
    /// headers, and body. It handles content encoding, content type, connection
    /// management, and content length.
    ///
    /// # Returns
    ///
    /// - `ResponseData` - The complete HTTP response bytes.
    pub fn build(&mut self) -> ResponseData {
        if self.get_reason_phrase().is_empty() || self.get_reason_phrase().as_str() == OK {
            // The phrase is written into the existing buffer rather than
            // replaced, so a response that announces the same status over and
            // over stops allocating for it.
            let phrase: &'static str = HttpStatus::phrase_ref(self.get_status_code());
            let stored: &mut ResponseReasonPhrase = self.get_mut_reason_phrase();
            stored.clear();
            stored.push_str(phrase);
        }
        let compress_type_opt: Option<Compress> = self
            .get_headers()
            .get(CONTENT_ENCODING)
            .and_then(|values: &VecDeque<String>| values.back())
            .map(|data: &String| data.parse::<Compress>().unwrap_or_default());
        if self
            .get_headers()
            .get(CONNECTION)
            .and_then(|values: &VecDeque<String>| values.back())
            .is_none()
        {
            self.set_header_without_check(CONNECTION, KEEP_ALIVE);
        }
        if self
            .get_headers()
            .get(CONTENT_TYPE)
            .and_then(|values: &VecDeque<String>| values.back())
            .is_none()
        {
            self.set_header_without_check(CONTENT_TYPE, default_content_type());
        }
        let is_event_stream: bool = self
            .get_headers()
            .get(CONTENT_TYPE)
            .and_then(|values: &VecDeque<String>| values.back())
            .is_some_and(|value: &String| value.eq_ignore_ascii_case(TEXT_EVENT_STREAM));
        let compressed_body: Option<Vec<u8>> = match compress_type_opt {
            Some(compress_type) if !compress_type.is_unknown() => Some(
                compress_type
                    .encode(self.get_body(), DEFAULT_BUFFER_SIZE)
                    .into_owned(),
            ),
            _ => None,
        };
        let body_len: usize = compressed_body
            .as_ref()
            .map_or_else(|| self.get_body().len(), Vec::len);
        if !is_event_stream {
            // The length is rendered into a stack buffer and then copied into
            // the header's own buffer, so announcing the length allocates
            // nothing beyond what the header already holds.
            let mut digits: [u8; B_20] = [0; B_20];
            let rendered: &str = render_length(body_len, &mut digits);
            self.set_header_without_check(CONTENT_LENGTH, rendered);
        }
        let mut head_size: usize = self.get_reason_phrase().len() + B_16 + HTTP_BR.len();
        head_size += self
            .get_headers()
            .iter()
            .map(|header_entry: (&String, &VecDeque<String>)| {
                let (header_key, header_values): (&String, &VecDeque<String>) = header_entry;
                header_values
                    .iter()
                    .map(|header_value: &String| {
                        header_key.len() + COLON.len() + header_value.len() + HTTP_BR.len()
                    })
                    .sum::<usize>()
            })
            .sum::<usize>();
        let mut response_bytes: Vec<u8> = Vec::with_capacity(head_size + body_len);
        response_bytes.extend_from_slice(self.get_version().as_str().as_bytes());
        response_bytes.push(b' ');
        let mut code: [u8; B_20] = [0; B_20];
        response_bytes
            .extend_from_slice(render_length(self.get_status_code(), &mut code).as_bytes());
        response_bytes.push(b' ');
        response_bytes.extend_from_slice(self.get_reason_phrase().as_bytes());
        response_bytes.extend_from_slice(HTTP_BR.as_bytes());
        self.get_headers()
            .iter()
            .for_each(|header_entry: (&String, &VecDeque<String>)| {
                let (header_key, header_values): (&String, &VecDeque<String>) = header_entry;
                for header_value in header_values.iter() {
                    Self::push_header(&mut response_bytes, header_key, header_value);
                }
            });
        response_bytes.extend_from_slice(HTTP_BR.as_bytes());
        match &compressed_body {
            Some(body) => response_bytes.extend_from_slice(body),
            None => response_bytes.extend_from_slice(self.get_body()),
        }
        response_bytes
    }
}
