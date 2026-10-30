use super::*;

#[test]
fn test_http_request_get_sets_method_and_url() {
    let request: HttpRequest = HttpRequest::get("http://example.com/a");
    assert_eq!(request.get_method(), Method::Get);
    assert!(request.get_method().is_get());
    assert_eq!(request.get_url(), "http://example.com/a");
    assert_eq!(request.get_url(), "http://example.com/a");
    assert!(request.get_headers().is_empty());
    assert!(request.get_body().get_bytes().as_slice().is_empty());
}

#[test]
fn test_http_request_post_sets_method_and_url() {
    let request: HttpRequest = HttpRequest::post("https://example.com/b");
    assert_eq!(request.get_method(), Method::Post);
    assert!(request.get_method().is_post());
    assert_eq!(request.get_url(), "https://example.com/b");
}

#[test]
fn test_http_request_accepts_owned_url() {
    let request: HttpRequest = HttpRequest::get(String::from("http://owned.test/"));
    assert_eq!(request.get_url(), "http://owned.test/");
}

#[test]
fn test_http_request_default_is_empty_request() {
    let request: HttpRequest = HttpRequest::default();
    assert_eq!(request.get_url(), "");
    assert_eq!(request.url, "");
    assert!(request.get_method().is_unknown());
    assert!(request.get_headers().is_empty());
    assert!(request.get_body().get_bytes().as_slice().is_empty());
    assert_eq!(
        *request.get_config().get_http_version(),
        HttpVersion::Http1_1
    );
}

#[test]
fn test_http_request_get_sets_url_matches_field() {
    let request: HttpRequest = HttpRequest::get("http://example.com/");
    assert_eq!(request.url, "http://example.com/");
    assert_eq!(request.method, Method::Get);
}

#[test]
fn test_http_request_set_method_overrides_default() {
    let mut request: HttpRequest = HttpRequest::get("http://example.com/");
    request.set_method(Method::Put);
    assert_eq!(request.get_method(), Method::Put);
    assert!(!request.get_method().is_get());
    assert_eq!(request.method, Method::Put);
}

#[test]
fn test_http_request_set_url_overrides_previous() {
    let mut request: HttpRequest = HttpRequest::get("http://first.test/");
    request.set_url("http://second.test/");
    assert_eq!(request.get_url(), "http://second.test/");
}

#[test]
fn test_http_request_set_header_normalizes_key_to_lowercase() {
    let mut request: HttpRequest = HttpRequest::get("http://example.com/");
    request.set_header("Content-Type", "application/json");
    let headers: &HashMap<String, String> = request.get_headers();
    assert_eq!(headers.len(), 1usize);
    assert_eq!(
        headers.get("content-type").map(String::as_str),
        Some("application/json")
    );
    assert!(headers.get("Content-Type").is_none());
}

#[test]
fn test_http_request_set_header_last_write_wins_across_cases() {
    let mut request: HttpRequest = HttpRequest::get("http://example.com/");
    request.set_header("X-Token", "first");
    request.set_header("x-token", "second");
    let headers: &HashMap<String, String> = request.get_headers();
    assert_eq!(headers.len(), 1usize);
    assert_eq!(headers.get("x-token").map(String::as_str), Some("second"));
    assert!(headers.get("X-Token").is_none());
}

#[test]
fn test_http_request_remove_header_is_case_insensitive() {
    let mut request: HttpRequest = HttpRequest::get("http://example.com/");
    request.set_header("X-Token", "value");
    request.remove_header("x-token");
    assert!(request.get_headers().is_empty());
    request.set_header("X-Token", "value");
    request.remove_header("X-TOKEN");
    assert!(request.get_headers().is_empty());
}

#[test]
fn test_http_request_remove_missing_header_is_noop() {
    let mut request: HttpRequest = HttpRequest::get("http://example.com/");
    request.set_header("X-Token", "value");
    request.remove_header("absent");
    assert_eq!(request.get_headers().len(), 1usize);
}

#[test]
fn test_http_request_clear_headers_drops_all() {
    let mut request: HttpRequest = HttpRequest::get("http://example.com/");
    request.set_header("A", "1");
    request.set_header("B", "2");
    assert_eq!(request.get_headers().len(), 2usize);
    request.clear_headers();
    assert!(request.get_headers().is_empty());
}

