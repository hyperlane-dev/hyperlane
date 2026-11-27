use super::*;

/// Thread-safe shared reference to a pluggable [`SecurityRule`].
pub type SharedSecurityRule = Arc<dyn SecurityRule>;
