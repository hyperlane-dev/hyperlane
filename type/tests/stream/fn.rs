use super::*;

#[tokio::test]
async fn test_http_request_rejects_transfer_encoding() {
    for headers in [
        "Transfer-Encoding: chunked\r\n",
        "transfer-encoding: identity\r\n",
        "TRANSFER-ENCODING: gzip, chunked\r\n",
        "Transfer-Encoding: \r\n",
        "Content-Length: 0\r\nTransfer-Encoding: chunked\r\n",
        "Transfer-Encoding: chunked\r\nContent-Length: 0\r\n",
    ] {
        let raw: String = format!("POST / HTTP/1.1\r\nHost: localhost\r\n{headers}\r\n0\r\n\r\n");
        let (mut client, server): (DuplexStream, DuplexStream) = duplex(4096);
        client.write_all(raw.as_bytes()).await.unwrap();
        client.shutdown().await.unwrap();
        let mut stream: Stream =
            Stream::from_transport(TestTransport::new(server), RequestConfig::default(), false);
        assert!(
            matches!(
                stream.try_get_http_request().await,
                Err(RequestError::HttpRead(HttpStatus::BadRequest))
            ),
            "accepted invalid request: {raw:?}"
        );
    }
}

#[tokio::test]
async fn test_http_request_rejects_duplicate_content_length() {
    for headers in [
        "Content-Length: 5\r\nContent-Length: 5\r\n",
        "Content-Length: 0\r\ncontent-length: 5\r\n",
        "content-length: 5\r\nCONTENT-LENGTH: 0\r\n",
        "Content-Length: 0\r\nContent-Length: 0\r\n",
    ] {
        let raw: String = format!("POST / HTTP/1.1\r\nHost: localhost\r\n{headers}\r\nhello");
        let (mut client, server): (DuplexStream, DuplexStream) = duplex(4096);
        client.write_all(raw.as_bytes()).await.unwrap();
        client.shutdown().await.unwrap();
        let mut stream: Stream =
            Stream::from_transport(TestTransport::new(server), RequestConfig::default(), false);
        assert!(
            matches!(
                stream.try_get_http_request().await,
                Err(RequestError::InvalidContentLength(HttpStatus::BadRequest))
            ),
            "accepted invalid request: {raw:?}"
        );
    }
}

#[tokio::test]
async fn test_http_request_rejects_content_length_list() {
    for value in ["5, 5", "5,5", "0,0", "5, 0"] {
        let raw: String =
            format!("POST / HTTP/1.1\r\nHost: localhost\r\nContent-Length: {value}\r\n\r\nhello");
        let (mut client, server): (DuplexStream, DuplexStream) = duplex(4096);
        client.write_all(raw.as_bytes()).await.unwrap();
        client.shutdown().await.unwrap();
        let mut stream: Stream =
            Stream::from_transport(TestTransport::new(server), RequestConfig::default(), false);
        assert!(
            matches!(
                stream.try_get_http_request().await,
                Err(RequestError::InvalidContentLength(HttpStatus::BadRequest))
            ),
            "accepted invalid request: {raw:?}"
        );
    }
}

#[tokio::test]
async fn test_http_request_rejects_non_decimal_content_length() {
    for value in ["5x", "+5", "-5", "0x5", "", "5 5", "5\t5", "５", "٥"] {
        let raw: String =
            format!("POST / HTTP/1.1\r\nHost: localhost\r\nContent-Length: {value}\r\n\r\nhello");
        let (mut client, server): (DuplexStream, DuplexStream) = duplex(4096);
        client.write_all(raw.as_bytes()).await.unwrap();
        client.shutdown().await.unwrap();
        let mut stream: Stream =
            Stream::from_transport(TestTransport::new(server), RequestConfig::default(), false);
        assert!(
            matches!(
                stream.try_get_http_request().await,
                Err(RequestError::InvalidContentLength(HttpStatus::BadRequest))
            ),
            "accepted invalid request: {raw:?}"
        );
    }
}

#[tokio::test]
async fn test_http_request_rejects_folded_headers() {
    for folded in [
        " continuation",
        "\tcontinuation",
        " Content-Length: 0",
        "\tTransfer-Encoding: chunked",
        " ",
        "\t",
    ] {
        let raw: String =
            format!("POST / HTTP/1.1\r\nHost: localhost\r\nX-Test: value\r\n{folded}\r\n\r\n");
        let (mut client, server): (DuplexStream, DuplexStream) = duplex(4096);
        client.write_all(raw.as_bytes()).await.unwrap();
        client.shutdown().await.unwrap();
        let mut stream: Stream =
            Stream::from_transport(TestTransport::new(server), RequestConfig::default(), false);
        assert!(
            matches!(
                stream.try_get_http_request().await,
                Err(RequestError::HttpRead(HttpStatus::BadRequest))
            ),
            "accepted invalid request: {raw:?}"
        );
    }
}

