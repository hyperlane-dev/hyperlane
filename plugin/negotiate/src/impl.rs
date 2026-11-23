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
            .map_err(|_: CompressConfig| "the compress configuration was already set".to_owned())
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
        let values: &RequestHeadersValue = request.get_headers().get(ACCEPT_ENCODING)?;
        let first: &String = values.front()?;
        if values.len() == 1 {
            return Compress::from_accept_encoding(first);
        }
        let joined: String = values.iter().cloned().collect::<Vec<String>>().join(COMMA);
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
        if response.get_headers().contains_key(CONTENT_ENCODING)
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
        Self(format!(
            "{ENTITY_TAG_QUOTE}{:016x}{ENTITY_TAG_QUOTE}",
            hasher.finish()
        ))
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
    /// Reads the validator of the response, computing one when it carries none.
    ///
    /// A response that already declares a validator keeps it, because the
    /// application knows what its representation is better than a digest of the
    /// bytes it happens to produce.
    ///
    /// # Arguments
    ///
    /// - `&mut Response` - The response to read or write the validator of.
    ///
    /// # Returns
    ///
    /// - `EntityTag` - The validator the response answers with.
    fn entity_tag(response: &mut Response) -> EntityTag {
        let existing: Option<String> = response
            .get_headers()
            .get(ETAG)
            .and_then(|values: &ResponseHeadersValue| values.back().cloned());
        match existing {
            Some(etag) => EntityTag(etag),
            None => {
                let etag: EntityTag = EntityTag::strong(response.get_body());
                response.set_header(ETAG, etag.as_str());
                etag
            }
        }
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
        if has_no_body(ctx.get_response().get_status_code()) {
            return Status::Continue;
        }
        let etag: EntityTag = Self::entity_tag(ctx.get_mut_response());
        if ctx.get_request().matches_if_none_match(etag.as_str()) {
            ctx.get_mut_response()
                .set_status_code(HttpStatus::NotModified.code())
                .set_body(EMPTY_STR);
        }
        Status::Continue
    }
}
