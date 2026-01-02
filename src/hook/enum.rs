use crate::*;

/// Represents different kinds of hooks in the server lifecycle.
///
/// Each variant corresponds to a specific hook that can be registered
/// and triggered at different stages of request handling or server events.
/// Hooks with an `Option<isize>` allow specifying a priority order; `None` indicates
/// the default order (0 or unspecified).
#[derive(Clone, Debug, Copy, DisplayDebug)]
pub enum HookType {
    /// Triggered when a panic occurs in the server.
    ///
    /// - `Option<isize>`- Optional priority of the panic. `None` means default.
    Panic(Option<isize>, ServerHookHandlerFactory),
    /// Triggered when a request error occurs during HTTP request processing.
    ///
    /// - `Option<isize>`- Optional priority of the request error. `None` means default.
    RequestError(Option<isize>, ServerHookHandlerFactory),
    /// Executed before a request reaches its designated route hook.
    ///
    /// - `Option<isize>`- Optional priority of the request middleware.
    RequestMiddleware(Option<isize>, ServerHookHandlerFactory),
    /// Represents a route hook for a specific path.
    ///
    /// - `&'static str`- The route path handled by this hook.
    Route(&'static str, ServerHookHandlerFactory),
    /// Executed after a route hook but before the response is sent.
    ///
    /// - `Option<isize>`- Optional priority of the response middleware.
    ResponseMiddleware(Option<isize>, ServerHookHandlerFactory),
}
