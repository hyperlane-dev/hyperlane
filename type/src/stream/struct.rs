use super::*;

#[derive(CustomDebug, Data, DisplayDebug, New)]
pub struct Stream {
    #[get_mut(pub(super))]
    #[set(pub(super))]
    pub(super) stream: TcpStream,
    #[get_mut(pub(super))]
    #[set(pub(super))]
    pub(super) request_config: RequestConfig,
    #[get(type(copy))]
    #[get_mut(pub(super))]
    pub(super) closed: bool,
}