#[test]
fn test_http_request_get_mut_headers_writes_directly() {
    let mut request: HttpRequest = HttpRequest::get("http://example.com/");
    let headers: &mut HashMap<String, String> = request.get_mut_headers();
    headers.insert("X-raw".to_string(), "v".to_string());
    assert_eq!(
        request.get_headers().get("X-raw").map(String::as_str),
        Some("v")
    );
}

#[test]
fn test_http_request_set_body_replaces_previous() {
    let mut request: HttpRequest = HttpRequest::post("http://example.com/");
    request.set_body(Body::from_bytes("data"));
    assert_eq!(request.get_body().get_bytes().as_slice(), b"data");
    assert_eq!(request.get_body().as_str(), Some("data"));
    request.set_body(Body::empty());
    assert!(request.get_body().get_bytes().as_slice().is_empty());
    assert_eq!(request.body, Body::empty());
}

#[test]
fn test_http_request_config_defaults() {
    let request: HttpRequest = HttpRequest::get("http://example.com/");
    assert_eq!(request.get_config().get_buffer_size(), 0usize);
    assert_eq!(request.get_config().get_timeout(), 0u64);
    assert_eq!(request.get_config().get_max_redirect_times(), 0usize);
    assert_eq!(
        *request.get_config().get_http_version(),
        HttpVersion::Http1_1
    );
    assert!(!request.get_config().get_redirect());
    assert!(!request.get_config().get_decode());
    assert!(request.get_config().get_proxy().is_none());
}

#[test]
fn test_http_request_config_setters_are_chainable() {
    let mut request: HttpRequest = HttpRequest::get("http://example.com/");
    request
        .get_mut_config()
        .set_buffer_size(4096usize)
        .set_timeout(1500u64)
        .set_max_redirect_times(3usize)
        .set_http_version(HttpVersion::Http2)
        .set_redirect(true)
        .set_decode(true);
    assert_eq!(request.get_config().get_buffer_size(), 4096usize);
    assert_eq!(request.get_config().get_timeout(), 1500u64);
    assert_eq!(request.get_config().get_max_redirect_times(), 3usize);
    assert_eq!(*request.get_config().get_http_version(), HttpVersion::Http2);
    assert!(request.get_config().get_redirect());
    assert!(request.get_config().get_decode());
}

#[test]
fn test_http_request_config_proxy_setter() {
    let mut request: HttpRequest = HttpRequest::get("http://example.com/");
    request
        .get_mut_config()
        .set_proxy(Some(Proxy::http("h", 1u16)));
    let proxy: Option<Proxy> = request.get_config().get_proxy();
    assert_eq!(
        proxy.as_ref().map(|item: &Proxy| item.host.clone()),
        Some("h".to_string())
    );
    request.get_mut_config().set_proxy(None);
    assert!(request.get_config().get_proxy().is_none());
}

#[test]
fn test_http_request_set_config_replaces_whole_config() {
    let mut source: HttpRequest = HttpRequest::get("http://source.test/");
    source.get_mut_config().set_timeout(1234u64);
    source.get_mut_config().set_buffer_size(64usize);
    let mut target: HttpRequest = HttpRequest::post("http://target.test/");
    target.get_mut_config().set_timeout(9999u64);
    target.set_config(source.get_config().clone());
    assert_eq!(target.get_config(), source.get_config());
    assert_eq!(target.get_config().get_timeout(), 1234u64);
    assert_eq!(target.get_config().get_buffer_size(), 64usize);
    assert_eq!(target.config, source.config);
}

#[test]
fn test_http_request_get_config_returns_owned_copy() {
    let request: HttpRequest = HttpRequest::get("http://example.com/");
    assert_eq!(request.get_config(), request.get_config());
    assert_eq!(request.config, request.get_config().clone());
}

#[test]
fn test_http_request_clone_copies_headers_and_body() {
    let mut request: HttpRequest = HttpRequest::post("http://example.com/");
    request.set_header("X-Token", "v");
    request.set_body(Body::from_bytes("payload"));
    let cloned: HttpRequest = request.clone();
    assert_eq!(cloned.get_url(), request.get_url());
    assert_eq!(cloned.get_method(), request.get_method());
    assert_eq!(cloned.get_headers(), request.get_headers());
    assert_eq!(cloned.get_body(), request.get_body());
}

