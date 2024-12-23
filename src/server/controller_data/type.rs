use crate::response::r#type::Response;
use std::net::TcpStream;

#[derive(Debug)]
pub struct ControllerData<'a> {
    pub(crate) stream: &'a TcpStream,
    pub(crate) response: Response<'a>,
}
