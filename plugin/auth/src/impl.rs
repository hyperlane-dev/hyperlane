use super::*;

/// Formatting for [`AuthError`].
impl Display for AuthError {
    /// Formats a human-readable error summary for [`AuthError`].
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
            Self::MissingCredentials => f.write_str(MSG_AUTH_MISSING),
            Self::InvalidCredentials => f.write_str(MSG_AUTH_INVALID),
            Self::TokenExpired => f.write_str(MSG_AUTH_EXPIRED),
            Self::TokenNotYetValid => f.write_str(MSG_AUTH_NOT_YET_VALID),
            Self::UnsupportedAlgorithm => f.write_str(MSG_AUTH_UNSUPPORTED_ALG),
            Self::ClaimMismatch => f.write_str(MSG_AUTH_CLAIM_MISMATCH),
            Self::Forbidden => f.write_str(MSG_AUTH_FORBIDDEN),
        }
    }
}

/// Implements [`Error`] for [`AuthError`].
impl Error for AuthError {}

/// Inherent implementation of [`JwtClaims`].
impl JwtClaims {
    /// Creates a minimal [`JwtClaims`] payload for `subject` expiring in `ttl_secs`.
    ///
    /// # Arguments
    ///
    /// - `S` - Subject identifier convertible into `String`.
    /// - `u64` - Lifetime in seconds from current Unix time.
    ///
    /// # Returns
    ///
    /// - `Self` - Constructed claims payload.
    pub fn new<S>(subject: S, ttl_secs: u64) -> Self
    where
        S: Into<String>,
    {
        let now: u64 = auth_now_secs();
        Self {
            sub: subject.into(),
            iss: None,
            aud: None,
            exp: Some(now.saturating_add(ttl_secs)),
            nbf: Some(now),
            iat: Some(now),
            jti: None,
            roles: Vec::new(),
            extra: HashMap::new(),
        }
    }

    /// Attaches RBAC roles to this claims payload.
    ///
    /// # Arguments
    ///
    /// - `I` - Iterator of role names.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated claims payload.
    pub fn with_roles<I, S>(mut self, roles: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.set_roles(roles.into_iter().map(Into::into).collect());
        self
    }

    /// Sets the `iss` issuer claim.
    ///
    /// # Arguments
    ///
    /// - `S` - Issuer identifier.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated claims payload.
    pub fn with_issuer<S>(mut self, issuer: S) -> Self
    where
        S: Into<String>,
    {
        self.set_iss(Some(issuer.into()));
        self
    }

    /// Sets the `aud` audience claim.
    ///
    /// # Arguments
    ///
    /// - `S` - Audience identifier.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated claims payload.
    pub fn with_audience<S>(mut self, audience: S) -> Self
    where
        S: Into<String>,
    {
        self.set_aud(Some(audience.into()));
        self
    }
}

/// Inherent implementation of [`JwtCodec`].
impl JwtCodec {
    /// Creates an HS256 [`JwtCodec`] signed and verified with `secret`.
    ///
    /// # Arguments
    ///
    /// - `S` - Shared HMAC-SHA256 secret bytes convertible into `Vec<u8>`.
    ///
    /// # Returns
    ///
    /// - `Self` - Ready JWT codec with 60 s default clock-skew leeway.
    pub fn new<S>(secret: S) -> Self
    where
        S: Into<Vec<u8>>,
    {
        Self {
            default_secret: secret.into(),
            default_kid: None,
            keyring: hash_map_xx_hash3_64(),
            leeway_secs: DEFAULT_JWT_LEEWAY_SECS,
            required_issuer: None,
            required_audience: None,
        }
    }

    /// Registers an additional verification key under `kid` for seamless key rotation.
    ///
    /// # Arguments
    ///
    /// - `K` - Key identifier convertible into `String`.
    /// - `S` - Secret bytes for this `kid`.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated codec.
    pub fn with_key<K, S>(mut self, kid: K, secret: S) -> Self
    where
        K: Into<String>,
        S: Into<Vec<u8>>,
    {
        let Self { keyring, .. } = &mut self;
        keyring.insert(kid.into(), secret.into());
        self
    }

