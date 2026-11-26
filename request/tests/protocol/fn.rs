use super::*;

#[derive(Debug, Deserialize, Eq, PartialEq)]
struct EchoPayload {
    ok: bool,
    count: usize,
}

fn build_test_tls_acceptor(alpn: Vec<Vec<u8>>) -> TlsAcceptor {
    let certs: Vec<CertificateDer<'static>> = CertificateDer::pem_slice_iter(TEST_LEAF_PEM)
        .map(|item: Result<CertificateDer<'static>, _>| item.unwrap())
        .collect();
    let key: PrivateKeyDer<'static> = PrivateKeyDer::from_pem_slice(TEST_LEAF_KEY).unwrap();
    let mut cfg: ServerConfig = ServerConfig::builder()
        .with_no_client_auth()
        .with_single_cert(certs, key)
        .unwrap();
    cfg.alpn_protocols = alpn;
    TlsAcceptor::from(Arc::new(cfg))
}

#[tokio::test]
async fn test_h1_keep_alive_pool_reuses_single_connection_async_and_sync() {
    let listener: TcpListener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr: SocketAddr = listener.local_addr().unwrap();
    let accepts: Arc<AtomicUsize> = Arc::new(AtomicUsize::new(0));
    let accepts_srv: Arc<AtomicUsize> = Arc::clone(&accepts);
    let server: JoinHandle<()> = spawn(async move {
        loop {
            let Ok((mut stream, _)) = listener.accept().await else {
                break;
            };
            accepts_srv.fetch_add(1, Ordering::SeqCst);
            spawn(async move {
                let mut buf: [u8; 1024] = [0u8; 1024];
                let mut acc: Vec<u8> = Vec::new();
                while let Ok(n) = stream.read(&mut buf).await {
                    if n == 0 {
                        break;
                    }
                    acc.extend_from_slice(&buf[..n]);
                    while let Some(end) = find_double_crlf(&acc, 0) {
                        acc.drain(..end + 4);
                        let resp: &[u8] = b"HTTP/1.1 200 OK\r\nContent-Length: 2\r\nConnection: keep-alive\r\n\r\nok";
                        if stream.write_all(resp).await.is_err() {
                            return;
                        }
                    }
                }
            });
        }
    });
    let url: String = format!("http://{addr}/ping");
    for _ in 0..8usize {
        let mut req: HttpRequest = RequestBuilder::new()
            .get(&url)
            .keep_alive(true)
            .timeout(2000)
            .build();
        let resp: HttpResponse = req.send_async().await.unwrap();
        assert_eq!(resp.get_status_code(), 200);
        assert_eq!(resp.bytes(), b"ok");
    }
    assert_eq!(accepts.load(Ordering::SeqCst), 1usize);
    let url_sync: String = url.clone();
    http_type::tokio::task::spawn_blocking(move || {
        for _ in 0..5usize {
            let mut req: HttpRequest = RequestBuilder::new()
                .get(&url_sync)
                .keep_alive(true)
                .timeout(2000)
                .build();
            let resp: HttpResponse = req.send().unwrap();
            assert_eq!(resp.get_status_code(), 200);
            assert_eq!(resp.bytes(), b"ok");
        }
    })
    .await
    .unwrap();
    assert_eq!(accepts.load(Ordering::SeqCst), 2usize);
    server.abort();
}

#[tokio::test]
async fn test_h1_keep_alive_transparently_redials_stale_pooled_connection() {
    let listener: TcpListener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr: SocketAddr = listener.local_addr().unwrap();
    let accepts: Arc<AtomicUsize> = Arc::new(AtomicUsize::new(0));
    let accepts_srv: Arc<AtomicUsize> = Arc::clone(&accepts);
    let server: JoinHandle<()> = spawn(async move {
        for index in 0..2usize {
            let (mut stream, _): (http_type::tokio::net::TcpStream, SocketAddr) =
                listener.accept().await.unwrap();
            accepts_srv.fetch_add(1, Ordering::SeqCst);
            let mut buf: [u8; 512] = [0u8; 512];
            let _: usize = stream.read(&mut buf).await.unwrap();
            let body: String = format!("hit-{index}");
            let resp: String = format!(
                "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nConnection: keep-alive\r\n\r\n{body}",
                body.len()
            );
            stream.write_all(resp.as_bytes()).await.unwrap();
            stream.shutdown().await.unwrap();
        }
    });
    let url: String = format!("http://{addr}/stale");
    let mut first: HttpRequest = RequestBuilder::new()
        .get(&url)
        .keep_alive(true)
        .timeout(2000)
        .build();
    let r1: HttpResponse = first.send_async().await.unwrap();
    assert_eq!(r1.bytes(), b"hit-0");
    sleep(Duration::from_millis(20)).await;
    let mut second: HttpRequest = RequestBuilder::new()
        .get(&url)
        .keep_alive(true)
        .timeout(2000)
        .build();
    let r2: HttpResponse = second.send_async().await.unwrap();
    assert_eq!(r2.bytes(), b"hit-1");
    assert_eq!(accepts.load(Ordering::SeqCst), 2usize);
    server.await.unwrap();
}

#[tokio::test]
async fn test_h2c_multiplexed_client_and_connection_reuse() {
    let listener: TcpListener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr: SocketAddr = listener.local_addr().unwrap();
    let accepts: Arc<AtomicUsize> = Arc::new(AtomicUsize::new(0));
    let accepts_srv: Arc<AtomicUsize> = Arc::clone(&accepts);
    let server: JoinHandle<()> = spawn(async move {
        loop {
            let Ok((tcp, _)) = listener.accept().await else {
                break;
            };
            accepts_srv.fetch_add(1, Ordering::SeqCst);
            spawn(async move {
                let Ok(mut conn) = h2::server::handshake(tcp).await else {
                    return;
                };
                while let Some(Ok((req, mut respond))) = conn.accept().await {
                    let uri_text: String = req.uri().to_string();
                    let resp: Response<()> = Response::builder().status(200).body(()).unwrap();
                    if let Ok(mut send) = respond.send_response(resp, false) {
                        let _: Result<(), h2::Error> = send.send_data(Bytes::from(uri_text), true);
                    }
                }
            });
        }
    });
    let mut warm: HttpRequest = RequestBuilder::new()
        .get(format!("http://{addr}/item?id=0"))
        .http2_only()
        .keep_alive(true)
        .timeout(3000)
        .build();
    let warm_resp: HttpResponse = warm.send_async().await.unwrap();
    assert_eq!(warm_resp.get_status_code(), 200);
    assert_eq!(*warm_resp.get_version(), HttpVersion::Http2);
    assert!(warm_resp.text().contains("id=0"));
    let mut handles: Vec<JoinHandle<()>> = Vec::new();
    for id in 1..=12usize {
        let url: String = format!("http://{addr}/item?id={id}");
        handles.push(spawn(async move {
            let mut req: HttpRequest = RequestBuilder::new()
                .get(url)
                .http2_only()
                .keep_alive(true)
                .timeout(3000)
                .build();
            let resp: HttpResponse = req.send_async().await.unwrap();
            assert_eq!(resp.get_status_code(), 200);
            assert!(resp.text().contains(&format!("id={id}")));
        }));
    }
    for handle in handles {
        handle.await.unwrap();
    }
    assert_eq!(accepts.load(Ordering::SeqCst), 1usize);
    server.abort();
}

#[tokio::test]
async fn test_https_h1_and_h2_with_custom_root_cert_pem() {
    let mut bad_builder: RequestBuilder = RequestBuilder::new();
    assert!(bad_builder.root_cert_pem(b"not-a-valid-pem").is_err());
    let listener: TcpListener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr: SocketAddr = listener.local_addr().unwrap();
    let acceptor: TlsAcceptor = build_test_tls_acceptor(vec![b"h2".to_vec(), b"http/1.1".to_vec()]);
    let server: JoinHandle<()> = spawn(async move {
        loop {
            let Ok((tcp, _)) = listener.accept().await else {
                break;
            };
            let acc: TlsAcceptor = acceptor.clone();
            spawn(async move {
                let Ok(mut tls) = acc.accept(tcp).await else {
                    return;
                };
                let is_h2: bool = tls.get_ref().1.alpn_protocol() == Some(b"h2".as_slice());
                if is_h2 {
                    let Ok(mut conn) = h2::server::handshake(tls).await else {
                        return;
                    };
                    while let Some(Ok((_, mut respond))) = conn.accept().await {
                        let resp: Response<()> = Response::builder()
                            .status(200)
                            .header("x-proto", "h2-tls")
                            .body(())
                            .unwrap();
                        if let Ok(mut send) = respond.send_response(resp, false) {
                            let _: Result<(), h2::Error> = send
                                .send_data(Bytes::from_static(b"{\"ok\":true,\"count\":2}"), true);
                        }
                    }
                } else {
                    let mut buf: [u8; 1024] = [0u8; 1024];
                    let _: usize = tls.read(&mut buf).await.unwrap_or(0);
                    let wire: &[u8] = b"HTTP/1.1 200 OK\r\nX-Proto: h1-tls\r\nContent-Length: 21\r\n\r\n{\"ok\":true,\"count\":1}";
                    let _: Result<(), io::Error> = tls.write_all(wire).await;
                }
            });
        }
    });
    let url: String = format!("https://127.0.0.1:{}/tls", addr.port());
    let mut untrusted: HttpRequest = RequestBuilder::new().get(&url).timeout(2000).build();
    assert!(untrusted.send_async().await.is_err());
    let mut h1_builder: RequestBuilder = RequestBuilder::new();
    let mut h1_req: HttpRequest = h1_builder
        .get(&url)
        .root_cert_pem(TEST_CA_PEM)
        .unwrap()
        .timeout(3000)
        .build();
    let h1_resp: HttpResponse = h1_req.send_async().await.unwrap();
    assert_eq!(h1_resp.get_status_code(), 200);
    assert_eq!(h1_resp.get_header("x-proto"), Some("h1-tls"));
    let h1_json: EchoPayload = h1_resp.json().unwrap();
    assert_eq!(h1_json, EchoPayload { ok: true, count: 1 });
    let mut h2_builder: RequestBuilder = RequestBuilder::new();
    let mut h2_req: HttpRequest = h2_builder
        .get(&url)
        .http2_only()
        .keep_alive(true)
        .root_cert_pem(TEST_CA_PEM)
        .unwrap()
        .timeout(3000)
        .build();
    let h2_resp: HttpResponse = h2_req.send_async().await.unwrap();
    assert_eq!(h2_resp.get_status_code(), 200);
    assert_eq!(*h2_resp.get_version(), HttpVersion::Http2);
    assert_eq!(h2_resp.get_header("x-proto"), Some("h2-tls"));
    let h2_json: EchoPayload = h2_resp.json().unwrap();
    assert_eq!(h2_json, EchoPayload { ok: true, count: 2 });
    server.abort();
}

#[tokio::test]
async fn test_retry_idempotent_get_recovers_from_503_and_post_does_not_retry() {
    let listener: TcpListener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr: SocketAddr = listener.local_addr().unwrap();
    let hits: Arc<AtomicUsize> = Arc::new(AtomicUsize::new(0));
    let hits_srv: Arc<AtomicUsize> = Arc::clone(&hits);
    let server: JoinHandle<()> = spawn(async move {
        loop {
            let Ok((mut stream, _)) = listener.accept().await else {
                break;
            };
            let current: usize = hits_srv.fetch_add(1, Ordering::SeqCst) + 1;
            let mut buf: [u8; 512] = [0u8; 512];
            let _: usize = stream.read(&mut buf).await.unwrap_or(0);
            let resp: &[u8] = if current < 3 || current == 4 {
                b"HTTP/1.1 503 Service Unavailable\r\nContent-Length: 4\r\n\r\nbusy"
            } else {
                b"HTTP/1.1 200 OK\r\nContent-Length: 2\r\n\r\nok"
            };
            let _: Result<(), io::Error> = stream.write_all(resp).await;
            let _: Result<(), io::Error> = stream.shutdown().await;
        }
    });
    let url: String = format!("http://{addr}/retry");
    let mut get_req: HttpRequest = RequestBuilder::new()
        .get(&url)
        .retry(3)
        .timeout(3000)
        .build();
    let get_resp: HttpResponse = get_req.send_async().await.unwrap();
    assert_eq!(get_resp.get_status_code(), 200);
    assert_eq!(hits.load(Ordering::SeqCst), 3usize);
    let mut post_req: HttpRequest = RequestBuilder::new()
        .post(&url)
        .body_text("order")
        .retry(3)
        .timeout(3000)
        .build();
    let post_resp: HttpResponse = post_req.send_async().await.unwrap();
    assert_eq!(post_resp.get_status_code(), 503);
    assert!(post_resp.is_server_error());
    assert_eq!(hits.load(Ordering::SeqCst), 4usize);
    server.abort();
}

#[test]
fn test_builder_extended_methods_query_auth_cookie_form_and_multipart() {
    let mut b1: RequestBuilder = RequestBuilder::new();
    let put_req: HttpRequest = b1
        .put("http://api.test/items#frag")
        .query("a", "hello world")
        .query_pairs([("k", "1+2"), ("empty", "")])
        .bearer_auth("tok-123")
        .cookie("sid", "abc")
        .cookie("theme", "dark")
        .connect_timeout(500)
        .retry(2)
        .keep_alive(true)
        .build();
    assert_eq!(put_req.get_method(), Method::Put);
    assert_eq!(
        put_req.get_url(),
        "http://api.test/items?a=hello%20world&k=1%2B2&empty=#frag"
    );
    assert_eq!(
        put_req
            .get_headers()
            .get("authorization")
            .map(String::as_str),
        Some("Bearer tok-123")
    );
    assert_eq!(
        put_req.get_headers().get("cookie").map(String::as_str),
        Some("sid=abc; theme=dark")
    );
    assert_eq!(put_req.get_config().get_connect_timeout(), 500u64);
    assert_eq!(put_req.get_config().get_max_retries(), 2usize);
    assert!(put_req.get_config().get_keep_alive());
    let mut b2: RequestBuilder = RequestBuilder::new();
    let del_req: HttpRequest = b2
        .delete("http://api.test/1")
        .basic_auth("alice", "s3cr3t")
        .build();
    assert_eq!(del_req.get_method(), Method::Delete);
    assert_eq!(
        del_req
            .get_headers()
            .get("authorization")
            .map(String::as_str),
        Some("Basic YWxpY2U6czNjcjN0")
    );
    let mut b3: RequestBuilder = RequestBuilder::new();
    let patch_req: HttpRequest = b3
        .patch("http://api.test/form")
        .body_form([("name", "a&b"), ("city", "sh")])
        .build();
    assert_eq!(patch_req.get_method(), Method::Patch);
    assert_eq!(
        patch_req
            .get_headers()
            .get("content-type")
            .map(String::as_str),
        Some("application/x-www-form-urlencoded")
    );
    assert_eq!(patch_req.get_body().as_str(), Some("name=a%26b&city=sh"));
    let mut b4: RequestBuilder = RequestBuilder::new();
    let parts: Vec<BodyPart> = vec![
        BodyPart::new(
            "field".to_owned(),
            String::new(),
            String::new(),
            hash_map_xx_hash3_64(),
            b"value-1".to_vec(),
        ),
        BodyPart::new(
            "upload".to_owned(),
            "report.txt".to_owned(),
            "text/plain".to_owned(),
            hash_map_xx_hash3_64(),
            b"file-bytes".to_vec(),
        ),
    ];
    let mp_req: HttpRequest = b4
        .post("http://api.test/upload")
        .body_multipart(&parts)
        .build();
    let ct: &str = mp_req.get_headers().get("content-type").unwrap();
    let boundary: String = MultipartFormData::get_boundary(ct).unwrap();
    let parsed: Vec<BodyPart> =
        MultipartFormData::parse(mp_req.get_body().as_slice(), &boundary).unwrap();
    assert_eq!(parsed.len(), 2usize);
    assert_eq!(
        MultipartFormData::try_get_part_text(&parsed, "field").as_deref(),
        Some("value-1")
    );
    let file_part: &BodyPart = MultipartFormData::try_get_part(&parsed, "upload").unwrap();
    assert_eq!(
        file_part.try_get_file_name().map(String::as_str),
        Some("report.txt")
    );
    assert_eq!(file_part.try_get_data(), Some(b"file-bytes".as_slice()));
    let mut b5: RequestBuilder = RequestBuilder::new();
    assert_eq!(
        b5.head("http://api.test/h").build().get_method(),
        Method::Head
    );
    let mut b6: RequestBuilder = RequestBuilder::new();
    assert_eq!(
        b6.options("http://api.test/o").build().get_method(),
        Method::Options
    );
    let err_resp: HttpResponse = HttpResponse::from_bytes(
        b"HTTP/1.1 404 Not Found\r\nSet-Cookie: sid=xyz; Path=/\r\n\r\nmissing",
    );
    assert!(err_resp.is_client_error());
    assert!(!err_resp.is_server_error());
    let cookies: Cookies = err_resp.try_get_cookies().unwrap();
    assert_eq!(cookies.get("sid").map(String::as_str), Some("xyz"));
}

#[tokio::test]
async fn test_security_redirect_strips_credentials_rewrites_post_and_blocks_loops() {
    let target_listener: TcpListener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let target_addr: SocketAddr = target_listener.local_addr().unwrap();
    let origin_listener: TcpListener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let origin_addr: SocketAddr = origin_listener.local_addr().unwrap();
    let origin_srv: JoinHandle<()> = spawn(async move {
        let (mut stream, _): (http_type::tokio::net::TcpStream, SocketAddr) =
            origin_listener.accept().await.unwrap();
        let mut buf: [u8; 1024] = [0u8; 1024];
        let _: usize = stream.read(&mut buf).await.unwrap();
        let resp: String = format!(
            "HTTP/1.1 302 Found\r\nLocation: http://{target_addr}/dest\r\nContent-Length: 0\r\n\r\n"
        );
        stream.write_all(resp.as_bytes()).await.unwrap();
    });
    let target_srv: JoinHandle<String> = spawn(async move {
        let (mut stream, _): (http_type::tokio::net::TcpStream, SocketAddr) =
            target_listener.accept().await.unwrap();
        let mut buf: [u8; 1024] = [0u8; 1024];
        let n: usize = stream.read(&mut buf).await.unwrap();
        let req_text: String = String::from_utf8_lossy(&buf[..n]).into_owned();
        stream
            .write_all(b"HTTP/1.1 200 OK\r\nContent-Length: 4\r\n\r\ndone")
            .await
            .unwrap();
        req_text
    });
    let mut req: HttpRequest = RequestBuilder::new()
        .post(format!("http://{origin_addr}/start"))
        .bearer_auth("secret-token")
        .cookie("session", "top-secret")
        .body_text("sensitive-body")
        .redirect()
        .max_redirect_times(3)
        .timeout(3000)
        .build();
    let resp: HttpResponse = req.send_async().await.unwrap();
    assert_eq!(resp.get_status_code(), 200);
    assert_eq!(resp.bytes(), b"done");
    let captured: String = target_srv.await.unwrap();
    origin_srv.await.unwrap();
    assert!(captured.starts_with("GET /dest HTTP/1.1\r\n"));
    assert!(!captured.to_ascii_lowercase().contains("authorization:"));
    assert!(!captured.to_ascii_lowercase().contains("cookie:"));
    assert!(!captured.contains("sensitive-body"));
    let loop_listener: TcpListener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let loop_addr: SocketAddr = loop_listener.local_addr().unwrap();
    let loop_url: String = format!("http://{loop_addr}/self");
    let loop_url_srv: String = loop_url.clone();
    let loop_srv: JoinHandle<()> = spawn(async move {
        let (mut stream, _): (http_type::tokio::net::TcpStream, SocketAddr) =
            loop_listener.accept().await.unwrap();
        let mut buf: [u8; 512] = [0u8; 512];
        let _: usize = stream.read(&mut buf).await.unwrap();
        let resp: String = format!(
            "HTTP/1.1 301 Moved Permanently\r\nLocation: {loop_url_srv}\r\nContent-Length: 0\r\n\r\n"
        );
        stream.write_all(resp.as_bytes()).await.unwrap();
    });
    let mut loop_req: HttpRequest = RequestBuilder::new()
        .get(&loop_url)
        .redirect()
        .max_redirect_times(5)
        .timeout(2000)
        .build();
    assert!(loop_req.send_async().await.is_err());
    loop_srv.await.unwrap();
}

#[tokio::test]
async fn test_security_async_timeout_interim_cap_and_proxy_hardening() {
    let slow_listener: TcpListener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let slow_addr: SocketAddr = slow_listener.local_addr().unwrap();
    let slow_srv: JoinHandle<()> = spawn(async move {
        let (_stream, _): (http_type::tokio::net::TcpStream, SocketAddr) =
            slow_listener.accept().await.unwrap();
        sleep(Duration::from_secs(2)).await;
    });
    let mut timeout_req: HttpRequest = RequestBuilder::new()
        .get(format!("http://{slow_addr}/slow"))
        .timeout(100)
        .build();
    assert!(matches!(
        timeout_req.send_async().await,
        Err(RequestError::ReadTimeout(_))
    ));
    slow_srv.abort();
    let interim_listener: TcpListener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let interim_addr: SocketAddr = interim_listener.local_addr().unwrap();
    let interim_srv: JoinHandle<()> = spawn(async move {
        let (mut stream, _): (http_type::tokio::net::TcpStream, SocketAddr) =
            interim_listener.accept().await.unwrap();
        let mut buf: [u8; 512] = [0u8; 512];
        let _: usize = stream.read(&mut buf).await.unwrap();
        let mut wire: Vec<u8> = Vec::new();
        for _ in 0..20usize {
            wire.extend_from_slice(b"HTTP/1.1 100 Continue\r\n\r\n");
        }
        wire.extend_from_slice(b"HTTP/1.1 200 OK\r\nContent-Length: 2\r\n\r\nok");
        let _: Result<(), io::Error> = stream.write_all(&wire).await;
    });
    let mut interim_req: HttpRequest = RequestBuilder::new()
        .get(format!("http://{interim_addr}/many-1xx"))
        .timeout(2000)
        .build();
    assert!(interim_req.send_async().await.is_err());
    interim_srv.await.unwrap();
    let mut bad_connect: HttpRequest = RequestBuilder::new()
        .get("http://example.com/")
        .proxy(Proxy::http("127.0.0.1", 9).auth("bad:user", "pass"))
        .build();
    assert!(bad_connect.send_async().await.is_err());
    let mut bad_socks: HttpRequest = RequestBuilder::new()
        .get("http://example.com/")
        .proxy(Proxy::socks5("127.0.0.1", 9).auth("u".repeat(300), "pass"))
        .build();
    assert!(bad_socks.send_async().await.is_err());
    let mut bad_method: HttpRequest = RequestBuilder::new()
        .method(Method::Unknown("GET\r\nInjected: 1".to_owned()))
        .url("http://127.0.0.1:9/")
        .build();
    assert!(bad_method.send_async().await.is_err());
    let mut bad_ver: HttpRequest = HttpRequest::get("http://127.0.0.1:9/");
    bad_ver
        .get_mut_config()
        .set_http_version(HttpVersion::Unknown("HTTP/1.1\r\nX: y".to_owned()));
    assert!(bad_ver.send_async().await.is_err());
}
