use super::r#type::ControllerData;
use hyperlane_log::*;

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
