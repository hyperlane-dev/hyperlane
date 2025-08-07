use crate::*;

/// A trait for handling errors that occur within a context.
///
/// This hook is triggered when an error is encountered, allowing for custom logic
/// such as logging, cleanup, or sending a response. The hook must be a function
/// that takes a `Context` and returns a future, and it must be `Send`, `Sync`,
/// and have a `'static` lifetime.
pub trait ContextErrorHook<Fut>: Fn(Context) -> Fut + Send + Sync + 'static
where
    Fut: Future<Output = ()> + Send,
{
}

/// A trait for functions that return a pinned, boxed, sendable future.
///
/// This trait is essential for creating type-erased async function pointers,
/// which is a common pattern for storing and dynamically dispatching different
/// asynchronous handlers in a collection.
pub trait ContextFnPinBoxSendSync:
    Fn(Context) -> Pin<Box<dyn Future<Output = ()> + Send>> + Send + Sync
{
}

/// A trait for static, sendable, synchronous functions that return a future.
///
/// This trait ensures that a handler function is safe to be sent across threads
/// and has a static lifetime, making it suitable for use in long-lived components
/// of the application, such as the main router.
pub trait ContextFnSendSyncStatic<Fut>: Fn(Context) -> Fut + Send + Sync + 'static
where
    Fut: Future<Output = ()> + Send,
{
}

/// A trait for futures that are sendable and have a static lifetime.
///
/// This marker trait simplifies generic bounds for asynchronous operations, ensuring
/// that futures can be safely managed by the async runtime without lifetime issues.
pub trait FutureSendStatic<T>: Future<Output = T> + Send + 'static {}
