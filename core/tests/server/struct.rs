use super::*;

pub(crate) struct TestSendRoute;

pub(crate) struct TaskPanicHook {
    pub(super) response_body: String,
    pub(super) content_type: String,
}

pub(crate) struct RequestErrorHook {
    pub(super) response_status_code: ResponseStatusCode,
    pub(super) response_body: String,
}

pub(crate) struct RequestMiddleware {
    pub(crate) socket_addr: String,
}

pub(crate) struct UpgradeMiddleware;

pub(crate) struct ResponseMiddleware;

pub(crate) struct RootRoute {
    pub(super) response_body: String,
    pub(super) cookie1: String,
    pub(super) cookie2: String,
}

pub(crate) struct SseRoute;

pub(crate) struct WebsocketRoute;

pub(crate) struct DynamicRoute {
    pub(super) params: RouteParams,
}

pub(crate) struct GetAllRoutes;
