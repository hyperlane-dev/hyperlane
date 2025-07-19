use crate::*;

impl StdError for ServerError {}

impl StdError for RouteError {}

impl PanicInfo {
    pub(crate) fn new(message: String, location: Option<String>, payload: String) -> Self {
        Self {
            message,
            location,
            payload,
        }
    }

    pub(crate) fn from_panic_hook_info(info: &PanicHookInfo<'_>) -> Self {
        let message: String = if let Some(s) = info.payload().downcast_ref::<&str>() {
            s.to_string()
        } else if let Some(s) = info.payload().downcast_ref::<String>() {
            s.clone()
        } else {
            "Unknown panic".to_string()
        };
        let location: Option<String> = info
            .location()
            .map(|loc| format!("{}:{}:{}", loc.file(), loc.line(), loc.column()));
        let payload: String = info.to_string();
        Self {
            message,
            location,
            payload,
        }
    }
}
