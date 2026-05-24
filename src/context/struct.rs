use crate::*;

/// Represents the internal state of the application context.
///
/// This structure holds all the data associated with a single request-response cycle,
/// including the request, response, and any custom attributes.
#[derive(Clone, CustomDebug, Data, DisplayDebug)]
pub struct Context {
    /// The incoming HTTP request.
    #[get(pub)]
    #[set(pub)]
    pub(super) request: Request,
    /// The outgoing HTTP response.
    #[get(pub)]
    #[get_mut(pub)]
    #[set(pub)]
    pub(super) response: Response,
    /// Parameters extracted from the route path.
    #[get_mut(skip)]
    #[set(pub)]
    pub(super) route_params: RouteParams,
    /// A collection of custom attributes for sharing data within the request lifecycle.
    #[get_mut(pub)]
    #[set(pub)]
    pub(super) attributes: ThreadSafeAttributeStore,
}
