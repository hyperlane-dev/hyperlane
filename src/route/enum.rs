use crate::*;

#[derive(CustomDebug, Clone)]
pub(crate) enum RouteSegment {
    Static(String),
    Dynamic(String),
    Regex(String, Regex),
}
