use crate::server::controller_data::r#type::ControllerData;
use http_type::ArcRwLock;

pub type MiddlewareArcLock = ArcRwLock<Vec<Box<dyn Fn(&mut ControllerData) + Send + Sync>>>;
