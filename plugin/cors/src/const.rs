/// Wildcard value used for `Access-Control-Allow-Origin` and header lists.
pub const CORS_WILDCARD: &str = "*";

/// Standard `Access-Control-Allow-Private-Network` response header name.
pub const ACCESS_CONTROL_ALLOW_PRIVATE_NETWORK: &str = "access-control-allow-private-network";

/// Standard `Access-Control-Request-Private-Network` request header name.
pub const ACCESS_CONTROL_REQUEST_PRIVATE_NETWORK: &str = "access-control-request-private-network";

/// Default preflight cache duration in seconds (10 minutes).
pub const DEFAULT_CORS_MAX_AGE_SECS: u64 = 600;

/// Header value emitted for boolean `true` CORS directives.
pub(crate) const HEADER_VALUE_TRUE: &str = "true";

/// Separator used when joining multiple CORS header tokens.
pub(crate) const COMMA_SPACE: &str = ", ";

/// `Vary` token representing the incoming `Origin` header.
pub(crate) const VARY_ORIGIN: &str = "Origin";

/// `Vary` token for preflight request method negotiation.
pub(crate) const VARY_PREFLIGHT_METHOD: &str = "Access-Control-Request-Method";

/// `Vary` token for preflight request header negotiation.
pub(crate) const VARY_PREFLIGHT_HEADERS: &str = "Access-Control-Request-Headers";

/// Default allowed HTTP methods when none are explicitly configured.
pub(crate) const DEFAULT_ALLOWED_METHODS: [&str; 6] =
    ["GET", "POST", "PUT", "PATCH", "DELETE", "HEAD"];

/// Default allowed request headers when none are explicitly configured.
pub(crate) const DEFAULT_ALLOWED_HEADERS: [&str; 3] =
    ["content-type", "authorization", "x-requested-with"];

/// Debug label rendered for `AllowOrigin::Any`.
pub(crate) const DEBUG_ALLOW_ORIGIN_ANY: &str = "Any";

/// Debug label rendered for `AllowOrigin::Exact`.
pub(crate) const DEBUG_ALLOW_ORIGIN_EXACT: &str = "Exact";

/// Debug label rendered for `AllowOrigin::Suffix`.
pub(crate) const DEBUG_ALLOW_ORIGIN_SUFFIX: &str = "Suffix";

/// Debug label rendered for `AllowOrigin::Custom`.
pub(crate) const DEBUG_ALLOW_ORIGIN_CUSTOM: &str = "Custom(<fn>)";

/// Context attribute key marking that CORS headers were already applied during request handling.
pub(crate) const ATTR_CORS_ORIGIN_APPLIED: &str = "hyperlane_plugin_cors_origin";
