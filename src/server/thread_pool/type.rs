use crate::server::worker::r#type::Worker;
use std::sync::mpsc::Sender;

#[allow(dead_code)]
pub struct ThreadPool {
    pub(super) workers: Vec<Worker>,
    pub(super) sender: Sender<Job>,
}

pub type Job = Box<dyn FnOnce() + Send + 'static>;
