use crate::server::{controller_data::r#type::ControllerData, route::r#type::VecRouterFuncBox};
use http_type::*;

pub type ArcRwLockHashMapMiddlewareFuncBox = ArcRwLock<VecRouterFuncBox>;
pub type MiddlewareArcLock = ArcRwLock<Vec<Box<dyn Fn(&mut ControllerData) + Send + Sync>>>;
