use super::r#type::ThreadPool;
use crate::server::worker::r#type::Worker;
use http_type::ArcMutex;
use std::sync::mpsc::{self, Receiver};
pub use std_macro_extensions::*;

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        let (sender, receiver) = mpsc::channel();
        let receiver: ArcMutex<Receiver<Box<dyn FnOnce() + Send>>> = Arc::new(Mutex::new(receiver));
        let mut workers: Vec<Worker> = Vec::with_capacity(size);
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }
        ThreadPool { workers, sender }
    }

    pub fn execute<F>(&self, job: F)
    where
        F: FnOnce() + Send + 'static,
    {
        self.sender.send(Box::new(job)).unwrap();
    }
}
