use super::*;

/// Represents the internal, mutable state of the web server.
///
/// This struct consolidates all the core components required for the server to operate,
/// including configuration, routing, middleware, and various hooks for extending functionality.
#[derive(Clone, CustomDebug, Data, DisplayDebug)]
pub struct Server {
    /// Stores the server's configuration settings, such as address, port, and timeouts.
    pub server_config: ServerConfig,
    /// The configuration for HTTP request.
    pub request_config: RequestConfig,
    /// The routing component responsible for matching incoming requests to their registered handlers.
    #[set(skip)]
    pub route_matcher: RouteMatcher,
    /// A collection of request error handlers that are invoked when a request error occurs during HTTP request processing.
    #[debug(skip)]
    #[set(skip)]
    pub request_error: ServerHookList,
    /// A collection of task panic handlers that are invoked when a panic occurs during request processing.
    #[debug(skip)]
    #[set(skip)]
    pub task_panic: ServerHookList,
    /// A collection of request middleware handlers.
    #[debug(skip)]
    #[set(skip)]
    pub request_middleware: ServerHookList,
    /// A collection of response middleware handlers.
    #[debug(skip)]
    #[set(skip)]
    pub response_middleware: ServerHookList,
}
