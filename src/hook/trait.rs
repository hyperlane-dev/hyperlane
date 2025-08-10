use crate::*;

/// A trait for functions that return a pinned, boxed, sendable future.
///
/// This trait is essential for creating type-erased async function pointers,
/// which is a common pattern for storing and dynamically dispatching different
/// asynchronous handlers in a collection.
pub trait FnContextPinBoxSendSync:
    Fn(Context) -> Pin<Box<dyn Future<Output = ()> + Send>> + Send + Sync
{
}

/// A trait for static, sendable, synchronous functions that return a future.
///
/// This trait ensures that a handler function is safe to be sent across threads
/// and has a static lifetime, making it suitable for use in long-lived components
/// of the application, such as the main router.
pub trait FnContextSendSyncStatic<Fut>: Fn(Context) -> Fut + Send + Sync + 'static
where
    Fut: Future<Output = ()> + Send,
{
}

/// A trait for futures that are sendable and have a static lifetime.
///
/// This marker trait simplifies generic bounds for asynchronous operations, ensuring
/// that futures can be safely managed by the async runtime without lifetime issues.
pub trait FutureSendStatic<T>: Future<Output = T> + Send + 'static {}