#[tokio::test]
async fn test_http_request_rejects_whitespace_before_colon() {
    for header in [
        "Foo : bar",
        "Foo\t: bar",
        "Host : localhost",
        "Content-Length : 0",
        "Transfer-Encoding : chunked",
    ] {
        let raw: String = format!("GET / HTTP/1.1\r\nHost: localhost\r\n{header}\r\n\r\n");
        let (mut client, server): (DuplexStream, DuplexStream) = duplex(4096);
        client.write_all(raw.as_bytes()).await.unwrap();
        client.shutdown().await.unwrap();
        let mut stream: Stream =
            Stream::from_transport(TestTransport::new(server), RequestConfig::default(), false);
        assert!(
            matches!(
                stream.try_get_http_request().await,
                Err(RequestError::HttpRead(HttpStatus::BadRequest))
            ),
            "accepted invalid request: {raw:?}"
        );
    }
}

#[tokio::test]
async fn test_http_request_rejects_control_bytes_in_header_values() {
    for control in (0_u8..=31).chain([127]) {
        if control == b'\t' {
            continue;
        }
        let (mut client, server): (DuplexStream, DuplexStream) = duplex(4096);
        let mut raw: Vec<u8> = b"GET / HTTP/1.1\r\nHost: localhost\r\nX-Test: before".to_vec();
        raw.push(control);
        raw.extend_from_slice(b"after\r\n\r\n");
        client.write_all(&raw).await.unwrap();
        client.shutdown().await.unwrap();
        let mut stream: Stream =
            Stream::from_transport(TestTransport::new(server), RequestConfig::default(), false);
        assert!(
            matches!(
                stream.try_get_http_request().await,
                Err(RequestError::HttpRead(HttpStatus::BadRequest))
            ),
            "accepted control byte {control}"
        );
    }
}

#[tokio::test]
async fn test_http_request_rejects_control_bytes_at_value_edges() {
    for value in [
        "\x0bvalue",
        "value\x0b",
        "\x0cvalue",
        "value\x0c",
        "value\r",
        "\0",
    ] {
        let raw: String = format!("GET / HTTP/1.1\r\nHost: localhost\r\nX-Test: {value}\r\n\r\n");
        let (mut client, server): (DuplexStream, DuplexStream) = duplex(4096);
        client.write_all(raw.as_bytes()).await.unwrap();
        client.shutdown().await.unwrap();
        let mut stream: Stream =
            Stream::from_transport(TestTransport::new(server), RequestConfig::default(), false);
        assert!(
            matches!(
                stream.try_get_http_request().await,
                Err(RequestError::HttpRead(HttpStatus::BadRequest))
            ),
            "accepted invalid request: {raw:?}"
        );
    }
}

#[tokio::test]
async fn test_http_request_rejects_duplicate_host() {
    for headers in [
        "Host: localhost\r\nHost: localhost\r\n",
        "Host: localhost\r\nhost: attacker.test\r\n",
        "HOST: \r\nhOsT: attacker.test\r\n",
    ] {
        let raw: String = format!("GET / HTTP/1.1\r\n{headers}\r\n");
        let (mut client, server): (DuplexStream, DuplexStream) = duplex(4096);
        client.write_all(raw.as_bytes()).await.unwrap();
        client.shutdown().await.unwrap();
        let mut stream: Stream =
            Stream::from_transport(TestTransport::new(server), RequestConfig::default(), false);
        assert!(
            matches!(
                stream.try_get_http_request().await,
                Err(RequestError::HttpRead(HttpStatus::BadRequest))
            ),
            "accepted invalid request: {raw:?}"
        );
    }
}

#[tokio::test]
async fn test_http_request_rejects_http_1_1_without_host() {
    for headers in ["", "X-Test: value\r\n", "Content-Length: 0\r\n"] {
        let raw: String = format!("GET / HTTP/1.1\r\n{headers}\r\n");
        let (mut client, server): (DuplexStream, DuplexStream) = duplex(4096);
        client.write_all(raw.as_bytes()).await.unwrap();
        client.shutdown().await.unwrap();
        let mut stream: Stream =
            Stream::from_transport(TestTransport::new(server), RequestConfig::default(), false);
        assert!(
            matches!(
                stream.try_get_http_request().await,
                Err(RequestError::HttpRead(HttpStatus::BadRequest))
            ),
            "accepted invalid request: {raw:?}"
        );
    }
}

#[tokio::test]
async fn test_http_request_rejects_request_line_field_count() {
    for line in [
        "GET / HTTP/1.1 EXTRA",
        "GET / HTTP/1.1 EXTRA MORE",
        "GET /",
        "GET",
        "",
    ] {
        let raw: String = format!("{line}\r\nHost: localhost\r\n\r\n");
        let (mut client, server): (DuplexStream, DuplexStream) = duplex(4096);
        client.write_all(raw.as_bytes()).await.unwrap();
        client.shutdown().await.unwrap();
        let mut stream: Stream =
            Stream::from_transport(TestTransport::new(server), RequestConfig::default(), false);
        assert!(
            matches!(
                stream.try_get_http_request().await,
                Err(RequestError::HttpRequestPartsInsufficient(
                    HttpStatus::BadRequest
                ))
            ),
            "accepted invalid request: {raw:?}"
        );
    }
}

