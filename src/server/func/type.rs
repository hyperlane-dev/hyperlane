use crate::*;

pub type DynFunc = dyn Fn(ArcRwLock<ControllerData>) + Send + Sync + 'static;
pub type BoxDynFunc = Box<DynFunc>;
pub type BoxAsyncFunc = Box<dyn AsyncFunc>;

pub type VecBoxDynFunc = Vec<BoxDynFunc>;
pub type VecBoxAsyncFunc = Vec<BoxAsyncFunc>;
