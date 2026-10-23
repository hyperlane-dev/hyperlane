use super::*;

/// The configuration of the cross origin plugin.
#[derive(Clone, Debug, Getter)]
pub struct CorsConfig {
    /// The origins that are allowed, or one wildcard for every origin.
    pub allowed_origins: Vec<String>,
    /// The methods that are allowed.
    pub allowed_methods: Vec<String>,
    /// The request headers that are allowed.
    pub allowed_headers: Vec<String>,
    /// The seconds a browser may cache the answer of a preflight request.
    #[get(type(copy))]
    pub max_age: Option<u64>,
    /// Whether the answer may carry credentials.
    #[get(type(copy))]
    pub allow_credentials: bool,
}

/// The plugin that answers the browsers that ask for another origin.
#[derive(Clone, Copy, Debug, Default)]
pub struct CorsPlugin;

/// The middleware that writes the cross origin headers.
#[derive(Clone, Copy, Debug, Default)]
#[hyperlane::request_middleware(-110)]
pub struct CorsMiddleware;
