/// Enum to identify different kinds of hooks.
///
/// Each variant represents a specific type of hook that can be registered
/// and triggered during the server lifecycle.
#[derive(Clone, Debug, PartialEq, Eq, Copy)]
pub enum HookType {
    /// Hook representing a panic hook, triggered when a panic occurs.
    PanicHook,
    /// Hook representing a directive to disable the default HTTP handler
    /// for a given route.
    ///
    /// - `&'static str`: The route path for which the default HTTP handler is disabled.
    DisableHttpHook(&'static str),
    /// Hook representing a directive to disable the default WebSocket handler
    /// for a given route.
    ///
    /// - `&'static str`: The route path for which the default WebSocket handler is disabled.
    DisableWsHook(&'static str),
    /// Hook representing a connected hook, triggered when a client
    /// successfully establishes a connection.
    ConnectedHook,
    /// Hook representing a pre-upgrade hook, triggered before a protocol upgrade
    /// such as upgrading from HTTP to WebSocket.
    PreUpgradeHook,
    /// Hook representing a request middleware, executed before a request
    /// reaches its designated route handler.
    RequestMiddleware,
    /// Hook representing a route handler for a specific path.
    ///
    /// - `&'static str`: The route path handled by this hook.
    Route(&'static str),
    /// Hook representing a response middleware, executed after a route handler
    /// but before the response is sent back to the client.
    ResponseMiddleware,
}
