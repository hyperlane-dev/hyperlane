use std::{
    borrow::{Borrow, BorrowMut},
    io::Write,
};

#[test]
fn test_server() {
    use crate::*;
    Server::new()
        .host("0.0.0.0")
        .port(80)
        .router("/", |server, controller_data| {
            println!("sqs");
            let mut response = controller_data.response.clone();
            let body = "sqs".as_bytes().to_vec();
            let res = response.body(body).header("sqs", "hhh").build();
            if let Err(e) = controller_data.stream.write_all(&res) {
                println!("Failed to respond to preflight request: {}", e);
            } else {
                println!("ok");
            }
        })
        .listen();
}
