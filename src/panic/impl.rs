use crate::*;

/// Implementation of methods for the `Panic` struct.
impl Panic {
    /// Creates a new `Panic` instance from its constituent parts.
    ///
    /// # Arguments
    ///
    /// - `OptionString` - The panic message.
    /// - `OptionString` - The source code location of the panic.
    /// - `OptionString` - The panic payload.
    ///
    /// # Returns
    ///
    /// - `Panic` - A new panic instance.
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

    /// Attempts to extract a string from a dynamic `&dyn Any` panic payload.
    ///
    /// This function handles payloads that are either `&str` or `String`.
    ///
    /// # Arguments
    ///
    /// - `&dyn Any` - The payload from a `PanicInfo` object.
    ///
    /// # Returns
    ///
    /// - `OptionString` - The extracted message, or None if the payload is not a string type.
    fn extract_panic_message_from_any(panic_payload: &dyn Any) -> OptionString {
        if let Some(s) = panic_payload.downcast_ref::<&str>() {
            Some(s.to_string())
        } else if let Some(s) = panic_payload.downcast_ref::<String>() {
            Some(s.clone())
        } else {
            None
        }
    }

    /// Formats a `Location` struct into a "file:line:column" string.
    ///
    /// # Arguments
    ///
    /// - `OptionLocationRef<'_, '_>` - The location of the panic.
    ///
    /// # Returns
    ///
    /// - `OptionString` - The formatted location, or None if not available.
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

    /// Creates a `Panic` instance from the standard library's `PanicInfo`.
    ///
    /// This is the primary constructor used by the global panic hook to capture
    /// details about a panic.
    ///
    /// # Arguments
    ///
    /// - `&PanicHookInfo<'_>` - The panic info from the hook callback.
    ///
    /// # Returns
    ///
    /// - `Panic` - A new panic instance with data from info.
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

    /// Creates a `Panic` instance from a `tokio::task::JoinError`.
    ///
    /// This is used to handle panics that occur within spawned asynchronous tasks,
    /// extracting the panic message from the `JoinError`.
    ///
    /// # Arguments
    ///
    /// - `JoinError` - The error from a panicked task.
    ///
    /// # Returns
    ///
    /// - `Panic` - A new panic instance with message from error.
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
