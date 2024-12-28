use crate::server::log::r#type::Log;
use http_type::*;
use lombok_macros::*;
use std::{net::TcpStream, sync::Arc};

pub type ControllerDataStream = Arc<TcpStream>;
pub type ControllerDataStreamOpt = Option<ControllerDataStream>;
pub type ControllerDataRequest = Request;
pub type ControllerDataRequestOpt = Option<ControllerDataRequest>;
pub type ControllerDataResponse = Response;
pub type ControllerDataResponseOpt = Option<ControllerDataResponse>;

#[derive(Clone, Debug, Lombok)]
pub struct ControllerData {
    pub(super) stream: ControllerDataStreamOpt,
    pub(super) request: ControllerDataRequestOpt,
    pub(super) response: ControllerDataResponseOpt,
    pub(super) log: Log,
}
