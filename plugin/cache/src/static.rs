use super::*;

/// Global process-wide [`CachePlugin`] singleton initialized via [`CachePlugin::init`].
pub(crate) static CACHE_PLUGIN: OnceLock<CachePlugin> = OnceLock::new();
