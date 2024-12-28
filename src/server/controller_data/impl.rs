use super::r#type::ControllerData;
use crate::server::log::r#type::Log;

impl ControllerData {
    pub fn new() -> Self {
        ControllerData {
            stream: None,
            request: None,
            response: None,
            log: Log::default(),
        }
    }
}
