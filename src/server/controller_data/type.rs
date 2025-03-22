use crate::*;

pub type RwLockWriteControllerData<'a> = RwLockWriteGuard<'a, InnerControllerData>;
pub type RwLockReadControllerData<'a> = RwLockReadGuard<'a, InnerControllerData>;

#[derive(Clone, Debug, Lombok, Default)]
pub struct InnerControllerData {
    stream: OptionArcRwLockStream,
    request: Request,
    response: Response,
    log: Log,
}

#[derive(Clone, Debug)]
pub struct ControllerData(pub(super) ArcRwLock<InnerControllerData>);
