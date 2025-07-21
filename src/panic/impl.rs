use crate::*;

impl Panic {
    pub(crate) fn new(
        message: OptionString,
        location: OptionString,
        payload: OptionString,
    ) -> Self {
        Self {
            message,
            location,
            payload,
        }
    }

    fn extract_panic_message_from_any(panic_payload: &dyn Any) -> OptionString {
        if let Some(s) = panic_payload.downcast_ref::<&str>() {
            Some(s.to_string())
        } else if let Some(s) = panic_payload.downcast_ref::<String>() {
            Some(s.clone())
        } else {
            None
        }
    }

    fn format_panic_location(location: OptionLocationRef<'_, '_>) -> OptionString {
        location.map(|data| {
            format!(
                "{}{}{}{}{}",
                data.file(),
                COLON_SPACE_SYMBOL,
                data.line(),
                COLON_SPACE_SYMBOL,
                data.column()
            )
        })
    }

    pub(crate) fn from_panic_hook(info: &PanicHookInfo<'_>) -> Self {
        let message_string: String = info.to_string();
        let message: OptionString = if message_string.is_empty() {
            Some(message_string)
        } else {
            None
        };
        let payload: OptionString = Self::extract_panic_message_from_any(info.payload());
        let location: OptionString = Self::format_panic_location(info.location());
        Self {
            message,
            location,
            payload,
        }
    }

    pub(crate) fn from_join_error(join_error: JoinError) -> Self {
        let default_message: String = join_error.to_string();
        let mut message: OptionString = if let Ok(panic_join_error) = join_error.try_into_panic() {
            Self::extract_panic_message_from_any(&panic_join_error)
        } else {
            None
        };
        if (message.is_none() || message.clone().unwrap_or_default().is_empty())
            && !default_message.is_empty()
        {
            message = Some(default_message);
        }
        let panic: Panic = Panic::new(message, None, None);
        panic
    }
}
