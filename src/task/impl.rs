use crate::*;

/// Creates a default Task instance.
impl Default for Task {
    /// Creates a default Task instance.
    ///
    /// # Returns
    ///
    /// - `Self`: The default instance.
    #[inline(always)]
    fn default() -> Self {
        let worker_count: usize = Handle::try_current()
            .map(|handle: Handle| handle.metrics().num_workers())
            .unwrap_or_default()
            .max(1);
        Self::new(worker_count)
    }
}

/// Automatically shuts down the task pool when the Task instance is dropped.
impl Drop for Task {
    /// Shuts down the task pool when the Task instance is dropped.
    #[inline(always)]
    fn drop(&mut self) {
        self.shutdown();
    }
}

impl Task {
    /// Creates a new Task instance.
    ///
    /// # Arguments
    ///
    /// - `usize`: The number of worker threads to spawn.
    ///
    /// # Returns
    ///
    /// - `Self`: The new instance.
    pub fn new(worker_count: usize) -> Self {
        let mut pool: Vec<UnboundedSender<AsyncTask>> = Vec::with_capacity(worker_count);
        let counter: Arc<AtomicUsize> = Arc::new(AtomicUsize::new(0));
        let shutdown: Arc<AtomicBool> = Arc::new(AtomicBool::new(false));
        let notifies: Vec<Arc<Notify>> =
            (0..worker_count).map(|_| Arc::new(Notify::new())).collect();
        for notify in notifies.iter().take(worker_count) {
            let (sender, mut receiver): (UnboundedSender<AsyncTask>, UnboundedReceiver<AsyncTask>) =
                unbounded_channel();
            pool.push(sender);
            let shutdown_clone: Arc<AtomicBool> = shutdown.clone();
            let notify_clone: Arc<Notify> = notify.clone();
            spawn_blocking(move || {
                Handle::current().block_on(LocalSet::new().run_until(async move {
                    loop {
                        if shutdown_clone.load(atomic::Ordering::Relaxed) {
                            break;
                        }
                        match receiver.try_recv() {
                            Ok(task) => {
                                spawn_local(task);
                            }
                            Err(_) => {
                                notify_clone.notified().await;
                            }
                        }
                    }
                }));
            });
        }
        Self {
            pool,
            counter,
            shutdown,
            notifies,
        }
    }

    /// Attempts to spawn a server task onto the global server task pool.
    ///
    /// This function sends the task to one of the worker threads in the pool.
    /// The worker is selected using a round-robin algorithm based on an atomic counter,
    /// or a forced index if provided.
    ///
    /// # Arguments
    ///
    /// - `Option<usize>`: An optional index to force selection of a specific worker.
    ///   If None, the worker is selected using round-robin distribution.
    /// - `Future<Output = ()> + Send + 'static`: The future to spawn on the task pool.
    ///
    /// # Returns
    ///
    /// - `bool`: true if the task was successfully sent, false otherwise.
    pub fn try_spawn_local<F>(&self, index_opt: Option<usize>, hook: F) -> bool
    where
        F: Future<Output = ()> + Send + 'static,
    {
        if self.get_pool().is_empty() {
            return false;
        }
        let index: usize = index_opt
            .unwrap_or(self.get_counter().fetch_add(1, atomic::Ordering::Relaxed))
            .wrapping_rem(self.get_pool().len());
        if let Some(sender) = self.get_pool().get(index) {
            let result: bool = sender.send(Box::pin(hook)).is_ok();
            if result && let Some(notify) = self.get_notifies().get(index) {
                notify.notify_one()
            }
            return result;
        }
        false
    }

    /// Shuts down the task pool.
    ///
    /// This function sets the shutdown flag and wakes all waiting worker threads,
    /// causing them to exit their processing loops.
    #[inline(always)]
    pub fn shutdown(&self) {
        self.get_shutdown().store(true, atomic::Ordering::Relaxed);
        self.get_notifies()
            .iter()
            .for_each(|notify: &Arc<Notify>| notify.notify_one());
    }
}
