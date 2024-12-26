use http_type::*;
use lombok_macros::*;
use std::{net::TcpStream, sync::Arc};

#[derive(Clone, Debug, Lombok)]
pub struct ControllerData {
    pub(super) stream: Option<Arc<TcpStream>>,
    pub(super) request: Option<Request>,
    pub(super) response: Option<Response>,
}
