use crate::*;

#[derive(Clone, Data)]
pub(crate) struct ServerConfig {
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(super) host: String,
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(super) port: usize,
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(super) ws_buffer_size: usize,
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(super) http_buffer_size: usize,
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(super) nodelay: bool,
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(super) linger: OptionDuration,
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(super) ttl: OptionU32,
    #[set(skip)]
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    pub(super) disable_http_handler: HashSetXxHash3_64<String>,
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(skip)]
    pub(super) disable_ws_handler: HashSetXxHash3_64<String>,
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(skip)]
    pub(super) route_matcher: RouteMatcher,
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(super) error_hook: ArcErrorHandlerSendSync,
}