#[test]
fn test_http_request_debug_lists_url() {
    let request: HttpRequest = HttpRequest::get("http://debug.test/");
    let debugged: String = format!("{request:?}");
    assert!(debugged.contains("http://debug.test/"));
}

#[test]
fn client_rejects_control_bytes_in_header_values_before_sending() {
    for byte in (0u8..=31).chain([127]) {
        if byte == b'\t' {
            continue;
        }
        let value: String = format!("before{}after", char::from(byte));
        let mut request: HttpRequest = HttpRequest::get("http://127.0.0.1:9/");
        request.set_header("X-Foo", value);
        assert!(request.header_bytes(0).is_err(), "{byte}");
        assert!(
            matches!(request.send(), Err(RequestError::Request(message)) if message == "Invalid HTTP request header"),
            "{byte}"
        );
    }
    let mut request: HttpRequest = HttpRequest::get("http://127.0.0.1:9/");
    request.set_header("X-Foo", "bar\r\nX-Injected: evil");
    assert!(request.header_bytes(0).is_err());
    assert!(request.send().is_err());
}

#[test]
fn client_rejects_invalid_header_names_before_sending() {
    for name in [
        "",
        "X:Injected",
        "X\r\nInjected",
        "X Foo",
        "X\tFoo",
        "X\0Foo",
        "X\x7fFoo",
        "X(Foo)",
        "X/Foo",
        "X@Foo",
        "X\\Foo",
        "X\"Foo",
        "X[Foo]",
        "X{Foo}",
        "X=Foo",
        "X?Foo",
        "X,Foo",
        "X;Foo",
        "X<Foo>",
        "X-é",
    ] {
        let mut request: HttpRequest = HttpRequest::get("http://127.0.0.1:9/");
        request.set_header(name, "value");
        assert!(request.header_bytes(0).is_err(), "{name:?}");
        assert!(
            matches!(request.send(), Err(RequestError::Request(message)) if message == "Invalid HTTP request header"),
            "{name:?}"
        );
    }
    for byte in 0u8..=31 {
        let name: String = format!("X{}Foo", char::from(byte));
        let mut request: HttpRequest = HttpRequest::get("http://127.0.0.1:9/");
        request.set_header(name, "value");
        assert!(request.header_bytes(0).is_err(), "{byte}");
    }
}

#[test]
fn client_accepts_header_tokens_tabs_and_case_insensitive_defaults() {
    let mut request: HttpRequest = HttpRequest::get("http://example.test/");
    request.set_header("!#$%&'*+-.^_`|~AZaz09", "a\tb café");
    assert!(request.header_bytes(0).is_ok());
    request.get_mut_headers().extend(HashMap::from([
        ("hOsT".to_owned(), "override.test".to_owned()),
        ("cOnTeNt-LeNgTh".to_owned(), "0".to_owned()),
        ("aCcEpT".to_owned(), "text/plain".to_owned()),
        ("uSeR-aGeNt".to_owned(), "client".to_owned()),
    ]));
    let headers: Vec<u8> = request.header_bytes(0).unwrap();
    assert_eq!(
        response_header_value(&headers, b"host").unwrap(),
        Some(b"override.test".as_slice())
    );
    assert_eq!(
        response_header_value(&headers, b"content-length").unwrap(),
        Some(b"0".as_slice())
    );
    assert_eq!(
        response_header_value(&headers, b"accept").unwrap(),
        Some(b"text/plain".as_slice())
    );
    assert_eq!(
        response_header_value(&headers, b"user-agent").unwrap(),
        Some(b"client".as_slice())
    );
}

