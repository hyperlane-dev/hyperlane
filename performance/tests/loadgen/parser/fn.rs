use super::*;

fn head(lines: &[&[u8]]) -> Vec<u8> {
    let mut bytes: Vec<u8> = Vec::new();
    for line in lines {
        bytes.extend_from_slice(line);
    }
    bytes
}

#[test]
fn parses_the_content_length_of_a_head() {
    let bytes: Vec<u8> = head(&[b"HTTP/1.1 200 OK\r\n", b"Content-Length: 25\r\n", b"\r\n"]);
    assert_eq!(parse_content_length(&bytes), 25);
}

#[test]
fn parses_the_content_length_in_any_case() {
    let bytes: Vec<u8> = head(&[b"HTTP/1.1 200 OK\r\n", b"content-length: 7\r\n", b"\r\n"]);
    assert_eq!(parse_content_length(&bytes), 7);
}

#[test]
fn reports_no_length_for_a_head_without_one() {
    let bytes: Vec<u8> = head(&[
        b"HTTP/1.1 200 OK\r\n",
        b"Connection: keep-alive\r\n",
        b"\r\n",
    ]);
    assert_eq!(parse_content_length(&bytes), 0);
}

#[test]
fn ignores_a_length_name_inside_another_value() {
    let bytes: Vec<u8> = head(&[
        b"HTTP/1.1 200 OK\r\n",
        b"X-Note: content-length: 99\r\n",
        b"Content-Length: 3\r\n",
        b"\r\n",
    ]);
    assert_eq!(parse_content_length(&bytes), 3);
}

#[test]
fn reports_no_length_for_a_head_with_an_empty_length() {
    let bytes: Vec<u8> = head(&[b"HTTP/1.1 200 OK\r\n", b"Content-Length: \r\n", b"\r\n"]);
    assert_eq!(parse_content_length(&bytes), 0);
}

#[test]
fn finds_a_subsequence_at_its_offset() {
    let haystack: &[u8] = b"abc\r\ndef\r\n";
    assert_eq!(find_subsequence(haystack, b"\r\n"), Some(3));
}

#[test]
fn reports_an_absent_subsequence() {
    let haystack: &[u8] = b"abc";
    assert_eq!(find_subsequence(haystack, b"\r\n"), None);
}

#[test]
fn reports_zero_for_an_empty_sample_set() {
    assert_eq!(percentile(&[], 0.5), 0);
}

#[test]
fn reports_the_only_sample() {
    assert_eq!(percentile(&[42], 0.99), 42);
}

#[test]
fn interpolates_between_two_samples() {
    assert_eq!(percentile(&[0, 100], 0.5), 50);
}

#[test]
fn reads_the_arguments() {
    let args: Vec<String> = vec![
        "--addr".to_owned(),
        "127.0.0.1:9000".to_owned(),
        "--connections".to_owned(),
        "32".to_owned(),
        "--duration".to_owned(),
        "5".to_owned(),
        "--warmup".to_owned(),
        "1".to_owned(),
    ];
    let config: Config = match Config::from_args(&args) {
        Ok(config) => config,
        Err(error) => panic!("{error}"),
    };
    assert_eq!(config.get_address(), "127.0.0.1:9000");
    assert_eq!(config.get_connections(), 32);
    assert_eq!(config.get_duration_secs(), 5);
    assert_eq!(config.get_warmup_secs(), 1);
}

#[test]
fn refuses_an_argument_without_a_value() {
    let args: Vec<String> = vec!["--addr".to_owned()];
    assert!(Config::from_args(&args).is_err());
}

#[test]
fn refuses_an_unknown_argument() {
    let args: Vec<String> = vec!["--nonsense".to_owned()];
    assert!(Config::from_args(&args).is_err());
}

#[test]
fn refuses_an_argument_with_a_bad_number() {
    let args: Vec<String> = vec!["--connections".to_owned(), "many".to_owned()];
    assert!(Config::from_args(&args).is_err());
}

#[tokio::test]
async fn excludes_warmup_requests_from_throughput() {
    let listener: TcpListener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address: String = listener.local_addr().unwrap().to_string();
    let server: JoinHandle<()> = spawn(async move {
        let (mut stream, _): (TcpStream, SocketAddr) = listener.accept().await.unwrap();
        let mut buffer: [u8; 1024] = [0; 1024];
        while let Ok(read) = stream.read(&mut buffer).await {
            if read == 0 {
                break;
            }
            sleep(Duration::from_millis(1)).await;
            if stream
                .write_all(
                    b"HTTP/1.1 200 OK\r\nContent-Length: 25\r\n\r\nhello hyperlane benchmark",
                )
                .await
                .is_err()
            {
                break;
            }
        }
    });
    let cases: Vec<Case> = build_cases().into_iter().take(1).collect();
    let warmup_until: Instant = Instant::now() + Duration::from_millis(150);
    let stats: ConnStats = run_connection(
        address,
        cases,
        Arc::new(Barrier::new(1)),
        warmup_until,
        warmup_until + Duration::from_millis(150),
        Protocol::Http1,
        None,
    )
    .await;
    server.abort();
    assert_eq!(stats.errors, 0);
    assert!(stats.completed > 0);
    assert_eq!(stats.completed as usize, stats.latencies_us.len());
}
