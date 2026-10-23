use super::*;

/// The configuration the plugin was given, when it was given one.
pub(super) static CONFIG: OnceLock<ShutdownConfig> = OnceLock::new();

/// The handler that runs when the server stops, when one was set.
pub(super) static HANDLER: OnceLock<ServerControlHookHandler<()>> = OnceLock::new();
