use crate::*;

pub type RwLockWriteControllerData<'a> = RwLockWriteGuard<'a, ControllerData>;
pub type RwLockReadControllerData<'a> = RwLockReadGuard<'a, ControllerData>;

#[derive(Clone, Debug, Lombok)]
pub struct ControllerData {
    pub(super) stream: OptionArcRwLockStream,
    pub(super) request: Request,
    pub(super) response: Response,
    pub(super) log: Log,
}

#[derive(Clone, Debug)]
pub struct ArcRwLockControllerData(pub(super) ArcRwLock<ControllerData>);