#[test]
fn client_rejects_request_target_controls_and_spaces() {
    for byte in (0u8..=32).chain([127]) {
        let path: String = format!("http://127.0.0.1:9/a{}b", char::from(byte));
        let mut request: HttpRequest = HttpRequest::get(path);
        assert!(matches!(
            request.full_path(),
            Err(RequestError::InvalidUrlPath(HttpStatus::BadRequest))
        ));
        assert!(matches!(
            request.send(),
            Err(RequestError::InvalidUrlPath(HttpStatus::BadRequest))
        ));
        let query: String = format!("http://127.0.0.1:9/a?x=a{}b", char::from(byte));
        request.set_url(query);
        assert!(matches!(
            request.full_path(),
            Err(RequestError::InvalidUrlQuery(HttpStatus::BadRequest))
        ));
        assert!(matches!(
            request.send(),
            Err(RequestError::InvalidUrlQuery(HttpStatus::BadRequest))
        ));
    }
    let request: HttpRequest = HttpRequest::get("http://example.test/a%20b?x=%0d%0a");
    assert_eq!(request.full_path().unwrap(), "/a%20b?x=%0d%0a");
}

#[test]
fn client_rejects_non_http_or_missing_schemes_before_connecting() {
    for url in [
        "file:///etc/passwd",
        "ftp://127.0.0.1:9/",
        "gopher://127.0.0.1:9/",
        "//127.0.0.1:9/",
        "127.0.0.1:9/",
        "://127.0.0.1:9/",
        "",
    ] {
        let mut request: HttpRequest = HttpRequest::get(url);
        assert!(
            matches!(
                request.send(),
                Err(RequestError::InvalidUrlScheme(HttpStatus::BadRequest))
            ),
            "{url:?}"
        );
    }
    for url in ["HtTp://example.test/", "hTtPs://example.test/"] {
        assert!(HttpRequest::get(url).parse_url().is_ok());
    }
}

#[test]
fn client_rejects_missing_or_header_unsafe_hosts_before_connecting() {
    for url in [
        "http://",
        "http:///path",
        "http://:80/",
        "http://user:pass@/",
        "http://bad host/",
        "http://bad\r\nhost/",
        "http://bad\x00host/",
        "http://bad\x7fhost/",
        "http://host\\other/",
    ] {
        let mut request: HttpRequest = HttpRequest::get(url);
        assert!(
            matches!(
                request.send(),
                Err(RequestError::InvalidUrlHost(HttpStatus::BadRequest))
            ),
            "{url:?}"
        );
    }
    for byte in 0u8..=32 {
        let url: String = format!("http://bad{}host/", char::from(byte));
        let mut request: HttpRequest = HttpRequest::get(url);
        assert!(
            matches!(
                request.send(),
                Err(RequestError::InvalidUrlHost(HttpStatus::BadRequest))
            ),
            "{byte}"
        );
    }
}

#[test]
fn client_userinfo_never_enters_the_host_header_or_request_target() {
    let request: HttpRequest = HttpRequest::get("http://user:pass@example.test:8080/path?ok=1");
    assert_eq!(
        request.parse_url().unwrap().get_host().as_deref(),
        Some("example.test")
    );
    let headers: Vec<u8> = request.header_bytes(0).unwrap();
    assert_eq!(
        response_header_value(&headers, b"host").unwrap(),
        Some(b"example.test".as_slice())
    );
    let target: String = request.full_path().unwrap();
    assert_eq!(target, "/path?ok=1");
    let wire: Vec<u8> = build_http_request("GET", target, headers, None, "HTTP/1.1".to_owned());
    assert!(wire.starts_with(b"GET /path?ok=1 HTTP/1.1\r\n"));
    assert!(find_pattern_case_insensitive(&wire, b"user:pass@").is_none());
}

#[test]
fn client_header_injection_is_rejected_without_opening_sync_or_async_connections() {
    let runtime: http_type::tokio::runtime::Runtime =
        http_type::tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();
    runtime.block_on(async {
        let listener: http_type::tokio::net::TcpListener = http_type::tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let url: String = format!("http://{}/", listener.local_addr().unwrap());
        let mut request: HttpRequest = HttpRequest::get(url);
        request.get_mut_config().set_timeout(100u64).set_buffer_size(64usize);
        request.set_header("X-Foo", "bar\r\nX-Injected: evil");
        assert!(matches!(request.send(), Err(RequestError::Request(message)) if message == "Invalid HTTP request header"));
        let result: RequestResult = http_type::tokio::time::timeout(Duration::from_millis(100), request.send_async()).await.unwrap();
        assert!(matches!(result, Err(RequestError::Request(message)) if message == "Invalid HTTP request header"));
        assert!(http_type::tokio::time::timeout(Duration::from_millis(10), listener.accept()).await.is_err());
    });
}

