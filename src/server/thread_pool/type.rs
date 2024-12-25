use crate::server::worker::r#type::Worker;
use std::sync::mpsc::Sender;

pub struct ThreadPool {
    pub(crate) workers: Vec<Worker>,
    pub(crate) sender: Sender<Job>,
}

pub type Job = Box<dyn FnOnce() + Send + 'static>;
