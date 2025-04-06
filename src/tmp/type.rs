use crate::*;

pub(crate) type ArcRwLockTmp = ArcRwLock<Tmp>;

#[derive(Clone, Lombok, Default)]
pub(crate) struct Tmp {
    pub(super) log: Log,
}
