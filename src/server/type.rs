use super::{config::r#type::ServerConfig, route::r#type::RouterFuncBox};
use std::collections::HashMap;

#[allow(dead_code)]
pub struct Server<'a> {
    pub(crate) cfg: ServerConfig<'a>,
    pub(crate) router_func: HashMap<&'a str, RouterFuncBox>,
    pub(crate) static_dir: Option<&'a str>,
    pub(crate) middleware: Vec<RouterFuncBox>,
}
