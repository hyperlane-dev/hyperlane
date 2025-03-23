use crate::*;

pub type RwLockWriteInnerControllerData<'a> = RwLockWriteGuard<'a, InnerControllerData>;
pub type RwLockReadInnerControllerData<'a> = RwLockReadGuard<'a, InnerControllerData>;

#[derive(Clone, Lombok, Default)]
pub struct InnerControllerData {
    stream: OptionArcRwLockStream,
    request: Request,
    response: Response,
    log: Log,
}

#[derive(Clone, Default)]
pub struct ControllerData(pub(super) ArcRwLock<InnerControllerData>);
