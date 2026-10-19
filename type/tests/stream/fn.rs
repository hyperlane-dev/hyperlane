use super::*;

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
