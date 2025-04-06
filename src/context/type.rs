use crate::*;

pub type RwLockWriteInnerContext<'a> = RwLockWriteGuard<'a, InnerContext>;
pub type RwLockReadInnerContext<'a> = RwLockReadGuard<'a, InnerContext>;
pub type HashMapArcAnySendSync = HashMap<String, ArcAnySendSync>;

#[derive(Clone, Lombok, Default)]
pub struct InnerContext {
    stream: OptionArcRwLockStream,
    request: Request,
    response: Response,
    log: Log,
    attribute: HashMapArcAnySendSync,
    route_params: RouteParams,
}

#[derive(Clone, Default)]
pub struct Context(pub(super) ArcRwLock<InnerContext>);
