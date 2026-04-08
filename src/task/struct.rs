use crate::*;

/// Task manager for handling async tasks across worker threads.
///
/// This structure manages a pool of task senders distributed across
/// multiple worker threads, enabling efficient round-robin task scheduling
/// with event-driven wake-up mechanism.
#[derive(Clone, CustomDebug, Data, DisplayDebug)]
pub struct Task {
    /// Pool of unbounded senders for distributing tasks to worker threads.
    #[get(pub(super))]
    #[get_mut(pub(super))]
    #[set(pub(super))]
    pub(super) pool: Vec<UnboundedSender<AsyncTask>>,
    /// Atomic counter for round-robin task distribution across workers.
    #[get(pub(super))]
    #[get_mut(pub(super))]
    #[set(pub(super))]
    pub(super) counter: Arc<AtomicUsize>,
    /// Flag indicating whether the task pool should shut down.
    #[get(pub(super))]
    #[get_mut(pub(super))]
    #[set(pub(super))]
    pub(super) shutdown: Arc<AtomicBool>,
    /// Notification handles for precise wake-up of specific workers.
    #[get(pub(super))]
    #[get_mut(pub(super))]
    #[set(pub(super))]
    pub(super) notifies: Vec<Arc<Notify>>,
}
