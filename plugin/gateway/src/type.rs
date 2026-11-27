use super::*;

/// Shared trait-object handle for a pluggable [`GatewayInterceptor`].
pub type SharedGatewayInterceptor = Arc<dyn GatewayInterceptor>;
