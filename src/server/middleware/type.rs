use crate::*;

pub type ArcRwLockHashMapMiddlewareFuncBox = ArcRwLock<VecBoxDynFunc>;
pub type AsyncArcRwLockHashMapMiddlewareFuncBox = AsyncArcRwLock<VecBoxAsyncFunc>;
pub type MiddlewareArcLock = ArcRwLock<Vec<Box<dyn Fn(ArcRwLock<ControllerData>) + Send + Sync>>>;
