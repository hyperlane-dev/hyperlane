use crate::{server::controller_data::r#type::ControllerData, Server};

pub type RouterFunc = dyn for<'a> Fn(&'a Server<'a>, &'a mut ControllerData<'a>);
pub type RouterFuncBox = Box<RouterFunc>;
