use super::*;

fn to_hex(bytes: &[u8]) -> String {
    let mut out: String = String::with_capacity(bytes.len() * 2);
    for b in bytes {
        out.push_str(&format!("{b:02x}"));
    }
    out
}

#[test]
fn test_auth_sha256_hmac_rfc4231_and_jwt_rotation_and_security() {
    assert_eq!(
        to_hex(&sha256(b"abc")),
        "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
    );
    assert_eq!(
        to_hex(&hmac_sha256(b"Jefe", b"what do ya want for nothing?")),
        "5bdcc146bf60754e6a042426089575c75a003f089d2739839dec58b964ec3843"
    );
    let codec: JwtCodec = JwtCodec::new(b"primary-key-v2".to_vec())
        .with_key("v1", b"legacy-key-v1".to_vec())
        .with_default_kid("v2")
        .with_leeway_secs(5)
        .with_required_issuer("hyperlane-idp")
        .with_required_audience("orders-api");
    let claims: JwtClaims = JwtClaims::new("user-42", 300)
        .with_roles(["admin", "reader"])
        .with_issuer("hyperlane-idp")
        .with_audience("orders-api");
    let token: String = codec.encode(&claims).unwrap();
    let decoded: JwtClaims = codec.decode(&token).unwrap();
    assert_eq!(decoded.get_sub(), "user-42");
    assert_eq!(
        decoded.get_roles(),
        &vec!["admin".to_owned(), "reader".to_owned()]
    );
    let tampered: String = format!("{token}x");
    assert_eq!(codec.decode(&tampered), Err(AuthError::InvalidCredentials));
    let expired_at: u64 = claims.try_get_exp().unwrap() + 10;
    assert_eq!(
        codec.decode_at(&token, expired_at),
        Err(AuthError::TokenExpired)
    );
    let alg_none_header: String = base64url_encode(br#"{"alg":"none","typ":"JWT"}"#);
    let payload: String = base64url_encode(br#"{"sub":"attacker"}"#);
    let forged: String = format!("{alg_none_header}.{payload}.AAAA");
    assert_eq!(codec.decode(&forged), Err(AuthError::UnsupportedAlgorithm));
}

#[tokio::test]
async fn test_auth_composite_jwt_apikey_and_basic_authenticators() {
    let codec: JwtCodec = JwtCodec::new(b"jwt-secret".to_vec());
    let jwt_token: String = codec
        .encode(&JwtClaims::new("alice", 600).with_roles(["editor"]))
        .unwrap();
    let composite: CompositeAuthenticator = CompositeAuthenticator::new(vec![
        Arc::new(JwtAuthenticator::new(codec)),
        Arc::new(ApiKeyAuthenticator::default().with_key("k_live_999", "svc-billing", ["service"])),
        Arc::new(BasicAuthenticator::new().with_user("ops", "s3cr3t", ["ops"])),
    ]);
    let mut jwt_ctx: Context = Context::default();
    jwt_ctx.get_mut_request().get_mut_headers().insert(
        AUTHORIZATION.to_owned(),
        VecDeque::from([format!("Bearer {jwt_token}")]),
    );
    let jwt_principal: AuthPrincipal = composite.authenticate(&jwt_ctx).await.unwrap();
    assert_eq!(jwt_principal.get_subject(), "alice");
    assert_eq!(jwt_principal.get_scheme(), AuthScheme::BearerJwt);
    assert!(jwt_principal.has_role("editor"));
    let mut key_ctx: Context = Context::default();
    key_ctx.get_mut_request().get_mut_headers().insert(
        X_API_KEY.to_owned(),
        VecDeque::from(["k_live_999".to_owned()]),
    );
    let key_principal: AuthPrincipal = composite.authenticate(&key_ctx).await.unwrap();
    assert_eq!(key_principal.get_subject(), "svc-billing");
    assert_eq!(key_principal.get_scheme(), AuthScheme::ApiKey);
    let mut basic_ctx: Context = Context::default();
    basic_ctx.get_mut_request().get_mut_headers().insert(
        AUTHORIZATION.to_owned(),
        VecDeque::from(["Basic b3BzOnMzY3IzdA==".to_owned()]),
    );
    let basic_principal: AuthPrincipal = composite.authenticate(&basic_ctx).await.unwrap();
    assert_eq!(basic_principal.get_subject(), "ops");
    assert_eq!(basic_principal.get_scheme(), AuthScheme::Basic);
    let empty_ctx: Context = Context::default();
    assert_eq!(
        composite.authenticate(&empty_ctx).await,
        Err(AuthError::MissingCredentials)
    );
}
