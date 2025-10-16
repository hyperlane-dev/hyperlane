use crate::*;

/// Trait for route handlers that process HTTP requests.
///
/// Route handlers are responsible for processing matched routes and generating
/// responses. Each route handler receives the `Context` directly through its
/// `new` method for initialization and through the `handle` method for processing.
pub trait Route: Send + Sync + 'static {
    /// Creates a new instance of this route handler from the context.
    ///
    /// This method is called by the framework to instantiate the route handler,
    /// passing in the `Context` directly.
    ///
    /// # Arguments
    ///
    /// - `Context` - The request context containing all request/response data.
    ///
    /// # Returns
    ///
    /// A future that resolves to a new instance of this route handler.
    fn new(ctx: Context) -> impl Future<Output = Self> + Send;

    /// Executes the route handling logic.
    ///
    /// This method contains the actual business logic for processing the request
    /// and generating a response. It receives the `Context` as a parameter.
    ///
    /// # Arguments
    ///
    /// - `Context` - The request context for accessing request/response data.
    ///
    /// # Returns
    ///
    /// A future that resolves when the route handling is complete.
    fn handle(self, ctx: Context) -> impl Future<Output = ()> + Send;
}

/// Trait for middleware that can process requests or responses.
///
/// Middleware handlers are executed before and after route handlers, allowing
/// for cross-cutting concerns like logging, authentication, and response
/// modification. Middleware receives the `Context` directly through its
/// `new` method for initialization and through the `handle` method for processing.
pub trait Middleware: Send + Sync + 'static {
    /// Creates a new instance of this middleware from the context.
    ///
    /// This method is called by the framework to instantiate the middleware,
    /// passing in the `Context` directly.
    ///
    /// # Arguments
    ///
    /// - `Context` - The request context containing all request/response data.
    ///
    /// # Returns
    ///
    /// A future that resolves to a new instance of this middleware.
    fn new(ctx: Context) -> impl Future<Output = Self> + Send;

    /// Executes the middleware logic.
    ///
    /// This method contains the middleware's processing logic, which may modify
    /// the request, response, or context. It receives the `Context` as a parameter.
    ///
    /// # Arguments
    ///
    /// - `Context` - The request context for accessing request/response data.
    ///
    /// # Returns
    ///
    /// A future that resolves when the middleware processing is complete.
    fn handle(self, ctx: Context) -> impl Future<Output = ()> + Send;
}

/// Trait for panic hook handlers that process panics during request processing.
///
/// Panic hooks are executed when a panic occurs in a route handler or middleware,
/// allowing for custom error handling, logging, and recovery. Panic hooks receive
/// the `Context` directly through its `new` method for initialization and through
/// the `handle` method for processing.
pub trait PanicHook: Send + Sync + 'static {
    /// Creates a new instance of this panic hook from the context.
    ///
    /// This method is called by the framework to instantiate the panic hook,
    /// passing in the `Context` directly.
    ///
    /// # Arguments
    ///
    /// - `Context` - The request context containing all request/response data.
    ///
    /// # Returns
    ///
    /// A future that resolves to a new instance of this panic hook.
    fn new(ctx: Context) -> impl Future<Output = Self> + Send;

    /// Executes the panic hook logic.
    ///
    /// This method contains the panic handling logic, which may log the error,
    /// send error responses, or perform cleanup. It receives the `Context`
    /// (which contains the panic information) as a parameter.
    ///
    /// # Arguments
    ///
    /// - `Context` - The request context for accessing panic information.
    ///
    /// # Returns
    ///
    /// A future that resolves when the panic handling is complete.
    fn handle(self, ctx: Context) -> impl Future<Output = ()> + Send;
}
