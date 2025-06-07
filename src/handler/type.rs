use crate::*;

pub type ArcFunc = Arc<dyn Func>;
pub type VecArcFunc = Vec<ArcFunc>;
pub type ArcErrorHandle = Arc<dyn ErrorHandle + Send + Sync + 'static>;
pub type PinBoxFutureSend = Pin<Box<(dyn Future<Output = ()> + Send + 'static)>>;
pub type ArcRwLockVecArcFunc = ArcRwLock<VecArcFunc>;
