use super::*;

/// Global initialized [`RpcPlugin`] instance.
pub(crate) static RPC_PLUGIN: OnceLock<RpcPlugin> = OnceLock::new();
