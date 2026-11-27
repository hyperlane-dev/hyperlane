use super::*;

/// Default construction for [`CompressConfig`].
impl Default for CompressConfig {
    /// Builds the configuration with the default body length threshold.
    ///
    /// # Returns
    ///
    /// - `Self` - The configuration the middleware works from by default.
    fn default() -> Self {
        Self::new(DEFAULT_MIN_LENGTH)
    }
}

/// Inherent implementation of [`CompressConfig`].
impl CompressConfig {
    /// Builds the configuration from a body length threshold.
    ///
    /// # Arguments
    ///
    /// - `usize` - The body length below which a response is answered without a
    ///   content coding.
    ///
    /// # Returns
    ///
    /// - `Self` - The configuration.
    pub fn new(min_length: usize) -> Self {
        Self { min_length }
    }

    /// The body length below which a response is answered without a content coding.
    ///
    /// # Returns
    ///
    /// - `usize` - The threshold.
    pub fn get_min_length(&self) -> usize {
        self.min_length
    }

    /// Updates the body length threshold in place.
    ///
    /// # Arguments
    ///
    /// - `usize` - New minimum response body length for compression.
    ///
    /// # Returns
    ///
    /// - `&mut Self` - Mutable reference for fluent chaining.
    pub fn set_min_length(&mut self, min_length: usize) -> &mut Self {
        self.min_length = min_length;
        self
    }

    /// Returns a copy of this configuration with an updated body length threshold.
    ///
    /// # Arguments
    ///
    /// - `usize` - New minimum response body length for compression.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated configuration.
    pub fn with_min_length(mut self, min_length: usize) -> Self {
        self.set_min_length(min_length);
        self
    }
}

/// Inherent implementation of [`CompressMiddleware`].
impl CompressMiddleware {
    /// Installs the configuration the middleware works from.
    ///
    /// The configuration is process wide, because the framework registers a
    /// middleware by its type and a hook is built per request rather than per
    /// server, so a server cannot carry a configuration of its own.
    ///
    /// # Arguments
    ///
    /// - `CompressConfig` - The configuration to work from.
    ///
    /// # Returns
    ///
    /// - `Result<(), String>` - Nothing, or the reason the configuration was refused.
    pub fn init(config: CompressConfig) -> Result<(), String> {
        CONFIG
            .set(config)
            .map_err(|_: CompressConfig| COMPRESS_CONFIG_ALREADY_SET.to_owned())
    }

    /// Returns the installed global [`CompressConfig`], when [`CompressMiddleware::init`] was called.
    ///
    /// # Returns
    ///
    /// - `Option<&'static CompressConfig>` - Installed configuration, or `None`.
    pub fn get() -> Option<&'static CompressConfig> {
        CONFIG.get()
    }

    /// The configuration the middleware works from.
    ///
    /// # Returns
    ///
    /// - `CompressConfig` - The installed configuration, or the default one.
    fn config() -> CompressConfig {
        CONFIG.get().copied().unwrap_or_default()
    }

    /// Reads the coding the client accepts best out of every `Accept-Encoding` field.
    ///
    /// A request may carry the field more than once, and every occurrence names
    /// the codings the client accepts, so the occurrences are read together.
    ///
    /// # Arguments
    ///
    /// - `&Request` - The request to read the field from.
    ///
    /// # Returns
    ///
    /// - `Option<Compress>` - The coding, or `None` when the client accepts none.
    fn negotiated_coding(request: &Request) -> Option<Compress> {
        let mut total_values: usize = 0;
        let mut total_len: usize = 0;
        let mut single: Option<&str> = None;
        for (key, values) in request.get_headers() {
            if !key.eq_ignore_ascii_case(ACCEPT_ENCODING) {
                continue;
            }
            for value in values {
                total_values += 1;
                total_len = total_len.saturating_add(value.len());
                if total_values == 1 {
                    single = Some(value.as_str());
                }
            }
        }
        if total_values == 0 {
            return None;
        }
        if total_values == 1 {
            return single.and_then(Compress::from_accept_encoding);
        }
        let mut joined: String = String::with_capacity(total_len + total_values);
        let mut first: bool = true;
        for (key, values) in request.get_headers() {
            if !key.eq_ignore_ascii_case(ACCEPT_ENCODING) {
                continue;
            }
            for value in values {
                if !first {
                    joined.push_str(COMMA);
                }
                joined.push_str(value);
                first = false;
            }
        }
        Compress::from_accept_encoding(joined)
    }
}

