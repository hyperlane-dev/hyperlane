use super::*;

async fn parse_h1_wire(raw: &[u8], cfg: RequestConfig) -> Result<Request, RequestError> {
    let listener: TcpListener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr: SocketAddr = listener.local_addr().unwrap();
    let payload: Vec<u8> = raw.to_vec();
    let writer: JoinHandle<()> = spawn(async move {
        let mut client: TcpStream = TcpStream::connect(addr).await.unwrap();
        let _: Result<(), io::Error> = client.write_all(&payload).await;
        let _: Result<(), io::Error> = client.shutdown().await;
    });
    let (server, _): (TcpStream, SocketAddr) = listener.accept().await.unwrap();
    let mut stream: Stream = Stream::new(server, cfg, false);
    let res: Result<Request, RequestError> = stream.try_get_http_request().await;
    let _: Result<(), JoinError> = writer.await;
    res
}

#[tokio::test]
async fn test_security_pr52_pr54_pr56_http1_smuggling_and_bounds() {
    let mut cfg: RequestConfig = RequestConfig::default();
    cfg.set_max_body_size(64).set_max_path_size(256);
    let cl_and_te: &[u8] = b"POST / HTTP/1.1\r\nHost: ex\r\nContent-Length: 5\r\nTransfer-Encoding: chunked\r\n\r\n5\r\nhello\r\n0\r\n\r\n";
    assert!(parse_h1_wire(cl_and_te, cfg).await.is_err());
    let dup_cl: &[u8] =
        b"POST / HTTP/1.1\r\nHost: ex\r\nContent-Length: 5\r\nContent-Length: 6\r\n\r\nhello!";
    assert!(parse_h1_wire(dup_cl, cfg).await.is_err());
    let obs_fold: &[u8] = b"GET / HTTP/1.1\r\nHost: ex\r\nX-Fold: a\r\n b\r\n\r\n";
    assert!(parse_h1_wire(obs_fold, cfg).await.is_err());
    let h2_on_h1: &[u8] = b"GET / HTTP/2.0\r\nHost: ex\r\n\r\n";
    assert!(parse_h1_wire(h2_on_h1, cfg).await.is_err());
    let huge_cl: &[u8] = b"POST / HTTP/1.1\r\nHost: ex\r\nContent-Length: 999999999\r\n\r\n";
    assert!(parse_h1_wire(huge_cl, cfg).await.is_err());
    let hash_before_query: &[u8] = b"GET /safe#frag?evil=1 HTTP/1.1\r\nHost: ex\r\n\r\n";
    let parsed_frag: Request = parse_h1_wire(hash_before_query, cfg).await.unwrap();
    assert_eq!(parsed_frag.get_path(), "/safe");
    assert!(parsed_frag.get_querys().is_empty());
    let multi_conn: &[u8] = b"GET / HTTP/1.1\r\nHost: ex\r\nConnection: keep-alive, close\r\n\r\n";
    let parsed_conn: Request = parse_h1_wire(multi_conn, cfg).await.unwrap();
    assert!(!parsed_conn.is_enable_keep_alive());
}

#[test]
fn test_security_pr52_pr56_issue26_response_crlf_and_framing() {
    let mut resp: Response = Response::default();
    resp.set_version(HttpVersion::Http2)
        .set_status_code(200)
        .set_header("X-Inject", "ok\r\nSet-Cookie: evil=1")
        .set_body("hi");
    let wire: ResponseData = resp.build();
    let wire_text: String = String::from_utf8_lossy(&wire).into_owned();
    assert!(!wire_text.contains("Set-Cookie: evil=1"));
    resp.reset();
    assert_eq!(*resp.get_version(), HttpVersion::Http1_1);
    let mut te_resp: Response = Response::default();
    te_resp
        .set_status_code(200)
        .set_header(TRANSFER_ENCODING, CHUNKED)
        .set_header(CONTENT_LENGTH, "5")
        .set_body("hello");
    let te_wire: String = String::from_utf8_lossy(&te_resp.build()).into_owned();
    assert!(!te_wire.to_ascii_lowercase().contains("content-length:"));
    let mut reset_resp: Response = Response::default();
    reset_resp.set_status_code(205).set_body("must-not-send");
    let reset_wire: String = String::from_utf8_lossy(&reset_resp.build()).into_owned();
    assert!(!reset_wire.contains("must-not-send"));
    let mut cookie_resp: Response = Response::default();
    cookie_resp
        .add_header(SET_COOKIE, "a=1")
        .add_header(SET_COOKIE, "b=2");
    let cookie_wire: String = String::from_utf8_lossy(&cookie_resp.build()).into_owned();
    assert!(cookie_wire.contains("a=1"));
    assert!(cookie_wire.contains("b=2"));
}

#[test]
fn test_security_websocket_frame_and_cookie_multipart_file_extension() {
    let bad_rsv: [u8; 2] = [0xC1, 0x00];
    assert!(WebSocketFrame::decode_ws_frame_checked(&bad_rsv, 1024).is_err());
    let reserved_op: [u8; 2] = [0x83, 0x00];
    assert!(WebSocketFrame::decode_ws_frame_checked(&reserved_op, 1024).is_err());
    let frag_ping: [u8; 2] = [0x09, 0x00];
    assert!(WebSocketFrame::decode_ws_frame_checked(&frag_ping, 1024).is_err());
    let long_ping: Vec<u8> = [vec![0x89u8, 126u8, 0u8, 126u8], vec![b'a'; 126]].concat();
    assert!(WebSocketFrame::decode_ws_frame_checked(&long_ping, 1024).is_err());
    let msb_set_64: [u8; 10] = [0x82, 127, 0x80, 0, 0, 0, 0, 0, 0, 1];
    assert!(WebSocketFrame::decode_ws_frame_checked(&msb_set_64, 1024).is_err());
    let non_minimal_16: [u8; 4] = [0x82, 126, 0x00, 10];
    assert!(WebSocketFrame::decode_ws_frame_checked(&non_minimal_16, 1024).is_err());
    let cookies: Cookies = Cookie::parse("sid=first; sid=second; theme=dark");
    assert_eq!(cookies.get("sid").map(String::as_str), Some("first"));
    assert_eq!(FileExtension::get_extension_name("/dir.v1/readme"), "");
    assert_eq!(
        FileExtension::get_extension_name("/dir.v1/index.html"),
        "html"
    );
    let ct: &str = "multipart/form-data; param=\"a;b\"; boundary=\"bnd123\"";
    assert_eq!(
        MultipartFormData::get_boundary(ct).as_deref(),
        Some("bnd123")
    );
    let mp_body: &[u8] =
        b"--bnd123\ncontent-disposition: form-data; name=\"k\"\n\nval\n--bnd123--\nepilogue";
    let parts: Vec<BodyPart> = MultipartFormData::parse(mp_body, "bnd123").unwrap();
    assert_eq!(
        MultipartFormData::try_get_part_text(&parts, "k").as_deref(),
        Some("val")
    );
}
