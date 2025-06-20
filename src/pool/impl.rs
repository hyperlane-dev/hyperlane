use crate::*;

impl<T> ObjectPool<T> {
    pub(crate) fn new(factory: fn() -> T, max_size: usize) -> Self {
        Self {
            pool: RwLock::new(Vec::with_capacity(max_size.min(16))),
            factory,
        }
    }

    pub(crate) fn get(&self) -> T {
        if let Ok(mut pool) = self.pool.try_write() {
            if let Some(item) = pool.pop() {
                return item;
            }
        }
        (self.factory)()
    }
}
