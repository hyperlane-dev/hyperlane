/// The path of the static route.
pub(crate) const ROUTE_ROOT: &str = "/";

/// The path of the dynamic route.
pub(crate) const ROUTE_USER: &str = "/api/user/{id}";

/// The path of the query route.
pub(crate) const ROUTE_SEARCH: &str = "/api/search";

/// The path of the header route.
pub(crate) const ROUTE_HEADERS: &str = "/api/headers";

/// The path of the body route.
pub(crate) const ROUTE_ECHO: &str = "/api/echo";

/// The body the static route answers with.
pub(crate) const BODY_ROOT: &[u8] = b"hello hyperlane benchmark";

/// The body the dynamic route answers with, before the path parameter.
pub(crate) const BODY_USER: &[u8] = b"user ";

/// The name of the path parameter the dynamic route extracts.
pub(crate) const PARAM_ID: &str = "id";

/// The name of the query parameter the query route reads.
pub(crate) const PARAM_QUERY: &str = "q";

/// The name of the header the header route reads.
pub(crate) const HEADER_TAG: &str = "x-bench-tag";

/// The environment variable that names the address to bind.
pub(crate) const ENV_ADDRESS: &str = "BENCH_ADDR";

/// The address the server binds when the environment names none.
pub(crate) const DEFAULT_ADDRESS: &str = "0.0.0.0:8080";

/// The status code every route answers with.
pub(crate) const STATUS_OK: usize = 200;
