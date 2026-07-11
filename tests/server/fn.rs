use crate::*;

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
    let _ = SERVER_REF.set(server.clone());
    let server_control_hook_1: ServerControlHook = server.run().await.unwrap_or_default();
    let server_control_hook_2: ServerControlHook = server_control_hook_1.clone();
    spawn(async move {
        sleep(Duration::from_secs(60)).await;
        server_control_hook_2.shutdown().await;
    });
    server_control_hook_1.wait().await;
}
