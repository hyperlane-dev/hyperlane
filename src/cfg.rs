#[test]
fn test_server_basic_usage() {
    use crate::*;
    let mut server: Server = Server::new();
    server.host("0.0.0.0");
    server.port(80);
    server.thread_pool_size(10);
    server.log_dir("./logs");
    server.log_size(1_024_000);
    server.middleware(|controller_data| {
        let request: Request = controller_data.get_request().clone().unwrap();
        controller_data
            .get_log()
            .log_debug(format!("Request => {:?}", request), |log_data| {
                println!("{}", log_data);
                log_data.clone()
            });
    });
    server.router("/", |controller_data| {
        controller_data
            .get_log()
            .log_info("visit path /", |log_data| {
                println!("visit / {}", log_data);
                log_data.clone()
            });
        let mut response: Response = controller_data.get_response().clone().unwrap();
        let body: Vec<u8> = "404 Not Found".as_bytes().to_vec();
        let stream: ControllerDataStream = controller_data.get_stream().clone().unwrap();
        let res: ResponseResult = response
            .set_body(body)
            .set_status_code(404)
            .set_header("server", "hyperlane")
            .send(&stream);
        controller_data
            .get_log()
            .log_info(format!("Response => {:?}", res), |log_data| {
                println!("{}", log_data);
                log_data.clone()
            });
    });
    server.router("/hello", |controller_data| {
        controller_data
            .get_log()
            .log_info("visit path /", |log_data| {
                println!("visit / {}", log_data);
                log_data.clone()
            });
        let mut response: Response = controller_data.get_response().clone().unwrap();
        let body: Vec<u8> = "hello world!".as_bytes().to_vec();
        let stream: ControllerDataStream = controller_data.get_stream().clone().unwrap();
        let res: ResponseResult = response
            .set_body(body)
            .set_status_code(200)
            .set_header("server", "hyperlane")
            .send(&stream);
        controller_data
            .get_log()
            .log_info(format!("Response => {:?}", res), |log_data| {
                println!("{}", log_data);
                log_data.clone()
            });
    });
    server.listen();
}
