use super::*;

/// Inherent implementation of [`SecurityViolation`].
impl SecurityViolation {
    /// Maps this violation to its canonical HTTP status code.
    ///
    /// # Returns
    ///
    /// - `usize` - HTTP status code (`400`, `403`, `413`, `414`, or custom).
    pub fn status_code(&self) -> usize {
        match self {
            Self::BodyTooLarge => HttpStatus::PayloadTooLarge.code(),
            Self::UriTooLong => HttpStatus::URITooLong.code(),
            Self::InvalidHost | Self::PathTraversal => HttpStatus::BadRequest.code(),
            Self::CsrfInvalid => HttpStatus::Forbidden.code(),
            Self::Custom { status_code, .. } => *status_code,
        }
    }
}

/// Formatting for [`SecurityViolation`].
impl Display for SecurityViolation {
    /// Formats a human-readable description of the [`SecurityViolation`].
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
            Self::BodyTooLarge => f.write_str(MSG_BODY_TOO_LARGE),
            Self::UriTooLong => f.write_str(MSG_URI_TOO_LONG),
            Self::InvalidHost => f.write_str(MSG_INVALID_HOST),
            Self::PathTraversal => f.write_str(MSG_PATH_TRAVERSAL),
            Self::CsrfInvalid => f.write_str(MSG_CSRF_INVALID),
            Self::Custom { message, .. } => f.write_str(message),
        }
    }
}

/// Implements [`Error`] for [`SecurityViolation`].
impl Error for SecurityViolation {}

/// Default construction for [`SecurityHeadersConfig`].
impl Default for SecurityHeadersConfig {
    /// Constructs baseline modern HTTP security headers suitable for most services.
    fn default() -> Self {
        Self {
            hsts: Some(DEFAULT_HSTS_VALUE.to_owned()),
            content_security_policy: None,
            x_frame_options: Some(DEFAULT_X_FRAME_OPTIONS.to_owned()),
            x_content_type_options: Some(DEFAULT_X_CONTENT_TYPE_OPTIONS.to_owned()),
            referrer_policy: Some(DEFAULT_REFERRER_POLICY.to_owned()),
            permissions_policy: Some(DEFAULT_PERMISSIONS_POLICY.to_owned()),
            coop: Some(DEFAULT_COOP_VALUE.to_owned()),
            coep: None,
            corp: Some(DEFAULT_CORP_VALUE.to_owned()),
            remove_powered_by: true,
        }
    }
}

/// Inherent presets for [`SecurityHeadersConfig`].
impl SecurityHeadersConfig {
    /// Builds a strict browser-hardened header preset including strict CSP and COEP.
    ///
    /// # Returns
    ///
    /// - `Self` - Strict security headers configuration.
    pub fn strict() -> Self {
        Self {
            hsts: Some(DEFAULT_HSTS_VALUE.to_owned()),
            content_security_policy: Some(DEFAULT_CSP_STRICT.to_owned()),
            x_frame_options: Some(DEFAULT_X_FRAME_OPTIONS.to_owned()),
            x_content_type_options: Some(DEFAULT_X_CONTENT_TYPE_OPTIONS.to_owned()),
            referrer_policy: Some(DEFAULT_REFERRER_POLICY.to_owned()),
            permissions_policy: Some(DEFAULT_PERMISSIONS_POLICY.to_owned()),
            coop: Some(DEFAULT_COOP_VALUE.to_owned()),
            coep: Some(DEFAULT_COEP_VALUE.to_owned()),
            corp: Some(DEFAULT_CORP_VALUE.to_owned()),
            remove_powered_by: true,
        }
    }

