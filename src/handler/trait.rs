use crate::*;

/// Trait for route handlers that process HTTP requests.
///
/// Route handlers are responsible for processing matched routes and generating
/// responses. Each route handler receives the previous handler in the chain
/// through its `new` method, allowing it to extract the `Context` and any
/// other state from the previous stage.
pub trait Route: Send + Sync + 'static {
    /// The type of the previous handler in the processing chain.
    ///
    /// This associated type must be `Send` to ensure thread safety.
    type Prev: Send;

    /// Creates a new instance of this route handler from the previous handler.
    ///
    /// This method is called by the framework to instantiate the route handler,
    /// passing in a reference to the previous handler from which the `Context` can be extracted.
    ///
    /// # Arguments
    ///
    /// - `&Self::Prev` - A reference to the previous handler in the chain, which contains the `Context`.
    ///
    /// # Returns
    ///
    /// A future that resolves to a new instance of this route handler.
    fn new(prev: &Self::Prev) -> impl Future<Output = Self> + Send;

    /// Executes the route handling logic.
    ///
    /// This method contains the actual business logic for processing the request
    /// and generating a response. It has access to the `Context` through `self`.
    ///
    /// # Returns
    ///
    /// A future that resolves when the route handling is complete.
    fn handle(self) -> impl Future<Output = ()> + Send;
}

/// Trait for middleware that can process requests or responses.
///
/// Middleware handlers are executed before and after route handlers, allowing
/// for cross-cutting concerns like logging, authentication, and response
/// modification. Like route handlers, middleware receives the previous handler
/// through its `new` method.
pub trait Middleware: Send + Sync + 'static {
    /// The type of the previous handler in the processing chain.
    ///
    /// This associated type must be `Send` to ensure thread safety.
    type Prev: Send;

    /// Creates a new instance of this middleware from the previous handler.
    ///
    /// This method is called by the framework to instantiate the middleware,
    /// passing in a reference to the previous handler from which the `Context` can be extracted.
    ///
    /// # Arguments
    ///
    /// - `&Self::Prev` - A reference to the previous handler in the chain, which contains the `Context`.
    ///
    /// # Returns
    ///
    /// A future that resolves to a new instance of this middleware.
    fn new(prev: &Self::Prev) -> impl Future<Output = Self> + Send;

    /// Executes the middleware logic.
    ///
    /// This method contains the middleware's processing logic, which may modify
    /// the request, response, or context. It has access to the `Context` through `self`.
    ///
    /// # Returns
    ///
    /// A future that resolves when the middleware processing is complete.
    fn handle(self) -> impl Future<Output = ()> + Send;
}

/// Trait for panic hook handlers that process panics during request processing.
///
/// Panic hooks are executed when a panic occurs in a route handler or middleware,
/// allowing for custom error handling, logging, and recovery. Like other handlers,
/// panic hooks receive the previous handler through their `new` method.
pub trait PanicHook: Send + Sync + 'static {
    /// The type of the previous handler in the processing chain.
    ///
    /// This associated type must be `Send` to ensure thread safety.
    type Prev: Send;

    /// Creates a new instance of this panic hook from the previous handler.
    ///
    /// This method is called by the framework to instantiate the panic hook,
    /// passing in a reference to the previous handler from which the `Context` can be extracted.
    ///
    /// # Arguments
    ///
    /// - `&Self::Prev` - A reference to the previous handler in the chain, which contains the `Context`.
    ///
    /// # Returns
    ///
    /// A future that resolves to a new instance of this panic hook.
    fn new(prev: &Self::Prev) -> impl Future<Output = Self> + Send;

    /// Executes the panic hook logic.
    ///
    /// This method contains the panic handling logic, which may log the error,
    /// send error responses, or perform cleanup. It has access to the `Context`
    /// (which contains the panic information) through `self`.
    ///
    /// # Returns
    ///
    /// A future that resolves when the panic handling is complete.
    fn handle(self) -> impl Future<Output = ()> + Send;
}
