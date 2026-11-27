use super::*;

/// Global process-wide [`CorsPlugin`] singleton initialized via [`CorsPlugin::init`].
pub(crate) static CORS_PLUGIN: OnceLock<CorsPlugin> = OnceLock::new();
