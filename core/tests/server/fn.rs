use super::*;

async fn start_server_with<Register>(register: Register) -> (ServerControlHook, u16)
where
    Register: FnOnce(&mut Server),
{
    let listener: TcpListener = TcpListener::bind("127.0.0.1:0").unwrap();
    let port: u16 = listener.local_addr().unwrap().port();
    drop(listener);
    let mut server: Server = Server::default();
    let mut server_config: ServerConfig = ServerConfig::default();
    server_config.set_address(format!("127.0.0.1:{port}"));
    server_config.set_nodelay(Some(false));
    server.server_config(server_config);
    server.response_middleware::<ResponseMiddleware>();
    register(&mut server);
    let leaked: &'static mut Server = Box::leak(Box::new(server));
    let control_hook: ServerControlHook = leaked.run().await.unwrap_or_default();
    (control_hook, port)
}

async fn client_write_request(port: u16, request: &[u8]) {
    let mut stream: TcpStream = TcpStream::connect(("127.0.0.1", port)).await.unwrap();
    stream.write_all(request).await.unwrap();
    stream.flush().await.unwrap();
    drop(stream);
}

#[test]
fn server_partial_eq() {
    let server1: Server = Server::default();
    let server2: Server = Server::default();
    assert_eq!(server1, server2);
    let server1_clone: Server = server1.clone();
    assert_eq!(server1, server1_clone);
}

#[test]
fn server_from_address() {
    let mut server: Server = Server::default();
    server.set_request_config(RequestConfig::default());
    let server_address: usize = (&server).into();
    let server_from_addr: Server = server_address.into();
    assert_eq!(
        server.get_request_config(),
        server_from_addr.get_request_config()
    );
}

#[test]
fn server_ref_from_address() {
    let mut server: Server = Server::default();
    server.set_server_config(ServerConfig::default());
    let server_address: usize = (&server).into();
    let server_ref: &Server = server_address.into();
    assert_eq!(server.get_server_config(), server_ref.get_server_config());
}

#[test]
fn server_mut_from_address() {
    let mut server: Server = Server::default();
    let server_address: usize = (&mut server).into();
    let server_mut: &mut Server = server_address.into();
    let mut config: ServerConfig = ServerConfig::default();
    config.set_nodelay(Some(true));
    server_mut.set_server_config(config);
    assert!(server_mut.get_server_config().try_get_nodelay().is_some());
}

#[test]
fn server_from_server_config() {
    let mut server_config: ServerConfig = ServerConfig::default();
    server_config.set_nodelay(Some(true));
    let server: Server = server_config.clone().into();
    assert_eq!(server.get_request_config(), &RequestConfig::default());
    assert_eq!(server.get_server_config(), &server_config);
    assert!(server.get_task_panic().is_empty());
    assert!(server.get_request_error().is_empty());
    assert!(server.get_request_middleware().is_empty());
    assert!(server.get_response_middleware().is_empty());
}

#[test]
fn server_from_request_config() {
    let mut request_config: RequestConfig = RequestConfig::default();
    request_config.set_buffer_size(KB_1);
    let server: Server = request_config.into();
    assert_eq!(server.get_request_config(), &request_config);
    assert_eq!(server.get_server_config(), &ServerConfig::default());
    assert!(server.get_task_panic().is_empty());
    assert!(server.get_request_error().is_empty());
    assert!(server.get_request_middleware().is_empty());
    assert!(server.get_response_middleware().is_empty());
}

#[test]
fn server_inner_partial_eq() {
    let inner1: Server = Server::default();
    let inner2: Server = Server::default();
    assert_eq!(inner1, inner2);
}

#[test]
fn server_ref_into_address() {
    let server: Server = Server::default();
    let server_address: usize = (&server).into();
    assert!(server_address > 0);
}

#[test]
fn server_mut_into_address() {
    let mut server: Server = Server::default();
    let server_address: usize = (&mut server).into();
    assert!(server_address > 0);
}

