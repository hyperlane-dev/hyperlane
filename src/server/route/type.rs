use crate::*;

pub type HashMapRouterFuncBox = HashMap<&'static str, BoxDynFunc>;
pub type AsyncHashMapRouterFuncBox = HashMap<&'static str, BoxAsyncFunc>;

pub type ArcRwLockHashMapRouterFuncBox = ArcRwLock<HashMapRouterFuncBox>;
pub type AsyncArcRwLockHashMapRouterFuncBox = AsyncArcRwLock<AsyncHashMapRouterFuncBox>;

pub type RouterFuncArcLock =
    ArcRwLock<HashMap<&'static str, Box<dyn Fn(ArcRwLock<ControllerData>) + Send + Sync>>>;
