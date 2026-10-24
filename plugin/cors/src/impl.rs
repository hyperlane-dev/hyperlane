use super::*;

impl Default for CorsConfig {
    /// Builds the configuration the cross origin plugin uses when none is
    /// given.
    ///
    /// # Returns
    ///
    /// - `Self` - The default configuration.
    fn default() -> Self {
        Self {
            allowed_origins: vec![WILDCARD_ANY.to_owned()],
            allowed_methods: vec![ALL_METHODS.to_owned()],
            allowed_headers: vec![WILDCARD_ANY.to_owned()],
            max_age: None,
            allow_credentials: false,
        }
    }
}

impl CorsConfig {
    /// The rules the configuration works from.
    ///
    /// # Returns
    ///
    /// - `CorsPolicy` - The rules of the configuration.
    fn policy(&self) -> CorsPolicy {
        CorsPolicy {
            allowed_origins: self.get_allowed_origins().clone(),
            allowed_methods: self.get_allowed_methods().clone(),
            allowed_headers: self.get_allowed_headers().clone(),
            max_age: self.get_max_age(),
            allow_credentials: self.get_allow_credentials(),
        }
    }

    /// The origin that is written back, when the origin is allowed.
    ///
    /// A wildcard answers with the wildcard itself, a list answers with the
    /// origin that was asked for.
    ///
    /// # Arguments
    ///
    /// - `&str` - The origin the caller asked with.
    ///
    /// # Returns
    ///
    /// - `Option<String>` - The origin to write back, when the origin is
    ///   allowed.
    pub fn allowed_origin(&self, origin: &str) -> Option<String> {
        self.policy().allowed_origin(origin)
    }
}

impl CorsPlugin {
    /// Stores the configuration the plugin works from.
    ///
    /// # Arguments
    ///
    /// - `CorsConfig` - The configuration to store.
    ///
    /// # Returns
    ///
    /// - `Result<(), String>` - Nothing, or the reason the configuration was
    ///   refused.
    pub fn init(config: CorsConfig) -> Result<(), String> {
        let policy: CorsPolicy = config.policy();
        CONFIG
            .set(config)
            .map_err(|_: CorsConfig| MESSAGE_ALREADY_SET.to_owned())?;
        POLICY
            .set(policy)
            .map_err(|_: CorsPolicy| MESSAGE_ALREADY_SET.to_owned())
    }

    /// The configuration the plugin works from.
    ///
    /// # Returns
    ///
    /// - `Option<&'static CorsConfig>` - The configuration, when it was set.
    pub fn get() -> Option<&'static CorsConfig> {
        CONFIG.get()
    }
}

impl ServerHook for CorsMiddleware {
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

    /// Writes the cross origin headers of an allowed origin.
    ///
    /// A request without an origin, and a request from an origin that is not
    /// allowed, are left alone.
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
        let policy: &CorsPolicy = match POLICY.get() {
            Some(policy) => policy,
            None => return Status::Continue,
        };
        let origin: String = match ctx.get_request().try_get_header(ORIGIN) {
            Some(values) => match values.front() {
                Some(value) => value.clone(),
                None => return Status::Continue,
            },
            None => return Status::Continue,
        };
        let headers: Vec<(&'static str, String)> = match policy.headers(&origin) {
            Some(headers) => headers,
            None => return Status::Continue,
        };
        let response: &mut Response = ctx.get_mut_response();
        for (name, value) in headers {
            response.set_header(name, value);
        }
        Status::Continue
    }
}