#[test]
fn server_as_ref() {
    let mut server: Server = Server::default();
    server.set_server_config(ServerConfig::default());
    let server_ref: &Server = server.as_ref();
    assert_eq!(server.get_server_config(), server_ref.get_server_config());
    assert_eq!(server.get_request_config(), server_ref.get_request_config());
}

#[test]
fn server_as_mut() {
    let mut server: Server = Server::default();
    let server_mut: &mut Server = server.as_mut();
    let mut config: ServerConfig = ServerConfig::default();
    config.set_nodelay(Some(true));
    server_mut.set_server_config(config);
    assert!(server.get_server_config().try_get_nodelay().is_some());
}

#[test]
fn server_send_sync() {
    fn assert_send<T: Send>() {}
    fn assert_sync<T: Sync>() {}
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send::<Server>();
    assert_sync::<Server>();
    assert_send_sync::<Server>();
}

#[tokio::test]
async fn server_clone_across_threads() {
    let mut server: Server = Server::default();
    server.route::<TestSendRoute>("/test");
    let server_clone: Server = server.clone();
    let handle: JoinHandle<&'static str> = spawn(async move {
        let _server_in_thread: Server = server_clone;
        "success"
    });
    let result: &'static str = handle.await.unwrap();
    assert_eq!(result, "success");
}

#[tokio::test]
async fn server_share_across_threads() {
    let mut server: Server = Server::default();
    server.route::<TestSendRoute>("/test");
    let server: Arc<Server> = Arc::new(server);
    let server1: Arc<Server> = server.clone();
    let server2: Arc<Server> = server.clone();
    let handle1: JoinHandle<&'static str> = spawn(async move {
        let _server_in_thread1: Arc<Server> = server1;
        "thread1"
    });
    let handle2: JoinHandle<&'static str> = spawn(async move {
        let _server_in_thread2: Arc<Server> = server2;
        "thread2"
    });
    let result1: &'static str = handle1.await.unwrap();
    let result2: &'static str = handle2.await.unwrap();
    assert_eq!(result1, "thread1");
    assert_eq!(result2, "thread2");
}

#[tokio::test]
async fn main() {
    let mut server: Server = Server::default();
    let mut server_config: ServerConfig = ServerConfig::default();
    server_config
        .set_address(Server::format_bind_address(DEFAULT_HOST, 80))
        .set_nodelay(Some(false));
    server.server_config(server_config);
    server.task_panic::<TaskPanicHook>();
    server.request_error::<RequestErrorHook>();
    server.request_middleware::<RequestMiddleware>();
    server.request_middleware::<UpgradeMiddleware>();
    server.response_middleware::<ResponseMiddleware>();
    server.route::<RootRoute>("/");
    server.route::<SseRoute>("/sse");
    server.route::<WebsocketRoute>("/websocket");
    server.route::<GetAllRoutes>("/get/all/routes");
    server.route::<DynamicRoute>("/dynamic/{routing}");
    server.route::<DynamicRoute>("/regex/{file:^.*$}");
    let _: Result<(), Server> = SERVER_REF.set(server.clone());
    let server_control_hook_1: ServerControlHook = server.run().await.unwrap_or_default();
    let server_control_hook_2: ServerControlHook = server_control_hook_1.clone();
    spawn(async move {
        sleep(Duration::from_secs(60)).await;
        server_control_hook_2.shutdown().await;
    });
    server_control_hook_1.wait().await;
}

#[tokio::test]
async fn client_server_config_sync_setter_returns_mut_self() {
    let mut server: Server = Server::default();
    let server_config: ServerConfig = ServerConfig::default();
    {
        let returned: &mut Server = server.server_config(server_config);
        assert!(
            returned
                .get_server_config()
                .get_address()
                .contains("0.0.0.0")
        );
    }
}

#[tokio::test]
async fn client_config_from_json_populates_address_field() {
    let json: &str = r#"{"address":"127.0.0.1:9090","nodelay":false,"ttl":null}"#;
    let parsed: ServerConfig = ServerConfig::from_json(json).unwrap();
    let mut server: Server = Server::default();
    {
        let returned: &mut Server = server.config_from_json(json);
        let _ = returned;
    }
    assert_eq!(parsed.get_address(), "127.0.0.1:9090");
    assert_eq!(server.get_server_config().get_address(), "127.0.0.1:9090");
}

