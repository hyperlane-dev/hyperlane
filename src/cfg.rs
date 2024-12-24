#[test]
fn test_server() {
    use crate::*;
    let mut server: Server<'_> = Server::new();
    server.host("0.0.0.0");
    server.port(80);
    server.router("/", |controller_data| {
        let mut response: Response<'_> = controller_data.response.clone();
        let body: Vec<u8> = "404 Not Found".as_bytes().to_vec();
        let stream: &std::net::TcpStream = controller_data.stream();
        let res: Result<(), ResponseError> = response
            .body(body)
            .status_code(404)
            .header("server", "hyperlane")
            .send(stream);
        println!("{:?}", res);
    });
    server.router("/hello", |controller_data| {
        let mut response: Response<'_> = controller_data.response.clone();
        let body: Vec<u8> = "hello world!".as_bytes().to_vec();
        let stream: &std::net::TcpStream = controller_data.stream();
        let res: Result<(), ResponseError> = response
            .body(body)
            .status_code(200)
            .header("server", "hyperlane")
            .send(stream);
        println!("{:?}", res);
    });
    server.listen();
}
