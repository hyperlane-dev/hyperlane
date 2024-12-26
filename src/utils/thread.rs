pub fn get_thread_count() -> usize {
    match std::thread::available_parallelism() {
        Ok(count) => count.get(),
        Err(_) => 1,
    }
}
