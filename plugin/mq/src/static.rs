use super::*;

/// Global initialized [`MqPlugin`] instance.
pub(crate) static MQ_PLUGIN: OnceLock<MqPlugin> = OnceLock::new();
