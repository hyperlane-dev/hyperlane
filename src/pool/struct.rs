use crate::*;

pub(crate) struct ObjectPool<T> {
    pub(super) pool: RwLock<Vec<T>>,
    pub(super) factory: fn() -> T,
}
