use crate::*;

/// A blanket implementation for any function that takes a `Context` and returns a value.
///
/// This implementation makes it easy to use any compatible function as a `FnContext`,
/// promoting a flexible and functional programming style.
impl<F, R> FnContext<R> for F where F: Fn(&mut Context) -> R + Send + Sync {}

/// A blanket implementation for functions that return a pinned, boxed, sendable future.
///
/// This trait is a common pattern for asynchronous handlers in Rust, enabling type
/// erasure and dynamic dispatch for futures. It is essential for storing different
/// async functions in a collection.
impl<F, T> FnContextPinBox<T> for F where F: FnContext<FutureBox<T>> {}

/// A blanket implementation for static, sendable, synchronous functions that return a future.
///
/// This trait is used for handlers that are known at compile time, ensuring they
/// are safe to be sent across threads and have a static lifetime. This is crucial
/// for handlers that are part of the application's long-lived state.
impl<F, Fut, T> FnContextStatic<Fut, T> for F
where
    F: FnContext<Fut> + 'static,
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

/// Blanket implementation of `FutureFn` for any type that satisfies the bounds.
impl<T, O> FutureFn<O> for T where T: Fn() -> FutureBox<O> + Send + Sync {}

/// Provides a default implementation for `ServerControlHook`.
impl Default for ServerControlHook {
    /// Creates a new `ServerControlHook` instance with default no-op hooks.
    ///
    /// The default `wait_hook` and `shutdown_hook` do nothing, allowing the server
    /// to run without specific shutdown or wait logic unless configured otherwise.
    ///
    /// # Returns
    ///
    /// - `Self` - A new instance with default hooks.
    #[inline(always)]
    fn default() -> Self {
        Self {
            wait_hook: default_server_control_hook_handler(),
            shutdown_hook: default_server_control_hook_handler(),
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

/// Implements the `PartialEq` trait for `HookType`.
///
/// This allows for comparing two `HookType` instances for equality.
/// Function pointers are compared using `std::ptr::fn_addr_eq` for reliable comparison.
impl PartialEq for HookType {
    /// Checks if two `HookType` instances are equal.
    ///
    /// # Arguments
    ///
    /// - `&Self` - The other `HookType` instance to compare against.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` if the instances are equal, `false` otherwise.
    #[inline(always)]
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (HookType::TaskPanic(order1, factory1), HookType::TaskPanic(order2, factory2)) => {
                order1 == order2 && std::ptr::fn_addr_eq(*factory1, *factory2)
            }
            (
                HookType::RequestError(order1, factory1),
                HookType::RequestError(order2, factory2),
            ) => order1 == order2 && std::ptr::fn_addr_eq(*factory1, *factory2),
            (
                HookType::RequestMiddleware(order1, factory1),
                HookType::RequestMiddleware(order2, factory2),
            ) => order1 == order2 && std::ptr::fn_addr_eq(*factory1, *factory2),
            (HookType::Route(path1, factory1), HookType::Route(path2, factory2)) => {
                path1 == path2 && std::ptr::fn_addr_eq(*factory1, *factory2)
            }
            (
                HookType::ResponseMiddleware(order1, factory1),
                HookType::ResponseMiddleware(order2, factory2),
            ) => order1 == order2 && std::ptr::fn_addr_eq(*factory1, *factory2),
            _ => false,
        }
    }
}

/// Implements the `Eq` trait for `HookType`.
///
/// This indicates that `HookType` has a total equality relation.
impl Eq for HookType {}

/// Implements the `Hash` trait for `HookType`.
///
/// This allows `HookType` to be used as a key in hash-based collections.
/// Function pointers are hashed using their addresses.
impl Hash for HookType {
    /// Hashes the `HookType` instance.
    ///
    /// # Arguments
    ///
    /// - `&mut Hasher` - The hasher to use.
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            HookType::TaskPanic(order, factory) => {
                0u8.hash(state);
                order.hash(state);
                (factory as *const fn() -> ServerHookHandler).hash(state);
            }
            HookType::RequestError(order, factory) => {
                1u8.hash(state);
                order.hash(state);
                (factory as *const fn() -> ServerHookHandler).hash(state);
            }
            HookType::RequestMiddleware(order, factory) => {
                2u8.hash(state);
                order.hash(state);
                (factory as *const fn() -> ServerHookHandler).hash(state);
            }
            HookType::Route(path, factory) => {
                3u8.hash(state);
                path.hash(state);
                (factory as *const fn() -> ServerHookHandler).hash(state);
            }
            HookType::ResponseMiddleware(order, factory) => {
                4u8.hash(state);
                order.hash(state);
                (factory as *const fn() -> ServerHookHandler).hash(state);
            }
        }
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
    #[inline(always)]
    pub fn try_get_order(&self) -> Option<isize> {
        match *self {
            HookType::RequestMiddleware(order, _)
            | HookType::ResponseMiddleware(order, _)
            | HookType::TaskPanic(order, _)
            | HookType::RequestError(order, _) => order,
            _ => None,
        }
    }

    #[inline(always)]
    pub fn try_get_hook(&self) -> Option<ServerHookHandlerFactory> {
        match *self {
            HookType::RequestMiddleware(_, hook)
            | HookType::ResponseMiddleware(_, hook)
            | HookType::TaskPanic(_, hook)
            | HookType::RequestError(_, hook) => Some(hook),
            _ => None,
        }
    }
}

/// Implement `ServerHook` for `DefaultServerHook`
///
/// This implementation provides default no-op handlers for server hook operations.
impl ServerHook for DefaultServerHook {
    /// Creates a new `DefaultServerHook` instance.
    ///
    /// # Arguments
    ///
    /// - `&mut Context` - The context object providing server configuration and state
    ///
    /// # Returns
    ///
    /// - `Self` - A new instance of `DefaultServerHook`
    async fn new(_: &mut Context) -> Self {
        Self
    }

    /// Handles server hook operations with a no-op implementation.
    ///
    /// # Arguments
    ///
    /// - `&mut Context` - The context object providing server configuration and state
    async fn handle(self, _: &mut Context) {}
}
