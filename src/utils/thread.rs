use std::thread;

pub fn get_cpu_thread_count() -> usize {
    thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(1)
}
