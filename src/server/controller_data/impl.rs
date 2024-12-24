use http_type::*;
use std::{net::TcpStream, sync::Arc};

use super::r#type::ControllerData;

impl ControllerData {
    pub fn stream(&self) -> Arc<TcpStream> {
        self.stream.clone()
    }

    pub fn response(&self) -> Response {
        self.response.clone()
    }

    pub fn request(&self) -> Request {
        self.request.clone()
    }
}