    /// Sets the default `kid` stamped on newly encoded tokens (and registers its secret if absent).
    ///
    /// # Arguments
    ///
    /// - `K` - Active signing key identifier.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated codec.
    pub fn with_default_kid<K>(mut self, kid: K) -> Self
    where
        K: Into<String>,
    {
        let Self {
            default_kid,
            default_secret,
            keyring,
            ..
        } = &mut self;
        let kid_str: String = kid.into();
        keyring
            .entry(kid_str.clone())
            .or_insert_with(|| default_secret.clone());
        *default_kid = Some(kid_str);
        self
    }

    /// Sets the allowed clock-skew leeway in seconds for `exp` and `nbf` checks.
    ///
    /// # Arguments
    ///
    /// - `u64` - Leeway in seconds.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated codec.
    pub fn with_leeway_secs(mut self, leeway_secs: u64) -> Self {
        let Self {
            leeway_secs: slot, ..
        } = &mut self;
        *slot = leeway_secs;
        self
    }

    /// Requires every decoded token's `iss` claim to match `issuer`.
    ///
    /// # Arguments
    ///
    /// - `S` - Required issuer string.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated codec.
    pub fn with_required_issuer<S>(mut self, issuer: S) -> Self
    where
        S: Into<String>,
    {
        let Self {
            required_issuer, ..
        } = &mut self;
        *required_issuer = Some(issuer.into());
        self
    }

    /// Requires every decoded token's `aud` claim to match `audience`.
    ///
    /// # Arguments
    ///
    /// - `S` - Required audience string.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated codec.
    pub fn with_required_audience<S>(mut self, audience: S) -> Self
    where
        S: Into<String>,
    {
        let Self {
            required_audience, ..
        } = &mut self;
        *required_audience = Some(audience.into());
        self
    }

    /// Encodes and signs `claims` into a compact HS256 JWT (`header.payload.signature`).
    ///
    /// # Arguments
    ///
    /// - `&JwtClaims` - Claims payload to sign.
    ///
    /// # Returns
    ///
    /// - `Result<String, AuthError>` - Compact JWT string, or [`AuthError::InvalidCredentials`] on serialization failure.
    pub fn encode(&self, claims: &JwtClaims) -> Result<String, AuthError> {
        let Self {
            default_secret,
            default_kid,
            keyring,
            ..
        } = self;
        let header: JwtHeader = JwtHeader {
            alg: JWT_ALG_HS256.to_owned(),
            typ: Some(JWT_TYP_JWT.to_owned()),
            kid: default_kid.clone(),
        };
        let header_json: Vec<u8> = to_vec(&header).map_err(|_| AuthError::InvalidCredentials)?;
        let claims_json: Vec<u8> = to_vec(claims).map_err(|_| AuthError::InvalidCredentials)?;
        let header_b64: String = base64url_encode(&header_json);
        let claims_b64: String = base64url_encode(&claims_json);
        let mut signing_input: String =
            String::with_capacity(header_b64.len() + claims_b64.len() + 1);
        signing_input.push_str(&header_b64);
        signing_input.push('.');
        signing_input.push_str(&claims_b64);
        let secret: &[u8] = match default_kid {
            Some(kid) => keyring
                .get(kid)
                .map(Vec::as_slice)
                .unwrap_or(default_secret.as_slice()),
            None => default_secret.as_slice(),
        };
        let mac: [u8; 32] = hmac_sha256(secret, signing_input.as_bytes());
        let sig_b64: String = base64url_encode(&mac);
        let mut token: String = String::with_capacity(signing_input.len() + 1 + sig_b64.len());
        token.push_str(&signing_input);
        token.push('.');
        token.push_str(&sig_b64);
        Ok(token)
    }

