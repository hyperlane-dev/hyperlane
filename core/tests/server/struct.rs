use super::*;

pub(crate) struct TestSendRoute;

pub(crate) struct TaskPanicHook {
    pub(crate) response_body: String,
    pub(crate) content_type: String,
}

pub(crate) struct RequestErrorHook {
    pub(crate) response_status_code: ResponseStatusCode,
    pub(crate) response_body: String,
}

pub(crate) struct RequestMiddleware {
    pub(crate) socket_addr: String,
}

pub(crate) struct UpgradeMiddleware;

pub(crate) struct ResponseMiddleware;

pub(crate) struct RootRoute {
    pub(crate) response_body: String,
    pub(crate) cookie1: String,
    pub(crate) cookie2: String,
}

pub(crate) struct SseRoute;

pub(crate) struct WebsocketRoute;

pub(crate) struct DynamicRoute {
    pub(crate) params: RouteParams,
}

pub(crate) struct GetAllRoutes;
