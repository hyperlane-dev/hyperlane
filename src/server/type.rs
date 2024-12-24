use super::{config::r#type::ServerConfig, route::r#type::RouterFuncBox, tmp::r#type::Tmp};
use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

#[allow(dead_code)]
pub struct Server {
    pub(crate) cfg: ServerConfig<'static>,
    pub(crate) router_func: Arc<RwLock<HashMap<&'static str, RouterFuncBox>>>,
    pub(crate) middleware: Arc<RwLock<Vec<RouterFuncBox>>>,
    pub(crate) tmp: Tmp,
}
