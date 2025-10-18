use crate::*;

/// A type alias for a shared hook handler.
///
/// This type is used for storing handlers in a shared context, allowing multiple
/// parts of the application to safely access and execute the same handler.
pub type SharedHookHandler<T> = Arc<dyn FnContextPinBoxSendSync<T>>;
/// A type alias for an optional hook handler.
///
/// This is used when a handler may or may not be present, such as for optional
/// middleware or hooks.
pub type OptionalHookHandler<T> = Option<SharedHookHandler<T>>;
/// A type alias for a hook handler chain.
///
/// This type is used to represent a chain of middleware or hooks that can be
/// executed sequentially.
pub type HookHandlerChain<T> = Vec<SharedHookHandler<T>>;
/// A type alias for an asynchronous task.
///
/// This is a common return type for asynchronous handlers, providing a type-erased
/// future that can be easily managed by the async runtime.
pub type AsyncTask = Pin<Box<(dyn Future<Output = ()> + Send + 'static)>>;
/// A type alias for a sendable asynchronous task with a generic output.
///
/// This is often used to represent an asynchronous task that can be sent across threads.
pub type SendableAsyncTask<T> = Pin<Box<dyn Future<Output = T> + Send>>;
/// A type alias for a shared asynchronous task factory.
///
/// This is useful for creating and sharing asynchronous task factories.
pub type SharedAsyncTaskFactory<T> = Arc<dyn FnPinBoxFutureSend<T>>;
/// A type alias for a hook handler factory function.
///
/// This function pointer type is used to create ServerHookHandler instances
/// based on generic types. It allows delayed instantiation of hooks.
pub type ServerHookHandlerFactory = fn() -> ServerHookHandler;
