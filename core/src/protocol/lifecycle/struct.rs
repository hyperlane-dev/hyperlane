use super::*;

/// Runs the framework lifecycle for a request that arrived on a protocol stream.
///
/// The lifecycle is the same one an HTTP/1.1 request runs: the request
/// middleware, the matched route and the response middleware, in that order,
/// against the same context and the same error and panic hooks. It receives a
/// real `Stream` over a duplex pipe rather than a socket, so a handler that
/// writes a response, splits a write or streams takes exactly the code path it
/// takes on HTTP/1.1, and the framing pump observes those writes as they happen.
///
/// Dropping the stream when the lifecycle returns closes the pipe, which is how
/// the pump learns the response has ended.
#[derive(Data, Debug)]
pub(crate) struct ServerLifecycle {
    /// The server whose pipeline serves the request.
    #[get(type(clone), pub(crate))]
    #[get_mut(skip)]
    #[set(pub(crate))]
    pub(super) server: Arc<Server>,
    /// The request converted from the protocol head and body.
    #[get(type(clone), pub(crate))]
    #[get_mut(skip)]
    #[set(pub(crate))]
    pub(super) request: Request,
}
