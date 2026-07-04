use crate::*;

/// Default server hook
#[derive(
    Clone, Copy, Debug, Deserialize, DisplayDebug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize,
)]
pub struct DefaultServerHook;

/// Namespace for hook handler factory and utility functions.
///
/// This zero-size struct provides associated functions for creating
/// and managing hook handlers in a semantically organized way.
#[derive(
    Clone, Copy, Debug, Deserialize, DisplayDebug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize,
)]
pub struct Hook;

/// Represents the hooks for managing the server's lifecycle, specifically for waiting and shutting down.
#[derive(Clone, CustomDebug, DisplayDebug, Getter, Setter)]
pub struct ServerControlHook {
    /// A hook that returns a future, which completes when the server's main task finishes.
    /// This is typically used to wait for the server to stop accepting connections before
    /// the application exits.
    #[debug(skip)]
    #[get(pub)]
    #[set(pub(crate))]
    pub(super) wait_hook: ServerControlHookHandler<()>,
    /// A hook that, when called, initiates a graceful shutdown of the server.
    /// This will stop the server from accepting new connections and allow existing ones
    /// to complete.
    #[debug(skip)]
    #[get(pub)]
    #[set(pub(crate))]
    pub(super) shutdown_hook: ServerControlHookHandler<()>,
}
