use super::*;

/// Global process-wide [`ShutdownPlugin`] singleton initialized via [`ShutdownPlugin::init`].
pub(crate) static SHUTDOWN_PLUGIN: OnceLock<ShutdownPlugin> = OnceLock::new();
