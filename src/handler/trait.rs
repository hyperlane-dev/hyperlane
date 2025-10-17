use crate::*;

/// Trait for server lifecycle hooks that process requests.
///
/// `ServerHook` provides a unified interface for different types of request processing
/// handlers in the server lifecycle, including route handlers, middleware, and panic hooks.
/// All hooks follow the same pattern: instantiation via `new` and execution via `handle`.
///
/// This trait is designed to work with the server's request processing pipeline, where
/// each hook receives the `Context` directly for both initialization and processing.
pub trait ServerHook: Send + Sync + 'static {
    /// Creates a new instance of this hook from the context.
    ///
    /// This method is called by the framework to instantiate the hook,
    /// passing in the `Context` directly.
    ///
    /// # Arguments
    ///
    /// - `&Context` - The request context containing all request/response data.
    ///
    /// # Returns
    ///
    /// A future that resolves to a new instance of this hook.
    fn new(ctx: &Context) -> impl Future<Output = Self> + Send;

    /// Executes the hook's processing logic.
    ///
    /// This method contains the actual logic for processing the request.
    /// It receives the `Context` as a parameter for accessing request/response data.
    ///
    /// # Arguments
    ///
    /// - `&Context` - The request context for accessing request/response data.
    ///
    /// # Returns
    ///
    /// A future that resolves when the processing is complete.
    fn handle(self, ctx: &Context) -> impl Future<Output = ()> + Send;
}
