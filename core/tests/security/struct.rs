#[derive(Clone, Copy, Default)]
pub(crate) struct RejectGuardMiddleware;

#[derive(Clone, Copy, Default)]
pub(crate) struct SetAttrRoute;

#[derive(Clone, Copy, Default)]
pub(crate) struct CheckIsolationRoute;

#[derive(Clone, Copy, Default)]
pub(crate) struct PanicRoute;

#[derive(Clone, Copy, Default)]
pub(crate) struct RegexEchoRoute;

#[derive(Clone, Copy, Default)]
pub(crate) struct AdminRoute;

#[derive(Clone, Copy, Default)]
pub(crate) struct SecuritySendMiddleware;
