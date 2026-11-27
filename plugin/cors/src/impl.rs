use super::*;

/// Debug formatting for [`AllowOrigin`].
impl Debug for AllowOrigin {
    /// Formats the [`AllowOrigin`] policy without requiring `Debug` on closures.
    ///
    /// # Arguments
    ///
    /// - `&mut Formatter<'_>` - Output formatter.
    ///
    /// # Returns
    ///
    /// - `fmt::Result` - Formatting outcome.
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Any => f.write_str(DEBUG_ALLOW_ORIGIN_ANY),
            Self::Exact(set) => f.debug_tuple(DEBUG_ALLOW_ORIGIN_EXACT).field(set).finish(),
            Self::Suffix(list) => f
                .debug_tuple(DEBUG_ALLOW_ORIGIN_SUFFIX)
                .field(list)
                .finish(),
            Self::Custom(_) => f.write_str(DEBUG_ALLOW_ORIGIN_CUSTOM),
        }
    }
}

/// Inherent helpers for [`AllowOrigin`].
impl AllowOrigin {
    /// Constructs an `O(1)` exact-origin allowlist (stored lowercased for case-insensitive matching).
    ///
    /// # Arguments
    ///
    /// - `I` - Iterator of allowed origin strings (for example `"https://app.example.com"`).
    ///
    /// # Returns
    ///
    /// - `Self` - Exact-match allowlist policy.
    pub fn exact<I, S>(origins: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let mut set: HashSetXxHash3_64<String> = hash_set_xx_hash3_64();
        for origin in origins {
            let normalized: String = origin.as_ref().trim().to_ascii_lowercase();
            if !normalized.is_empty() {
                set.insert(normalized);
            }
        }
        Self::Exact(set)
    }

    /// Constructs a host suffix matcher (for example `[".example.com"]`).
    ///
    /// # Arguments
    ///
    /// - `I` - Iterator of allowed domain suffixes.
    ///
    /// # Returns
    ///
    /// - `Self` - Suffix-match policy.
    pub fn suffix<I, S>(suffixes: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let list: Vec<String> = suffixes
            .into_iter()
            .map(|item: S| item.as_ref().trim().to_ascii_lowercase())
            .filter(|item: &String| !item.is_empty())
            .collect();
        Self::Suffix(list)
    }

    /// Constructs a custom predicate-based origin matcher.
    ///
    /// # Arguments
    ///
    /// - `F` - Thread-safe predicate receiving the raw `Origin` header value.
    ///
    /// # Returns
    ///
    /// - `Self` - Predicate policy.
    pub fn custom<F>(predicate: F) -> Self
    where
        F: Fn(&str) -> bool + Send + Sync + 'static,
    {
        Self::Custom(Arc::new(predicate))
    }

    /// Reports whether `origin` is permitted by this policy.
    ///
    /// # Arguments
    ///
    /// - `&str` - Incoming `Origin` header value.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` when `origin` is allowed.
    pub fn is_allowed(&self, origin: &str) -> bool {
        let trimmed: &str = origin.trim();
        if trimmed.is_empty() {
            return false;
        }
        match self {
            Self::Any => true,
            Self::Exact(set) => {
                if set.contains(trimmed) {
                    return true;
                }
                set.contains(&trimmed.to_ascii_lowercase())
            }
            Self::Suffix(suffixes) => {
                let host: String = extract_origin_host(trimmed).to_ascii_lowercase();
                if host.is_empty() {
                    return false;
                }
                suffixes.iter().any(|suffix: &String| {
                    if let Some(bare) = suffix.strip_prefix('.') {
                        host == bare || host.ends_with(suffix.as_str())
                    } else {
                        host == *suffix
                    }
                })
            }
            Self::Custom(matcher) => matcher(trimmed),
        }
    }
}

/// Default construction for [`CorsConfig`].
impl Default for CorsConfig {
    /// Constructs a conservative default [`CorsConfig`] allowing any origin without credentials.
    fn default() -> Self {
        Self {
            allow_origin: AllowOrigin::Any,
            allow_methods: DEFAULT_ALLOWED_METHODS
                .iter()
                .map(|method: &&str| (*method).to_owned())
                .collect(),
            allow_headers: DEFAULT_ALLOWED_HEADERS
                .iter()
                .map(|header: &&str| (*header).to_owned())
                .collect(),
            expose_headers: Vec::new(),
            allow_credentials: false,
            max_age_secs: Some(DEFAULT_CORS_MAX_AGE_SECS),
            allow_private_network: false,
        }
    }
}

