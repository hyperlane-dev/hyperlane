use crate::*;

/// Represents the internal state of the application context.
///
/// This structure holds all the data associated with a single request-response cycle,
/// including the request, response, and any custom attributes.
#[derive(Clone, CustomDebug, Data, DisplayDebug)]
pub struct Context {
    /// The incoming HTTP request.
    #[get_mut(skip)]
    #[set(pub(crate))]
    pub(super) request: Request,
    /// The outgoing HTTP response.
    pub(super) response: Response,
    /// Parameters extracted from the route path.
    #[get_mut(skip)]
    #[set(pub(crate))]
    pub(super) route_params: RouteParams,
    /// A collection of custom attributes for sharing data within the request lifecycle.
    #[get_mut(pub(super))]
    #[set(pub(crate))]
    pub(super) attributes: ThreadSafeAttributeStore,
}
