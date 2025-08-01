use crate::*;

/// A trait for asynchronous functions that handle errors.
///
/// This trait defines the contract for error handlers, which are functions that
/// take a `Context` and return a future. This allows for flexible and
/// asynchronous error processing, such as logging or sending custom error responses.
pub trait ErrorHandler<Fut>: Fn(Context) -> Fut + Send + Sync + 'static
where
    Fut: Future<Output = ()> + Send,
{
}

/// A trait for functions that return a pinned, boxed, sendable future.
///
/// This trait is essential for creating type-erased async function pointers,
/// which is a common pattern for storing and dynamically dispatching different
/// asynchronous handlers in a collection.
pub trait FnPinBoxSendSync:
    Fn(Context) -> Pin<Box<dyn Future<Output = ()> + Send>> + Send + Sync
{
}

/// A trait for static, sendable, synchronous functions that return a future.
///
/// This trait ensures that a handler function is safe to be sent across threads
/// and has a static lifetime, making it suitable for use in long-lived components
/// of the application, such as the main router.
pub trait FnSendSyncStatic<Fut>: Fn(Context) -> Fut + Send + Sync + 'static
where
    Fut: Future<Output = ()> + Send,
{
}

/// A trait for futures that are sendable and have a static lifetime.
///
/// This marker trait simplifies generic bounds for asynchronous operations, ensuring
/// that futures can be safely managed by the async runtime without lifetime issues.
pub trait FutureSendStatic<T>: Future<Output = T> + Send + 'static {}
