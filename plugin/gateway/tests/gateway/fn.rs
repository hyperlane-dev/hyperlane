use super::*;

#[test]
fn test_gateway_prefix_boundary_hop_by_hop_and_weighted_selection() {
    assert!(path_matches_prefix("/api", "/api"));
    assert!(path_matches_prefix("/api/v1/users", "/api"));
    assert!(!path_matches_prefix("/apiv2/users", "/api"));
    assert_eq!(
        build_upstream_url("http://127.0.0.1:9000", "/api/users", "a=1", "/api", true),
        "http://127.0.0.1:9000/users?a=1"
    );
    assert_eq!(
        build_upstream_url("http://127.0.0.1:9000", "/api", "", "/api", true),
        "http://127.0.0.1:9000/"
    );
    let extra: HashSet<String> = HashSet::from(["x-internal-hop".to_owned()]);
    assert!(is_hop_by_hop_header("Connection", &extra));
    assert!(is_hop_by_hop_header("Transfer-Encoding", &extra));
    assert!(is_hop_by_hop_header("X-Internal-Hop", &extra));
    assert!(!is_hop_by_hop_header("X-Request-Id", &extra));
    let mut route: GatewayRoute = GatewayRoute::new(
        "/svc",
        vec![
            UpstreamTarget::new("http://u1.local").with_weight(2),
            UpstreamTarget::new("http://u2.local").with_weight(1),
        ],
    );
    route.set_strategy(LoadBalanceStrategy::WeightedRoundRobin);
    let mut cfg: GatewayConfig = GatewayConfig::default();
    cfg.set_routes(vec![route]);
    let runtime: GatewayRuntime = GatewayRuntime::new(cfg);
    let picks: Vec<String> = (0..3usize)
        .map(|_: usize| runtime.select_target_url(None, "/svc/ping").unwrap())
        .collect();
    assert_eq!(
        picks,
        vec!["http://u1.local", "http://u1.local", "http://u2.local"]
    );
}

#[tokio::test]
async fn test_gateway_end_to_end_forward_rate_limit_and_circuit_breaker() {
    let listener: TcpListener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr: SocketAddr = listener.local_addr().unwrap();
    let server: tokio::task::JoinHandle<()> = tokio::spawn(async move {
        while let Ok((mut stream, _)) = listener.accept().await {
            tokio::spawn(async move {
                let mut buf: [u8; 1024] = [0u8; 1024];
                let n: usize = stream.read(&mut buf).await.unwrap_or(0);
                let req_str: String = String::from_utf8_lossy(&buf[..n]).into_owned();
                let resp_body: String = if req_str.starts_with("GET /hello") {
                    "proxied-ok".to_owned()
                } else {
                    "unexpected".to_owned()
                };
                let wire: String = format!(
                    "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nConnection: close\r\nX-Backend: up\r\n\r\n{resp_body}",
                    resp_body.len()
                );
                let _: Result<(), Error> = stream.write_all(wire.as_bytes()).await;
            });
        }
    });
    let mut route: GatewayRoute =
        GatewayRoute::new("/api", vec![UpstreamTarget::new(format!("http://{addr}"))]);
    route
        .set_strip_prefix(true)
        .set_rate_limit(Some(RateLimitConfig {
            requests_per_second: 1,
            burst: 1,
        }));
    let dead_route: GatewayRoute =
        GatewayRoute::new("/dead", vec![UpstreamTarget::new("http://127.0.0.1:1")]);
    let mut cfg: GatewayConfig = GatewayConfig::default();
    cfg.set_routes(vec![route, dead_route])
        .set_circuit_failure_threshold(1)
        .set_circuit_reset_ms(5_000);
    let runtime: GatewayRuntime = GatewayRuntime::new(cfg);
    let mut ctx: Context = Context::default();
    ctx.get_mut_request()
        .set_method(hyperlane::Method::Get)
        .set_path("/api/hello".to_owned());
    ctx.get_mut_request()
        .get_mut_headers()
        .insert(HOST.to_owned(), VecDeque::from(["gateway.test".to_owned()]));
    ctx.get_mut_request().get_mut_headers().insert(
        CONNECTION.to_owned(),
        VecDeque::from(["keep-alive, x-secret-hop".to_owned()]),
    );
    ctx.get_mut_request().get_mut_headers().insert(
        "x-secret-hop".to_owned(),
        VecDeque::from(["drop-me".to_owned()]),
    );
    assert!(runtime.forward(&mut ctx, Some("10.0.0.5")).await);
    assert_eq!(ctx.get_response().get_status_code(), 200usize);
    assert_eq!(ctx.get_response().get_body().as_slice(), b"proxied-ok");
    assert_eq!(
        ctx.get_response()
            .try_get_header_back("x-backend")
            .as_deref(),
        Some("up")
    );
    let mut limited_ctx: Context = Context::default();
    limited_ctx
        .get_mut_request()
        .set_method(hyperlane::Method::Get)
        .set_path("/api/hello".to_owned());
    assert!(runtime.forward(&mut limited_ctx, None).await);
    assert_eq!(
        limited_ctx.get_response().get_status_code(),
        HttpStatus::TooManyRequests.code()
    );
    let mut dead_ctx1: Context = Context::default();
    dead_ctx1
        .get_mut_request()
        .set_method(hyperlane::Method::Get)
        .set_path("/dead/ping".to_owned());
    assert!(runtime.forward(&mut dead_ctx1, None).await);
    assert_eq!(
        dead_ctx1.get_response().get_status_code(),
        HttpStatus::BadGateway.code()
    );
    let mut dead_ctx2: Context = Context::default();
    dead_ctx2
        .get_mut_request()
        .set_method(hyperlane::Method::Get)
        .set_path("/dead/ping".to_owned());
    assert!(runtime.forward(&mut dead_ctx2, None).await);
    assert_eq!(
        dead_ctx2.get_response().get_status_code(),
        HttpStatus::ServiceUnavailable.code()
    );
    server.abort();
}
