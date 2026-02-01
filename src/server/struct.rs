use crate::*;

/// Represents the state associated with a single connection hook.
///
/// This struct encapsulates the necessary context for processing a connection,
/// including a reference to the network stream and request configuration. It is created
/// for each connection and passed to the relevant handlers.
#[derive(Clone, CustomDebug, DisplayDebug, Getter)]
pub(crate) struct HandlerState {
    /// A reference to the underlying network stream for the connection.
    pub(super) stream: ArcRwLockStream,
    /// The request config for the current connection.
    pub(super) request_config: RequestConfigData,
}

/// Represents the internal, mutable state of the web server.
///
/// This struct consolidates all the core components required for the server to operate,
/// including configuration, routing, middleware, and various hooks for extending functionality.
/// It is not intended to be used directly by end-users, but rather wrapped within the `Server` struct
/// for thread-safe access.
#[derive(Clone, CustomDebug, Data, DisplayDebug)]
pub(crate) struct ServerData {
    /// Stores the server's configuration settings, such as address, port, and timeouts.
    #[get(pub(super))]
    #[get_mut(pub(super))]
    #[set(pub(super))]
    pub(super) server_config: ServerConfigData,
    /// The configuration for HTTP request.
    #[get(pub(crate))]
    #[get_mut(pub(super))]
    #[set(pub(super))]
    pub(super) request_config: RequestConfigData,
    /// The routing component responsible for matching incoming requests to their registered handlers.
    #[get(pub(super))]
    #[get_mut(pub(super))]
    #[set(pub(super))]
    pub(super) route_matcher: RouteMatcher,
    /// A collection of request error handlers that are invoked when a request error occurs during HTTP request processing.
    /// This allows for graceful error recovery and customized error responses.
    #[debug(skip)]
    #[get(pub(super))]
    #[get_mut(pub(super))]
    #[set(pub(super))]
    pub(super) request_error: ServerHookList,
    /// A collection of task panic handlers that are invoked when a panic occurs during request processing.
    /// This allows for graceful error recovery and customized error responses.
    #[debug(skip)]
    #[get(pub(super))]
    #[get_mut(pub(super))]
    #[set(pub(super))]
    pub(super) task_panic: ServerHookList,
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
/// This struct acts as a public-facing wrapper around an `Arc<RwLock<ServerData>>`.
/// It allows multiple parts of the application to safely share and modify the server's
/// configuration and state across different threads and asynchronous tasks.
#[derive(Clone, CustomDebug, Default, DisplayDebug, Getter)]
pub struct Server(#[get(pub(super))] pub(super) SharedServerState);
