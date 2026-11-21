use super::*;

/// Represents the internal state of the application context.
///
/// This structure holds all the data associated with a single request-response cycle,
/// including the request, response, and any custom attributes.
#[derive(Clone, CustomDebug, Data, DisplayDebug)]
pub struct Context {
    /// The incoming HTTP request.
    pub(crate) request: Request,
    /// The outgoing HTTP response.
    pub(crate) response: Response,
    /// Parameters extracted from the route path.
    pub(crate) route_params: RouteParams,
    /// The original wire request target, before routing or query-map conversion.
    ///
    /// The target is read on every request and written on every request, so it
    /// lives in its own field rather than in the attribute map: the map would
    /// cost a hash, an allocation and a downcast for every request, and the
    /// attribute map is for the attributes the application sets.
    #[get_mut(pub(super))]
    #[set(pub(super))]
    pub(super) original_target: Option<String>,
    /// Whether the original wire method was HEAD, independent of route rewrites.
    #[get(pub(crate))]
    #[get_mut(skip)]
    #[set(pub(crate))]
    pub(crate) original_head: Option<bool>,
    /// A collection of custom attributes for sharing data within the request lifecycle.
    pub(crate) attributes: ThreadSafeAttributeStore,
}
