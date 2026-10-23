use super::*;

impl Default for CompressionConfig {
    /// Builds the configuration the plugin uses when none is given.
    ///
    /// # Returns
    ///
    /// - `Self` - The default configuration.
    fn default() -> Self {
        Self {
            min_size: DEFAULT_MIN_SIZE,
            encodings: vec![Compress::Gzip, Compress::Br],
        }
    }
}

impl CompressionConfig {
    /// The encoding the client asked for, when one of the offered encodings
    /// is accepted.
    ///
    /// The offered encodings are tried in the order they are preferred, so the
    /// first one the client accepts is the one that is used.
    ///
    /// # Arguments
    ///
    /// - `&str` - The `accept-encoding` header of the request.
    ///
    /// # Returns
    ///
    /// - `Option<Compress>` - The encoding to answer with, when the client
    ///   accepts one of the offered encodings.
    pub fn pick(&self, accept: &str) -> Option<Compress> {
        let tokens: Vec<String> = Self::accepted(accept);
        for encoding in self.get_encodings().iter() {
            if encoding.is_unknown() {
                continue;
            }
            let name: String = encoding.to_string();
            if tokens
                .iter()
                .any(|token: &String| token == TOKEN_ANY || token == &name)
            {
                return Some(*encoding);
            }
        }
        None
    }

    /// Whether a body of that content type is worth compressing.
    ///
    /// A body that is already compressed is sent as it is, and the stream of
    /// events is never compressed.
    ///
    /// # Arguments
    ///
    /// - `&str` - The content type of the answer.
    ///
    /// # Returns
    ///
    /// - `bool` - Whether the body may be compressed.
    pub fn is_compressible(&self, content_type: &str) -> bool {
        let content_type: String = content_type.trim().to_lowercase();
        if content_type.is_empty() {
            return true;
        }
        if content_type == TEXT_EVENT_STREAM {
            return false;
        }
        if CONTENT_TYPES_COMPRESSED.contains(&content_type.as_str()) {
            return false;
        }
        !CONTENT_TYPE_PREFIXES_COMPRESSED
            .iter()
            .any(|prefix: &&str| content_type.starts_with(*prefix))
    }

    /// The encodings the client accepts, without the ones it refused.
    ///
    /// # Arguments
    ///
    /// - `&str` - The `accept-encoding` header of the request.
    ///
    /// # Returns
    ///
    /// - `Vec<String>` - The tokens the client accepts.
    fn accepted(accept: &str) -> Vec<String> {
        let mut tokens: Vec<String> = Vec::new();
        for part in accept.split(SEPARATOR_COMMA) {
            let token: String = part.split(';').next().unwrap_or("").trim().to_lowercase();
            if token.is_empty() {
                continue;
            }
            let refused: bool = part.split(';').skip(1).any(|parameter: &str| {
                let quality: Option<&str> = parameter.trim().strip_prefix(PARAMETER_QUALITY);
                match quality {
                    Some(quality) => quality
                        .parse::<f32>()
                        .map(|value: f32| value <= 0.0)
                        .unwrap_or(false),
                    None => false,
                }
            });
            if !refused {
                tokens.push(token);
            }
        }
        tokens
    }
}

impl CompressionPlugin {
    /// Stores the configuration the plugin works from.
    ///
    /// # Arguments
    ///
    /// - `CompressionConfig` - The configuration to store.
    ///
    /// # Returns
    ///
    /// - `Result<(), String>` - Nothing, or the reason the configuration was
    ///   refused.
    pub fn init(config: CompressionConfig) -> Result<(), String> {
        CONFIG.set(config).map_err(|_: CompressionConfig| {
            "the compression configuration was already set".to_owned()
        })
    }

    /// The configuration the plugin works from.
    ///
    /// # Returns
    ///
    /// - `Option<&'static CompressionConfig>` - The configuration, when it was
    ///   set.
    pub fn get() -> Option<&'static CompressionConfig> {
        CONFIG.get()
    }
}

impl ServerHook for CompressionMiddleware {
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

    /// Marks the answer with the encoding the client asked for.
    ///
    /// The body is compressed when the answer is built, so the middleware only
    /// writes the header that names the encoding.
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
        let config: &CompressionConfig = match CONFIG.get() {
            Some(config) => config,
            None => return Status::Continue,
        };
        let accept: String = match ctx.get_request().try_get_header(ACCEPT_ENCODING) {
            Some(values) => match values.front() {
                Some(value) => value.clone(),
                None => return Status::Continue,
            },
            None => return Status::Continue,
        };
        let encoding: Compress = match config.pick(&accept) {
            Some(encoding) => encoding,
            None => return Status::Continue,
        };
        let response: &mut Response = ctx.get_mut_response();
        if response.try_get_header_back(CONTENT_ENCODING).is_some() {
            return Status::Continue;
        }
        if response.get_body().len() < config.get_min_size() {
            return Status::Continue;
        }
        let content_type: String = response
            .try_get_header_back(CONTENT_TYPE)
            .unwrap_or_default();
        if !config.is_compressible(&content_type) {
            return Status::Continue;
        }
        response.set_header(CONTENT_ENCODING, encoding.to_string());
        response.set_header(VARY, VARY_ACCEPT_ENCODING);
        Status::Continue
    }
}
