use http_type::*;
use std::net::TcpStream;

#[derive(Debug)]
pub struct ControllerData<'a> {
    pub(crate) stream: &'a TcpStream,
    pub(crate) request: Request<'a>,
    pub(crate) response: Response<'a>,
}
