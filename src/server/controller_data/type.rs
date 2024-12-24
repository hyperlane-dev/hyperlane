use http_type::*;
use std::{net::TcpStream, sync::Arc};

#[derive(Debug)]
pub struct ControllerData {
    pub(crate) stream: Arc<TcpStream>,
    pub(crate) request: Request,
    pub(crate) response: Response,
}