#[tokio::test]
async fn test_http_request_rejects_unknown_version() {
    for version in ["HTTP/9.9", "HTTP/abc", "HTTP/1.2"] {
        let raw: String = format!("GET / {version}\r\nHost: localhost\r\n\r\n");
        let (mut client, server): (DuplexStream, DuplexStream) = duplex(4096);
        client.write_all(raw.as_bytes()).await.unwrap();
        client.shutdown().await.unwrap();
        let mut stream: Stream =
            Stream::from_transport(TestTransport::new(server), RequestConfig::default(), false);
        assert!(
            matches!(
                stream.try_get_http_request().await,
                Err(RequestError::HttpRequestPartsInsufficient(
                    HttpStatus::BadRequest
                ))
            ),
            "accepted invalid request: {raw:?}"
        );
    }
}

#[tokio::test]
async fn test_http_request_rejects_content_length_overflow() {
    for value in [
        "18446744073709551616",
        "9999999999999999999999999999999999999999",
    ] {
        let raw: String =
            format!("POST / HTTP/1.1\r\nHost: localhost\r\nContent-Length: {value}\r\n\r\n");
        let (mut client, server): (DuplexStream, DuplexStream) = duplex(4096);
        client.write_all(raw.as_bytes()).await.unwrap();
        client.shutdown().await.unwrap();
        let mut stream: Stream =
            Stream::from_transport(TestTransport::new(server), RequestConfig::default(), false);
        assert!(
            matches!(
                stream.try_get_http_request().await,
                Err(RequestError::InvalidContentLength(HttpStatus::BadRequest))
            ),
            "accepted invalid request: {raw:?}"
        );
    }
}

#[tokio::test]
async fn test_http_request_rejects_malformed_header_name() {
    for header in [
        "No-Colon",
        ": value",
        "Bad(Name): value",
        "Bad\x7fName: value",
    ] {
        let raw: String = format!("GET / HTTP/1.1\r\nHost: localhost\r\n{header}\r\n\r\n");
        let (mut client, server): (DuplexStream, DuplexStream) = duplex(4096);
        client.write_all(raw.as_bytes()).await.unwrap();
        client.shutdown().await.unwrap();
        let mut stream: Stream =
            Stream::from_transport(TestTransport::new(server), RequestConfig::default(), false);
        assert!(
            matches!(
                stream.try_get_http_request().await,
                Err(RequestError::HttpRead(HttpStatus::BadRequest))
            ),
            "accepted invalid request: {raw:?}"
        );
    }
}

#[tokio::test]
async fn test_http_request_accepts_http_1_0_without_host() {
    let (mut client, server): (DuplexStream, DuplexStream) = duplex(4096);
    client
        .write_all(b"GET /legacy HTTP/1.0\r\n\r\n")
        .await
        .unwrap();
    let mut stream: Stream =
        Stream::from_transport(TestTransport::new(server), RequestConfig::default(), false);
    let request: Request = stream.try_get_http_request().await.unwrap();
    assert_eq!(*request.get_version(), RequestVersion::Http1_0);
    assert_eq!(request.get_path(), "/legacy");
    assert!(request.get_host().is_empty());
}

#[tokio::test]
async fn test_http_request_accepts_decimal_content_length_with_optional_whitespace() {
    for value in ["5 ", "\t0005\t "] {
        let (mut client, server): (DuplexStream, DuplexStream) = duplex(4096);
        let raw: String = format!(
            "POST / HTTP/1.1\r\nHost: localhost\r\nContent-Length: {value}\r\n\r\nhelloGET /next HTTP/1.1\r\nHost: localhost\r\n\r\n"
        );
        client.write_all(raw.as_bytes()).await.unwrap();
        let mut stream: Stream =
            Stream::from_transport(TestTransport::new(server), RequestConfig::default(), false);
        let first: Request = stream.try_get_http_request().await.unwrap();
        assert_eq!(first.get_body(), b"hello");
        let second: Request = stream.try_get_http_request().await.unwrap();
        assert_eq!(second.get_path(), "/next");
        assert!(second.get_body().is_empty());
    }
}

#[tokio::test]
async fn test_http_request_preserves_repeated_non_framing_headers_and_tabs() {
    let (mut client, server): (DuplexStream, DuplexStream) = duplex(4096);
    client.write_all(b"GET / HTTP/1.1\r\nHost: localhost\r\nX-Test: before\tafter\r\nSet-Cookie: one=1\r\nSet-Cookie: two=2\r\n\r\n").await.unwrap();
    let mut stream: Stream =
        Stream::from_transport(TestTransport::new(server), RequestConfig::default(), false);
    let request: Request = stream.try_get_http_request().await.unwrap();
    let values: &VecDeque<String> = request.get_headers().get("set-cookie").unwrap();
    assert_eq!(values.len(), 2);
    assert_eq!(values.front().unwrap(), "one=1");
    assert_eq!(values.back().unwrap(), "two=2");
    assert_eq!(
        request
            .get_headers()
            .get("x-test")
            .unwrap()
            .front()
            .unwrap(),
        "before\tafter"
    );
}

#[tokio::test]
async fn test_http_request_requires_host_after_http_1_0_pipeline() {
    let (mut client, server): (DuplexStream, DuplexStream) = duplex(4096);
    client
        .write_all(b"GET /legacy HTTP/1.0\r\n\r\nGET /new HTTP/1.1\r\n\r\n")
        .await
        .unwrap();
    let mut stream: Stream =
        Stream::from_transport(TestTransport::new(server), RequestConfig::default(), false);
    let mut request: Request = Request::default();
    stream.try_fill_http_request(&mut request).await.unwrap();
    assert_eq!(*request.get_version(), RequestVersion::Http1_0);
    assert!(matches!(
        stream.try_fill_http_request(&mut request).await,
        Err(RequestError::HttpRead(HttpStatus::BadRequest))
    ));
}

