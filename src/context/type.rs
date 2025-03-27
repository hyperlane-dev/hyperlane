use crate::*;

pub type RwLockWriteInnerContext<'a> = RwLockWriteGuard<'a, InnerContext>;
pub type RwLockReadInnerContext<'a> = RwLockReadGuard<'a, InnerContext>;

#[derive(Clone, Lombok, Default)]
pub struct InnerContext {
    stream: OptionArcRwLockStream,
    request: Request,
    response: Response,
    log: Log,
}

#[derive(Clone, Default)]
pub struct Context(pub(super) ArcRwLock<InnerContext>);
