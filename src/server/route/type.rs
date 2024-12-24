use crate::server::controller_data::r#type::ControllerData;

pub type RouterFunc = dyn Fn(ControllerData);
pub type RouterFuncBox = Box<RouterFunc>;