    /// Prebuilds a flat `(header_name, value)` vector for zero-allocation hot-path injection.
    ///
    /// # Returns
    ///
    /// - `Vec<(&'static str, String)>` - Compiled static header pairs.
    pub(crate) fn compile(&self) -> Vec<(&'static str, String)> {
        let mut out: Vec<(&'static str, String)> = Vec::with_capacity(9);
        if let Some(val) = self.get_hsts() {
            out.push((STRICT_TRANSPORT_SECURITY, val));
        }
        if let Some(val) = self.get_content_security_policy() {
            out.push((CONTENT_SECURITY_POLICY, val));
        }
        if let Some(val) = self.get_x_frame_options() {
            out.push((X_FRAME_OPTIONS, val));
        }
        if let Some(val) = self.get_x_content_type_options() {
            out.push((X_CONTENT_TYPE_OPTIONS, val));
        }
        if let Some(val) = self.get_referrer_policy() {
            out.push((REFERRER_POLICY, val));
        }
        if let Some(val) = self.get_permissions_policy() {
            out.push((PERMISSIONS_POLICY, val));
        }
        if let Some(val) = self.get_coop() {
            out.push((CROSS_ORIGIN_OPENER_POLICY, val));
        }
        if let Some(val) = self.get_coep() {
            out.push((CROSS_ORIGIN_EMBEDDER_POLICY, val));
        }
        if let Some(val) = self.get_corp() {
            out.push((CROSS_ORIGIN_RESOURCE_POLICY, val));
        }
        out
    }
}

/// Default construction for [`CsrfConfig`].
impl Default for CsrfConfig {
    /// Constructs a disabled [`CsrfConfig`] with standard cookie and header names.
    fn default() -> Self {
        Self {
            enabled: false,
            secret: Vec::new(),
            cookie_name: DEFAULT_CSRF_COOKIE_NAME.to_owned(),
            header_name: DEFAULT_CSRF_HEADER_NAME.to_owned(),
            exempt_paths: Vec::new(),
        }
    }
}

/// Inherent constructors for [`CsrfConfig`].
impl CsrfConfig {
    /// Builds an enabled [`CsrfConfig`] using `secret` for stateless HMAC-SHA256 signing.
    ///
    /// # Arguments
    ///
    /// - `S` - Secret key bytes convertible into `Vec<u8>`.
    ///
    /// # Returns
    ///
    /// - `Self` - Enabled CSRF configuration.
    pub fn enabled<S>(secret: S) -> Self
    where
        S: Into<Vec<u8>>,
    {
        Self {
            enabled: true,
            secret: secret.into(),
            cookie_name: DEFAULT_CSRF_COOKIE_NAME.to_owned(),
            header_name: DEFAULT_CSRF_HEADER_NAME.to_owned(),
            exempt_paths: Vec::new(),
        }
    }
}

/// Default construction for [`RequestGuardConfig`].
impl Default for RequestGuardConfig {
    /// Constructs a [`RequestGuardConfig`] blocking path traversal and oversized URIs.
    fn default() -> Self {
        Self {
            max_body_bytes: DEFAULT_SECURITY_MAX_BODY_BYTES,
            max_uri_bytes: DEFAULT_MAX_URI_BYTES,
            allowed_hosts: Vec::new(),
            block_path_traversal: true,
        }
    }
}

/// Inherent presets and builders for [`SecurityConfig`].
impl SecurityConfig {
    /// Builds a strict browser-hardened [`SecurityConfig`] preset.
    ///
    /// # Returns
    ///
    /// - `Self` - Strict security configuration.
    pub fn strict() -> Self {
        Self {
            headers: SecurityHeadersConfig::strict(),
            csrf: CsrfConfig::default(),
            guard: RequestGuardConfig::default(),
        }
    }

    /// Sets the [`SecurityHeadersConfig`].
    ///
    /// # Arguments
    ///
    /// - `SecurityHeadersConfig` - Header settings.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated configuration.
    pub fn with_headers(mut self, headers: SecurityHeadersConfig) -> Self {
        self.set_headers(headers);
        self
    }

