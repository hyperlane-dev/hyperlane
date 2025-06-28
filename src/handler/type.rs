use crate::*;

pub type ArcFunc = Arc<dyn Func>;
pub type OptionArcFunc = Option<ArcFunc>;
pub type VecArcFunc = Vec<ArcFunc>;
pub type ArcErrorHandle = Arc<dyn ErrorHandle + Send + Sync + 'static>;
pub type PinBoxFutureSend = Pin<Box<(dyn Future<Output = ()> + Send + 'static)>>;
pub type ArcRwLockVecArcFunc = ArcRwLock<VecArcFunc>;
pub type RwLockReadGuardVecArcFunc<'a> = RwLockReadGuard<'a, Vec<Arc<dyn Func>>>;
