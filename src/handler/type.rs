use crate::*;

pub type ArcFnPinBoxSendSync = Arc<dyn FnPinBoxSendSync>;
pub type OptionArcFnPinBoxSendSync = Option<ArcFnPinBoxSendSync>;
pub type VecArcFnPinBoxSendSync = Vec<ArcFnPinBoxSendSync>;
pub type ArcErrorHandlerSendSync = Arc<dyn ErrorHandler + Send + Sync + 'static>;
pub type PinBoxFutureSendStatic = Pin<Box<(dyn Future<Output = ()> + Send + 'static)>>;
pub type ArcRwLockVecArcFnPinBoxSendSync = ArcRwLock<VecArcFnPinBoxSendSync>;
pub type RwLockReadGuardVecArcFnPinBoxSendSync<'a> = RwLockReadGuard<'a, VecArcFnPinBoxSendSync>;
