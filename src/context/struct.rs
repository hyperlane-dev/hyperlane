use crate::*;

/// Represents the internal state of the application context.
///
/// This structure holds all the data associated with a single request-response cycle,
/// including the stream, request, response, and any custom attributes.
#[derive(Clone, CustomDebug, Data, DisplayDebug)]
pub struct Context {
    /// A flag indicating whether the request handling has been aborted.
    #[get(type(copy))]
    pub(super) aborted: bool,
    /// A flag indicating whether the connection has been closed.
    #[get(type(copy))]
    pub(super) closed: bool,
    /// The underlying network stream for the connection.
    #[get(pub(crate))]
    #[get_mut(skip)]
    #[set(pub(crate))]
    pub(super) stream: Option<ArcRwLockStream>,
    /// The incoming HTTP request.
    #[get_mut(skip)]
    #[set(pub(super))]
    pub(super) request: Request,
    /// The outgoing HTTP response.
    pub(super) response: Response,
    /// Parameters extracted from the route path.
    #[get_mut(skip)]
    #[set(pub(crate))]
    pub(super) route_params: RouteParams,
    /// A collection of custom attributes for sharing data within the request lifecycle.
    #[get_mut(pub(super))]
    #[set(pub(super))]
    pub(super) attributes: ThreadSafeAttributeStore,
    /// The server for accessing server-wide configuration and state.
    #[get_mut(skip)]
    #[set(pub(super))]
    pub(super) server: &'static Server,
}
