use super::*;

/// A type alias for a boxed `Any` trait object.
///
/// This represents a dynamically dispatched trait object that owns its data.
pub type BoxAny = Box<dyn Any>;

/// A type alias for an `Rc` wrapped `Any` trait object.
///
/// This represents a reference-counted trait object that can be shared across threads.
pub type RcAny = Rc<dyn Any>;

/// A type alias for an `Arc` wrapped `Any` trait object.
///
/// This represents an atomically reference-counted trait object that can be shared across threads.
pub type ArcAny = Arc<dyn Any>;

/// A type alias for a boxed `Any + Send` trait object.
///
/// This represents a dynamically dispatched trait object that owns its data and is safe to send across threads.
pub type BoxAnySend = Box<dyn Any + Send>;

/// A type alias for an `Rc` wrapped `Any + Send` trait object.
///
/// This represents a reference-counted trait object that can be shared across threads and is safe to send.
pub type RcAnySend = Rc<dyn Any + Send>;

/// A type alias for an `Arc` wrapped `Any + Send` trait object.
///
/// This represents an atomically reference-counted trait object that can be shared across threads and is safe to send.
pub type ArcAnySend = Arc<dyn Any + Send>;

/// A type alias for a boxed `Any + Sync` trait object.
///
/// This represents a dynamically dispatched trait object that owns its data and is safe to share across threads.
pub type BoxAnySync = Box<dyn Any + Sync>;

/// A type alias for an `Rc` wrapped `Any + Sync` trait object.
///
/// This represents a reference-counted trait object that can be shared across threads and is safe to share.
pub type RcAnySync = Rc<dyn Any + Sync>;

/// A type alias for an `Arc` wrapped `Any + Sync` trait object.
///
/// This represents an atomically reference-counted trait object that can be shared across threads and is safe to share.
pub type ArcAnySync = Arc<dyn Any + Sync>;

/// A type alias for a boxed `Any + Send + Sync` trait object.
///
/// This represents a dynamically dispatched trait object that owns its data and is safe to send and share across threads.
pub type BoxAnySendSync = Box<dyn Any + Send + Sync>;

/// A type alias for an `Rc` wrapped `Any + Send + Sync` trait object.
///
/// This represents a reference-counted trait object that can be shared across threads and is safe to send and share.
pub type RcAnySendSync = Rc<dyn Any + Send + Sync>;

/// A type alias for an `Arc` wrapped `Any + Send + Sync` trait object.
///
/// This represents an atomically reference-counted trait object that can be shared across threads and is safe to send and share.
pub type ArcAnySendSync = Arc<dyn Any + Send + Sync>;