    /// Verifies and decodes a compact HS256 JWT at timestamp `now_secs`.
    ///
    /// # Arguments
    ///
    /// - `&str` - Compact JWT string (`header.payload.signature`).
    /// - `u64` - Current Unix timestamp in seconds.
    ///
    /// # Returns
    ///
    /// - `Result<JwtClaims, AuthError>` - Verified [`JwtClaims`] or detailed [`AuthError`].
    pub fn decode_at(&self, token: &str, now_secs: u64) -> Result<JwtClaims, AuthError> {
        let Self {
            default_secret,
            keyring,
            leeway_secs,
            required_issuer,
            required_audience,
            ..
        } = self;
        let mut parts: Split<'_, char> = token.split('.');
        let header_b64: &str = parts.next().ok_or(AuthError::InvalidCredentials)?;
        let payload_b64: &str = parts.next().ok_or(AuthError::InvalidCredentials)?;
        let sig_b64: &str = parts.next().ok_or(AuthError::InvalidCredentials)?;
        if parts.next().is_some()
            || header_b64.is_empty()
            || payload_b64.is_empty()
            || sig_b64.is_empty()
        {
            return Err(AuthError::InvalidCredentials);
        }
        let header_bytes: Vec<u8> =
            base64url_decode(header_b64).ok_or(AuthError::InvalidCredentials)?;
        let header: JwtHeader =
            from_slice(&header_bytes).map_err(|_| AuthError::InvalidCredentials)?;
        if header.get_alg() != JWT_ALG_HS256 {
            return Err(AuthError::UnsupportedAlgorithm);
        }
        let secret: &[u8] = match header.get_kid() {
            Some(kid) => keyring
                .get(&kid)
                .map(Vec::as_slice)
                .ok_or(AuthError::InvalidCredentials)?,
            None => default_secret.as_slice(),
        };
        let provided_sig: Vec<u8> =
            base64url_decode(sig_b64).ok_or(AuthError::InvalidCredentials)?;
        let signed_len: usize = header_b64.len() + 1 + payload_b64.len();
        let expected_mac: [u8; 32] = hmac_sha256(secret, &token.as_bytes()[..signed_len]);
        if !constant_time_eq(&provided_sig, &expected_mac) {
            return Err(AuthError::InvalidCredentials);
        }
        let payload_bytes: Vec<u8> =
            base64url_decode(payload_b64).ok_or(AuthError::InvalidCredentials)?;
        let claims: JwtClaims =
            from_slice(&payload_bytes).map_err(|_| AuthError::InvalidCredentials)?;
        if let Some(exp) = claims.get_exp()
            && now_secs > exp.saturating_add(*leeway_secs)
        {
            return Err(AuthError::TokenExpired);
        }
        if let Some(nbf) = claims.get_nbf()
            && now_secs.saturating_add(*leeway_secs) < nbf
        {
            return Err(AuthError::TokenNotYetValid);
        }
        if let Some(expected_iss) = required_issuer
            && claims.get_iss().as_deref() != Some(expected_iss.as_str())
        {
            return Err(AuthError::ClaimMismatch);
        }
        if let Some(expected_aud) = required_audience
            && claims.get_aud().as_deref() != Some(expected_aud.as_str())
        {
            return Err(AuthError::ClaimMismatch);
        }
        Ok(claims)
    }

    /// Verifies and decodes a compact HS256 JWT against current system time.
    ///
    /// # Arguments
    ///
    /// - `&str` - Compact JWT string.
    ///
    /// # Returns
    ///
    /// - `Result<JwtClaims, AuthError>` - Verified [`JwtClaims`] or [`AuthError`].
    pub fn decode(&self, token: &str) -> Result<JwtClaims, AuthError> {
        self.decode_at(token, auth_now_secs())
    }
}

