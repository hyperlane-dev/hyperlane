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
        Self {
            version: HttpVersion::Http1_1,
            status_code: HttpStatus::Ok.code(),
            reason_phrase: OK.to_owned(),
            headers: hash_map_xx_hash3_64(),
            body: Vec::new(),
        }
    }
}

/// Inherent implementation of [`Response`].
impl Response {
    /// Sets the response reason phrase while reusing the existing string capacity.
    ///
    /// # Arguments
    ///
    /// - `impl AsRef<str>` - The reason phrase text.
    ///
    /// # Returns
    ///
    /// - `&mut Self` - A mutable reference to self for chaining.
    #[inline(always)]
    pub fn set_reason_phrase(&mut self, val: impl AsRef<str>) -> &mut Self {
        let text: &str = val.as_ref();
        self.reason_phrase.clear();
        self.reason_phrase.push_str(text);
        self
    }

    /// Sets the response body while reusing the existing byte vector capacity.
    ///
    /// # Arguments
    ///
    /// - `impl AsRef<[u8]>` - The response body bytes.
    ///
    /// # Returns
    ///
    /// - `&mut Self` - A mutable reference to self for chaining.
    #[inline(always)]
    pub fn set_body(&mut self, val: impl AsRef<[u8]>) -> &mut Self {
        let bytes: &[u8] = val.as_ref();
        self.body.clear();
        self.body.extend_from_slice(bytes);
        self
    }

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
        from_slice(self.get_body())
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
        return_header_entries(headers.drain());
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

    /// Removes framing fields before a header-only response is serialized.
    #[cold]
    fn clear_framing_headers(&mut self) {
        self.get_mut_headers()
            .retain(|name: &String, _: &mut ResponseHeadersValue| {
                !name.eq_ignore_ascii_case(CONTENT_LENGTH)
                    && !name.eq_ignore_ascii_case(TRANSFER_ENCODING)
            });
    }

    /// Builds the full HTTP response as a byte vector.
    ///
    /// This method constructs the complete HTTP response, including the status line,
    /// headers, and body. It handles content encoding, content type, connection
    /// management, and content length.
    /// Informational, 204, 205, and 304 responses never serialize the body.
    /// A 205 response declares a zero length; the other bodyless statuses
    /// omit framing headers. For 304, the body cannot establish the length
    /// of the selected representation.
    ///
    /// # Returns
    ///
    /// - `ResponseData` - The complete HTTP response bytes.
    #[inline]
    pub fn build(&mut self) -> ResponseData {
        Self::build_wire::<true>(self)
    }

    /// Builds an HTTP response to `HEAD` without serializing or compressing its body.
    ///
    /// Set the body to the matching `GET` representation if its length is known.
    /// An empty buffer, an event stream, or a body that would be compressed does
    /// not establish that length, so the generated response omits it.
    ///
    /// # Returns
    ///
    /// - `ResponseData` - The status line and headers ending at the header terminator.
    #[must_use]
    pub fn build_head(&mut self) -> ResponseData {
        Self::build_wire::<false>(self)
    }

