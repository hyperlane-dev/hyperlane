use super::{
    config::r#type::ServerConfig, middleware::r#type::ArcRwLockHashMapMiddlewareFuncBox,
    route::r#type::ArcRwLockHashMapRouterFuncBox, tmp::r#type::Tmp,
};
use http_type::*;
use lombok_macros::*;

#[derive(Clone, Lombok)]
pub struct Server {
    pub(crate) cfg: ArcRwLock<ServerConfig<'static>>,
    pub(crate) router_func: ArcRwLockHashMapRouterFuncBox,
    pub(crate) middleware: ArcRwLockHashMapMiddlewareFuncBox,
    pub(crate) tmp: ArcRwLock<Tmp>,
}
