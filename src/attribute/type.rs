use crate::*;

/// A type alias for a thread-safe attribute storage.
///
/// This type is used for storing attributes that can be safely shared across threads.
pub type ThreadSafeAttributeStore = HashMap<String, ArcAnySendSync>;
