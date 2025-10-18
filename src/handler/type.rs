use crate::*;

/// Type alias for a shared server hook handler.
///
/// This type allows storing handlers (routes and middleware) of different concrete types
/// in the same collection. The handler takes a `&Context` and returns
/// a pinned, boxed future that resolves to `()`.
pub type ServerHookHandler = Arc<dyn Fn(&Context) -> SendableAsyncTask<()> + Send + Sync>;
/// Type alias for an optional server hook handler.
///
/// This type allows storing optional handlers of different concrete types in
/// the same collection. The handler takes a `&Context` and returns
/// a pinned, boxed future that resolves to `()`.
pub type OptionalServerHookHandler = Option<ServerHookHandler>;
/// Type alias for a list of server hooks.
///
/// Used to store middleware handlers in the request/response processing pipeline.
pub type ServerHookList = Vec<ServerHookHandler>;
/// Type alias for a map of server hook handlers.
///
/// Used for fast lookup of exact-match routes.
pub type ServerHookMap = HashMapXxHash3_64<String, ServerHookHandler>;
/// Type alias for a collection of pattern-based server hook routes.
///
/// Used to store dynamic and regex route handlers with their matching patterns.
pub(crate) type ServerHookPatternRoutes = Vec<(RoutePattern, ServerHookHandler)>;