#[tokio::test]
async fn test_http_request_allows_http_1_0_after_http_1_1_pipeline() {
    let (mut client, server): (DuplexStream, DuplexStream) = duplex(4096);
    client
        .write_all(b"GET /new HTTP/1.1\r\nHost: localhost\r\n\r\nGET /legacy HTTP/1.0\r\n\r\n")
        .await
        .unwrap();
    let mut stream: Stream =
        Stream::from_transport(TestTransport::new(server), RequestConfig::default(), false);
    let mut request: Request = Request::default();
    stream.try_fill_http_request(&mut request).await.unwrap();
    assert_eq!(*request.get_version(), RequestVersion::Http1_1);
    stream.try_fill_http_request(&mut request).await.unwrap();
    assert_eq!(*request.get_version(), RequestVersion::Http1_0);
    assert!(request.get_host().is_empty());
}

#[tokio::test]
async fn test_http_request_rejects_missing_host_after_keep_alive() {
    let (mut client, server): (DuplexStream, DuplexStream) = duplex(4096);
    client
        .write_all(b"GET /one HTTP/1.1\r\nHost: localhost\r\n\r\nGET /two HTTP/1.1\r\n\r\n")
        .await
        .unwrap();
    let mut stream: Stream =
        Stream::from_transport(TestTransport::new(server), RequestConfig::default(), false);
    let mut request: Request = Request::default();
    stream.try_fill_http_request(&mut request).await.unwrap();
    assert_eq!(request.get_host(), "localhost");
    assert!(matches!(
        stream.try_fill_http_request(&mut request).await,
        Err(RequestError::HttpRead(HttpStatus::BadRequest))
    ));
}

#[tokio::test]
async fn test_http_request_rejects_unterminated_header_section() {
    for raw in [
        b"GET / HTTP/1.1\r\nHost: localhost\r\n".as_slice(),
        b"GET / HTTP/1.1\r\nHost: localhost".as_slice(),
    ] {
        let (mut client, server): (DuplexStream, DuplexStream) = duplex(4096);
        client.write_all(raw).await.unwrap();
        client.shutdown().await.unwrap();
        let mut stream: Stream =
            Stream::from_transport(TestTransport::new(server), RequestConfig::default(), false);
        assert!(matches!(
            stream.try_get_http_request().await,
            Err(RequestError::HttpRead(HttpStatus::BadRequest))
        ));
    }
}

#[test]
fn test_socket_host_type_alias() {
    let socket_host: SocketHost = "127.0.0.1".parse().unwrap();
    assert!(socket_host.is_ipv4());
    let socket_host_v6: SocketHost = "::1".parse().unwrap();
    assert!(socket_host_v6.is_ipv6());
}

#[test]
fn test_socket_port_type_alias() {
    let socket_port: SocketPort = 8080;
    assert_eq!(socket_port, 8080_u16);
}

#[tokio::test]
async fn test_from_transport_wraps_duplex_as_transport() {
    let (mut client, server): (DuplexStream, DuplexStream) = duplex(64);
    let config: RequestConfig = RequestConfig::default();
    let mut stream: Stream = Stream::from_transport(TestTransport::new(server), config, false);
    assert!(!stream.get_closed());
    assert!(stream.is_keep_alive(true));
    assert!(!stream.is_keep_alive(false));
    let sent: Vec<u8> = b"ping".to_vec();
    assert!(stream.try_send(&sent).await.is_ok());
    assert!(stream.try_flush().await.is_ok());
    let mut received: Vec<u8> = vec![0_u8; 4];
    client.read_exact(&mut received).await.unwrap();
    assert_eq!(received, b"ping".to_vec());
}

#[tokio::test]
async fn test_from_transport_reads_http_request_over_duplex() {
    let (mut client, server): (DuplexStream, DuplexStream) = duplex(1024);
    let raw: Vec<u8> = b"GET /trans HTTP/1.1\r\nHost: localhost\r\n\r\n".to_vec();
    client.write_all(&raw).await.unwrap();
    let config: RequestConfig = RequestConfig::default();
    let mut stream: Stream = Stream::from_transport(TestTransport::new(server), config, false);
    let request: Request = stream.try_get_http_request().await.unwrap();
    assert_eq!(*request.get_method(), RequestMethod::Get);
    assert_eq!(*request.get_path(), "/trans");
    assert_eq!(*request.get_version(), RequestVersion::Http1_1);
    assert_eq!(*request.get_host(), "localhost");
    assert!(request.get_body().is_empty());
}

#[tokio::test]
async fn test_from_transport_send_list_over_duplex() {
    let (mut client, server): (DuplexStream, DuplexStream) = duplex(64);
    let config: RequestConfig = RequestConfig::default();
    let mut stream: Stream = Stream::from_transport(TestTransport::new(server), config, false);
    let parts: Vec<&[u8]> = vec![b"HTTP/1.1 200 OK\r\n", b"Content-Length: 0\r\n\r\n"];
    assert!(stream.try_send_list(parts).await.is_ok());
    assert!(stream.try_flush().await.is_ok());
    let mut received: Vec<u8> = vec![0_u8; 38];
    client.read_exact(&mut received).await.unwrap();
    assert_eq!(&received[..4], b"HTTP");
    assert_eq!(&received[36..], b"\r\n");
}

