use crate::*;

/// Represents the hooks for managing the server's lifecycle, specifically for waiting and shutting down.
///
/// This struct is returned by the `run` method and provides two key hooks:
/// - `wait_hook`- A future that resolves when the server has stopped accepting new connections.
/// - `shutdown_hook`- A function that can be called to gracefully shut down the server.
#[derive(Clone, CustomDebug, DisplayDebug, Getter, Setter)]
pub struct ServerControlHook {
    /// A hook that returns a future, which completes when the server's main task finishes.
    /// This is typically used to wait for the server to stop accepting connections before
    /// the application exits.
    #[debug(skip)]
    #[get(pub)]
    #[set(pub(crate))]
    pub(super) wait_hook: SharedAsyncTaskFactory<()>,
    /// A hook that, when called, initiates a graceful shutdown of the server.
    /// This will stop the server from accepting new connections and allow existing ones
    /// to complete.
    #[debug(skip)]
    #[get(pub)]
    #[set(pub(crate))]
    pub(super) shutdown_hook: SharedAsyncTaskFactory<()>,
}

/// Represents a route definition created by a macro.
///
/// This struct encapsulates the necessary information to register a new hook.
#[derive(Getter, Setter, Clone, CustomDebug, PartialEq, Eq)]
pub struct HookMacro {
    /// Represents the asynchronous handler that is executed when
    /// the associated hook is triggered.
    #[debug(skip)]
    pub handler: HookHandler,
    /// Represents the type of the hook that determines when the handler
    /// should be executed.
    pub hook_type: HookType,
}

impl HookMacro {
    /// Creates a new HookMacro for a panic hook with a generic type.
    ///
    /// # Type Parameters
    ///
    /// - `P: ServerHook` - The panic hook type.
    ///
    /// # Arguments
    ///
    /// - `order` - Optional execution priority.
    ///
    /// # Returns
    ///
    /// - `Self` - The created HookMacro instance.
    pub fn panic_hook<P: ServerHook>(order: Option<isize>) -> Self {
        Self {
            handler: HookHandler::Factory(create_panic_hook::<P>),
            hook_type: HookType::PanicHook(order),
        }
    }

    /// Creates a new HookMacro for request middleware with a generic type.
    ///
    /// # Type Parameters
    ///
    /// - `M: ServerHook` - The middleware type.
    ///
    /// # Arguments
    ///
    /// - `order` - Optional execution priority.
    ///
    /// # Returns
    ///
    /// - `Self` - The created HookMacro instance.
    pub fn request_middleware<M: ServerHook>(order: Option<isize>) -> Self {
        Self {
            handler: HookHandler::Factory(create_middleware_hook::<M>),
            hook_type: HookType::RequestMiddleware(order),
        }
    }

    /// Creates a new HookMacro for response middleware with a generic type.
    ///
    /// # Type Parameters
    ///
    /// - `M: ServerHook` - The middleware type.
    ///
    /// # Arguments
    ///
    /// - `order` - Optional execution priority.
    ///
    /// # Returns
    ///
    /// - `Self` - The created HookMacro instance.
    pub fn response_middleware<M: ServerHook>(order: Option<isize>) -> Self {
        Self {
            handler: HookHandler::Factory(create_middleware_hook::<M>),
            hook_type: HookType::ResponseMiddleware(order),
        }
    }

    /// Creates a new HookMacro for a route with a generic type.
    ///
    /// # Type Parameters
    ///
    /// - `R: ServerHook` - The route handler type.
    ///
    /// # Arguments
    ///
    /// - `path` - The route path.
    ///
    /// # Returns
    ///
    /// - `Self` - The created HookMacro instance.
    pub fn route<R: ServerHook>(path: &'static str) -> Self {
        Self {
            handler: HookHandler::Factory(create_route_hook::<R>),
            hook_type: HookType::Route(path),
        }
    }
}