/// Inherent helpers for [`AuthPrincipal`].
impl AuthPrincipal {
    /// Checks whether this principal possesses `role`.
    ///
    /// # Arguments
    ///
    /// - `&str` - Role name to check.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` when the role is granted.
    pub fn has_role(&self, role: &str) -> bool {
        self.get_roles().iter().any(|item: &String| item == role)
    }
}

/// Inherent implementation of [`JwtAuthenticator`].
impl JwtAuthenticator {
    /// Creates a [`JwtAuthenticator`] backed by `codec`.
    ///
    /// # Arguments
    ///
    /// - `JwtCodec` - Configured HS256 codec.
    ///
    /// # Returns
    ///
    /// - `Self` - Bearer JWT authenticator.
    pub fn new(codec: JwtCodec) -> Self {
        Self { codec }
    }
}

/// Implements [`Authenticator`] for [`JwtAuthenticator`].
impl Authenticator for JwtAuthenticator {
    /// Authenticates `Authorization: Bearer <token>` requests via [`JwtCodec`].
    ///
    /// # Arguments
    ///
    /// - `&'a self` - Shared reference to self.
    /// - `&'a Context` - Incoming request context.
    ///
    /// # Returns
    ///
    /// - `AuthFuture<'a>` - Future resolving to [`AuthPrincipal`] or [`AuthError`].
    fn authenticate<'a>(&'a self, ctx: &'a Context) -> AuthFuture<'a> {
        let Self { codec } = self;
        let result: Result<AuthPrincipal, AuthError> =
            match ctx.get_request().try_get_header_back(AUTHORIZATION) {
                None => Err(AuthError::MissingCredentials),
                Some(header) => match strip_auth_scheme(&header, PREFIX_BEARER) {
                    None => Err(AuthError::MissingCredentials),
                    Some(token) => codec.decode(token).map(|claims: JwtClaims| AuthPrincipal {
                        subject: claims.get_sub().clone(),
                        roles: claims.get_roles().clone(),
                        scheme: AuthScheme::BearerJwt,
                        claims: Some(claims),
                    }),
                },
            };
        Box::pin(async move { result })
    }
}

/// Default construction for [`ApiKeyAuthenticator`].
impl Default for ApiKeyAuthenticator {
    /// Constructs an empty [`ApiKeyAuthenticator`] reading `X-Api-Key`.
    fn default() -> Self {
        Self::new(X_API_KEY)
    }
}

/// Inherent implementation of [`ApiKeyAuthenticator`].
impl ApiKeyAuthenticator {
    /// Creates an [`ApiKeyAuthenticator`] reading `header_name`.
    ///
    /// # Arguments
    ///
    /// - `H` - Header name convertible into `String`.
    ///
    /// # Returns
    ///
    /// - `Self` - Empty API-key authenticator.
    pub fn new<H>(header_name: H) -> Self
    where
        H: Into<String>,
    {
        Self {
            header_name: header_name.into(),
            entries: Vec::new(),
        }
    }

    /// Registers an allowed API key associated with `subject` and `roles`.
    ///
    /// # Arguments
    ///
    /// - `K` - Secret API key string.
    /// - `S` - Subject name assigned when matched.
    /// - `I` - Iterator of role names.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated authenticator.
    pub fn with_key<K, S, I, R>(mut self, api_key: K, subject: S, roles: I) -> Self
    where
        K: Into<String>,
        S: Into<String>,
        I: IntoIterator<Item = R>,
        R: Into<String>,
    {
        let Self { entries, .. } = &mut self;
        entries.push((
            api_key.into().into_bytes(),
            subject.into(),
            roles.into_iter().map(Into::into).collect(),
        ));
        self
    }
}