/// Inherent builders and presets for [`CorsConfig`].
impl CorsConfig {
    /// Builds a permissive development policy allowing any origin, method, and header.
    ///
    /// # Returns
    ///
    /// - `Self` - Permissive CORS configuration.
    pub fn permissive() -> Self {
        Self {
            allow_origin: AllowOrigin::Any,
            allow_methods: DEFAULT_ALLOWED_METHODS
                .iter()
                .map(|method: &&str| (*method).to_owned())
                .collect(),
            allow_headers: vec![CORS_WILDCARD.to_owned()],
            expose_headers: Vec::new(),
            allow_credentials: false,
            max_age_secs: Some(DEFAULT_CORS_MAX_AGE_SECS),
            allow_private_network: false,
        }
    }

    /// Builds a strict allowlist policy accepting only the specified origins.
    ///
    /// # Arguments
    ///
    /// - `I` - Iterator of exact origin strings.
    ///
    /// # Returns
    ///
    /// - `Self` - Strict CORS configuration.
    pub fn strict<I, S>(origins: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        Self::default().with_allow_origin(AllowOrigin::exact(origins))
    }

    /// Sets the [`AllowOrigin`] policy.
    ///
    /// # Arguments
    ///
    /// - `AllowOrigin` - Desired origin matching policy.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated configuration.
    pub fn with_allow_origin(mut self, allow_origin: AllowOrigin) -> Self {
        self.set_allow_origin(allow_origin);
        self
    }

    /// Sets the allowed HTTP methods for preflight responses.
    ///
    /// # Arguments
    ///
    /// - `I` - Iterator of HTTP method names.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated configuration.
    pub fn with_allow_methods<I, S>(mut self, methods: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.set_allow_methods(methods.into_iter().map(Into::into).collect());
        self
    }

    /// Sets the allowed request headers for preflight responses.
    ///
    /// # Arguments
    ///
    /// - `I` - Iterator of header names.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated configuration.
    pub fn with_allow_headers<I, S>(mut self, headers: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.set_allow_headers(headers.into_iter().map(Into::into).collect());
        self
    }

    /// Sets the response headers exposed to cross-origin clients.
    ///
    /// # Arguments
    ///
    /// - `I` - Iterator of exposed header names.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated configuration.
    pub fn with_expose_headers<I, S>(mut self, headers: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.set_expose_headers(headers.into_iter().map(Into::into).collect());
        self
    }

    /// Enables or disables `Access-Control-Allow-Credentials: true`.
    ///
    /// # Arguments
    ///
    /// - `bool` - Whether credentials may be sent cross-origin.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated configuration.
    pub fn with_allow_credentials(mut self, allow_credentials: bool) -> Self {
        self.set_allow_credentials(allow_credentials);
        self
    }

    /// Configures the `Access-Control-Max-Age` cache duration in seconds.
    ///
    /// # Arguments
    ///
    /// - `Option<u64>` - Optional cache lifetime in seconds.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated configuration.
    pub fn with_max_age_secs(mut self, max_age_secs: Option<u64>) -> Self {
        self.set_max_age_secs(max_age_secs);
        self
    }

    /// Enables or disables `Access-Control-Allow-Private-Network: true` responses.
    ///
    /// # Arguments
    ///
    /// - `bool` - Whether private-network preflight requests are approved.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated configuration.
    pub fn with_allow_private_network(mut self, allow_private_network: bool) -> Self {
        self.set_allow_private_network(allow_private_network);
        self
    }
}

/// Inherent implementation of [`CompiledCors`].
impl CompiledCors {
    /// Precompiles a [`CorsConfig`] into zero-allocation header values.
    ///
    /// # Arguments
    ///
    /// - `CorsConfig` - Source CORS configuration.
    ///
    /// # Returns
    ///
    /// - `Self` - Precompiled policy ready for hot-path request evaluation.
    pub fn compile(config: CorsConfig) -> Self {
        let methods_header: String = config.get_allow_methods().join(COMMA_SPACE);
        let headers_header: Option<String> =
            if should_mirror_request_headers(config.get_allow_headers()) {
                None
            } else {
                Some(config.get_allow_headers().join(COMMA_SPACE))
            };
        let expose_header: Option<String> = if config.get_expose_headers().is_empty() {
            None
        } else {
            Some(config.get_expose_headers().join(COMMA_SPACE))
        };
        let max_age_header: Option<String> = config
            .try_get_max_age_secs()
            .map(|secs: u64| secs.to_string());
        let vary_origin: bool = config.get_allow_credentials()
            || !matches!(config.get_allow_origin(), AllowOrigin::Any);
        Self {
            config,
            methods_header,
            headers_header,
            expose_header,
            max_age_header,
            vary_origin,
        }
    }