    /// Sets the [`CsrfConfig`].
    ///
    /// # Arguments
    ///
    /// - `CsrfConfig` - CSRF protection settings.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated configuration.
    pub fn with_csrf(mut self, csrf: CsrfConfig) -> Self {
        self.set_csrf(csrf);
        self
    }

    /// Sets the [`RequestGuardConfig`].
    ///
    /// # Arguments
    ///
    /// - `RequestGuardConfig` - Request guard settings.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated configuration.
    pub fn with_guard(mut self, guard: RequestGuardConfig) -> Self {
        self.set_guard(guard);
        self
    }
}

/// Inherent implementation of [`SecurityPlugin`].
impl SecurityPlugin {
    /// Compiles a standalone [`SecurityPlugin`] instance (useful for direct invocation or testing).
    ///
    /// # Arguments
    ///
    /// - `SecurityConfig` - Security headers, CSRF, and guard settings.
    /// - `Vec<SharedSecurityRule>` - Additional custom security inspection rules.
    ///
    /// # Returns
    ///
    /// - `Self` - Compiled security engine.
    pub fn new(config: SecurityConfig, rules: Vec<SharedSecurityRule>) -> Self {
        let compiled_headers: Vec<(&'static str, String)> = config.get_headers().compile();
        Self {
            config,
            compiled_headers: Arc::new(compiled_headers),
            rules: Arc::new(rules),
        }
    }

    /// Initializes the global [`SecurityPlugin`] once with `config`.
    ///
    /// # Arguments
    ///
    /// - `SecurityConfig` - Security hardening configuration.
    ///
    /// # Returns
    ///
    /// - `&'static Self` - Reference to the initialized global plugin.
    pub fn init(config: SecurityConfig) -> &'static Self {
        Self::init_with_rules(config, Vec::new())
    }

    /// Initializes the global [`SecurityPlugin`] once with `config` and custom [`SecurityRule`] hooks.
    ///
    /// # Arguments
    ///
    /// - `SecurityConfig` - Security hardening configuration.
    /// - `Vec<SharedSecurityRule>` - Custom request security rules.
    ///
    /// # Returns
    ///
    /// - `&'static Self` - Reference to the initialized global plugin.
    pub fn init_with_rules(
        config: SecurityConfig,
        rules: Vec<SharedSecurityRule>,
    ) -> &'static Self {
        SECURITY_PLUGIN.get_or_init(|| Self::new(config, rules))
    }

