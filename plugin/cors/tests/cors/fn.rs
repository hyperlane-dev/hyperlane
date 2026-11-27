use super::*;

#[test]
fn test_cors_exact_suffix_custom_and_credentials_safety() {
    let exact: CompiledCors = CompiledCors::compile(
        CorsConfig::strict(["https://app.example.com"])
            .with_allow_credentials(true)
            .with_expose_headers(["x-request-id"]),
    );
    assert_eq!(
        exact.resolve_origin("https://APP.example.com"),
        Some("https://APP.example.com")
    );
    assert_eq!(exact.resolve_origin("https://evil.example.com"), None);
    let any_creds: CompiledCors =
        CompiledCors::compile(CorsConfig::permissive().with_allow_credentials(true));
    assert_eq!(
        any_creds.resolve_origin("https://tenant.io"),
        Some("https://tenant.io")
    );
    let suffix: CompiledCors = CompiledCors::compile(
        CorsConfig::default().with_allow_origin(AllowOrigin::suffix([".svc.local"])),
    );
    assert_eq!(
        suffix.resolve_origin("https://api.svc.local:8443"),
        Some("https://api.svc.local:8443")
    );
    assert_eq!(suffix.resolve_origin("https://evilsvc.local"), None);
    let custom: CompiledCors = CompiledCors::compile(CorsConfig::default().with_allow_origin(
        AllowOrigin::custom(|origin: &str| origin.starts_with("https://ok.")),
    ));
    assert_eq!(
        custom.resolve_origin("https://ok.domain"),
        Some("https://ok.domain")
    );
    assert_eq!(custom.resolve_origin("http://ok.domain"), None);
}

#[test]
fn test_cors_preflight_and_normal_request_headers() {
    let policy: CompiledCors = CompiledCors::compile(
        CorsConfig::strict(["https://web.example.com"])
            .with_allow_credentials(true)
            .with_allow_headers(["*"])
            .with_expose_headers(["x-trace-id"])
            .with_allow_private_network(true),
    );
    let mut preflight_ctx: Context = Context::default();
    preflight_ctx
        .get_mut_request()
        .set_method(Method::Options)
        .get_mut_headers()
        .insert(
            ORIGIN.to_owned(),
            VecDeque::from(["https://web.example.com".to_owned()]),
        );
    preflight_ctx.get_mut_request().get_mut_headers().insert(
        ACCESS_CONTROL_REQUEST_METHOD.to_owned(),
        VecDeque::from(["POST".to_owned()]),
    );
    preflight_ctx.get_mut_request().get_mut_headers().insert(
        ACCESS_CONTROL_REQUEST_HEADERS.to_owned(),
        VecDeque::from(["authorization, x-custom".to_owned()]),
    );
    preflight_ctx.get_mut_request().get_mut_headers().insert(
        ACCESS_CONTROL_REQUEST_PRIVATE_NETWORK.to_owned(),
        VecDeque::from(["true".to_owned()]),
    );
    assert!(policy.apply_request(&mut preflight_ctx));
    assert_eq!(
        preflight_ctx.get_response().get_status_code(),
        HttpStatus::NoContent.code()
    );
    assert_eq!(
        preflight_ctx
            .get_response()
            .try_get_header_back(ACCESS_CONTROL_ALLOW_ORIGIN)
            .as_deref(),
        Some("https://web.example.com")
    );
    assert_eq!(
        preflight_ctx
            .get_response()
            .try_get_header_back(ACCESS_CONTROL_ALLOW_CREDENTIALS)
            .as_deref(),
        Some("true")
    );
    assert_eq!(
        preflight_ctx
            .get_response()
            .try_get_header_back(ACCESS_CONTROL_ALLOW_HEADERS)
            .as_deref(),
        Some("authorization, x-custom")
    );
    assert!(
        preflight_ctx
            .get_response()
            .try_get_header_back(ACCESS_CONTROL_ALLOW_METHODS)
            .is_some()
    );
    assert_eq!(
        preflight_ctx
            .get_response()
            .try_get_header_back(ACCESS_CONTROL_MAX_AGE)
            .as_deref(),
        Some("600")
    );
    assert_eq!(
        preflight_ctx
            .get_response()
            .try_get_header_back(ACCESS_CONTROL_ALLOW_PRIVATE_NETWORK)
            .as_deref(),
        Some("true")
    );
    let mut normal_ctx: Context = Context::default();
    normal_ctx
        .get_mut_request()
        .set_method(Method::Get)
        .get_mut_headers()
        .insert(
            ORIGIN.to_owned(),
            VecDeque::from(["https://web.example.com".to_owned()]),
        );
    assert!(!policy.apply_request(&mut normal_ctx));
    assert_eq!(
        normal_ctx
            .get_response()
            .try_get_header_back(ACCESS_CONTROL_EXPOSE_HEADERS)
            .as_deref(),
        Some("x-trace-id")
    );
    assert_eq!(
        normal_ctx
            .get_response()
            .try_get_header_back(VARY)
            .as_deref(),
        Some("Origin")
    );
    normal_ctx.get_mut_response().clear_headers();
    policy.ensure_response_headers(&mut normal_ctx);
    assert_eq!(
        normal_ctx
            .get_response()
            .try_get_header_back(ACCESS_CONTROL_ALLOW_ORIGIN)
            .as_deref(),
        Some("https://web.example.com")
    );
}
