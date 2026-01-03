use crate::*;

/// Represents different types of hooks in the server lifecycle.
///
/// Each variant corresponds to a specific hook that can be registered
/// and triggered at different stages of request handling or server events.
/// Hooks with an `Option<isize>` allow specifying a priority order; `None` indicates
/// the default order (0 or unspecified).
#[derive(Clone, Debug, Copy, DisplayDebug)]
pub enum HookType {
    /// Hook triggered when a panic occurs during request processing.
    ///
    /// - `Option<isize>` - Optional execution priority. Higher values execute first.
    /// - `ServerHookHandlerFactory` - Factory function creating the panic handler.
    Panic(Option<isize>, ServerHookHandlerFactory),
    /// Hook triggered when a request read error occurs during HTTP request processing.
    ///
    /// - `Option<isize>` - Optional execution priority. Higher values execute first.
    /// - `ServerHookHandlerFactory` - Factory function creating the error handler.
    RequestError(Option<isize>, ServerHookHandlerFactory),
    /// Hook executed before a request reaches its designated route handler.
    ///
    /// - `Option<isize>` - Optional execution priority. Higher values execute first.
    /// - `ServerHookHandlerFactory` - Factory function creating the middleware handler.
    RequestMiddleware(Option<isize>, ServerHookHandlerFactory),
    /// Hook representing a route handler for a specific path.
    ///
    /// - `&'static str` - The route path pattern handled by this hook.
    /// - `ServerHookHandlerFactory` - Factory function creating the route handler.
    Route(&'static str, ServerHookHandlerFactory),
    /// Hook executed after a route handler but before the response is sent.
    ///
    /// - `Option<isize>` - Optional execution priority. Higher values execute first.
    /// - `ServerHookHandlerFactory` - Factory function creating the middleware handler.
    ResponseMiddleware(Option<isize>, ServerHookHandlerFactory),
}