#[tokio::test]
async fn test_from_transport_closed_rejects_writes() {
    let (_client, server): (DuplexStream, DuplexStream) = duplex(64);
    let config: RequestConfig = RequestConfig::default();
    let mut stream: Stream = Stream::from_transport(TestTransport::new(server), config, true);
    assert!(stream.get_closed());
    assert!(!stream.is_keep_alive(true));
    assert!(stream.try_send(b"nope").await.is_err());
    assert!(stream.try_send_list(vec![b"nope"]).await.is_err());
    assert!(stream.try_flush().await.is_err());
    assert!(stream.try_get_http_request().await.is_err());
}

#[tokio::test]
async fn test_from_transport_transport_without_socket_address_reports_error() {
    let (_client, server): (DuplexStream, DuplexStream) = duplex(64);
    let config: RequestConfig = RequestConfig::default();
    let stream: Stream = Stream::from_transport(TestTransport::new(server), config, false);
    assert!(stream.get_peer_addr().is_err());
    assert!(stream.get_local_addr().is_err());
    assert!(stream.get_stream().peer_addr().is_err());
    assert!(stream.get_stream().local_addr().is_err());
}

#[tokio::test]
async fn test_new_tcp_stream_reports_peer_and_local_address() {
    let listener: TcpListener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let bound: SocketAddr = listener.local_addr().unwrap();
    let client: TcpStream = TcpStream::connect(bound).await.unwrap();
    let (server, peer): (TcpStream, SocketAddr) = listener.accept().await.unwrap();
    assert_ne!(peer, bound);
    let config: RequestConfig = RequestConfig::default();
    let stream: Stream = Stream::new(server, config, false);
    assert_eq!(stream.get_peer_addr().unwrap(), peer);
    assert_eq!(stream.get_local_addr().unwrap(), bound);
    assert_eq!(stream.get_stream().peer_addr().unwrap(), peer);
    drop(client);
}

#[tokio::test]
async fn test_new_tcp_stream_parses_request_over_socket() {
    let listener: TcpListener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let bound: SocketAddr = listener.local_addr().unwrap();
    let mut client: TcpStream = TcpStream::connect(bound).await.unwrap();
    let (server, peer): (TcpStream, SocketAddr) = listener.accept().await.unwrap();
    let raw: Vec<u8> =
        b"POST /tcp HTTP/1.1\r\nHost: localhost\r\nContent-Length: 4\r\n\r\nbody".to_vec();
    client.write_all(&raw).await.unwrap();
    let config: RequestConfig = RequestConfig::default();
    let mut stream: Stream = Stream::new(server, config, false);
    let request: Request = stream.try_get_http_request().await.unwrap();
    assert_eq!(*request.get_method(), RequestMethod::Post);
    assert_eq!(*request.get_path(), "/tcp");
    assert_eq!(request.get_body(), b"body");
    assert_eq!(stream.get_peer_addr().unwrap(), peer);
    assert_eq!(stream.get_local_addr().unwrap(), bound);
    let response: Vec<u8> = b"HTTP/1.1 200 OK\r\nContent-Length: 0\r\n\r\n".to_vec();
    assert!(stream.try_send(&response).await.is_ok());
    assert!(stream.try_flush().await.is_ok());
    let mut echoed: Vec<u8> = vec![0_u8; 38];
    client.read_exact(&mut echoed).await.unwrap();
    assert_eq!(&echoed[..4], b"HTTP");
}

#[tokio::test]
async fn test_from_transport_keeps_pipelined_request_in_leftover() {
    let (mut client, server): (DuplexStream, DuplexStream) = duplex(2048);
    let first: Vec<u8> = b"GET /one HTTP/1.1\r\nHost: localhost\r\n\r\n".to_vec();
    let second: Vec<u8> = b"GET /two HTTP/1.1\r\nHost: localhost\r\n\r\n".to_vec();
    client.write_all(&[first, second].concat()).await.unwrap();
    let config: RequestConfig = RequestConfig::default();
    let mut stream: Stream = Stream::from_transport(TestTransport::new(server), config, false);
    let one: Request = stream.try_get_http_request().await.unwrap();
    assert_eq!(*one.get_path(), "/one");
    let two: Request = stream.try_get_http_request().await.unwrap();
    assert_eq!(*two.get_path(), "/two");
}

