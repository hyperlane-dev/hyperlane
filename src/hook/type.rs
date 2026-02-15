use crate::*;

/// A type alias for a shared hook handler.
///
/// This type is used for storing handlers in a shared context, allowing multiple
/// parts of the application to safely access and execute the same handler.
pub type HookHandler<T> = Arc<dyn FnContextPinBox<T>>;

/// A type alias for a hook handler chain.
///
/// This type is used to represent a chain of middleware handlers that can be
/// executed sequentially.
pub type HookHandlerChain<T> = Vec<HookHandler<T>>;

/// A type alias for an asynchronous task.
///
/// This is a common return type for asynchronous handlers, providing a type-erased
/// future that can be easily managed by the async runtime.
pub type AsyncTask = Pin<Box<dyn Future<Output = ()> + Send + 'static>>;

/// A type alias for a boxed future with a generic output that can be sent across threads.
///
/// This is often used to represent an asynchronous task that can be sent across threads.
pub type FutureBox<T> = Pin<Box<dyn Future<Output = T> + Send>>;

/// A type alias for a server control hook handler.
///
/// This type represents a thread-safe, reference-counted function that returns
/// a boxed future when invoked. It is used for server lifecycle hooks such as
/// graceful shutdown and wait operations.
pub type ServerControlHookHandler<T> = Arc<dyn FutureFn<T>>;

/// A type alias for a hook handler factory function.
///
/// This function pointer type is used to create ServerHookHandler instances
/// based on generic types. It allows delayed instantiation of handlers.
pub type ServerHookHandlerFactory = fn() -> ServerHookHandler;

/// Type alias for a shared server hook handler.
///
/// This type allows storing handlers (route and middleware) of different concrete types
/// in the same collection. The handler takes a `&Context` and returns
/// a pinned, boxed future that resolves to `()`.
pub type ServerHookHandler = Arc<dyn Fn(&mut Context) -> FutureBox<()> + Send + Sync>;

/// Type alias for a list of server hook handlers.
///
/// Used to store middleware handlers in the request/response processing pipeline.
pub type ServerHookList = Vec<ServerHookHandler>;

/// Type alias for a map of server route handlers.
///
/// Used for fast lookup of exact-match routes.
pub type ServerHookMap = HashMapXxHash3_64<String, ServerHookHandler>;

/// Type alias for a collection of pattern-based server hook route grouped by segment count.
///
/// The outer HashMap uses segment count as key for fast filtering.
/// The inner Vec stores patterns with the same segment count, maintaining insertion order.
pub type ServerHookPatternRoute = HashMapXxHash3_64<usize, Vec<(RoutePattern, ServerHookHandler)>>;
