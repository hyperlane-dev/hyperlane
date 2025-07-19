use crate::*;

impl StdError for ServerError {}

impl StdError for RouteError {}

impl Panic {
    pub(crate) fn new(message: String, location: String, payload: String) -> Self {
        Self {
            message,
            location,
            payload,
        }
    }

    fn extract_panic_message_from_any(panic_payload: &dyn Any) -> String {
        if let Some(s) = panic_payload.downcast_ref::<&str>() {
            s.to_string()
        } else if let Some(s) = panic_payload.downcast_ref::<String>() {
            s.clone()
        } else {
            EMPTY_STR.to_string()
        }
    }

    fn format_panic_location(location: OptionLocationRef<'_, '_>) -> String {
        location
            .map(|data| {
                format!(
                    "{}{}{}{}{}",
                    data.file(),
                    COLON_SPACE_SYMBOL,
                    data.line(),
                    COLON_SPACE_SYMBOL,
                    data.column()
                )
            })
            .unwrap_or_default()
    }

    pub(crate) fn from_panic_hook(info: &PanicHookInfo<'_>) -> Self {
        let message: String = Self::extract_panic_message_from_any(info.payload());
        let location: String = Self::format_panic_location(info.location());
        let payload: String = info.to_string();
        Self {
            message,
            location,
            payload,
        }
    }

    pub(crate) fn from_join_error(join_error: JoinError) -> Self {
        let panic_join_error: BoxAnySend = join_error.into_panic();
        let message: String = Self::extract_panic_message_from_any(&panic_join_error);
        let panic: Panic = Panic::new(message, String::new(), String::new());
        panic
    }
}
