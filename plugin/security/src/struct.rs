use super::*;

/// Configurable HTTP security response headers.
#[derive(Clone, Data, Debug, Eq, PartialEq)]
pub struct SecurityHeadersConfig {
    /// `Strict-Transport-Security` value.
    #[get(type(clone))]
    pub(crate) hsts: Option<String>,
    /// `Content-Security-Policy` value.
    #[get(type(clone))]
    pub(crate) content_security_policy: Option<String>,
    /// `X-Frame-Options` value.
    #[get(type(clone))]
    pub(crate) x_frame_options: Option<String>,
    /// `X-Content-Type-Options` value.
    #[get(type(clone))]
    pub(crate) x_content_type_options: Option<String>,
    /// `Referrer-Policy` value.
    #[get(type(clone))]
    pub(crate) referrer_policy: Option<String>,
    /// `Permissions-Policy` value.
    #[get(type(clone))]
    pub(crate) permissions_policy: Option<String>,
    /// `Cross-Origin-Opener-Policy` value.
    #[get(type(clone))]
    pub(crate) coop: Option<String>,
    /// `Cross-Origin-Embedder-Policy` value.
    #[get(type(clone))]
    pub(crate) coep: Option<String>,
    /// `Cross-Origin-Resource-Policy` value.
    #[get(type(clone))]
    pub(crate) corp: Option<String>,
    /// Strips `X-Powered-By` from outgoing responses when `true`.
    #[get(type(copy))]
    pub(crate) remove_powered_by: bool,
}

/// Stateless double-submit HMAC-SHA256 CSRF protection settings.
#[derive(Clone, Data, Debug, Eq, PartialEq)]
pub struct CsrfConfig {
    /// Enables CSRF enforcement on state-changing HTTP methods (`POST`, `PUT`, `PATCH`, `DELETE`).
    #[get(type(copy))]
    pub(crate) enabled: bool,
    /// Server-side HMAC-SHA256 signing secret.
    pub(crate) secret: Vec<u8>,
    /// Cookie name holding the signed CSRF token.
    pub(crate) cookie_name: String,
    /// Request header name carrying the matching CSRF token.
    pub(crate) header_name: String,
    /// Exact paths exempted from CSRF checks (for example webhook callbacks).
    pub(crate) exempt_paths: Vec<String>,
}

/// Built-in request guard limits protecting routing and handlers from malformed traffic.
#[derive(Clone, Data, Debug, Eq, PartialEq)]
pub struct RequestGuardConfig {
    /// Maximum allowed request body length in bytes (`0` disables this plugin-level cap).
    #[get(type(copy))]
    pub(crate) max_body_bytes: usize,
    /// Maximum allowed request URI path length in bytes (`0` disables cap).
    #[get(type(copy))]
    pub(crate) max_uri_bytes: usize,
    /// Optional case-insensitive `Host` header allowlist (empty allows any host).
    pub(crate) allowed_hosts: Vec<String>,
    /// Blocks `..`, `%2e%2e`, backslashes, and NUL bytes in request paths when `true`.
    #[get(type(copy))]
    pub(crate) block_path_traversal: bool,
}

/// Unified security hardening configuration for [`SecurityPlugin`].
#[derive(Clone, Data, Debug, Default, Eq, PartialEq)]
pub struct SecurityConfig {
    /// Outgoing HTTP security headers configuration.
    pub(crate) headers: SecurityHeadersConfig,
    /// Double-submit HMAC CSRF protection settings.
    pub(crate) csrf: CsrfConfig,
    /// Incoming request guard settings.
    pub(crate) guard: RequestGuardConfig,
}

/// Process-wide [`SecurityPlugin`] singleton holding prebuilt headers, guards, and custom rules.
#[derive(Clone)]
pub struct SecurityPlugin {
    /// Source security configuration.
    pub(crate) config: SecurityConfig,
    /// Prebuilt `(header_name, header_value)` list applied without per-request allocation.
    pub(crate) compiled_headers: Arc<Vec<(&'static str, String)>>,
    /// Extensible custom security rules evaluated on each incoming request.
    pub(crate) rules: Arc<Vec<SharedSecurityRule>>,
}

/// Opt-in request middleware enforcing guards, CSRF checks, custom rules, and applying security headers.
#[derive(Clone, Copy, Debug, Default)]
#[hyperlane::request_middleware(-105)]
pub struct SecurityRequestHook;

/// Opt-in response middleware guaranteeing security headers and stripping `X-Powered-By`.
#[derive(Clone, Copy, Debug, Default)]
#[hyperlane::response_middleware(-75)]
pub struct SecurityResponseHook;
