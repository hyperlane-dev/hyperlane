use http_type::*;
use hyperlane_log::*;
use lombok_macros::*;

#[derive(Clone, Debug, Lombok)]
pub struct ControllerData {
    pub(super) stream: OptionArcTcpStream,
    pub(super) request: Request,
    pub(super) response: Response,
    pub(super) log: Log,
}