/// Implements [`Authenticator`] for [`ApiKeyAuthenticator`].
impl Authenticator for ApiKeyAuthenticator {
    /// Authenticates incoming API key headers using constant-time key comparisons.
    ///
    /// # Arguments
    ///
    /// - `&'a self` - Shared reference to self.
    /// - `&'a Context` - Incoming request context.
    ///
    /// # Returns
    ///
    /// - `AuthFuture<'a>` - Future resolving to [`AuthPrincipal`] or [`AuthError`].
    fn authenticate<'a>(&'a self, ctx: &'a Context) -> AuthFuture<'a> {
        let Self {
            header_name,
            entries,
        } = self;
        let result: Result<AuthPrincipal, AuthError> =
            match ctx.get_request().try_get_header_back(header_name) {
                None => Err(AuthError::MissingCredentials),
                Some(candidate) => {
                    let candidate_bytes: &[u8] = candidate.trim().as_bytes();
                    let mut matched: Option<AuthPrincipal> = None;
                    for (secret, subject, roles) in entries {
                        if constant_time_eq(candidate_bytes, secret) {
                            matched = Some(AuthPrincipal {
                                subject: subject.clone(),
                                roles: roles.clone(),
                                scheme: AuthScheme::ApiKey,
                                claims: None,
                            });
                        }
                    }
                    matched.ok_or(AuthError::InvalidCredentials)
                }
            };
        Box::pin(async move { result })
    }
}

/// Default construction for [`BasicAuthenticator`].
impl Default for BasicAuthenticator {
    /// Constructs an empty [`BasicAuthenticator`].
    fn default() -> Self {
        Self::new()
    }
}

/// Inherent implementation of [`BasicAuthenticator`].
impl BasicAuthenticator {
    /// Creates an empty [`BasicAuthenticator`].
    ///
    /// # Returns
    ///
    /// - `Self` - Empty HTTP Basic authenticator.
    pub fn new() -> Self {
        Self {
            users: hash_map_xx_hash3_64(),
        }
    }

    /// Registers a Basic-auth user with `username`, `password`, and `roles`.
    ///
    /// # Arguments
    ///
    /// - `U` - Username string.
    /// - `P` - Password string.
    /// - `I` - Iterator of role names.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated authenticator.
    pub fn with_user<U, P, I, R>(mut self, username: U, password: P, roles: I) -> Self
    where
        U: Into<String>,
        P: Into<String>,
        I: IntoIterator<Item = R>,
        R: Into<String>,
    {
        let Self { users } = &mut self;
        users.insert(
            username.into(),
            (
                password.into().into_bytes(),
                roles.into_iter().map(Into::into).collect(),
            ),
        );
        self
    }
}

/// Implements [`Authenticator`] for [`BasicAuthenticator`].
impl Authenticator for BasicAuthenticator {
    /// Authenticates `Authorization: Basic <base64>` using constant-time password checks.
    ///
    /// # Arguments
    ///
    /// - `&'a self` - Shared reference to self.
    /// - `&'a Context` - Incoming request context.
    ///
    /// # Returns
    ///
    /// - `AuthFuture<'a>` - Future resolving to [`AuthPrincipal`] or [`AuthError`].
    fn authenticate<'a>(&'a self, ctx: &'a Context) -> AuthFuture<'a> {
        let Self { users } = self;
        let result: Result<AuthPrincipal, AuthError> =
            match ctx.get_request().try_get_header_back(AUTHORIZATION) {
                None => Err(AuthError::MissingCredentials),
                Some(header) => match strip_auth_scheme(&header, PREFIX_BASIC) {
                    None => Err(AuthError::MissingCredentials),
                    Some(encoded) => match base64_decode(encoded)
                        .and_then(|raw: Vec<u8>| String::from_utf8(raw).ok())
                    {
                        None => Err(AuthError::InvalidCredentials),
                        Some(decoded) => match decoded.split_once(':') {
                            None => Err(AuthError::InvalidCredentials),
                            Some((user, pass)) => match users.get(user) {
                                Some((expected_pass, roles))
                                    if constant_time_eq(pass.as_bytes(), expected_pass) =>
                                {
                                    Ok(AuthPrincipal {
                                        subject: user.to_owned(),
                                        roles: roles.clone(),
                                        scheme: AuthScheme::Basic,
                                        claims: None,
                                    })
                                }
                                _ => Err(AuthError::InvalidCredentials),
                            },
                        },
                    },
                },
            };
        Box::pin(async move { result })
    }
}

