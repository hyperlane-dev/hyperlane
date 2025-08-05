use crate::*;

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
    pub(super) config: ServerConfig,
    /// The routing component responsible for matching incoming requests to their registered handlers.
    #[get(pub(super))]
    #[get_mut(pub(super))]
    #[set(pub(super))]
    pub(super) route: RouteMatcher,
    /// A collection of middleware functions that are executed for every incoming request
    /// before it is passed to the corresponding route handler.
    #[debug(skip)]
    #[get(pub(super))]
    #[get_mut(pub(super))]
    #[set(pub(super))]
    pub(super) request_middleware: VecArcFnPinBoxSendSync,
    /// A collection of middleware functions that are executed for every outgoing response
    /// before it is sent back to the client.
    #[debug(skip)]
    #[get(pub(super))]
    #[get_mut(pub(super))]
    #[set(pub(super))]
    pub(super) response_middleware: VecArcFnPinBoxSendSync,
    /// A collection of hooks that are executed before a connection is upgraded to WebSocket.
    /// This allows for custom logic, such as authentication, to be performed.
    #[debug(skip)]
    #[get(pub(super))]
    #[get_mut(pub(super))]
    #[set(pub(super))]
    pub(super) pre_upgrade_hook: VecArcFnPinBoxSendSync,
    /// A collection of hooks that are executed immediately after a new client connection is established.
    #[debug(skip)]
    #[get(pub(super))]
    #[get_mut(pub(super))]
    #[set(pub(super))]
    pub(super) connected_hook: VecArcFnPinBoxSendSync,
    /// A route matcher used to specify routes for which the default HTTP hook should be disabled.
    #[get(pub(super))]
    #[get_mut(pub(super))]
    #[set(pub(super))]
    pub(super) disable_http_hook: RouteMatcher,
    /// A route matcher used to specify routes for which the default WebSocket hook should be disabled.
    #[get(pub(super))]
    #[get_mut(pub(super))]
    #[set(pub(super))]
    pub(super) disable_ws_hook: RouteMatcher,
    /// A custom error handler that is invoked when a panic occurs during request processing.
    /// This allows for graceful error recovery and customized error responses.
    #[debug(skip)]
    #[get(pub(super))]
    #[get_mut(pub(super))]
    #[set(pub(super))]
    pub(super) panic_hook: ArcErrorHandlerSendSync,
}

/// The primary server structure that provides a thread-safe interface to the server's state.
///
/// This struct acts as a public-facing wrapper around an `Arc<RwLock<ServerInner>>`.
/// It allows multiple parts of the application to safely share and modify the server's
/// configuration and state across different threads and asynchronous tasks.
#[derive(Clone, Getter, CustomDebug, DisplayDebug)]
pub struct Server(#[get(pub(super))] pub(super) ArcRwLockServerInner);

/// Represents the state associated with a single connection handler.
///
/// This struct encapsulates the necessary context for processing a connection,
/// including a reference to the network stream and the request context. It is created
/// for each connection and passed to the relevant handlers.
#[derive(Clone, CustomDebug, DisplayDebug)]
pub(crate) struct HandlerState<'a> {
    /// A reference to the underlying network stream for the connection.
    /// This provides access to the raw TCP stream for reading and writing data.
    pub(super) stream: &'a ArcRwLockStream,
    /// A reference to the context of the current request.
    /// This contains request-specific information, such as headers, method, and URI.
    pub(super) ctx: &'a Context,
}
