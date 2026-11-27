use super::*;

#[tokio::test]
async fn test_static_files_cache_etag_range_sidecar_and_traversal_defense() {
    let dir: PathBuf = temp_dir().join(format!("hyperlane-static-test-{}", process::id()));
    let _: io::Result<()> = remove_dir_all(&dir);
    create_dir_all(&dir).unwrap();
    write(dir.join("app.js"), b"console.log('hyperlane');").unwrap();
    write(dir.join("app.js.br"), b"BROTLI_PAYLOAD").unwrap();
    write(dir.join(".env"), b"SECRET=1").unwrap();
    let cfg: StaticFilesConfig = StaticFilesConfig::new(&dir).with_mount_prefix("/assets");
    let runtime: StaticFilesRuntime = StaticFilesRuntime::new(cfg).unwrap();
    let mut ctx: Context = Context::default();
    ctx.get_mut_request()
        .set_method(Method::Get)
        .set_path("/assets/app.js".to_owned());
    assert!(runtime.serve_context(&mut ctx).await);
    assert_eq!(ctx.get_response().get_status_code(), HttpStatus::Ok.code());
    assert_eq!(
        ctx.get_response().get_body().as_slice(),
        b"console.log('hyperlane');"
    );
    assert!(
        ctx.get_response()
            .try_get_header_back(CONTENT_TYPE)
            .unwrap()
            .contains("javascript")
    );
    assert!(runtime.cached_bytes() >= 25usize);
    let etag: String = ctx.get_response().try_get_header_back(ETAG).unwrap();
    let mut inm_ctx: Context = Context::default();
    inm_ctx
        .get_mut_request()
        .set_method(Method::Get)
        .set_path("/assets/app.js".to_owned());
    inm_ctx
        .get_mut_request()
        .get_mut_headers()
        .insert(IF_NONE_MATCH.to_owned(), VecDeque::from([etag]));
    assert!(runtime.serve_context(&mut inm_ctx).await);
    assert_eq!(
        inm_ctx.get_response().get_status_code(),
        HttpStatus::NotModified.code()
    );
    assert!(inm_ctx.get_response().get_body().is_empty());
    let mut range_ctx: Context = Context::default();
    range_ctx
        .get_mut_request()
        .set_method(Method::Get)
        .set_path("/assets/app.js".to_owned());
    range_ctx
        .get_mut_request()
        .get_mut_headers()
        .insert(RANGE.to_owned(), VecDeque::from(["bytes=0-6".to_owned()]));
    assert!(runtime.serve_context(&mut range_ctx).await);
    assert_eq!(
        range_ctx.get_response().get_status_code(),
        HttpStatus::PartialContent.code()
    );
    assert_eq!(range_ctx.get_response().get_body().as_slice(), b"console");
    assert_eq!(
        range_ctx
            .get_response()
            .try_get_header_back(CONTENT_RANGE)
            .as_deref(),
        Some("bytes 0-6/25")
    );
    let mut br_ctx: Context = Context::default();
    br_ctx
        .get_mut_request()
        .set_method(Method::Get)
        .set_path("/assets/app.js".to_owned());
    br_ctx.get_mut_request().get_mut_headers().insert(
        ACCEPT_ENCODING.to_owned(),
        VecDeque::from(["gzip, br".to_owned()]),
    );
    assert!(runtime.serve_context(&mut br_ctx).await);
    assert_eq!(
        br_ctx
            .get_response()
            .try_get_header_back(CONTENT_ENCODING)
            .as_deref(),
        Some("br")
    );
    assert_eq!(
        br_ctx.get_response().get_body().as_slice(),
        b"BROTLI_PAYLOAD"
    );
    let mut trav_ctx: Context = Context::default();
    trav_ctx
        .get_mut_request()
        .set_method(Method::Get)
        .set_path("/assets/%2e%2e/etc/passwd".to_owned());
    assert!(!runtime.serve_context(&mut trav_ctx).await);
    let mut dot_ctx: Context = Context::default();
    dot_ctx
        .get_mut_request()
        .set_method(Method::Get)
        .set_path("/assets/.env".to_owned());
    assert!(!runtime.serve_context(&mut dot_ctx).await);
    let _: io::Result<()> = remove_dir_all(&dir);
}
