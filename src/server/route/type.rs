use crate::server::controller_data::r#type::ControllerData;
use http_type::ArcRwLock;
use std::collections::HashMap;

pub type RouterFunc = dyn Fn(&mut ControllerData) + Send + Sync + 'static;
pub type RouterFuncBox = Box<RouterFunc>;
pub type RouterFuncArcLock =
    ArcRwLock<HashMap<&'static str, Box<dyn Fn(&mut ControllerData) + Send + Sync>>>;