#[tokio::test]
async fn client_task_panic_setter_returns_mut_self() {
    let mut server: Server = Server::default();
    {
        let returned: &mut Server = server.task_panic::<TaskPanicHook>();
        assert!(!returned.get_task_panic().is_empty());
    }
    assert_eq!(server.get_task_panic().len(), 1);
}

#[tokio::test]
async fn client_request_error_setter_returns_mut_self() {
    let mut server: Server = Server::default();
    {
        let returned: &mut Server = server.request_error::<RequestErrorHook>();
        assert!(!returned.get_request_error().is_empty());
    }
    assert_eq!(server.get_request_error().len(), 1);
}

#[tokio::test]
async fn client_request_middleware_setter_returns_mut_self() {
    let mut server: Server = Server::default();
    {
        let returned: &mut Server = server.request_middleware::<RequestMiddleware>();
        assert!(!returned.get_request_middleware().is_empty());
    }
    assert_eq!(server.get_request_middleware().len(), 1);
}

#[tokio::test]
async fn client_response_middleware_setter_returns_mut_self() {
    let mut server: Server = Server::default();
    {
        let returned: &mut Server = server.response_middleware::<ResponseMiddleware>();
        assert!(!returned.get_response_middleware().is_empty());
    }
    assert_eq!(server.get_response_middleware().len(), 1);
}

#[tokio::test]
async fn client_format_bind_address_concatenates_host_and_port() {
    let formatted: String = Server::format_bind_address("127.0.0.1", 8080);
    assert_eq!(formatted, "127.0.0.1:8080");
    let formatted_ipv6: String = Server::format_bind_address("::1", 443);
    assert_eq!(formatted_ipv6, "::1:443");
}

#[tokio::test]
async fn client_try_flush_stdout_returns_ok() {
    let result: std::io::Result<()> = Server::try_flush_stdout();
    assert!(result.is_ok());
}

#[tokio::test]
async fn client_flush_stdout_does_not_panic() {
    Server::flush_stdout();
}

#[tokio::test]
async fn client_try_flush_stderr_returns_ok() {
    let result: std::io::Result<()> = Server::try_flush_stderr();
    assert!(result.is_ok());
}

#[tokio::test]
async fn client_flush_stderr_does_not_panic() {
    Server::flush_stderr();
}

#[tokio::test]
async fn client_try_flush_stdout_and_stderr_returns_ok() {
    let result: std::io::Result<()> = Server::try_flush_stdout_and_stderr();
    assert!(result.is_ok());
}

#[tokio::test]
async fn client_flush_stdout_and_stderr_does_not_panic() {
    Server::flush_stdout_and_stderr();
}

#[tokio::test]
async fn client_handle_hook_dispatches_to_correct_handler_list() {
    let mut server: Server = Server::default();
    server.handle_hook(HookType::Route("/", Hook::factory::<TestSendRoute>));
    let route_keys: usize = server.get_route_matcher().get_static_route().len();
    assert!(route_keys >= 1);
    server.handle_hook(HookType::TaskPanic(None, Hook::default_handler));
    assert_eq!(server.get_task_panic().len(), 1);
}

#[tokio::test]
async fn client_route_basic_serial_e2e() {
    let (control, port) = start_server_with(|register: &mut Server| {
        register.route::<RootRoute>("/");
    })
    .await;
    let request: &[u8] = b"GET / HTTP/1.1\r\nHost: 127.0.0.1\r\nConnection: close\r\n\r\n";
    client_write_request(port, request).await;
    control.shutdown().await;
    control.wait().await;
}

#[tokio::test]
async fn client_dynamic_route_serial_e2e() {
    let (control, port) = start_server_with(|register: &mut Server| {
        register.route::<DynamicRoute>("/dynamic/:id");
    })
    .await;
    let request: &[u8] =
        b"GET /dynamic/42 HTTP/1.1\r\nHost: 127.0.0.1\r\nConnection: close\r\n\r\n";
    client_write_request(port, request).await;
    control.shutdown().await;
    control.wait().await;
}

