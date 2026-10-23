use super::*;

impl Default for CorsConfig {
    /// Builds the configuration the plugin uses when none is given.
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
        if self
            .get_allowed_origins()
            .iter()
            .any(|allowed: &String| allowed == WILDCARD_ANY)
        {
            return Some(WILDCARD_ANY.to_owned());
        }
        if self
            .get_allowed_origins()
            .iter()
            .any(|allowed: &String| allowed == origin)
        {
            return Some(origin.to_owned());
        }
        None
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
        CONFIG
            .set(config)
            .map_err(|_: CorsConfig| "the cross origin configuration was already set".to_owned())
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
        let config: &CorsConfig = match CONFIG.get() {
            Some(config) => config,
            None => return Status::Continue,
        };
        let origin: String = match ctx.get_request().try_get_header(ORIGIN) {
            Some(values) => match values.front() {
                Some(value) => value.clone(),
                None => return Status::Continue,
            },
            None => return Status::Continue,
        };
        let allowed: String = match config.allowed_origin(&origin) {
            Some(allowed) => allowed,
            None => return Status::Continue,
        };
        let response: &mut Response = ctx.get_mut_response();
        response.set_header(ACCESS_CONTROL_ALLOW_ORIGIN, allowed);
        response.set_header(
            ACCESS_CONTROL_ALLOW_METHODS,
            config.get_allowed_methods().join(SEPARATOR),
        );
        response.set_header(
            ACCESS_CONTROL_ALLOW_HEADERS,
            config.get_allowed_headers().join(SEPARATOR),
        );
        if let Some(max_age) = config.get_max_age() {
            response.set_header(ACCESS_CONTROL_MAX_AGE, max_age.to_string());
        }
        if config.get_allow_credentials() {
            response.set_header(ACCESS_CONTROL_ALLOW_CREDENTIALS, VALUE_TRUE);
        }
        response.set_header(VARY, ORIGIN);
        Status::Continue
    }
}
