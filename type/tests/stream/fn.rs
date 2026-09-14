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
