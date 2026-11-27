use super::*;

/// Global process-wide [`WebSocketPlugin`] singleton initialized via [`WebSocketPlugin::init`].
pub(crate) static WEBSOCKET_PLUGIN: OnceLock<WebSocketPlugin> = OnceLock::new();
