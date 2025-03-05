#[tokio::test]
async fn test_server_basic_usage() {
    use crate::*;

    async fn test_middleware(controller_data: ControllerData) {
        let socket_addr: String = controller_data
            .get_socket_addr()
            .await
            .unwrap_or(DEFAULT_SOCKET_ADDR)
            .to_string();
        controller_data
            .set_response_header(SERVER, "hyperlane")
            .await
            .set_response_header(CONNECTION, CONNECTION_KEEP_ALIVE)
            .await
            .set_response_header("SocketAddr", socket_addr)
            .await;
    }

    async fn root_router(controller_data: ControllerData) {
        let send_res: ResponseResult = controller_data
            .send_response(200, "hello hyperlane => /")
            .await;
        controller_data
            .log_info(
                format!("Response result => {:?}", send_res),
                log_debug_format_handler,
            )
            .await;
    }

    async fn panic_route(_controller_data: ControllerData) {
        panic!("test panic");
    }

    async fn run_server() {
        let mut server: Server = Server::new();
        server.host("0.0.0.0").await;
        server.port(60000).await;
        server.log_dir("./logs").await;
        server.log_size(100_024_000).await;
        server.log_interval_millis(1000).await;
        server.middleware(test_middleware).await;
        server.router("/", root_router).await;
        server.router("/panic", panic_route).await;
        let test_string: String = "test".to_owned();
        server
            .router(
                "/test/func",
                async_func!(test_string, |data| {
                    println_success!(test_string);
                    println_success!(format!("{:?}", data));
                }),
            )
            .await;
        server.listen().await;
    }

    recoverable_spawn::r#async::recoverable_spawn(run_server);
    std::thread::sleep(std::time::Duration::from_secs(60));
}
