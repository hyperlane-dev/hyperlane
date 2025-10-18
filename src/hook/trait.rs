use crate::*;

/// A generic trait for functions that take a `Context` and return a value.
///
/// This trait encapsulates the common behavior of being a sendable, synchronous
/// function that accepts a `Context`. It is used as a base for other, more
/// specific function traits.
pub trait FnContextSendSync<R>: Fn(Context) -> R + Send + Sync {}
/// A trait for functions that return a pinned, boxed, sendable future.
///
/// This trait is essential for creating type-erased async function pointers,
/// which is a common pattern for storing and dynamically dispatching different
/// asynchronous handlers in a collection.
pub trait FnContextPinBoxSendSync<T>: FnContextSendSync<SendableAsyncTask<T>> {}
/// A trait for static, sendable, synchronous functions that return a future.
///
/// This trait ensures that a handler function is safe to be sent across threads
/// and has a static lifetime, making it suitable for use in long-lived components
/// of the application, such as the main router.
pub trait FnContextSendSyncStatic<Fut, T>: FnContextSendSync<Fut> + 'static
where
    Fut: Future<Output = T> + Send,
{
}
/// A trait for futures that are sendable and have a static lifetime.
///
/// This marker trait simplifies generic bounds for asynchronous operations, ensuring
/// that futures can be safely managed by the async runtime without lifetime issues.
pub trait FutureSendStatic<T>: Future<Output = T> + Send + 'static {}
/// A trait for `Send`-able futures with a generic output.
pub trait FutureSend<T>: Future<Output = T> + Send {}
/// A trait for thread-safe, reference-counted closures that produce a sendable async task.
pub trait FnPinBoxFutureSend<T>: Fn() -> SendableAsyncTask<T> + Send + Sync {}
