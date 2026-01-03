use crate::*;

/// Default server hook
#[derive(Clone, Copy, Debug, DisplayDebug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DefaultServerHook;

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
