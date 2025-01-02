use super::r#type::ControllerData;
use crate::{ControllerDataRequest, ControllerDataResponse};
use hyperlane_log::*;

impl ControllerData {
    pub fn new() -> Self {
        ControllerData {
            stream: None,
            request: ControllerDataRequest::default(),
            response: ControllerDataResponse::default(),
            log: Log::default(),
        }
    }
}
