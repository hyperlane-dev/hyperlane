use crate::*;

/// Represents the internal state of the application context.
///
/// This structure holds all the data associated with a single request-response cycle,
/// including the stream, request, response, and any custom attributes.
#[derive(Clone, CustomDebug, Data, Default, DisplayDebug)]
pub struct Context {
    /// A flag indicating whether the request handling has been aborted.
    #[get(type(copy))]
    aborted: bool,
    /// A flag indicating whether the connection has been closed.
    #[get(type(copy))]
    closed: bool,
    /// The underlying network stream for the connection.
    #[get(pub(super))]
    #[get_mut(skip)]
    #[set(pub(super))]
    stream: Option<ArcRwLockStream>,
    /// The incoming HTTP request.
    #[get_mut(skip)]
    #[set(pub(super))]
    request: Request,
    /// The outgoing HTTP response.
    response: Response,
    /// Parameters extracted from the route path.
    #[get_mut(skip)]
    #[set(pub(crate))]
    route_params: RouteParams,
    /// A collection of custom attributes for sharing data within the request lifecycle.
    #[get_mut(pub(super))]
    #[set(pub(super))]
    attributes: ThreadSafeAttributeStore,
    /// The server for accessing server-wide configuration and state.
    #[get_mut(skip)]
    #[set(pub(super))]
    server: ArcServer,
}
