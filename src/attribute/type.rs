use crate::*;

/// A type alias for a HashMap storing string keys and thread-safe, shareable values.
///
/// This type is used for storing attributes that can be safely shared across threads.
pub type HashMapArcAnySendSync = HashMap<String, ArcAnySendSync>;
