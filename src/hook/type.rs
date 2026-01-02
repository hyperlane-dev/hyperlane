use crate::*;

/// A type alias for a shared hook hook.
///
/// This type is used for storing handlers in a shared context, allowing multiple
/// parts of the application to safely access and execute the same hook.
pub type HookHandler<T> = Arc<dyn FnContextPinBoxSendSync<T>>;

/// A type alias for a hook hook chain.
///
/// This type is used to represent a chain of middleware or hooks that can be
/// executed sequentially.
pub type HookHandlerChain<T> = Vec<HookHandler<T>>;

/// A type alias for an asynchronous task.
///
/// This is a common return type for asynchronous handlers, providing a type-erased
/// future that can be easily managed by the async runtime.
pub type AsyncTask = Pin<Box<dyn Future<Output = ()> + Send + 'static>>;

/// A type alias for a sendable asynchronous task with a generic output.
///
/// This is often used to represent an asynchronous task that can be sent across threads.
pub type SendableAsyncTask<T> = Pin<Box<dyn Future<Output = T> + Send>>;

/// A type alias for a shared asynchronous task factory.
///
/// This is useful for creating and sharing asynchronous task factories.
pub type SharedAsyncTaskFactory<T> = Arc<dyn FnPinBoxFutureSend<T>>;

/// A type alias for a hook hook factory function.
///
/// This function pointer type is used to create ServerHookHandler instances
/// based on generic types. It allows delayed instantiation of hooks.
pub type ServerHookHandlerFactory = fn() -> ServerHookHandler;

/// Type alias for a shared server hook hook.
///
/// This type allows storing handlers (route and middleware) of different concrete types
/// in the same collection. The hook takes a `&Context` and returns
/// a pinned, boxed future that resolves to `()`.
pub type ServerHookHandler = Arc<dyn Fn(&Context) -> SendableAsyncTask<()> + Send + Sync>;

/// Type alias for a list of server hooks.
///
/// Used to store middleware handlers in the request/response processing pipeline.
pub type ServerHookList = Vec<ServerHookHandler>;

/// Type alias for a map of server hook handlers.
///
/// Used for fast lookup of exact-match route.
pub type ServerHookMap = HashMapXxHash3_64<String, ServerHookHandler>;

/// Type alias for a collection of pattern-based server hook route grouped by segment count.
///
/// The outer HashMap uses segment count as key for fast filtering.
/// The inner Vec stores patterns with the same segment count, maintaining insertion order.
pub type ServerHookPatternRoute = HashMapXxHash3_64<usize, Vec<(RoutePattern, ServerHookHandler)>>;
