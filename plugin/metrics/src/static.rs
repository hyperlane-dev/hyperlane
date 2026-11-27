use super::*;

/// Global process-wide [`MetricsPlugin`] singleton initialized via [`MetricsPlugin::init`].
pub(crate) static METRICS_PLUGIN: OnceLock<MetricsPlugin> = OnceLock::new();