#[tokio::test]
async fn test_pipeline_reuses_read_ahead_allocation() {
    let (mut client, server): (DuplexStream, DuplexStream) = duplex(4096);
    let raw: &[u8] = b"GET /one HTTP/1.1\r\nHost: localhost\r\n\r\nGET /two HTTP/1.1\r\nHost: localhost\r\n\r\nGET /three HTTP/1.1\r\nHost: localhost\r\n\r\n";
    client.write_all(raw).await.unwrap();
    let mut stream: Stream =
        Stream::from_transport(TestTransport::new(server), RequestConfig::default(), false);
    let mut request: Request = Request::default();
    stream.try_fill_http_request(&mut request).await.unwrap();
    assert_eq!(request.get_path(), "/one");
    let allocation: *const u8 = stream.get_leftover().as_ptr();
    let first_start: usize = stream.get_leftover_start();
    assert!(first_start > 0);
    stream.try_fill_http_request(&mut request).await.unwrap();
    assert_eq!(request.get_path(), "/two");
    assert_eq!(stream.get_leftover().as_ptr(), allocation);
    assert!(stream.get_leftover_start() > first_start);
    stream.try_fill_http_request(&mut request).await.unwrap();
    assert_eq!(request.get_path(), "/three");
    assert!(stream.get_leftover().is_empty());
    assert_eq!(stream.get_leftover_start(), 0);
    assert_eq!(stream.get_leftover_end(), 0);
}

#[tokio::test]
async fn test_pipeline_spans_small_buffers_and_preserves_binary_bodies() {
    let (mut client, server): (DuplexStream, DuplexStream) = duplex(32768);
    let body: Vec<u8> = (0..=255).collect();
    let mut raw: Vec<u8> = Vec::new();
    for index in 0..32 {
        raw.extend_from_slice(
            format!(
                "POST /item/{index} HTTP/1.1\r\nHost: localhost\r\nContent-Length: {}\r\n\r\n",
                body.len()
            )
            .as_bytes(),
        );
        raw.extend_from_slice(&body);
    }
    client.write_all(&raw).await.unwrap();
    let mut config: RequestConfig = RequestConfig::default();
    config.set_buffer_size(64);
    let mut stream: Stream = Stream::from_transport(TestTransport::new(server), config, false);
    let mut request: Request = Request::default();
    for index in 0..32 {
        stream.try_fill_http_request(&mut request).await.unwrap();
        assert_eq!(request.get_path(), &format!("/item/{index}"));
        assert_eq!(request.get_body(), &body);
        assert_eq!(*request.get_method(), RequestMethod::Post);
    }
}

#[tokio::test]
async fn test_fragmented_pipeline_resumes_after_pending() {
    let (mut client, server): (DuplexStream, DuplexStream) = duplex(32);
    let writer: JoinHandle<()> = spawn(async move {
        let raw: &[u8] = b"POST /one HTTP/1.1\r\nHost: localhost\r\nContent-Length: 6\r\n\r\none\xff\0\nGET /two HTTP/1.1\r\nHost: localhost\r\n\r\n";
        for chunk in raw.chunks(7) {
            client.write_all(chunk).await.unwrap();
            yield_now().await;
        }
    });
    let mut config: RequestConfig = RequestConfig::default();
    config.set_buffer_size(16);
    let mut stream: Stream = Stream::from_transport(TestTransport::new(server), config, false);
    let mut request: Request = Request::default();
    stream.try_fill_http_request(&mut request).await.unwrap();
    assert_eq!(request.get_path(), "/one");
    assert_eq!(request.get_body(), b"one\xff\0\n");
    stream.try_fill_http_request(&mut request).await.unwrap();
    assert_eq!(request.get_path(), "/two");
    assert!(request.get_body().is_empty());
    writer.await.unwrap();
}

#[tokio::test]
async fn test_idle_reader_resumes_after_buffer_is_recycled() {
    let (mut client, server): (DuplexStream, DuplexStream) = duplex(128);
    let writer: JoinHandle<()> = spawn(async move {
        sleep(Duration::from_millis(10)).await;
        client
            .write_all(b"GET /wake HTTP/1.1\r\nHost: localhost\r\n\r\n")
            .await
            .unwrap();
    });
    let mut stream: Stream =
        Stream::from_transport(TestTransport::new(server), RequestConfig::default(), false);
    let request: Request = stream.try_get_http_request().await.unwrap();
    assert_eq!(request.get_path(), "/wake");
    writer.await.unwrap();
}

#[tokio::test]
async fn test_partial_header_still_obeys_read_timeout() {
    let (mut client, server): (DuplexStream, DuplexStream) = duplex(128);
    client
        .write_all(b"GET /timeout HTTP/1.1\r\nHo")
        .await
        .unwrap();
    let mut config: RequestConfig = RequestConfig::default();
    config.set_read_timeout_ms(10);
    let mut stream: Stream = Stream::from_transport(TestTransport::new(server), config, false);
    assert!(stream.try_get_http_request().await.is_err());
}

#[tokio::test]
async fn test_truncated_body_is_rejected_after_buffer_is_drained() {
    let (mut client, server): (DuplexStream, DuplexStream) = duplex(128);
    client
        .write_all(b"POST /short HTTP/1.1\r\nHost: localhost\r\nContent-Length: 16\r\n\r\nshort")
        .await
        .unwrap();
    client.shutdown().await.unwrap();
    let mut config: RequestConfig = RequestConfig::default();
    config.set_buffer_size(16);
    let mut stream: Stream = Stream::from_transport(TestTransport::new(server), config, false);
    assert!(stream.try_get_http_request().await.is_err());
}

