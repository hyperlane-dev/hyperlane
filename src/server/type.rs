use super::{config::r#type::ServerConfig, route::r#type::RouterFuncBox, tmp::r#type::Tmp};
use http_type::ArcRwLock;
use lombok_macros::Lombok;
use std::collections::HashMap;

#[derive(Clone, Lombok)]
pub struct Server {
    pub(crate) cfg: ArcRwLock<ServerConfig<'static>>,
    pub(crate) router_func: ArcRwLock<HashMap<&'static str, RouterFuncBox>>,
    pub(crate) middleware: ArcRwLock<Vec<RouterFuncBox>>,
    pub(crate) tmp: ArcRwLock<Tmp>,
}