    /// Returns the initialized global [`SecurityPlugin`], if initialized.
    ///
    /// # Returns
    ///
    /// - `Option<&'static Self>` - Reference to the global plugin, or `None`.
    pub fn get() -> Option<&'static Self> {
        SECURITY_PLUGIN.get()
    }

    /// Returns the active [`SecurityConfig`].
    ///
    /// # Returns
    ///
    /// - `&SecurityConfig` - Active configuration.
    pub fn get_config(&self) -> &SecurityConfig {
        &self.config
    }

    /// Evaluates request guards, double-submit HMAC CSRF protection, and custom rules on `ctx`.
    ///
    /// # Arguments
    ///
    /// - `&Context` - Incoming request context.
    ///
    /// # Returns
    ///
    /// - `Result<(), SecurityViolation>` - `Ok(())` when all checks pass.
    pub fn inspect_request(&self, ctx: &Context) -> Result<(), SecurityViolation> {
        let Self { config, rules, .. } = self;
        let guard: &RequestGuardConfig = config.get_guard();
        let path: &str = ctx.get_request().get_path();
        if guard.get_max_uri_bytes() > 0 && path.len() > guard.get_max_uri_bytes() {
            return Err(SecurityViolation::UriTooLong);
        }
        if guard.get_max_body_bytes() > 0
            && ctx.get_request().get_body().len() > guard.get_max_body_bytes()
        {
            return Err(SecurityViolation::BodyTooLarge);
        }
        if guard.get_block_path_traversal() && has_path_traversal(path) {
            return Err(SecurityViolation::PathTraversal);
        }
        if !guard.get_allowed_hosts().is_empty() {
            let host_ok: bool =
                ctx.get_request()
                    .try_get_header_back(HOST)
                    .is_some_and(|incoming: String| {
                        let trimmed: &str = incoming.trim();
                        guard
                            .get_allowed_hosts()
                            .iter()
                            .any(|allowed: &String| allowed.eq_ignore_ascii_case(trimmed))
                    });
            if !host_ok {
                return Err(SecurityViolation::InvalidHost);
            }
        }
        let csrf: &CsrfConfig = config.get_csrf();
        if csrf.get_enabled()
            && !is_csrf_safe_method(ctx.get_request().get_method())
            && !csrf
                .get_exempt_paths()
                .iter()
                .any(|exempt: &String| exempt == path)
        {
            let Some(header_token) = ctx
                .get_request()
                .try_get_header_back(csrf.get_header_name())
            else {
                return Err(SecurityViolation::CsrfInvalid);
            };
            let Some(raw_cookie) = ctx.get_request().try_get_header_back(COOKIE) else {
                return Err(SecurityViolation::CsrfInvalid);
            };
            let cookies: Cookies = Cookie::parse(raw_cookie);
            let Some(cookie_token) = cookies.get(csrf.get_cookie_name()) else {
                return Err(SecurityViolation::CsrfInvalid);
            };
            if !constant_time_eq(header_token.as_bytes(), cookie_token.as_bytes())
                || !verify_csrf_token(csrf.get_secret(), &header_token)
            {
                return Err(SecurityViolation::CsrfInvalid);
            }
        }
        for rule in rules.iter() {
            rule.inspect(ctx)?;
        }
        Ok(())
    }

    /// Applies prebuilt security headers onto `response` and optionally strips `X-Powered-By`.
    ///
    /// # Arguments
    ///
    /// - `&mut Response` - Outgoing HTTP response.
    pub fn apply_headers(&self, response: &mut Response) {
        let Self {
            config,
            compiled_headers,
            ..
        } = self;
        for (name, value) in compiled_headers.iter() {
            if response.try_get_header_back(*name).is_none() {
                response.set_header(*name, value);
            }
        }
        if config.get_headers().get_remove_powered_by() {
            response.remove_header(X_POWERED_BY);
        }
    }
}

/// Implements [`ServerHook`] for [`SecurityRequestHook`].
impl ServerHook for SecurityRequestHook {
    /// Constructs a per-request [`SecurityRequestHook`].
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

    /// Enforces request security guards, CSRF validation, and custom rules, and seeds response security headers.
    ///
    /// # Arguments
    ///
    /// - `Self` - Hook instance.
    /// - `&mut Stream` - Connection stream.
    /// - `&mut Context` - Request context.
    ///
    /// # Returns
    ///
    /// - `Status` - `Status::Continue` when permitted; `Status::Reject` after sending an error response on violation.
    async fn handle(self, stream: &mut Stream, ctx: &mut Context) -> Status {
        let Some(plugin) = SecurityPlugin::get() else {
            return Status::Continue;
        };
        plugin.apply_headers(ctx.get_mut_response());
        if let Err(violation) = plugin.inspect_request(ctx) {
            ctx.get_mut_response()
                .set_status_code(violation.status_code())
                .set_body(Vec::new());
            let wire: ResponseData = ctx.get_mut_response().build();
            let _: Result<(), ResponseError> = stream.try_send(&wire).await;
            return Status::Reject;
        }
        Status::Continue
    }
}

/// Implements [`ServerHook`] for [`SecurityResponseHook`].
impl ServerHook for SecurityResponseHook {
    /// Constructs a per-response [`SecurityResponseHook`].
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

    /// Ensures configured security headers remain present and removes `X-Powered-By`.
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
        let Some(plugin) = SecurityPlugin::get() else {
            return Status::Continue;
        };
        plugin.apply_headers(ctx.get_mut_response());
        Status::Continue
    }
}
