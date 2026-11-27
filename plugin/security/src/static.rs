use super::*;

/// Global process-wide [`SecurityPlugin`] singleton initialized via [`SecurityPlugin::init`].
pub(crate) static SECURITY_PLUGIN: OnceLock<SecurityPlugin> = OnceLock::new();