/// Inherent implementation of [`CompositeAuthenticator`].
impl CompositeAuthenticator {
    /// Creates a [`CompositeAuthenticator`] chaining `chain` in priority order.
    ///
    /// # Arguments
    ///
    /// - `Vec<SharedAuthenticator>` - Ordered list of authenticators.
    ///
    /// # Returns
    ///
    /// - `Self` - Composite authenticator.
    pub fn new(chain: Vec<SharedAuthenticator>) -> Self {
        Self { chain }
    }
}

/// Implements [`Authenticator`] for [`CompositeAuthenticator`].
impl Authenticator for CompositeAuthenticator {
    /// Evaluates each authenticator in order, returning the first verified [`AuthPrincipal`].
    ///
    /// # Arguments
    ///
    /// - `&'a self` - Shared reference to self.
    /// - `&'a Context` - Incoming request context.
    ///
    /// # Returns
    ///
    /// - `AuthFuture<'a>` - Future resolving to the first matching [`AuthPrincipal`] or last non-missing error.
    fn authenticate<'a>(&'a self, ctx: &'a Context) -> AuthFuture<'a> {
        let Self { chain } = self;
        Box::pin(async move {
            let mut last_err: AuthError = AuthError::MissingCredentials;
            for auth in chain {
                match auth.authenticate(ctx).await {
                    Ok(principal) => return Ok(principal),
                    Err(AuthError::MissingCredentials) => {}
                    Err(err) => last_err = err,
                }
            }
            Err(last_err)
        })
    }
}

/// Default construction for [`AuthConfig`].
impl Default for AuthConfig {
    /// Constructs a default [`AuthConfig`] with Bearer realm challenge and no exempt paths.
    fn default() -> Self {
        Self {
            exempt_paths: Vec::new(),
            required_roles: Vec::new(),
            require_all_roles: false,
            www_authenticate: DEFAULT_WWW_AUTHENTICATE.to_owned(),
        }
    }
}

/// Inherent builders for [`AuthConfig`].
impl AuthConfig {
    /// Configures exact paths excluded from authentication checks.
    ///
    /// # Arguments
    ///
    /// - `I` - Iterator of exempt paths.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated configuration.
    pub fn with_exempt_paths<I, S>(mut self, paths: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.set_exempt_paths(paths.into_iter().map(Into::into).collect());
        self
    }

    /// Configures roles required to access protected routes.
    ///
    /// # Arguments
    ///
    /// - `I` - Iterator of required role names.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated configuration.
    pub fn with_required_roles<I, S>(mut self, roles: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.set_required_roles(roles.into_iter().map(Into::into).collect());
        self
    }
}

