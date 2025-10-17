use crate::*;

/// A blanket implementation for any function that takes a `Context` and returns a value.
///
/// This implementation makes it easy to use any compatible function as a `FnContextSendSync`,
/// promoting a flexible and functional programming style.
impl<F, R> FnContextSendSync<R> for F where F: Fn(Context) -> R + Send + Sync {}

/// A blanket implementation for functions that return a pinned, boxed, sendable future.
///
/// This trait is a common pattern for asynchronous handlers in Rust, enabling type
/// erasure and dynamic dispatch for futures. It is essential for storing different
/// async functions in a collection.
impl<F, T> FnContextPinBoxSendSync<T> for F where F: FnContextSendSync<PinBoxFutureSend<T>> {}

/// A blanket implementation for static, sendable, synchronous functions that return a future.
///
/// This trait is used for handlers that are known at compile time, ensuring they
/// are safe to be sent across threads and have a static lifetime. This is crucial
/// for handlers that are part of the application's long-lived state.
impl<F, Fut, T> FnContextSendSyncStatic<Fut, T> for F
where
    F: FnContextSendSync<Fut> + 'static,
    Fut: Future<Output = T> + Send,
{
}

/// A blanket implementation for any future that is sendable and has a static lifetime.
///
/// This is a convenient trait for working with futures in an asynchronous context,
/// ensuring that they can be safely managed by the async runtime across different
/// threads.
impl<T, R> FutureSendStatic<R> for T where T: Future<Output = R> + Send + 'static {}

/// Blanket implementation of `FutureSend` for any type that satisfies the bounds.
impl<T, O> FutureSend<O> for T where T: Future<Output = O> + Send {}

/// Blanket implementation of `FnPinBoxFutureSend` for any type that satisfies the bounds.
impl<T, O> FnPinBoxFutureSend<O> for T where T: Fn() -> PinBoxFutureSend<O> + Send + Sync {}

/// Provides a default implementation for `ServerControlHook`.
impl Default for ServerControlHook {
    /// Creates a new `ServerControlHook` instance with default no-op hooks.
    ///
    /// The default `wait_hook` and `shutdown_hook` do nothing, allowing the server
    /// to run without specific shutdown or wait logic unless configured otherwise.
    ///
    /// # Returns
    ///
    /// - `Self` - A new `ServerControlHook` instance with default hooks.
    fn default() -> Self {
        Self {
            wait_hook: Arc::new(|| Box::pin(async {})),
            shutdown_hook: Arc::new(|| Box::pin(async {})),
        }
    }
}

/// Manages server lifecycle hooks, including waiting and shutdown procedures.
///
/// This struct holds closures that are executed during specific server lifecycle events.
impl ServerControlHook {
    /// Waits for the server's shutdown signal or completion.
    ///
    /// This method asynchronously waits until the server's `wait_hook` is triggered,
    /// typically indicating that the server has finished its operations or is ready to shut down.
    pub async fn wait(&self) {
        self.get_wait_hook()().await;
    }

    /// Initiates the server shutdown process.
    ///
    /// This method asynchronously calls the `shutdown_hook`, which is responsible for
    /// performing any necessary cleanup or graceful shutdown procedures.
    pub async fn shutdown(&self) {
        self.get_shutdown_hook()().await;
    }
}

/// Implementation block for `HookType`.
///
/// This block defines utility methods associated with the `HookType` enum.
/// These methods provide additional functionality for working with hooks,
/// such as extracting the execution order (priority) used in duplicate checks.
impl HookType {
    /// Returns the optional execution priority (`order`) of a hook.
    ///
    /// Hooks that carry an `order` indicate their execution priority.  
    /// Hooks without an `order` are considered unordered and are ignored in duplicate checks.
    ///
    /// # Returns
    ///
    /// - `Option<isize>` - `Some(order)` if the hook defines a priority, otherwise `None`.
    #[inline]
    pub fn try_get(&self) -> Option<isize> {
        match *self {
            HookType::RequestMiddleware(order)
            | HookType::ResponseMiddleware(order)
            | HookType::PanicHook(order) => order,
            _ => None,
        }
    }
}