#[test]
fn read_buffer_pool_reuses_owned_storage() {
    READ_BUFFER_POOL.with(|pool: &RefCell<Vec<Vec<u8>>>| pool.borrow_mut().clear());
    let buffer: Vec<u8> = take_read_buffer(128);
    let address: *const u8 = buffer.as_ptr();
    return_read_buffer(buffer);
    let reused: Vec<u8> = take_read_buffer(128);
    assert_eq!(reused.as_ptr(), address);
    assert_eq!(reused.len(), 128);
    return_read_buffer(reused);
    READ_BUFFER_POOL.with(|pool: &RefCell<Vec<Vec<u8>>>| pool.borrow_mut().clear());
}

#[test]
fn read_buffer_pool_grows_small_storage_to_the_requested_length() {
    READ_BUFFER_POOL.with(|pool: &RefCell<Vec<Vec<u8>>>| pool.borrow_mut().clear());
    return_read_buffer(vec![0; 8]);
    let buffer: Vec<u8> = take_read_buffer(256);
    assert_eq!(buffer.len(), 256);
    assert!(buffer.iter().all(|byte: &u8| *byte == 0));
}

#[test]
fn read_buffer_pool_drops_empty_transfer_sentinels() {
    READ_BUFFER_POOL.with(|pool: &RefCell<Vec<Vec<u8>>>| pool.borrow_mut().clear());
    return_read_buffer(Vec::new());
    assert_eq!(
        READ_BUFFER_POOL.with(|pool: &RefCell<Vec<Vec<u8>>>| pool.borrow().len()),
        0
    );
}

#[test]
fn read_buffer_pool_rejects_large_capacity_even_when_length_is_small() {
    READ_BUFFER_POOL.with(|pool: &RefCell<Vec<Vec<u8>>>| pool.borrow_mut().clear());
    let mut buffer: Vec<u8> = Vec::with_capacity(1024 * 1024);
    buffer.push(0);
    return_read_buffer(buffer);
    assert_eq!(
        READ_BUFFER_POOL.with(|pool: &RefCell<Vec<Vec<u8>>>| pool.borrow().len()),
        0
    );
}

#[test]
fn read_buffer_pool_enforces_its_entry_limit() {
    READ_BUFFER_POOL.with(|pool: &RefCell<Vec<Vec<u8>>>| pool.borrow_mut().clear());
    for _ in 0..MAX_POOLED_READ_BUFFERS + 8 {
        return_read_buffer(vec![0; 8]);
    }
    assert_eq!(
        READ_BUFFER_POOL.with(|pool: &RefCell<Vec<Vec<u8>>>| pool.borrow().len()),
        MAX_POOLED_READ_BUFFERS
    );
    READ_BUFFER_POOL.with(|pool: &RefCell<Vec<Vec<u8>>>| pool.borrow_mut().clear());
}

#[tokio::test]
async fn pooled_reader_returns_drained_storage_while_pending_then_resumes() {
    READ_BUFFER_POOL.with(|pool: &RefCell<Vec<Vec<u8>>>| pool.borrow_mut().clear());
    let (mut client, server): (DuplexStream, DuplexStream) = duplex(128);
    let mut transport: Box<dyn Transport> = Box::new(TestTransport::new(server));
    let mut reader: PooledReader<'_> = PooledReader::new(&mut transport, Vec::new(), 128);
    for _ in 0..2 {
        let pending: bool = poll_fn(|cx: &mut Context<'_>| {
            Poll::Ready(Pin::new(&mut reader).poll_fill_buf(cx).is_pending())
        })
        .await;
        assert!(pending);
        assert_eq!(
            READ_BUFFER_POOL.with(|pool: &RefCell<Vec<Vec<u8>>>| pool.borrow().len()),
            1
        );
    }
    client.write_all(b"ready").await.unwrap();
    assert_eq!(reader.fill_buf().await.unwrap(), b"ready");
    Pin::new(&mut reader).consume(5);
    drop(reader);
    assert_eq!(
        READ_BUFFER_POOL.with(|pool: &RefCell<Vec<Vec<u8>>>| pool.borrow().len()),
        1
    );
    READ_BUFFER_POOL.with(|pool: &RefCell<Vec<Vec<u8>>>| pool.borrow_mut().clear());
}

#[test]
fn header_entry_pool_preserves_duplicate_value_order() {
    HEADER_ENTRY_POOL
        .with(|pool: &RefCell<Vec<(String, VecDeque<String>)>>| pool.borrow_mut().clear());
    let values: VecDeque<String> = VecDeque::from(["first".to_owned(), "second".to_owned()]);
    return_header_entry(("x-value".to_owned(), values.clone()));
    assert_eq!(take_header_entry(), Some(("x-value".to_owned(), values)));
    assert!(take_header_entry().is_none());
}

#[test]
fn string_pool_returns_the_same_allocation() {
    STRING_POOL.with(|pool: &RefCell<Vec<String>>| pool.borrow_mut().clear());
    let value: String = String::from("content length");
    let address: *const u8 = value.as_ptr();
    return_recycled_string(value);
    let recycled: String = take_recycled_string().unwrap();
    assert_eq!(recycled, "content length");
    assert_eq!(recycled.as_ptr(), address);
    assert!(take_recycled_string().is_none());
}