fn exchange_client_response(response: &'static [u8], synchronous: bool) -> RequestResult {
    let runtime: http_type::tokio::runtime::Runtime =
        http_type::tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();
    runtime.block_on(async {
        let listener: http_type::tokio::net::TcpListener =
            http_type::tokio::net::TcpListener::bind("127.0.0.1:0")
                .await
                .unwrap();
        let url: String = format!("http://{}/", listener.local_addr().unwrap());
        let server: http_type::tokio::task::JoinHandle<()> = http_type::tokio::spawn(async move {
            let mut stream: http_type::tokio::net::TcpStream = listener.accept().await.unwrap().0;
            let mut wire: Vec<u8> = Vec::new();
            let mut buffer: [u8; 128] = [0; 128];
            loop {
                let length: usize = stream.read(&mut buffer).await.unwrap();
                assert!(length > 0);
                wire.extend_from_slice(&buffer[..length]);
                if find_double_crlf(&wire, 0).is_some() {
                    break;
                }
            }
            assert!(wire.starts_with(b"GET / HTTP/1.1\r\n"));
            stream.write_all(response).await.unwrap();
            stream.shutdown().await.unwrap();
        });
        let mut request: HttpRequest = HttpRequest::get(url);
        request
            .get_mut_config()
            .set_timeout(1000u64)
            .set_buffer_size(8usize);
        let result: RequestResult = if synchronous {
            http_type::tokio::task::spawn_blocking(move || request.send())
                .await
                .unwrap()
        } else {
            http_type::tokio::time::timeout(Duration::from_secs(1), request.send_async())
                .await
                .unwrap()
        };
        server.await.unwrap();
        result
    })
}

#[test]
fn client_propagates_response_framing_errors_through_sync_and_async_sends() {
    for synchronous in [true, false] {
        for response in [
            b"HTTP/1.1 200 OK\r\nContent-Length: 0\r\nTransfer-Encoding: chunked\r\n\r\n0\r\n\r\n"
                .as_slice(),
            b"HTTP/1.1 200 OK\r\nContent-Length: 5x\r\n\r\nhello".as_slice(),
            b"HTTP/1.1 200 OK\r\nContent-Length: 99999999999999999999999999\r\n\r\n".as_slice(),
            b"HTTP/1.1 200 OK\r\nTransfer-Encoding: gzip\r\n\r\n".as_slice(),
            b"HTTP/1.1 200 OK\r\nTransfer-Encoding: chunked\r\n\r\nffffffffffffffff\r\n".as_slice(),
            b"HTTP/1.1 200 OK\r\nTransfer-Encoding: chunked\r\n\r\n+1\r\nx\r\n0\r\n\r\n".as_slice(),
            b"HTTP/1.1 200 OK\r\nTransfer-Encoding: chunked\r\n\r\n1\r\nxXX0\r\n\r\n".as_slice(),
            b"HTTP/1.1 200 OK\r\nContent-Length: 5\r\n\r\nno".as_slice(),
            b"HTTP/1.1 200 OK\r\nX-Test: a\0b\r\nContent-Length: 0\r\n\r\n".as_slice(),
        ] {
            assert!(
                exchange_client_response(response, synchronous).is_err(),
                "{synchronous}: {response:?}"
            );
        }
    }
}

#[test]
fn client_accepts_valid_fragmented_fixed_length_and_chunked_responses() {
    for synchronous in [true, false] {
        for response in [
            b"HTTP/1.1 200 OK\r\nX-Content-Length: 999\r\nContent-Length: 5\r\n\r\nhello".as_slice(),
            b"HTTP/1.1 200 OK\r\nTransfer-Encoding: chunked\r\n\r\n1;foo=bar\r\nx\r\n0\r\nX-Trailer: yes\r\n\r\n".as_slice(),
        ] {
            assert!(exchange_client_response(response, synchronous).is_ok(), "{synchronous}: {response:?}");
        }
    }
}
