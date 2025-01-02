use http_type::*;
use hyperlane_log::*;
use lombok_macros::*;
use std::{net::TcpStream, sync::Arc};

pub type ControllerDataStream = Arc<TcpStream>;
pub type ControllerDataStreamOpt = Option<ControllerDataStream>;
pub type ControllerDataRequest = Request;
pub type ControllerDataResponse = Response;

#[derive(Clone, Debug, Lombok)]
pub struct ControllerData {
    pub(super) stream: ControllerDataStreamOpt,
    pub(super) request: ControllerDataRequest,
    pub(super) response: ControllerDataResponse,
    pub(super) log: Log,
}
