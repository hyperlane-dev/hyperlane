use crate::*;

pub type ArcRwLockTmp = ArcRwLock<Tmp>;

#[derive(Clone, Lombok, Default)]
pub struct Tmp {
    pub(super) log: Log,
}
