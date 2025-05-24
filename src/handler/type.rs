use crate::*;

pub(crate) type ArcFunc = Arc<dyn Func>;
pub(crate) type VecArcFunc = Vec<ArcFunc>;
pub(crate) type ArcErrorHandle = Arc<dyn ErrorHandle + Send + Sync + 'static>;
pub(crate) type PinBoxFutureSend = Pin<Box<(dyn Future<Output = ()> + Send + 'static)>>;
