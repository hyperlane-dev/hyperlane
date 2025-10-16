use crate::*;

/// Type alias for a thread-safe, type-erased handler function.
///
/// This type allows storing handlers (routes and middleware) of different concrete types
/// in the same collection. The handler takes a `DefaultInitialHook` and returns
/// a pinned, boxed future that resolves to `()`.
pub(crate) type ArcPinBoxFutureSendSync =
    Arc<dyn Fn(DefaultInitialHook) -> PinBoxFutureSend<()> + Send + Sync>;

/// Type alias for an optional handler function.
///
/// This type allows storing optional handlers of different concrete types in
/// the same collection. The handler takes a `DefaultInitialHook` and returns
/// a pinned, boxed future that resolves to `()`.
pub(crate) type OptionArcPinBoxFutureSendSync = Option<ArcPinBoxFutureSendSync>;

/// Type alias for a vector of route handlers with their patterns.
///
/// Used to store multiple route handlers in the routing system.
pub(crate) type VecArcPinBoxFutureSendSync = Vec<(RoutePattern, ArcPinBoxFutureSendSync)>;

/// Type alias for a vector of handler functions.
///
/// Used to store middleware handlers in the request/response processing pipeline.
pub(crate) type VecHandlers = Vec<ArcPinBoxFutureSendSync>;

/// Type alias for a hash map of static route paths to handlers.
///
/// Used for fast lookup of exact-match routes.
pub(crate) type HashMapStringArcPinBoxFutureSendSync =
    HashMapXxHash3_64<String, ArcPinBoxFutureSendSync>;
