use super::*;

/// Thread-safe predicate deciding whether an incoming `Origin` header value is permitted.
pub type OriginMatcher = Arc<dyn Fn(&str) -> bool + Send + Sync>;
