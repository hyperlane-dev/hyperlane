use crate::*;

pub type RwLockWriteControllerData<'a> = RwLockWriteGuard<'a, InnerControllerData>;
pub type RwLockReadControllerData<'a> = RwLockReadGuard<'a, InnerControllerData>;

#[derive(Clone, Debug, Lombok)]
pub struct InnerControllerData {
    pub(super) stream: OptionArcRwLockStream,
    pub(super) request: Request,
    pub(super) response: Response,
    pub(super) log: Log,
}

#[derive(Clone, Debug)]
pub struct ControllerData(pub(super) ArcRwLock<InnerControllerData>);
