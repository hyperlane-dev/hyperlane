use http_type::*;
use std::net::TcpStream;

use super::r#type::ControllerData;

impl<'a> ControllerData<'a> {
    pub fn stream(&self) -> &TcpStream {
        self.stream
    }

    pub fn response(&self) -> Response {
        self.response.clone()
    }

    pub fn request(&self) -> Request {
        self.request.clone()
    }
}
