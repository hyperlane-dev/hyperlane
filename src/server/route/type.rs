use crate::server::controller_data::r#type::ControllerData;

pub type RouterFunc = dyn Fn(&mut ControllerData) + Send + Sync + 'static;
pub type RouterFuncBox = Box<RouterFunc>;
