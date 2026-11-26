use super::*;

/// Global initialized [`GatewayPlugin`] instance.
pub(crate) static GATEWAY_PLUGIN: OnceLock<GatewayPlugin> = OnceLock::new();
