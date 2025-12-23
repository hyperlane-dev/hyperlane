use crate::*;

/// Represents different handler types for hooks.
#[derive(Clone)]
pub enum HookHandlerSpec {
    /// Arc handler (used for request/response middleware and route)
    Handler(ServerHookHandler),
    /// Factory function that creates a handler when called
    Factory(ServerHookHandlerFactory),
}

/// Represents different kinds of hooks in the server lifecycle.
///
/// Each variant corresponds to a specific hook that can be registered
/// and triggered at different stages of request handling or server events.
/// Hooks with an `Option<isize>` allow specifying a priority order; `None` indicates
/// the default order (0 or unspecified).
#[derive(Clone, Debug, PartialEq, Eq, Copy, Hash, DisplayDebug)]
pub enum HookType {
    /// Triggered when a panic occurs in the server.
    ///
    /// - `Option<isize>`- Optional priority of the panic hook. `None` means default.
    PanicHook(Option<isize>),
    /// Executed before a request reaches its designated route handler.
    ///
    /// - `Option<isize>`- Optional priority of the request middleware.
    RequestMiddleware(Option<isize>),
    /// Represents a route handler for a specific path.
    ///
    /// - `&'static str`- The route path handled by this hook.
    Route(&'static str),
    /// Executed after a route handler but before the response is sent.
    ///
    /// - `Option<isize>`- Optional priority of the response middleware.
    ResponseMiddleware(Option<isize>),
}
