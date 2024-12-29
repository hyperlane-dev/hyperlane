#[test]
fn test_server_basic_usage() {
    use crate::*;
    fn println(data: &str) {
        OutputListBuilder::new()
            .add(
                OutputBuilder::new()
                    .text(&current_time())
                    .blod(true)
                    .bg_color(ColorType::Use(Color::Yellow))
                    .color(ColorType::Rgb(255, 255, 255))
                    .build(),
            )
            .add(
                OutputBuilder::new()
                    .text(COLON_SPACE_SYMBOL)
                    .blod(true)
                    .bg_color(ColorType::Use(Color::Magenta))
                    .color(ColorType::Rgb(255, 255, 255))
                    .build(),
            )
            .add(
                OutputBuilder::new()
                    .text(data)
                    .blod(true)
                    .bg_color(ColorType::Use(Color::Green))
                    .color(ColorType::Rgb(255, 255, 255))
                    .endl(true)
                    .build(),
            )
            .run();
    }
    fn common_log(log_data: &String) -> String {
        println(&log_data);
        let write_data: String = format!("{}: {}\n", current_time(), log_data);
        write_data.clone()
    }
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
            .log_debug(format!("Request => \n{:#?}", request), common_log);
    });
    server.router("/", |controller_data| {
        controller_data
            .get_log()
            .log_info("visit path /", common_log);
        let mut response: Response = controller_data.get_response().clone().unwrap();
        let body: Vec<u8> = "404 Not Found".as_bytes().to_vec();
        let stream: ControllerDataStream = controller_data.get_stream().clone().unwrap();
        let res: ResponseResult = response
            .set_body(body)
            .set_status_code(404)
            .set_header("server", "hyperlane")
            .send(&stream);
        controller_data.get_log().log_info(
            format!("Response => {:?}", String::from_utf8_lossy(&res.unwrap())),
            common_log,
        );
    });
    server.router("/hello", |controller_data| {
        controller_data
            .get_log()
            .log_info("visit path /hello", common_log);
        let mut response: Response = controller_data.get_response().clone().unwrap();
        let body: Vec<u8> = "hello world!".as_bytes().to_vec();
        let stream: ControllerDataStream = controller_data.get_stream().clone().unwrap();
        let res: ResponseResult = response
            .set_body(body)
            .set_status_code(200)
            .set_header("server", "hyperlane")
            .send(&stream);
        controller_data.get_log().log_info(
            format!("Response => {:?}", String::from_utf8_lossy(&res.unwrap())),
            common_log,
        );
    });
    server.router("/panic", |_controller_data| {
        panic!("test panic");
    });
    server.listen();
}
