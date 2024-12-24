use crate::server::controller_data::r#type::ControllerData;
use std::sync::{Arc, RwLock};

pub type MiddlewareArcLock = Arc<RwLock<Vec<Box<dyn Fn(&mut ControllerData) + Send + Sync>>>>;