/// Implements [`ServerHook`] for [`CompressMiddleware`].
impl ServerHook for CompressMiddleware {
    /// Builds the middleware.
    ///
    /// # Arguments
    ///
    /// - `&mut Stream` - The stream the request arrived on.
    /// - `&mut Context` - The context of the request.
    ///
    /// # Returns
    ///
    /// - `Self` - The middleware.
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    /// Negotiates the content coding of the response the pipeline is about to send.
    ///
    /// The middleware never writes to the stream: it only declares the coding on
    /// the response, and the response builder applies the coding when the sender
    /// serializes the response. A response that already declares a coding is left
    /// alone, because coding a coded body produces bytes no client can read.
    ///
    /// # Arguments
    ///
    /// - `Self` - The middleware.
    /// - `&mut Stream` - The stream the request arrived on.
    /// - `&mut Context` - The context of the request.
    ///
    /// # Returns
    ///
    /// - `Status` - Whether the pipeline continues.
    async fn handle(self, _: &mut Stream, ctx: &mut Context) -> Status {
        let Some(coding) = Self::negotiated_coding(ctx.get_request()) else {
            return Status::Continue;
        };
        let response: &mut Response = ctx.get_mut_response();
        if has_response_header_ci(response, CONTENT_ENCODING)
            || has_no_transform(response)
            || response.get_body().len() < Self::config().get_min_length()
            || has_no_body(response.get_status_code())
            || is_event_stream(response)
        {
            return Status::Continue;
        }
        response.set_header(CONTENT_ENCODING, coding.as_str());
        vary_accept_encoding(response);
        Status::Continue
    }
}

/// Inherent implementation of [`EntityTag`].
impl EntityTag {
    /// Formats a 64-bit digest into a quoted 18-byte ASCII strong tag without `format!`.
    ///
    /// # Arguments
    ///
    /// - `u64` - 64-bit hash digest.
    ///
    /// # Returns
    ///
    /// - `[u8; 18]` - ASCII bytes `\"<16 lowercase hex digits>\"`.
    fn format_strong_bytes(digest: u64) -> [u8; 18] {
        let mut buf: [u8; 18] = [b'"'; 18];
        let mut idx: usize = 0;
        while idx < 16 {
            let shift: u32 = (15 - idx) as u32 * 4;
            let nibble: usize = ((digest >> shift) & 0x0f) as usize;
            buf[idx + 1] = HEX_LOWER[nibble];
            idx += 1;
        }
        buf
    }

    /// Builds the strong entity-tag of a representation.
    ///
    /// The tag is the quoted hexadecimal digest of the data, so the same data
    /// always produces the same tag, across processes and across restarts.
    ///
    /// # Arguments
    ///
    /// - `&[u8]` - The representation to tag.
    ///
    /// # Returns
    ///
    /// - `Self` - The entity-tag.
    pub fn strong(data: &[u8]) -> Self {
        let mut hasher: XxHash3_64 = XxHash3_64::default();
        hasher.write(data);
        let buf: [u8; 18] = Self::format_strong_bytes(hasher.finish());
        let tag: String = from_utf8(&buf).unwrap_or_default().to_owned();
        Self(tag)
    }

    /// Builds a weak entity-tag (`W/"<hex>"`) of a representation per RFC 9110.
    ///
    /// # Arguments
    ///
    /// - `&[u8]` - The representation to tag.
    ///
    /// # Returns
    ///
    /// - `Self` - The weak entity-tag.
    pub fn weak(data: &[u8]) -> Self {
        let mut hasher: XxHash3_64 = XxHash3_64::default();
        hasher.write(data);
        let buf: [u8; 18] = Self::format_strong_bytes(hasher.finish());
        let mut out: String = String::with_capacity(20);
        out.push_str(WEAK_ETAG_PREFIX);
        if let Ok(text) = from_utf8(&buf) {
            out.push_str(text);
        }
        Self(out)
    }

