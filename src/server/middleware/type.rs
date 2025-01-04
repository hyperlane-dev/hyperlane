use crate::server::route::r#type::VecRouterFuncBox;
use crate::*;

pub type ArcRwLockHashMapMiddlewareFuncBox = ArcRwLock<VecRouterFuncBox>;
pub type MiddlewareArcLock = ArcRwLock<Vec<Box<dyn Fn(&mut ControllerData) + Send + Sync>>>;
