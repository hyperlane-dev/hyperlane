use crate::server::controller_data::r#type::ControllerData;
use http_type::*;
use std::collections::HashMap;

pub type RouterFunc = dyn Fn(&mut ControllerData) + Send + Sync + 'static;
pub type RouterFuncBox = Box<RouterFunc>;
pub type VecRouterFuncBox = Vec<RouterFuncBox>;
pub type ArcRwLockHashMapRouterFuncBox = ArcRwLock<HashMap<&'static str, RouterFuncBox>>;
pub type RouterFuncArcLock =
    ArcRwLock<HashMap<&'static str, Box<dyn Fn(&mut ControllerData) + Send + Sync>>>;
