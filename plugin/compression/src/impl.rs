use super::*;

impl Default for CompressionConfig {
    /// Builds the configuration the compression plugin uses when none is given.
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
    /// The rules the configuration works from.
    ///
    /// The encodings that are not known are left out, so the plugin never
    /// answers with an encoding it cannot write.
    ///
    /// # Returns
    ///
    /// - `CompressionPolicy` - The rules of the configuration.
    fn policy(&self) -> CompressionPolicy {
        CompressionPolicy {
            encodings: self
                .get_encodings()
                .iter()
                .filter(|encoding: &&Compress| !encoding.is_unknown())
                .map(|encoding: &Compress| encoding.to_string())
                .collect(),
            min_size: self.get_min_size(),
        }
    }

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
        let encoding: String = self.policy().pick(accept)?;
        encoding.parse::<Compress>().ok()
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
        CompressionPolicy::is_compressible(content_type)
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
        let policy: CompressionPolicy = config.policy();
        CONFIG
            .set(config)
            .map_err(|_: CompressionConfig| MESSAGE_ALREADY_SET.to_owned())?;
        POLICY
            .set(policy)
            .map_err(|_: CompressionPolicy| MESSAGE_ALREADY_SET.to_owned())
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
        let policy: &CompressionPolicy = match POLICY.get() {
            Some(policy) => policy,
            None => return Status::Continue,
        };
        let accept: String = match ctx.get_request().try_get_header(ACCEPT_ENCODING) {
            Some(values) => match values.front() {
                Some(value) => value.clone(),
                None => return Status::Continue,
            },
            None => return Status::Continue,
        };
        let encoding: String = match policy.pick(&accept) {
            Some(encoding) => encoding,
            None => return Status::Continue,
        };
        let response: &mut Response = ctx.get_mut_response();
        if response.try_get_header_back(CONTENT_ENCODING).is_some() {
            return Status::Continue;
        }
        if response.get_body().len() < policy.get_min_size() {
            return Status::Continue;
        }
        let content_type: String = response
            .try_get_header_back(CONTENT_TYPE)
            .unwrap_or_default();
        if !CompressionPolicy::is_compressible(&content_type) {
            return Status::Continue;
        }
        response.set_header(CONTENT_ENCODING, encoding);
        response.set_header(VARY, VARY_ACCEPT_ENCODING);
        Status::Continue
    }
}
