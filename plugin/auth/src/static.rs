use super::*;

/// Global process-wide [`AuthPlugin`] singleton initialized via [`AuthPlugin::init`].
pub(crate) static AUTH_PLUGIN: OnceLock<AuthPlugin> = OnceLock::new();
