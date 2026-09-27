use super::*;

#[tokio::test]
async fn main() {
    let mut server: Server = Server::default();
    let request_config: RequestConfig = RequestConfig::low_security();
    server.request_config(request_config);
    server.task_panic::<TaskPanicHook>();
    server.request_error::<RequestErrorHook>();
    server.request_middleware::<RequestMiddleware>();
    server.request_middleware::<UpgradeHook>();
    server.route::<GroupChat>("/{group_name}");
    server.route::<PrivateChat>("/{my_name}/{your_name}");
    let server_control_hook_1: ServerControlHook = server.run().await.unwrap_or_default();
    let server_control_hook_2: ServerControlHook = server_control_hook_1.clone();
    spawn(async move {
        sleep(std::time::Duration::from_secs(60)).await;
        server_control_hook_2.shutdown().await;
    });
    server_control_hook_1.wait().await;
}
