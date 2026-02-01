use crate::*;

/// Represents the internal state of the application context.
///
/// This structure holds all the data associated with a single request-response cycle,
/// including the stream, request, response, and any custom attributes.
#[derive(Clone, CustomDebug, Data, Default, DisplayDebug)]
pub(crate) struct ContextData {
    /// A flag indicating whether the request handling has been aborted.
    #[get(pub(super), type(copy))]
    #[get_mut(pub(super))]
    #[set(pub(super))]
    aborted: bool,
    /// A flag indicating whether the connection has been closed.
    #[get(pub(super), type(copy))]
    #[get_mut(pub(super))]
    #[set(pub(super))]
    closed: bool,
    /// The underlying network stream for the connection.
    #[get(pub(super))]
    #[get_mut(pub(super))]
    #[set(pub(super))]
    stream: Option<ArcRwLockStream>,
    /// The incoming HTTP request.
    #[get(pub(super))]
    #[get_mut(pub(super))]
    #[set(pub(super))]
    request: Request,
    /// The outgoing HTTP response.
    #[get(pub(super))]
    #[get_mut(pub(super))]
    #[set(pub(super))]
    response: Response,
    /// Parameters extracted from the route path.
    #[get(pub(super))]
    #[get_mut(pub(super))]
    #[set(pub(super))]
    route_params: RouteParams,
    /// A collection of custom attributes for sharing data within the request lifecycle.
    #[get(pub(super))]
    #[get_mut(pub(super))]
    #[set(pub(super))]
    attributes: ThreadSafeAttributeStore,
}

/// The main application context, providing thread-safe access to request and response data.
///
/// This is a wrapper around `ContextData` that uses an `Arc<RwLock<>>` to allow
/// for shared, mutable access across asynchronous tasks.
#[derive(Clone, CustomDebug, Default, DisplayDebug, Getter)]
pub struct Context(#[get(pub(super))] pub(super) ArcRwLock<ContextData>);
