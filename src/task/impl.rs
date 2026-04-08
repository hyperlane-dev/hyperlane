use crate::*;

/// Creates a clone of the Task instance.
impl Clone for Task {
    /// Creates a clone of the Task instance.
    ///
    /// # Returns
    ///
    /// - `Self`: The cloned instance.
    #[inline(always)]
    fn clone(&self) -> Self {
        Self {
            pool: self.get_pool().clone(),
            counter: AtomicUsize::new(self.get_counter().load(atomic::Ordering::Relaxed)),
            shutdown: self.get_shutdown(),
            notifies: self.get_notifies(),
        }
    }
}

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
        let shutdown: &'static AtomicBool = Box::leak(Box::new(AtomicBool::new(false)));
        let mut pool: Vec<UnboundedSender<AsyncTask>> = Vec::with_capacity(worker_count);
        let notifies: &'static Vec<Notify> =
            Box::leak(Box::new((0..worker_count).map(|_| Notify::new()).collect()));
        for notify in notifies.iter().take(worker_count) {
            let (sender, mut receiver): (UnboundedSender<AsyncTask>, UnboundedReceiver<AsyncTask>) =
                unbounded_channel();
            pool.push(sender);
            spawn_blocking(move || {
                Handle::current().block_on(LocalSet::new().run_until(async move {
                    loop {
                        if shutdown.load(atomic::Ordering::Relaxed) {
                            break;
                        }
                        match receiver.try_recv() {
                            Ok(task) => {
                                spawn_local(task);
                            }
                            Err(_) => {
                                notify.notified().await;
                            }
                        }
                    }
                }));
            });
        }
        Self {
            pool,
            counter: AtomicUsize::new(0),
            shutdown,
            notifies,
        }
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
            .for_each(|notify: &Notify| notify.notify_one());
    }
}
