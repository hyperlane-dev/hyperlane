/// Enum to identify different kinds of hooks in the server lifecycle.
///
/// Each variant represents a specific type of hook that can be registered
/// and triggered at different stages of request handling or server events.
#[derive(Clone, Debug, PartialEq, Eq, Copy)]
pub enum HookType {
    /// Hook triggered when a panic occurs in the server.
    ///
    /// - `isize`: Priority of the panic hook.
    PanicHook(isize),
    /// Hook to disable the default HTTP handler for a specific route.
    ///
    /// - `&'static str`: The route path for which the default HTTP handler is disabled.
    DisableHttpHook(&'static str),
    /// Hook to disable the default WebSocket handler for a specific route.
    ///
    /// - `&'static str`: The route path for which the default WebSocket handler is disabled.
    DisableWsHook(&'static str),
    /// Hook triggered when a client successfully establishes a connection.
    ///
    /// - `isize`: Priority of the connected hook.
    ConnectedHook(isize),
    /// Hook triggered before a protocol upgrade (e.g., HTTP to WebSocket).
    ///
    /// - `isize`: Priority of the pre-upgrade hook.
    PreUpgradeHook(isize),
    /// Hook executed before a request reaches its designated route handler.
    ///
    /// - `isize`: Priority of the request middleware.
    RequestMiddleware(isize),
    /// Hook representing a route handler for a specific path.
    ///
    /// - `&'static str`: The route path handled by this hook.
    Route(&'static str),
    /// Hook executed after a route handler but before the response is sent.
    ///
    /// - `isize`: Priority of the response middleware.
    ResponseMiddleware(isize),
}
