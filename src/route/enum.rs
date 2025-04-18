#[derive(Debug, Clone)]
pub(crate) enum RouteSegment {
    Static(String),
    Dynamic(String),
}