/// Inherent implementation of [`AuthPlugin`].
impl AuthPlugin {
    /// Initializes the global [`AuthPlugin`] once with `config` and `authenticator`.
    ///
    /// # Arguments
    ///
    /// - `AuthConfig` - Path-exclusion and RBAC policy.
    /// - `SharedAuthenticator` - Pluggable authenticator implementation.
    ///
    /// # Returns
    ///
    /// - `&'static Self` - Reference to the initialized global plugin.
    pub fn init(config: AuthConfig, authenticator: SharedAuthenticator) -> &'static Self {
        AUTH_PLUGIN.get_or_init(|| Self {
            config,
            authenticator,
        })
    }

    /// Returns the initialized global [`AuthPlugin`], if initialized.
    ///
    /// # Returns
    ///
    /// - `Option<&'static Self>` - Reference to the global plugin, or `None`.
    pub fn get() -> Option<&'static Self> {
        AUTH_PLUGIN.get()
    }

    /// Returns the active [`AuthConfig`].
    ///
    /// # Returns
    ///
    /// - `&AuthConfig` - Active authorization policy.
    pub fn get_config(&self) -> &AuthConfig {
        &self.config
    }

    /// Returns the active [`SharedAuthenticator`].
    ///
    /// # Returns
    ///
    /// - `&SharedAuthenticator` - Active authenticator.
    pub fn get_authenticator(&self) -> &SharedAuthenticator {
        &self.authenticator
    }

    /// Retrieves the verified [`AuthPrincipal`] previously stored in `ctx` by [`AuthHook`].
    ///
    /// # Arguments
    ///
    /// - `&Context` - Request context.
    ///
    /// # Returns
    ///
    /// - `Option<AuthPrincipal>` - Verified caller principal, if authenticated.
    pub fn principal(ctx: &Context) -> Option<AuthPrincipal> {
        ctx.try_get_attribute(ATTR_AUTH_PRINCIPAL)
    }

    /// Checks whether `principal` satisfies the configured RBAC role requirements.
    ///
    /// # Arguments
    ///
    /// - `&AuthPrincipal` - Verified principal.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` when authorized.
    pub fn is_authorized(&self, principal: &AuthPrincipal) -> bool {
        let required: &Vec<String> = self.get_config().get_required_roles();
        if required.is_empty() {
            return true;
        }
        if self.get_config().get_require_all_roles() {
            required
                .iter()
                .all(|role: &String| principal.has_role(role))
        } else {
            required
                .iter()
                .any(|role: &String| principal.has_role(role))
        }
    }
}

/// Implements [`ServerHook`] for [`AuthHook`].
impl ServerHook for AuthHook {
    /// Constructs a per-request [`AuthHook`].
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

    /// Authenticates protected requests, verifies required roles, and attaches [`AuthPrincipal`].
    ///
    /// # Arguments
    ///
    /// - `Self` - Hook instance.
    /// - `&mut Stream` - Connection stream.
    /// - `&mut Context` - Request context.
    ///
    /// # Returns
    ///
    /// - `Status` - `Status::Continue` when unconfigured, exempt, or authorized; `Status::Reject` on `401`/`403`.
    async fn handle(self, stream: &mut Stream, ctx: &mut Context) -> Status {
        let Some(plugin) = AuthPlugin::get() else {
            return Status::Continue;
        };
        let path: &str = ctx.get_request().get_path();
        if plugin
            .get_config()
            .get_exempt_paths()
            .iter()
            .any(|exempt: &String| exempt == path)
        {
            return Status::Continue;
        }
        match plugin.get_authenticator().authenticate(ctx).await {
            Ok(principal) => {
                if !plugin.is_authorized(&principal) {
                    ctx.get_mut_response()
                        .set_status_code(HttpStatus::Forbidden.code())
                        .set_body(Vec::new());
                    let wire: ResponseData = ctx.get_mut_response().build();
                    let _: Result<(), ResponseError> = stream.try_send(&wire).await;
                    return Status::Reject;
                }
                ctx.set_attribute(ATTR_AUTH_PRINCIPAL, principal);
                Status::Continue
            }
            Err(AuthError::Forbidden) => {
                ctx.get_mut_response()
                    .set_status_code(HttpStatus::Forbidden.code())
                    .set_body(Vec::new());
                let wire: ResponseData = ctx.get_mut_response().build();
                let _: Result<(), ResponseError> = stream.try_send(&wire).await;
                Status::Reject
            }
            Err(_) => {
                ctx.get_mut_response()
                    .set_status_code(HttpStatus::Unauthorized.code())
                    .set_header(WWW_AUTHENTICATE, plugin.get_config().get_www_authenticate())
                    .set_body(Vec::new());
                let wire: ResponseData = ctx.get_mut_response().build();
                let _: Result<(), ResponseError> = stream.try_send(&wire).await;
                Status::Reject
            }
        }
    }
}
