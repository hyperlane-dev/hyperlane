use super::*;

/// Global process-wide [`HealthPlugin`] singleton initialized via [`HealthPlugin::init`].
pub(crate) static HEALTH_PLUGIN: OnceLock<HealthPlugin> = OnceLock::new();