#[tokio::test]
async fn client_regex_route_serial_e2e() {
    let (control, port) = start_server_with(|register: &mut Server| {
        register.route::<WebsocketRoute>("/ws");
    })
    .await;
    let request: &[u8] = b"GET /ws HTTP/1.1\r\nHost: 127.0.0.1\r\nConnection: close\r\n\r\n";
    client_write_request(port, request).await;
    control.shutdown().await;
    control.wait().await;
}

#[tokio::test]
async fn client_request_error_404_serial_e2e() {
    let (control, port) = start_server_with(|register: &mut Server| {
        register.route::<RootRoute>("/");
        register.request_error::<RequestErrorHook>();
    })
    .await;
    let request: &[u8] = b"GET /missing HTTP/1.1\r\nHost: 127.0.0.1\r\nConnection: close\r\n\r\n";
    client_write_request(port, request).await;
    control.shutdown().await;
    control.wait().await;
}

#[tokio::test]
async fn client_request_middleware_serial_e2e() {
    let (control, port) = start_server_with(|register: &mut Server| {
        register.route::<RootRoute>("/");
        register.request_middleware::<RequestMiddleware>();
    })
    .await;
    let request: &[u8] = b"GET / HTTP/1.1\r\nHost: 127.0.0.1\r\nConnection: close\r\n\r\n";
    client_write_request(port, request).await;
    control.shutdown().await;
    control.wait().await;
}

#[tokio::test]
async fn client_response_middleware_serial_e2e() {
    let (control, port) = start_server_with(|register: &mut Server| {
        register.route::<RootRoute>("/");
        register.response_middleware::<ResponseMiddleware>();
    })
    .await;
    let request: &[u8] = b"GET / HTTP/1.1\r\nHost: 127.0.0.1\r\nConnection: close\r\n\r\n";
    client_write_request(port, request).await;
    control.shutdown().await;
    control.wait().await;
}

#[tokio::test]
async fn client_task_panic_handler_serial_e2e() {
    let (control, port) = start_server_with(|register: &mut Server| {
        register.route::<DynamicRoute>("/panic/:msg");
        register.task_panic::<TaskPanicHook>();
    })
    .await;
    let request: &[u8] =
        b"GET /panic/boom HTTP/1.1\r\nHost: 127.0.0.1\r\nConnection: close\r\n\r\n";
    client_write_request(port, request).await;
    control.shutdown().await;
    control.wait().await;
}

#[tokio::test]
async fn client_two_servers_on_distinct_ports_serial_e2e() {
    let (control_a, port_a) = start_server_with(|register: &mut Server| {
        register.route::<RootRoute>("/");
    })
    .await;
    let request: &[u8] = b"GET / HTTP/1.1\r\nHost: 127.0.0.1\r\nConnection: close\r\n\r\n";
    client_write_request(port_a, request).await;
    control_a.shutdown().await;
    control_a.wait().await;
    let (control_b, port_b) = start_server_with(|register: &mut Server| {
        register.route::<RootRoute>("/");
    })
    .await;
    assert_ne!(port_a, port_b);
    client_write_request(port_b, request).await;
    control_b.shutdown().await;
    control_b.wait().await;
}

#[tokio::test]
async fn client_concurrent_servers_on_distinct_ports_e2e() {
    let (control_a, port_a) = start_server_with(|register: &mut Server| {
        register.route::<RootRoute>("/");
    })
    .await;
    let (control_b, port_b) = start_server_with(|register: &mut Server| {
        register.route::<RootRoute>("/");
    })
    .await;
    assert_ne!(port_a, port_b);
    let request: &[u8] = b"GET / HTTP/1.1\r\nHost: 127.0.0.1\r\nConnection: close\r\n\r\n";
    client_write_request(port_a, request).await;
    client_write_request(port_b, request).await;
    control_a.shutdown().await;
    control_b.shutdown().await;
    control_a.wait().await;
    control_b.wait().await;
}
