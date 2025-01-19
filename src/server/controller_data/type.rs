use crate::*;
use std::sync::{RwLockReadGuard, RwLockWriteGuard};

pub type ArcRwLockControllerData = ArcRwLock<ControllerData>;
pub type RwLockWriteControllerData<'a> = RwLockWriteGuard<'a, ControllerData>;
pub type RwLockReadControllerData<'a> = RwLockReadGuard<'a, ControllerData>;

#[derive(Clone, Debug, Lombok)]
pub struct ControllerData {
    pub(super) stream: OptionArcTcpStream,
    pub(super) request: Request,
    pub(super) response: Response,
    pub(super) log: Log,
}
