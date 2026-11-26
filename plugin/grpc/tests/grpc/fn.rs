use super::*;

#[test]
fn test_grpc_frame_roundtrip_compression_bounds_and_timeout_parser() {
    let raw_payload: Vec<u8> = b"protobuf-encoded-payload-hello-world".to_vec();
    let frame: GrpcFrame = GrpcFrame::new(raw_payload.clone());
    let plain_wire: Vec<u8> = frame.encode(Compress::Unknown).unwrap();
    assert_eq!(plain_wire[0], 0u8);
    let decoded_plain: GrpcFrame = GrpcFrame::decode(&plain_wire, Compress::Unknown, 1024).unwrap();
    assert_eq!(decoded_plain.get_payload(), &raw_payload);
    let gzip_wire: Vec<u8> = frame.encode(Compress::Gzip).unwrap();
    assert_eq!(gzip_wire[0], 1u8);
    let decoded_gzip: GrpcFrame = GrpcFrame::decode(&gzip_wire, Compress::Gzip, 1024).unwrap();
    assert_eq!(decoded_gzip.get_payload(), &raw_payload);
    assert_eq!(
        GrpcFrame::decode(&gzip_wire, Compress::Unknown, 1024)
            .unwrap_err()
            .get_status(),
        GrpcStatus::Unimplemented
    );
    assert_eq!(
        GrpcFrame::decode(&[2u8, 0, 0, 0, 0], Compress::Unknown, 1024)
            .unwrap_err()
            .get_status(),
        GrpcStatus::Internal
    );
    let huge_bomb: Vec<u8> = vec![b'A'; 16 * 1024];
    let bomb_wire: Vec<u8> = GrpcFrame::new(huge_bomb).encode(Compress::Gzip).unwrap();
    assert_eq!(
        GrpcFrame::decode(&bomb_wire, Compress::Gzip, 1024)
            .unwrap_err()
            .get_status(),
        GrpcStatus::ResourceExhausted
    );
    assert_eq!(parse_grpc_timeout("5S"), Some(Duration::from_secs(5)));
    assert_eq!(parse_grpc_timeout("250m"), Some(Duration::from_millis(250)));
    assert_eq!(parse_grpc_timeout("bad"), None);
    assert!(is_grpc_content_type(
        "application/grpc+proto; charset=utf-8"
    ));
    assert!(!is_grpc_content_type("application/json"));
    assert_eq!(
        percent_encode_grpc_message("ok %100\r\n"),
        "ok %25100%0D%0A"
    );
}

#[tokio::test]
async fn test_grpc_router_dispatch_unimplemented_and_deadline_exceeded() {
    let mut router: GrpcRouter = GrpcRouter::new();
    router.register(
        "/demo.Echo/Say",
        |payload: Vec<u8>| async move { Ok(payload) },
    );
    router.register("/demo.Echo/Slow", |_payload: Vec<u8>| async move {
        tokio::time::sleep(Duration::from_millis(120)).await;
        Ok(b"late".to_vec())
    });
    let cfg: GrpcConfig = GrpcConfig::default();
    let req_wire: Vec<u8> = GrpcFrame::new(b"ping".to_vec())
        .encode(Compress::Unknown)
        .unwrap();
    let (status, msg, body): (GrpcStatus, String, Vec<u8>) = router
        .dispatch("/demo.Echo/Say", &req_wire, Compress::Unknown, None, &cfg)
        .await;
    assert_eq!(status, GrpcStatus::Ok);
    assert!(msg.is_empty());
    let resp_frame: GrpcFrame = GrpcFrame::decode(&body, Compress::Unknown, 1024).unwrap();
    assert_eq!(resp_frame.get_payload().as_slice(), b"ping");
    let (unimpl, _, _): (GrpcStatus, String, Vec<u8>) = router
        .dispatch(
            "/demo.Echo/Missing",
            &req_wire,
            Compress::Unknown,
            None,
            &cfg,
        )
        .await;
    assert_eq!(unimpl, GrpcStatus::Unimplemented);
    let (timed_out, _, _): (GrpcStatus, String, Vec<u8>) = router
        .dispatch(
            "/demo.Echo/Slow",
            &req_wire,
            Compress::Unknown,
            Some(Duration::from_millis(20)),
            &cfg,
        )
        .await;
    assert_eq!(timed_out, GrpcStatus::DeadlineExceeded);
}
