use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use crate::server::controller_data::r#type::ControllerData;

pub type RouterFunc = dyn Fn(&mut ControllerData) + Send + Sync + 'static;
pub type RouterFuncBox = Box<RouterFunc>;
pub type RouterFuncArcLock =
    Arc<RwLock<HashMap<&'static str, Box<dyn Fn(&mut ControllerData) + Send + Sync>>>>;
