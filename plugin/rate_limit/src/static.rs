use super::*;

/// Global process-wide [`RateLimitPlugin`] singleton initialized via [`RateLimitPlugin::init`].
pub(crate) static RATE_LIMIT_PLUGIN: OnceLock<RateLimitPlugin> = OnceLock::new();
