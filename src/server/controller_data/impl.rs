use super::r#type::ControllerData;

impl ControllerData {
    pub fn new() -> Self {
        ControllerData {
            stream: None,
            request: None,
            response: None,
        }
    }
}
