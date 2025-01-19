use crate::*;

impl ControllerData {
    #[inline]
    pub fn new() -> Self {
        ControllerData {
            stream: None,
            request: Request::default(),
            response: Response::default(),
            log: Log::default(),
        }
    }
}