#[tokio::test]
async fn websocket_discards_fragment_state_after_close_error() {
    let (mut client, server): (DuplexStream, DuplexStream) = duplex(1024);
    let mut stream: Stream =
        Stream::from_transport(TestTransport::new(server), RequestConfig::default(), false);
    let mut fragment: Vec<u8> = WebSocketFrame::create_frame_list(b"discarded").concat();
    fragment[0] &= 0x7f;
    fragment.extend_from_slice(&[0x88, 0]);
    client.write_all(&fragment).await.unwrap();
    assert!(
        timeout(Duration::from_secs(1), stream.try_get_websocket_request())
            .await
            .unwrap()
            .is_err()
    );
    client
        .write_all(&WebSocketFrame::create_frame_list(b"accepted").concat())
        .await
        .unwrap();
    assert_eq!(
        timeout(Duration::from_secs(1), stream.try_get_websocket_request())
            .await
            .unwrap()
            .unwrap(),
        b"accepted"
    );
}

#[tokio::test]
async fn websocket_retains_partial_frame_when_read_is_cancelled() {
    let (mut client, server): (DuplexStream, DuplexStream) = duplex(1024);
    let mut stream: Stream =
        Stream::from_transport(TestTransport::new(server), RequestConfig::default(), false);
    let frame: Vec<u8> = WebSocketFrame::create_frame_list(b"partial-message").concat();
    client.write_all(&frame[..3]).await.unwrap();
    let mut reading: Pin<Box<dyn Future<Output = Result<RequestBody, RequestError>> + '_>> =
        Box::pin(stream.try_get_websocket_request());
    let pending: bool =
        poll_fn(|cx: &mut Context<'_>| Poll::Ready(reading.as_mut().poll(cx).is_pending())).await;
    assert!(pending);
    drop(reading);
    client.write_all(&frame[3..]).await.unwrap();
    assert_eq!(
        timeout(Duration::from_secs(1), stream.try_get_websocket_request())
            .await
            .unwrap()
            .unwrap(),
        b"partial-message"
    );
}

#[tokio::test]
async fn websocket_preserves_multiple_frames_read_together() {
    let (mut client, server): (DuplexStream, DuplexStream) = duplex(1024);
    let mut stream: Stream =
        Stream::from_transport(TestTransport::new(server), RequestConfig::default(), false);
    let first: Vec<u8> = WebSocketFrame::create_frame_list(b"first").concat();
    let second: Vec<u8> = WebSocketFrame::create_frame_list(b"second").concat();
    client.write_all(&[first, second].concat()).await.unwrap();
    for expected in [b"first".as_slice(), b"second".as_slice()] {
        assert_eq!(
            timeout(Duration::from_secs(1), stream.try_get_websocket_request())
                .await
                .unwrap()
                .unwrap(),
            expected
        );
    }
}

#[tokio::test]
async fn websocket_idle_read_returns_storage_to_pool() {
    READ_BUFFER_POOL.with(|pool: &RefCell<Vec<Vec<u8>>>| pool.borrow_mut().clear());
    let (_client, server): (DuplexStream, DuplexStream) = duplex(1024);
    let mut stream: Stream =
        Stream::from_transport(TestTransport::new(server), RequestConfig::default(), false);
    let mut reading: Pin<Box<dyn Future<Output = Result<RequestBody, RequestError>> + '_>> =
        Box::pin(stream.try_get_websocket_request());
    assert!(
        poll_fn(|cx: &mut Context<'_>| { Poll::Ready(reading.as_mut().poll(cx).is_pending()) })
            .await
    );
    assert_eq!(
        READ_BUFFER_POOL.with(|pool: &RefCell<Vec<Vec<u8>>>| pool.borrow().len()),
        1
    );
    drop(reading);
    READ_BUFFER_POOL.with(|pool: &RefCell<Vec<Vec<u8>>>| pool.borrow_mut().clear());
}

#[tokio::test]
async fn websocket_reads_frame_buffered_after_http_upgrade() {
    let (mut client, server): (DuplexStream, DuplexStream) = duplex(4096);
    let mut stream: Stream =
        Stream::from_transport(TestTransport::new(server), RequestConfig::default(), false);
    let mut bytes: Vec<u8> = b"GET /ws HTTP/1.1\r\nHost: localhost\r\n\r\n".to_vec();
    bytes.extend(WebSocketFrame::create_frame_list(b"upgrade-data").concat());
    client.write_all(&bytes).await.unwrap();
    assert_eq!(
        stream.try_get_http_request().await.unwrap().get_path(),
        "/ws"
    );
    assert_eq!(
        timeout(Duration::from_secs(1), stream.try_get_websocket_request())
            .await
            .unwrap()
            .unwrap(),
        b"upgrade-data"
    );
}

#[tokio::test]
async fn websocket_rejects_half_closed_partial_frame_without_hanging() {
    let (mut client, server): (DuplexStream, DuplexStream) = duplex(1024);
    let mut stream: Stream =
        Stream::from_transport(TestTransport::new(server), RequestConfig::default(), false);
    let frame: Vec<u8> = WebSocketFrame::create_frame_list(b"truncated").concat();
    client.write_all(&frame[..3]).await.unwrap();
    client.shutdown().await.unwrap();
    assert!(
        timeout(Duration::from_secs(1), stream.try_get_websocket_request())
            .await
            .unwrap()
            .is_err()
    );
}
