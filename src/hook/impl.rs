use crate::*;

/// A blanket implementation for functions that return a pinned, boxed, sendable future.
///
/// This trait is a common pattern for asynchronous handlers in Rust, enabling type
/// erasure and dynamic dispatch for futures. It is essential for storing different
/// async functions in a collection.
impl<F> FnContextPinBoxSendSync for F where
    F: Fn(Context) -> Pin<Box<dyn Future<Output = ()> + Send>> + Send + Sync
{
}

/// A blanket implementation for static, sendable, synchronous functions that return a future.
///
/// This trait is used for handlers that are known at compile time, ensuring they
/// are safe to be sent across threads and have a static lifetime. This is crucial
/// for handlers that are part of the application's long-lived state.
impl<F, Fut> FnContextSendSyncStatic<Fut> for F
where
    F: Fn(Context) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = ()> + Send,
{
}

/// A blanket implementation for any future that is sendable and has a static lifetime.
///
/// This is a convenient trait for working with futures in an asynchronous context,
/// ensuring that they can be safely managed by the async runtime across different
/// threads.
impl<T, R> FutureSendStatic<R> for T where T: Future<Output = R> + Send + 'static {}
