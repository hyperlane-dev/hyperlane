use crate::*;

/// Represents the state associated with a single connection handler.
///
/// This struct encapsulates the necessary context for processing a connection,
/// including a reference to the network stream and the request context. It is created
/// for each connection and passed to the relevant handlers.
#[derive(Clone, CustomDebug, DisplayDebug, Getter)]
pub(crate) struct HandlerState {
    /// A reference to the underlying network stream for the connection.
    /// This provides access to the raw TCP stream for reading and writing data.
    pub(super) stream: ArcRwLockStream,
    /// A reference to the context of the current request.
    /// This contains request-specific information, such as headers, method, and URI.
    pub(super) ctx: Context,
    /// The size of the buffer used for reading HTTP requests.
    /// This is used to determine the maximum size of the request body.
    pub(super) buffer: usize,
}

/// Represents the internal, mutable state of the web server.
///
/// This struct consolidates all the core components required for the server to operate,
/// including configuration, routing, middleware, and various hooks for extending functionality.
/// It is not intended to be used directly by end-users, but rather wrapped within the `Server` struct
/// for thread-safe access.
#[derive(Data, Clone, CustomDebug, DisplayDebug)]
pub(crate) struct ServerInner {
    /// Stores the server's configuration settings, such as address, port, and timeouts.
    #[get(pub(super))]
    #[get_mut(pub(super))]
    #[set(pub(super))]
    pub(super) config: ServerConfigInner,
    /// The routing component responsible for matching incoming requests to their registered handlers.
    #[get(pub(super))]
    #[get_mut(pub(super))]
    #[set(pub(super))]
    pub(super) route_matcher: RouteMatcher,
    /// A collection of panic hook handlers that are invoked when a panic occurs during request processing.
    /// This allows for graceful error recovery and customized error responses.
    #[debug(skip)]
    #[get(pub(super))]
    #[get_mut(pub(super))]
    #[set(pub(super))]
    pub(super) panic_hook: ServerHookList,
    /// A collection of request middleware handlers.
    #[debug(skip)]
    #[get(pub(super))]
    #[get_mut(pub(super))]
    #[set(pub(super))]
    pub(super) request_middleware: ServerHookList,
    /// A collection of response middleware handlers.
    #[debug(skip)]
    #[get(pub(super))]
    #[get_mut(pub(super))]
    #[set(pub(super))]
    pub(super) response_middleware: ServerHookList,
}

/// The primary server structure that provides a thread-safe interface to the server's state.
///
/// This struct acts as a public-facing wrapper around an `Arc<RwLock<ServerInner>>`.
/// It allows multiple parts of the application to safely share and modify the server's
/// configuration and state across different threads and asynchronous tasks.
#[derive(Clone, Getter, CustomDebug, DisplayDebug, Default)]
pub struct Server(#[get(pub(super))] pub(super) SharedServerState);
