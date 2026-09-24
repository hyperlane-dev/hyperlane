use super::*;

/// Default implementation for Body.
///
/// # Returns
///
/// - `Body` - Returns a Body with empty text content.
impl Default for Body {
    #[inline(always)]
    fn default() -> Self {
        Self::Text(EMPTY_STR.to_owned())
    }
}

/// Formats the Body for display.
///
/// # Arguments
///
/// - `&mut Formatter<'_>` - The formatter to write to.
///
/// # Returns
///
/// - `fmt::Result` - Result of the formatting operation.
impl Display for Body {
    #[inline(always)]
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Text(text) => write!(f, "{text}"),
            Self::Json(json) => write!(
                f,
                "{}",
                serde_json::to_string(json).unwrap_or_else(|_| String::from("{}"))
            ),
            Self::Binary(binary) => write!(f, "{binary:?}"),
        }
    }
}

/// Serializes the Body content.
///
/// # Arguments
///
/// - `S` - The type of the serializer.
///
/// # Returns
///
/// - `Result<S::Ok, S::Error>` - Result of the serialization.
impl Serialize for Body {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Text(text) => text.serialize(serializer),
            Self::Json(json) => json.serialize(serializer),
            Self::Binary(binary) => binary.serialize(serializer),
        }
    }
}