    /// Reports whether this entity-tag carries the weak `W/` prefix.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` when weak.
    pub fn is_weak(&self) -> bool {
        self.as_str().starts_with(WEAK_ETAG_PREFIX)
    }

    /// Returns the opaque quoted portion of the tag without any `W/` prefix.
    ///
    /// # Returns
    ///
    /// - `&str` - Opaque quoted tag slice.
    pub fn opaque_tag(&self) -> &str {
        self.as_str()
            .strip_prefix(WEAK_ETAG_PREFIX)
            .unwrap_or(self.as_str())
    }

    /// Performs RFC 9110 §8.8.3.2 weak comparison against `other`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Other entity-tag to compare.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` when opaque tags match.
    pub fn weak_eq(&self, other: &Self) -> bool {
        self.opaque_tag() == other.opaque_tag()
    }

    /// Performs RFC 9110 §8.8.3.2 strong comparison against `other`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Other entity-tag to compare.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` when both tags are strong and opaque tags match.
    pub fn strong_eq(&self, other: &Self) -> bool {
        !self.is_weak() && !other.is_weak() && self.opaque_tag() == other.opaque_tag()
    }

    /// The entity-tag as the `ETag` field carries it.
    ///
    /// # Returns
    ///
    /// - `&str` - The quoted tag.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Inherent implementation of [`ConditionalMiddleware`].
impl ConditionalMiddleware {
    /// Finds an existing `ETag` header slice on `response` without allocating.
    ///
    /// # Arguments
    ///
    /// - `&Response` - Response to inspect.
    ///
    /// # Returns
    ///
    /// - `Option<&str>` - Existing `ETag` header value if declared.
    fn existing_etag(response: &Response) -> Option<&str> {
        response
            .get_headers()
            .iter()
            .find(|(key, _): &(&String, &ResponseHeadersValue)| key.eq_ignore_ascii_case(ETAG))
            .and_then(|(_, values): (&String, &ResponseHeadersValue)| {
                values.back().map(String::as_str)
            })
    }
}

/// Implements [`ServerHook`] for [`ConditionalMiddleware`].
impl ServerHook for ConditionalMiddleware {
    /// Builds the middleware.
    ///
    /// # Arguments
    ///
    /// - `&mut Stream` - The stream the request arrived on.
    /// - `&mut Context` - The context of the request.
    ///
    /// # Returns
    ///
    /// - `Self` - The middleware.
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    /// Answers a conditional request with `304 Not Modified` while the validator matches.
    ///
    /// Only a safe method can be answered with `304`, because a request that
    /// changes state has to reach the handler whether or not the representation
    /// it addresses still matches. The body is dropped, so the response carries
    /// the status and the representation metadata alone.
    ///
    /// # Arguments
    ///
    /// - `Self` - The middleware.
    /// - `&mut Stream` - The stream the request arrived on.
    /// - `&mut Context` - The context of the request.
    ///
    /// # Returns
    ///
    /// - `Status` - Whether the pipeline continues.
    async fn handle(self, _: &mut Stream, ctx: &mut Context) -> Status {
        let method: &RequestMethod = ctx.get_request().get_method();
        if !method.is_get() && !method.is_head() {
            return Status::Continue;
        }
        let status_code: ResponseStatusCode = ctx.get_response().get_status_code();
        if !(200..300).contains(&status_code) || has_no_body(status_code) {
            return Status::Continue;
        }
        let matched: bool = if let Some(existing) = Self::existing_etag(ctx.get_response()) {
            ctx.get_request().matches_if_none_match(existing)
        } else {
            let computed: EntityTag = EntityTag::strong(ctx.get_response().get_body());
            let is_match: bool = ctx.get_request().matches_if_none_match(computed.as_str());
            ctx.get_mut_response().set_header(ETAG, computed.as_str());
            is_match
        };
        if matched {
            ctx.get_mut_response()
                .set_status_code(HttpStatus::NotModified.code())
                .set_body(EMPTY_STR);
        }
        Status::Continue
    }
}
