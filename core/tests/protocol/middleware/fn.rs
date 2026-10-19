use super::*;

pub(crate) fn register_probe_hooks(server: &mut Server) -> &mut Server {
    server.request_middleware::<ProbeMiddleware>();
    server.response_middleware::<ProbeResponseMiddleware>();
    server.route::<VersionRoute>(VERSION_ROUTE_PATH);
    server.route::<EchoRoute>(ECHO_ROUTE_PATH);
    server.route::<GatedSlowRoute>(SLOW_ROUTE_PATH);
    server.route::<SplitWriteRoute>(SPLIT_ROUTE_PATH);
    server.route::<DuplicateCookieRoute>(COOKIE_ROUTE_PATH);
    server.route::<LargeResponseRoute>(LARGE_ROUTE_PATH);
    server.route::<BinaryEchoRoute>(BINARY_ROUTE_PATH);
    server.route::<GatedSseStreamRoute>(SSE_ROUTE_PATH);
    server.route::<GatedSseStreamRouteHttp2>(SSE_ROUTE_PATH_HTTP2);
    server.route::<GatedSseStreamRouteHttp3>(SSE_ROUTE_PATH_HTTP3);
    server.route::<GatedSlowRouteHttp3>(SLOW_ROUTE_PATH_HTTP3);
    server.route::<HopByHopStrippingMiddleware>(HOP_BY_HOP_ROUTE_PATH);
    server.route::<HopByHopStrippingRoute>(HOP_BY_HOP_CLEAN_ROUTE_PATH);
    server.route::<SocketMetadataRoute>(SOCKET_METADATA_ROUTE_PATH);
    server.route::<UnframedBodyRoute>(UNFRAMED_BODY_ROUTE_PATH);
    server
}
