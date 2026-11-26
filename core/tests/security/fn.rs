use super::*;

async fn start_security_server(protocol: ServerProtocol) -> (ServerControlHook, SocketAddr) {
    let listener: TcpListener = TcpListener::bind("127.0.0.1:0").unwrap();
    let addr: SocketAddr = listener.local_addr().unwrap();
    drop(listener);
    let mut cfg: ServerConfig = ServerConfig::default();
    cfg.set_address(addr.to_string()).set_protocol(protocol);
    let mut server: Server = Server::default();
    server
        .server_config(cfg)
        .request_middleware::<RejectGuardMiddleware>()
        .route::<SetAttrRoute>("/set-attr")
        .route::<CheckIsolationRoute>("/check")
        .route::<PanicRoute>("/panic")
        .route::<RegexEchoRoute>("/items/{item:foo|foobar}")
        .route::<AdminRoute>("/admin")
        .route::<AdminRoute>("/deny")
        .response_middleware::<SecuritySendMiddleware>();
    let control: ServerControlHook = server.run().await.unwrap();
    (control, addr)
}

#[tokio::test]
async fn test_security_issue26_pr46_pr54_keepalive_isolation_pipelining_and_routing() {
    let (control, addr): (ServerControlHook, SocketAddr) =
        start_security_server(ServerProtocol::Http1).await;
    let mut stream: TcpStream = TcpStream::connect(addr).await.unwrap();
    let mut pipelined: Vec<u8> = b"GET /set-attr HTTP/1.1\r\nHost: localhost\r\n\r\n\
        GET /check HTTP/1.1\r\nHost: localhost\r\n\r\n\
        GET /items/foo HTTP/1.1\r\nHost: localhost\r\n\r\n\
        GET /items/foobar HTTP/1.1\r\nHost: localhost\r\n\r\n\
        GET /items/foobaz HTTP/1.1\r\nHost: localhost\r\n\r\n"
        .to_vec();
    pipelined.extend_from_slice(b"GET /");
    pipelined.extend_from_slice(
        b"/admin HTTP/1.1\r\nHost: localhost\r\n\r\n\
        GET /deny HTTP/1.1\r\nHost: localhost\r\nConnection: close\r\n\r\n",
    );
    stream.write_all(&pipelined).await.unwrap();
    let mut raw: Vec<u8> = Vec::new();
    let _: usize = timeout(Duration::from_secs(3), stream.read_to_end(&mut raw))
        .await
        .unwrap()
        .unwrap();
    let transcript: String = String::from_utf8_lossy(&raw).into_owned();
    assert!(transcript.contains("attr-set"));
    assert!(transcript.contains("false,false,false"));
    assert!(transcript.contains("matched:foo"));
    assert!(transcript.contains("matched:foobar"));
    assert!(!transcript.contains("matched:foobaz"));
    assert!(!transcript.contains("admin-secret"));
    assert!(transcript.contains("403 Forbidden"));
    assert!(transcript.contains("denied"));
    let mut panic_stream: TcpStream = TcpStream::connect(addr).await.unwrap();
    panic_stream
        .write_all(b"GET /panic HTTP/1.1\r\nHost: localhost\r\nConnection: close\r\n\r\n")
        .await
        .unwrap();
    let mut panic_raw: Vec<u8> = Vec::new();
    let _: Result<io::Result<usize>, Elapsed> = timeout(
        Duration::from_secs(2),
        panic_stream.read_to_end(&mut panic_raw),
    )
    .await;
    let mut post_panic: TcpStream = TcpStream::connect(addr).await.unwrap();
    post_panic
        .write_all(b"GET /check HTTP/1.1\r\nHost: localhost\r\nConnection: close\r\n\r\n")
        .await
        .unwrap();
    let mut post_raw: Vec<u8> = Vec::new();
    let _: usize = timeout(
        Duration::from_secs(2),
        post_panic.read_to_end(&mut post_raw),
    )
    .await
    .unwrap()
    .unwrap();
    assert!(String::from_utf8_lossy(&post_raw).contains("false,false,false"));
    control.shutdown().await;
    control.wait().await;
}

#[tokio::test]
async fn test_security_http2_rejects_forbidden_headers_and_duplicate_cl() {
    let (control, addr): (ServerControlHook, SocketAddr) =
        start_security_server(ServerProtocol::Http2).await;
    let tcp: TcpStream = TcpStream::connect(addr).await.unwrap();
    let (client, conn): (SendRequest<Bytes>, h2::client::Connection<TcpStream, Bytes>) =
        handshake(tcp).await.unwrap();
    let driver: JoinHandle<()> = spawn(async move {
        let _: Result<(), Error> = conn.await;
    });
    let mut ready_client: SendRequest<Bytes> = client.clone().ready().await.unwrap();
    let (ok_fut, _): (ResponseFuture, SendStream<Bytes>) = ready_client
        .send_request(
            Builder::new()
                .method("GET")
                .uri(format!("http://{addr}/check"))
                .body(())
                .unwrap(),
            true,
        )
        .unwrap();
    let ok_status: u16 = timeout(Duration::from_secs(2), ok_fut)
        .await
        .unwrap()
        .unwrap()
        .status()
        .as_u16();
    assert_eq!(ok_status, 200u16);
    let mut ready_bad: SendRequest<Bytes> = client.ready().await.unwrap();
    let (bad_fut, _): (ResponseFuture, SendStream<Bytes>) = ready_bad
        .send_request(
            Builder::new()
                .method("POST")
                .uri(format!("http://{addr}/check"))
                .header("content-length", "5")
                .header("content-length", "6")
                .body(())
                .unwrap(),
            true,
        )
        .unwrap();
    if let Ok(resp) = timeout(Duration::from_secs(2), bad_fut).await.unwrap() {
        assert_eq!(resp.status().as_u16(), 400u16);
    }
    driver.abort();
    control.shutdown().await;
    control.wait().await;
}
