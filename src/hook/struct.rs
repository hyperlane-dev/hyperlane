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

/// Represents different handler types for hooks.
#[derive(Clone)]
pub enum HookHandler {
    /// Function-based handler (used for panic hooks)
    Function(fn(Context) -> SendableAsyncTask<()>),
    /// Arc handler (used for request/response middleware and routes)
    Handler(ServerHookHandler),
}

impl std::fmt::Debug for HookHandler {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HookHandler::Function(_) => write!(f, "Function"),
            HookHandler::Handler(_) => write!(f, "Handler"),
        }
    }
}

impl PartialEq for HookHandler {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (HookHandler::Function(a), HookHandler::Function(b)) => {
                std::ptr::eq(a as *const _, b as *const _)
            }
            (HookHandler::Handler(a), HookHandler::Handler(b)) => Arc::ptr_eq(a, b),
            _ => false,
        }
    }
}

impl Eq for HookHandler {}

/// Represents a route definition created by a macro.
///
/// This struct encapsulates the necessary information to register a new hook.
#[derive(Getter, Setter, Clone, Debug, PartialEq, Eq)]
pub struct HookMacro {
    /// Represents the asynchronous handler that is executed when
    /// the associated hook is triggered.
    pub handler: HookHandler,
    /// Represents the type of the hook that determines when the handler
    /// should be executed.
    pub hook_type: HookType,
}
