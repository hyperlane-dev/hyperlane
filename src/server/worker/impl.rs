use super::r#type::Worker;
use crate::server::thread_pool::r#type::Job;
use http_type::*;
use std::{sync::mpsc::Receiver, thread::spawn};

impl Worker {
    pub fn new(id: usize, receiver: ArcMutex<Receiver<Job>>) -> Worker {
        let thread = spawn(move || loop {
            let job = receiver.lock().unwrap().recv().unwrap();
            job();
        });
        Worker {
            id,
            thread: Some(thread),
        }
    }
}
