use crate::*;

#[derive(Debug, Clone)]
pub(crate) enum RouteSegment {
    Static(String),
    Dynamic(String),
    Regex(String, Regex),
}
