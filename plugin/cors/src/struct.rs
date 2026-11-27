use super::*;

/// User-facing configuration for [`CorsPlugin`].
#[derive(Clone, Data, Debug)]
pub struct CorsConfig {
    /// Origin matching rule controlling `Access-Control-Allow-Origin`.
    pub(crate) allow_origin: AllowOrigin,
    /// Allowed HTTP methods emitted in preflight `Access-Control-Allow-Methods`.
    pub(crate) allow_methods: Vec<String>,
    /// Allowed request headers emitted in preflight `Access-Control-Allow-Headers` (`*` or empty reflects requested headers).
    pub(crate) allow_headers: Vec<String>,
    /// Response headers exposed to browser scripts via `Access-Control-Expose-Headers`.
    pub(crate) expose_headers: Vec<String>,
    /// Whether `Access-Control-Allow-Credentials: true` is emitted.
    #[get(type(copy))]
    pub(crate) allow_credentials: bool,
    /// Optional preflight cache lifetime in seconds (`Access-Control-Max-Age`).
    #[get(type(copy))]
    pub(crate) max_age_secs: Option<u64>,
    /// Whether to answer `Access-Control-Request-Private-Network: true` with `Access-Control-Allow-Private-Network: true`.
    #[get(type(copy))]
    pub(crate) allow_private_network: bool,
}

/// Precompiled zero-allocation CORS policy stored inside [`CorsPlugin`].
#[derive(Clone, Debug)]
pub struct CompiledCors {
    /// Original configuration retained for inspection and origin matching.
    pub(crate) config: CorsConfig,
    /// Pre-joined `Access-Control-Allow-Methods` value.
    pub(crate) methods_header: String,
    /// Pre-joined `Access-Control-Allow-Headers` value (`None` means mirror requested headers).
    pub(crate) headers_header: Option<String>,
    /// Pre-joined `Access-Control-Expose-Headers` value.
    pub(crate) expose_header: Option<String>,
    /// Pre-formatted `Access-Control-Max-Age` value.
    pub(crate) max_age_header: Option<String>,
    /// `true` when `Access-Control-Allow-Origin` varies by request `Origin`.
    pub(crate) vary_origin: bool,
}

/// Process-wide [`CorsPlugin`] singleton holding a precompiled [`CompiledCors`] policy.
#[derive(Clone, Debug)]
pub struct CorsPlugin {
    /// Precompiled CORS header policy.
    pub(crate) compiled: CompiledCors,
}

/// Opt-in request middleware applying CORS headers and answering `OPTIONS` preflight requests.
#[derive(Clone, Copy, Debug, Default)]
#[hyperlane::request_middleware(-100)]
pub struct CorsRequestHook;

/// Opt-in response middleware guaranteeing CORS headers persist even if a route replaced headers.
#[derive(Clone, Copy, Debug, Default)]
#[hyperlane::response_middleware(-80)]
pub struct CorsResponseHook;
