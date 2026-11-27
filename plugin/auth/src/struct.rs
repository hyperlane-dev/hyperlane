use super::*;

/// JWT JWS header decoded from the first segment of a compact token.
#[derive(Clone, Data, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct JwtHeader {
    /// JWS algorithm identifier (must be `"HS256"`).
    pub(crate) alg: String,
    /// Optional token type (`"JWT"`).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[get(type(clone))]
    pub(crate) typ: Option<String>,
    /// Optional key identifier used for zero-downtime key rotation.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[get(type(clone))]
    pub(crate) kid: Option<String>,
}

/// Standard and extensible JSON Web Token claims payload.
#[derive(Clone, Data, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct JwtClaims {
    /// Subject identifier (`sub`).
    pub(crate) sub: String,
    /// Optional issuer (`iss`).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[get(type(clone))]
    pub(crate) iss: Option<String>,
    /// Optional audience (`aud`).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[get(type(clone))]
    pub(crate) aud: Option<String>,
    /// Optional expiration Unix timestamp in seconds (`exp`).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[get(type(copy))]
    pub(crate) exp: Option<u64>,
    /// Optional not-before Unix timestamp in seconds (`nbf`).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[get(type(copy))]
    pub(crate) nbf: Option<u64>,
    /// Optional issued-at Unix timestamp in seconds (`iat`).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[get(type(copy))]
    pub(crate) iat: Option<u64>,
    /// Optional unique JWT identifier (`jti`).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[get(type(clone))]
    pub(crate) jti: Option<String>,
    /// Optional role list used for RBAC authorization checks.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub(crate) roles: Vec<String>,
    /// Arbitrary additional custom claims flattened into the JWT JSON object.
    #[serde(flatten, default, skip_serializing_if = "HashMap::is_empty")]
    pub(crate) extra: HashMap<String, Value>,
}

/// High-performance HS256 JWT signer and verifier with `kid` key-rotation support.
#[derive(Clone, Debug)]
pub struct JwtCodec {
    /// Primary signing secret used when no `kid` is specified.
    pub(crate) default_secret: Vec<u8>,
    /// Optional default `kid` stamped into minted tokens.
    pub(crate) default_kid: Option<String>,
    /// Key rotation keyring indexed by `kid` (`XxHash3_64`).
    pub(crate) keyring: HashMapXxHash3_64<String, Vec<u8>>,
    /// Clock-skew leeway in seconds applied to `exp` and `nbf`.
    pub(crate) leeway_secs: u64,
    /// Optional required `iss` value.
    pub(crate) required_issuer: Option<String>,
    /// Optional required `aud` value.
    pub(crate) required_audience: Option<String>,
}

/// Verified caller identity attached to a Hyperlane [`Context`] by [`AuthHook`].
#[derive(Clone, Data, Debug, PartialEq)]
pub struct AuthPrincipal {
    /// Authenticated user or service subject identifier.
    pub(crate) subject: String,
    /// Granted RBAC roles.
    pub(crate) roles: Vec<String>,
    /// Scheme that authenticated this principal.
    #[get(type(copy))]
    pub(crate) scheme: AuthScheme,
    /// Decoded JWT claims when authenticated via [`AuthScheme::BearerJwt`].
    #[get(type(clone))]
    pub(crate) claims: Option<JwtClaims>,
}

/// Built-in [`Authenticator`] validating `Authorization: Bearer <jwt>` via [`JwtCodec`].
#[derive(Clone, Debug)]
pub struct JwtAuthenticator {
    /// Compiled JWT codec and validation policy.
    pub(crate) codec: JwtCodec,
}

/// Built-in [`Authenticator`] validating static or rotated API keys from a configured header.
#[derive(Clone, Debug)]
pub struct ApiKeyAuthenticator {
    /// Header name carrying the API key (default `X-Api-Key`).
    pub(crate) header_name: String,
    /// Registered `(api_key, subject, roles)` records scanned with constant-time comparison.
    pub(crate) entries: Vec<(Vec<u8>, String, Vec<String>)>,
}

/// Built-in [`Authenticator`] validating `Authorization: Basic <base64>` credentials.
#[derive(Clone, Debug)]
pub struct BasicAuthenticator {
    /// Registered `(username, password_bytes, roles)` records.
    pub(crate) users: HashMapXxHash3_64<String, (Vec<u8>, Vec<String>)>,
}

/// Composite [`Authenticator`] trying a sequence of authenticators until one succeeds.
#[derive(Clone)]
pub struct CompositeAuthenticator {
    /// Ordered authenticator chain.
    pub(crate) chain: Vec<SharedAuthenticator>,
}

/// Configuration policy for [`AuthPlugin`] and [`AuthHook`].
#[derive(Clone, Data, Debug, Eq, PartialEq)]
pub struct AuthConfig {
    /// Exact paths bypassed without authentication (for example `"/healthz"`, `"/login"`).
    pub(crate) exempt_paths: Vec<String>,
    /// Optional roles required on every protected request (any match permits if `require_all_roles` is `false`).
    pub(crate) required_roles: Vec<String>,
    /// When `true`, caller must hold every role in `required_roles`; when `false`, any single match suffices.
    #[get(type(copy))]
    pub(crate) require_all_roles: bool,
    /// `WWW-Authenticate` header value emitted on `401 Unauthorized`.
    pub(crate) www_authenticate: String,
}

/// Process-wide [`AuthPlugin`] singleton storing [`AuthConfig`] and the active [`Authenticator`].
#[derive(Clone)]
pub struct AuthPlugin {
    /// Authorization and path-exclusion configuration.
    pub(crate) config: AuthConfig,
    /// Active authenticator implementation.
    pub(crate) authenticator: SharedAuthenticator,
}

/// Opt-in [`ServerHook`] authenticating incoming requests and populating [`ATTR_AUTH_PRINCIPAL`].
#[derive(Clone, Copy, Debug, Default)]
#[hyperlane::request_middleware(-90)]
pub struct AuthHook;