    /// Returns the source [`CorsConfig`].
    ///
    /// # Returns
    ///
    /// - `&CorsConfig` - Active CORS configuration.
    pub fn get_config(&self) -> &CorsConfig {
        &self.config
    }

    /// Resolves the `Access-Control-Allow-Origin` header value for an incoming `Origin`.
    ///
    /// Never returns `"*"` when `allow_credentials` is `true`, satisfying the Fetch specification.
    ///
    /// # Arguments
    ///
    /// - `&'a str` - Incoming `Origin` header value.
    ///
    /// # Returns
    ///
    /// - `Option<&'a str>` - Header value to emit, or `None` when the origin is rejected.
    pub fn resolve_origin<'a>(&self, origin: &'a str) -> Option<&'a str> {
        let Self { config, .. } = self;
        let trimmed: &'a str = origin.trim();
        if !config.get_allow_origin().is_allowed(trimmed) {
            return None;
        }
        if matches!(config.get_allow_origin(), AllowOrigin::Any) && !config.get_allow_credentials()
        {
            Some(CORS_WILDCARD)
        } else {
            Some(trimmed)
        }
    }

    /// Applies CORS headers to `ctx`, returning `true` when `ctx` was answered as a preflight request.
    ///
    /// # Arguments
    ///
    /// - `&mut Context` - Incoming request and outgoing response context.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` when a preflight `OPTIONS` response was populated and ready to send.
    pub fn apply_request(&self, ctx: &mut Context) -> bool {
        let Self {
            config,
            methods_header,
            headers_header,
            expose_header,
            max_age_header,
            vary_origin,
        } = self;
        let Some(origin) = ctx.get_request().try_get_header_back(ORIGIN) else {
            return false;
        };
        let Some(allow_origin_val) = self.resolve_origin(&origin) else {
            return false;
        };
        let allow_origin_owned: String = allow_origin_val.to_owned();
        let req_method: Option<String> = ctx
            .get_request()
            .try_get_header_back(ACCESS_CONTROL_REQUEST_METHOD);
        let is_preflight: bool =
            ctx.get_request().get_method().is_options() && req_method.is_some();
        let req_headers: Option<String> = if is_preflight {
            ctx.get_request()
                .try_get_header_back(ACCESS_CONTROL_REQUEST_HEADERS)
        } else {
            None
        };
        let wants_private_net: bool = is_preflight
            && config.get_allow_private_network()
            && ctx
                .get_request()
                .try_get_header_back(ACCESS_CONTROL_REQUEST_PRIVATE_NETWORK)
                .is_some_and(|val: String| val.trim().eq_ignore_ascii_case(HEADER_VALUE_TRUE));
        let response: &mut Response = ctx.get_mut_response();
        response.set_header(ACCESS_CONTROL_ALLOW_ORIGIN, &allow_origin_owned);
        if *vary_origin {
            merge_vary_header(response, VARY_ORIGIN);
        }
        if config.get_allow_credentials() {
            response.set_header(ACCESS_CONTROL_ALLOW_CREDENTIALS, HEADER_VALUE_TRUE);
        }
        if is_preflight {
            if !methods_header.is_empty() {
                response.set_header(ACCESS_CONTROL_ALLOW_METHODS, methods_header);
            }
            if let Some(fixed_headers) = headers_header {
                response.set_header(ACCESS_CONTROL_ALLOW_HEADERS, fixed_headers);
            } else if let Some(requested) = req_headers {
                response.set_header(ACCESS_CONTROL_ALLOW_HEADERS, requested);
            }
            if let Some(max_age) = max_age_header {
                response.set_header(ACCESS_CONTROL_MAX_AGE, max_age);
            }
            if wants_private_net {
                response.set_header(ACCESS_CONTROL_ALLOW_PRIVATE_NETWORK, HEADER_VALUE_TRUE);
            }
            merge_vary_header(response, VARY_PREFLIGHT_METHOD);
            merge_vary_header(response, VARY_PREFLIGHT_HEADERS);
            response
                .set_status_code(HttpStatus::NoContent.code())
                .set_body(Vec::new());
            return true;
        }
        if let Some(expose) = expose_header {
            response.set_header(ACCESS_CONTROL_EXPOSE_HEADERS, expose);
        }
        ctx.set_attribute(ATTR_CORS_ORIGIN_APPLIED, allow_origin_owned);
        false
    }

    /// Re-applies CORS headers on the response path if a route cleared response headers.
    ///
    /// # Arguments
    ///
    /// - `&mut Context` - Active request/response context.
    pub fn ensure_response_headers(&self, ctx: &mut Context) {
        let Self {
            config,
            expose_header,
            vary_origin,
            ..
        } = self;
        let Some(allow_origin_val) = ctx.try_get_attribute::<String, _>(ATTR_CORS_ORIGIN_APPLIED)
        else {
            return;
        };
        let response: &mut Response = ctx.get_mut_response();
        if response
            .try_get_header_back(ACCESS_CONTROL_ALLOW_ORIGIN)
            .is_none()
        {
            response.set_header(ACCESS_CONTROL_ALLOW_ORIGIN, &allow_origin_val);
            if *vary_origin {
                merge_vary_header(response, VARY_ORIGIN);
            }
            if config.get_allow_credentials() {
                response.set_header(ACCESS_CONTROL_ALLOW_CREDENTIALS, HEADER_VALUE_TRUE);
            }
            if let Some(expose) = expose_header {
                response.set_header(ACCESS_CONTROL_EXPOSE_HEADERS, expose);
            }
        }
    }
}