    /// Serializes either a full response or the header-only response to `HEAD`.
    ///
    /// # Returns
    ///
    /// - `ResponseData` - The encoded HTTP response.
    fn build_wire<const INCLUDE_BODY: bool>(&mut self) -> ResponseData {
        let status_code: ResponseStatusCode = self.get_status_code();
        if self.get_reason_phrase().is_empty() || self.get_reason_phrase().as_str() == OK {
            // The phrase is written into the existing buffer rather than
            // replaced, so a response that announces the same status over and
            // over stops allocating for it.
            let phrase: &'static str = HttpStatus::phrase_ref(status_code);
            if self.get_reason_phrase().as_str() != phrase {
                let stored: &mut ResponseReasonPhrase = self.get_mut_reason_phrase();
                stored.clear();
                stored.push_str(phrase);
            }
        }
        if INCLUDE_BODY
            && self.get_headers().is_empty()
            && status_code >= HttpStatus::Ok.code()
            && status_code != HttpStatus::NoContent.code()
            && status_code != HttpStatus::ResetContent.code()
            && status_code != HttpStatus::NotModified.code()
        {
            let content_type: &str = default_content_type();
            let body_len: usize = self.get_body().len();
            let mut digits: [u8; B_20] = [0; B_20];
            let rendered_len: &str = render_length(body_len, &mut digits);
            let mut code_digits: [u8; B_20] = [0; B_20];
            let rendered_code: &str = render_length(status_code, &mut code_digits);
            let version_str: &str = self.get_version().as_str();
            let phrase_str: &str = self.get_reason_phrase().as_str();
            let total_size: usize = version_str.len()
                + 1
                + rendered_code.len()
                + 1
                + phrase_str.len()
                + HTTP_BR.len()
                + CONNECTION.len()
                + COLON.len()
                + KEEP_ALIVE.len()
                + HTTP_BR.len()
                + CONTENT_TYPE.len()
                + COLON.len()
                + content_type.len()
                + HTTP_BR.len()
                + CONTENT_LENGTH.len()
                + COLON.len()
                + rendered_len.len()
                + HTTP_BR.len()
                + HTTP_BR.len()
                + body_len;
            let mut response_bytes: Vec<u8> = Vec::with_capacity(total_size);
            response_bytes.extend_from_slice(version_str.as_bytes());
            response_bytes.push(b' ');
            response_bytes.extend_from_slice(rendered_code.as_bytes());
            response_bytes.push(b' ');
            response_bytes.extend_from_slice(phrase_str.as_bytes());
            response_bytes.extend_from_slice(HTTP_BR.as_bytes());
            Self::push_header(&mut response_bytes, CONTENT_LENGTH, rendered_len);
            Self::push_header(&mut response_bytes, CONNECTION, KEEP_ALIVE);
            Self::push_header(&mut response_bytes, CONTENT_TYPE, content_type);
            response_bytes.extend_from_slice(HTTP_BR.as_bytes());
            response_bytes.extend_from_slice(self.get_body().as_slice());
            self.populate_default_headers_batch(content_type, rendered_len);
            return response_bytes;
        }
        if self
            .get_headers()
            .get(CONNECTION)
            .and_then(|values: &VecDeque<String>| values.back())
            .is_none()
        {
            self.set_header_without_check(CONNECTION, KEEP_ALIVE);
        }
        let compressed_body: Option<ResponseBody>;
        let body: &[u8] = match status_code {
            100..=199 | 204 | 205 | 304 => {
                self.clear_framing_headers();
                if status_code == HttpStatus::ResetContent.code() {
                    self.set_header_without_check(CONTENT_LENGTH, ZERO_STR);
                }
                &[]
            }
            _ => {
                let content_type: Option<&String> = self
                    .get_headers()
                    .get(CONTENT_TYPE)
                    .and_then(|values: &ResponseHeadersValue| values.back());
                let is_event_stream: bool = if let Some(value) = content_type {
                    value.eq_ignore_ascii_case(TEXT_EVENT_STREAM)
                } else {
                    self.set_header_without_check(CONTENT_TYPE, default_content_type());
                    false
                };
                let compress_type: Option<Compress> = self
                    .get_headers()
                    .get(CONTENT_ENCODING)
                    .and_then(|values: &ResponseHeadersValue| values.back())
                    .map(|value: &String| value.parse::<Compress>().unwrap_or_default());
                if INCLUDE_BODY {
                    compressed_body = match compress_type {
                        Some(encoding) if !encoding.is_unknown() => Some(
                            encoding
                                .encode(self.get_body(), DEFAULT_BUFFER_SIZE)
                                .into_owned(),
                        ),
                        _ => None,
                    };
                    if !is_event_stream {
                        let body_len: usize = compressed_body
                            .as_ref()
                            .map_or_else(|| self.get_body().len(), Vec::len);
                        self.set_wire_content_length(body_len);
                    }
                    compressed_body
                        .as_deref()
                        .unwrap_or_else(|| self.get_body().as_slice())
                } else {
                    self.clear_framing_headers();
                    let body_len: usize = self.get_body().len();
                    if body_len != 0
                        && !is_event_stream
                        && !compress_type.is_some_and(|encoding: Compress| !encoding.is_unknown())
                    {
                        self.set_wire_content_length(body_len);
                    }
                    &[]
                }
            }
        };
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
        let mut response_bytes: Vec<u8> = Vec::with_capacity(head_size + body.len());
        response_bytes.extend_from_slice(self.get_version().as_str().as_bytes());
        response_bytes.push(b' ');
        let mut code: [u8; B_20] = [0; B_20];
        response_bytes.extend_from_slice(render_length(status_code, &mut code).as_bytes());
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
        response_bytes.extend_from_slice(body);
        response_bytes
    }

    /// Populates the three default wire headers using one thread-local pool lookup.
    ///
    /// # Arguments
    ///
    /// - `&str` - The default content type value.
    /// - `&str` - The decimal body length string.
    fn populate_default_headers_batch(&mut self, content_type: &str, rendered_len: &str) {
        let headers: &mut ResponseHeaders = self.get_mut_headers();
        let pairs: [(&str, &str); 3] = [
            (CONNECTION, KEEP_ALIVE),
            (CONTENT_TYPE, content_type),
            (CONTENT_LENGTH, rendered_len),
        ];
        HEADER_ENTRY_POOL.with(|pool: &RefCell<Vec<(String, VecDeque<String>)>>| {
            let mut pool: RefMut<'_, Vec<(String, VecDeque<String>)>> = pool.borrow_mut();
            for (key, value) in pairs {
                let (mut name, mut values): (String, VecDeque<String>) = pool
                    .pop()
                    .unwrap_or_else(|| (String::new(), VecDeque::new()));
                name.clear();
                name.push_str(key);
                while values.len() > 1 {
                    values.pop_back();
                }
                if let Some(slot) = values.front_mut() {
                    slot.clear();
                    slot.push_str(value);
                } else {
                    values.push_back(value.to_owned());
                }
                headers.insert(name, values);
            }
        });
    }

    /// Sets the generated content length using a stack buffer.
    ///
    /// # Arguments
    ///
    /// - `usize` - The selected representation's length in bytes.
    #[inline(always)]
    fn set_wire_content_length(&mut self, body_len: usize) {
        let mut digits: [u8; B_20] = [0; B_20];
        let rendered: &str = render_length(body_len, &mut digits);
        self.set_header_without_check(CONTENT_LENGTH, rendered);
    }
}
