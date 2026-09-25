use super::*;

/// Task manager for handling async tasks across worker threads.
///
/// This structure manages a pool of task senders distributed across
/// multiple worker threads, enabling efficient round-robin task scheduling
/// with event-driven wake-up mechanism.
#[derive(Clone, CustomDebug, Data, DisplayDebug)]
pub struct Task {
    /// Pool of unbounded senders for distributing tasks to worker threads.
    #[get(pub)]
    #[get_mut(pub)]
    #[set(pub)]
    pub pool: Vec<UnboundedSender<AsyncTask>>,
    /// Atomic counter for round-robin task distribution across workers.
    #[get(pub)]
    #[get_mut(pub)]
    #[set(pub)]
    pub counter: Arc<AtomicUsize>,
    /// Flag indicating whether the task pool should shut down.
    #[get(pub)]
    #[get_mut(pub)]
    #[set(pub)]
    pub shutdown: Arc<AtomicBool>,
    /// Notification handles for precise wake-up of specific workers.
    #[get(pub)]
    #[get_mut(pub)]
    #[set(pub)]
    pub notifies: Vec<Arc<Notify>>,
}