/// Inherent implementation of [`CorsPlugin`].
impl CorsPlugin {
    /// Initializes the global [`CorsPlugin`] once with `config`.
    ///
    /// # Arguments
    ///
    /// - `CorsConfig` - CORS policy configuration.
    ///
    /// # Returns
    ///
    /// - `&'static Self` - Reference to the initialized global plugin.
    pub fn init(config: CorsConfig) -> &'static Self {
        CORS_PLUGIN.get_or_init(|| Self {
            compiled: CompiledCors::compile(config),
        })
    }

    /// Returns the initialized global [`CorsPlugin`], if [`CorsPlugin::init`] was called.
    ///
    /// # Returns
    ///
    /// - `Option<&'static Self>` - Global plugin reference, or `None`.
    pub fn get() -> Option<&'static Self> {
        CORS_PLUGIN.get()
    }

    /// Returns the precompiled [`CompiledCors`] engine.
    ///
    /// # Returns
    ///
    /// - `&CompiledCors` - Precompiled CORS policy.
    pub fn get_compiled(&self) -> &CompiledCors {
        &self.compiled
    }
}

/// Implements [`ServerHook`] for [`CorsRequestHook`].
impl ServerHook for CorsRequestHook {
    /// Constructs a per-request [`CorsRequestHook`].
    ///
    /// # Arguments
    ///
    /// - `&mut Stream` - Connection stream.
    /// - `&mut Context` - Request context.
    ///
    /// # Returns
    ///
    /// - `Self` - Hook instance.
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    /// Applies CORS headers and short-circuits preflight `OPTIONS` requests with `204 No Content`.
    ///
    /// # Arguments
    ///
    /// - `Self` - Hook instance.
    /// - `&mut Stream` - Connection stream.
    /// - `&mut Context` - Request context.
    ///
    /// # Returns
    ///
    /// - `Status` - `Status::Reject` after sending a preflight response; `Status::Continue` otherwise.
    async fn handle(self, stream: &mut Stream, ctx: &mut Context) -> Status {
        let Some(plugin) = CorsPlugin::get() else {
            return Status::Continue;
        };
        if plugin.get_compiled().apply_request(ctx) {
            let wire: ResponseData = ctx.get_mut_response().build();
            let _: Result<(), ResponseError> = stream.try_send(&wire).await;
            return Status::Reject;
        }
        Status::Continue
    }
}

/// Implements [`ServerHook`] for [`CorsResponseHook`].
impl ServerHook for CorsResponseHook {
    /// Constructs a per-response [`CorsResponseHook`].
    ///
    /// # Arguments
    ///
    /// - `&mut Stream` - Connection stream.
    /// - `&mut Context` - Request context.
    ///
    /// # Returns
    ///
    /// - `Self` - Hook instance.
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    /// Ensures CORS headers remain present on the response before transmission.
    ///
    /// # Arguments
    ///
    /// - `Self` - Hook instance.
    /// - `&mut Stream` - Connection stream.
    /// - `&mut Context` - Request context.
    ///
    /// # Returns
    ///
    /// - `Status` - Always `Status::Continue`.
    async fn handle(self, _: &mut Stream, ctx: &mut Context) -> Status {
        let Some(plugin) = CorsPlugin::get() else {
            return Status::Continue;
        };
        plugin.get_compiled().ensure_response_headers(ctx);
        Status::Continue
    }
}
