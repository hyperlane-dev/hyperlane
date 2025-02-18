#[tokio::test]
async fn test_server_basic_usage() {
    use crate::*;

    async fn test_middleware(arc_lock_controller_data: ArcRwLockControllerData) {
        let mut controller_data: RwLockWriteControllerData =
            get_rw_lock_write_controller_data(&arc_lock_controller_data).await;
        let response: &mut Response = controller_data.get_mut_response();
        response
            .set_header(SERVER, "hyperlane")
            .set_header(CONNECTION, CONNECTION_KEEP_ALIVE);
    }

    async fn root_router(arc_lock_controller_data: ArcRwLockControllerData) {
        let send_res: ResponseResult =
            send_response(&arc_lock_controller_data, 200, "hello hyperlane => /").await;
        let controller_data: ControllerData = get_controller_data(&arc_lock_controller_data).await;
        controller_data.get_log().info(
            format!("Response result => {:?}", send_res),
            log_debug_format_handler,
        );
    }

    async fn panic_route(_controller_data: ArcRwLock<ControllerData>) {
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
