use super::*;

/// Global initialized [`GrpcPlugin`] instance.
pub(crate) static GRPC_PLUGIN: OnceLock<GrpcPlugin> = OnceLock::new();
