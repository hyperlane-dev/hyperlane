use http_type::*;
use std::{net::TcpStream, sync::Arc};

#[derive(Debug)]
pub struct ControllerData<'a> {
    pub(crate) stream: Arc<TcpStream>,
    pub(crate) request: Request<'a>,
    pub(crate) response: Response<'a>,
}
