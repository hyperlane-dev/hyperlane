use super::*;

fn to_hex(bytes: &[u8]) -> String {
    let mut out: String = String::with_capacity(bytes.len() * 2);
    for b in bytes {
        out.push_str(&format!("{b:02x}"));
    }
    out
}

#[test]
fn test_security_sha256_and_rfc4231_hmac_vectors() {
    let abc_hash: [u8; 32] = sha256(b"abc");
    assert_eq!(
        to_hex(&abc_hash),
        "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
    );
    let case1_key: [u8; 20] = [0x0bu8; 20];
    let case1_mac: [u8; 32] = hmac_sha256(&case1_key, b"Hi There");
    assert_eq!(
        to_hex(&case1_mac),
        "b0344c61d8db38535ca8afceaf0bf12b881dc200c9833da726e9376c2e32cff7"
    );
    let case2_mac: [u8; 32] = hmac_sha256(b"Jefe", b"what do ya want for nothing?");
    assert_eq!(
        to_hex(&case2_mac),
        "5bdcc146bf60754e6a042426089575c75a003f089d2739839dec58b964ec3843"
    );
    let case6_key: [u8; 131] = [0xaau8; 131];
    let case6_mac: [u8; 32] = hmac_sha256(
        &case6_key,
        b"Test Using Larger Than Block-Size Key - Hash Key First",
    );
    assert_eq!(
        to_hex(&case6_mac),
        "60e431591ee0b67f0d8a26aacbf5b77f8e0bc6213728c5140546040f0ee37f54"
    );
}

#[test]
fn test_security_headers_csrf_and_request_guards() {
    let secret: &[u8] = b"super-secret-csrf-signing-key";
    let mut guard: RequestGuardConfig = RequestGuardConfig::default();
    guard
        .set_max_body_bytes(64)
        .set_max_uri_bytes(64)
        .set_allowed_hosts(vec!["api.example.com".to_owned()]);
    let cfg: SecurityConfig = SecurityConfig::strict()
        .with_csrf(CsrfConfig::enabled(secret.to_vec()))
        .with_guard(guard);
    let plugin: SecurityPlugin = SecurityPlugin::new(cfg, Vec::new());
    let token: String = issue_csrf_token(secret, "nonce-12345");
    assert!(verify_csrf_token(secret, &token));
    assert!(!verify_csrf_token(b"wrong-secret", &token));
    let mut ok_ctx: Context = Context::default();
    ok_ctx
        .get_mut_request()
        .set_method(Method::Post)
        .set_path("/v1/orders".to_owned())
        .set_body(b"{\"item\":1}".to_vec());
    ok_ctx.get_mut_request().get_mut_headers().insert(
        HOST.to_owned(),
        VecDeque::from(["api.example.com".to_owned()]),
    );
    ok_ctx.get_mut_request().get_mut_headers().insert(
        DEFAULT_CSRF_HEADER_NAME.to_owned(),
        VecDeque::from([token.clone()]),
    );
    ok_ctx.get_mut_request().get_mut_headers().insert(
        COOKIE.to_owned(),
        VecDeque::from([format!("{DEFAULT_CSRF_COOKIE_NAME}={token}")]),
    );
    assert_eq!(plugin.inspect_request(&ok_ctx), Ok(()));
    ok_ctx.get_mut_response().set_header(X_POWERED_BY, "legacy");
    plugin.apply_headers(ok_ctx.get_mut_response());
    assert_eq!(
        ok_ctx
            .get_response()
            .try_get_header_back(STRICT_TRANSPORT_SECURITY)
            .as_deref(),
        Some(DEFAULT_HSTS_VALUE)
    );
    assert_eq!(
        ok_ctx
            .get_response()
            .try_get_header_back(X_FRAME_OPTIONS)
            .as_deref(),
        Some(DEFAULT_X_FRAME_OPTIONS)
    );
    assert_eq!(
        ok_ctx
            .get_response()
            .try_get_header_back(X_CONTENT_TYPE_OPTIONS)
            .as_deref(),
        Some(DEFAULT_X_CONTENT_TYPE_OPTIONS)
    );
    assert!(
        ok_ctx
            .get_response()
            .try_get_header_back(X_POWERED_BY)
            .is_none()
    );
    let mut trav_ctx: Context = Context::default();
    trav_ctx
        .get_mut_request()
        .set_method(Method::Get)
        .set_path("/files/%2e%2e/etc/passwd".to_owned());
    trav_ctx.get_mut_request().get_mut_headers().insert(
        HOST.to_owned(),
        VecDeque::from(["api.example.com".to_owned()]),
    );
    assert_eq!(
        plugin.inspect_request(&trav_ctx),
        Err(SecurityViolation::PathTraversal)
    );
    let mut bad_host: Context = Context::default();
    bad_host
        .get_mut_request()
        .set_method(Method::Get)
        .set_path("/ok".to_owned());
    bad_host.get_mut_request().get_mut_headers().insert(
        HOST.to_owned(),
        VecDeque::from(["evil.attacker".to_owned()]),
    );
    assert_eq!(
        plugin.inspect_request(&bad_host),
        Err(SecurityViolation::InvalidHost)
    );
    let mut bad_csrf: Context = Context::default();
    bad_csrf
        .get_mut_request()
        .set_method(Method::Post)
        .set_path("/v1/orders".to_owned());
    bad_csrf.get_mut_request().get_mut_headers().insert(
        HOST.to_owned(),
        VecDeque::from(["api.example.com".to_owned()]),
    );
    assert_eq!(
        plugin.inspect_request(&bad_csrf),
        Err(SecurityViolation::CsrfInvalid)
    );
}
