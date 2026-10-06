use super::*;

/// Implementation for `ContentType` enum.
impl ContentType {
    /// Serializes data into JSON format.
    ///
    /// # Arguments
    ///
    /// - `&T` - Data to serialize, which requires the `Serialize` and `Display` bounds
    ///
    /// # Returns
    ///
    /// - `String` - Serialized JSON string
    fn get_application_json<T>(data: &T) -> String
    where
        T: Serialize + Display,
    {
        serde_json::to_string(data).unwrap_or_default()
    }

    /// Serializes data into XML format.
    ///
    /// # Arguments
    ///
    /// - `&T` - Data to serialize, which requires the `Serialize` and `Display` bounds
    ///
    /// # Returns
    ///
    /// - `String` - Serialized XML string
    fn get_application_xml<T>(data: &T) -> String
    where
        T: Serialize + Display,
    {
        serde_xml_rs::to_string(data).unwrap_or_default()
    }

    /// Formats data into plain text.
    ///
    /// # Arguments
    ///
    /// - `&T` - Data to format, which requires the `Serialize`, `Debug`, `Clone`, `Default` and
    ///   `Display` bounds
    ///
    /// # Returns
    ///
    /// - `String` - Formatted plain text
    fn get_text_plain<T>(data: &T) -> String
    where
        T: Serialize + Debug + Clone + Default + Display,
    {
        data.to_string()
    }

    /// Formats data into HTML.
    ///
    /// # Arguments
    ///
    /// - `&T` - Data to format, which requires the `Serialize`, `Debug`, `Clone` and `Default`
    ///   bounds
    ///
    /// # Returns
    ///
    /// - `String` - Formatted HTML
    fn get_text_html<T>(data: &T) -> String
    where
        T: Serialize + Debug + Clone + Default,
    {
        let mut html: String = String::with_capacity(64);
        html.push_str(HTML_TABLE_OPEN);
        html.push_str(&format!("{data:?}"));
        html.push_str(HTML_TABLE_CLOSE);
        html
    }

    /// Serializes data into URL-encoded format.
    ///
    /// # Arguments
    ///
    /// - `&T` - Data to serialize, which requires the `Serialize` and `Display` bounds
    ///
    /// # Returns
    ///
    /// - `String` - URL-encoded string
    fn get_form_url_encoded<T>(data: &T) -> String
    where
        T: Serialize + Display,
    {
        serde_urlencoded::to_string(data).unwrap_or_default()
    }

    /// Formats data as hexadecimal string.
    ///
    /// # Arguments
    ///
    /// - `&T` - Data to format, which requires the `Serialize`, `Debug`, `Clone`, `Default` and
    ///   `Display` bounds
    ///
    /// # Returns
    ///
    /// - `String` - Hexadecimal encoded string
    fn get_binary<T>(data: &T) -> String
    where
        T: Serialize + Debug + Clone + Default + Display,
    {
        hex::encode(data.to_string())
    }

    /// Gets formatted body string for content type.
    ///
    /// # Arguments
    ///
    /// - `&T` - Data to format, which requires the `Serialize`, `Debug`, `Clone`, `Default` and
    ///   `Display` bounds
    ///
    /// # Returns
    ///
    /// - `String` - Formatted body string
    pub fn get_body_string<T>(&self, data: &T) -> String
    where
        T: Serialize + Debug + Clone + Default + Display,
    {
        match self {
            Self::ApplicationJson => Self::get_application_json(data),
            Self::ApplicationXml => Self::get_application_xml(data),
            Self::TextPlain => Self::get_text_plain(data),
            Self::TextHtml => Self::get_text_html(data),
            Self::FormUrlEncoded => Self::get_form_url_encoded(data),
            Self::Unknown => Self::get_binary(data),
        }
    }

    /// Formats content type with charset.
    ///
    /// # Arguments
    ///
    /// - `T` - Content type, which requires the `AsRef<str>` bound
    /// - `S` - Charset, which requires the `AsRef<str>` bound
    ///
    /// # Returns
    ///
    /// - `String` - Formatted string
    pub fn format_content_type_with_charset<T, S>(content_type: T, charset: S) -> String
    where
        T: AsRef<str>,
        S: AsRef<str>,
    {
        let content_type_ref: &str = content_type.as_ref();
        let charset_ref: &str = charset.as_ref();
        let mut result: String = String::with_capacity(
            content_type_ref.len()
                + SEMICOLON_SPACE.len()
                + CHARSET_EQUAL.len()
                + charset_ref.len(),
        );
        result.push_str(content_type_ref);
        result.push_str(SEMICOLON_SPACE);
        result.push_str(CHARSET_EQUAL);
        result.push_str(charset_ref);
        result
    }

    /// Formats content type with charset declaration.
    ///
    /// # Arguments
    ///
    /// - `T` - Content type, which requires the `AsRef<str>` bound
    /// - `S` - Charset declaration, which requires the `AsRef<str>` bound
    ///
    /// # Returns
    ///
    /// - `String` - Formatted string
    pub fn format_content_type_with_charset_declaration<T, S>(
        content_type: T,
        charset_with_key: S,
    ) -> String
    where
        T: AsRef<str>,
        S: AsRef<str>,
    {
        let content_type_ref: &str = content_type.as_ref();
        let charset_with_key_ref: &str = charset_with_key.as_ref();
        let mut result: String = String::with_capacity(
            content_type_ref.len() + SEMICOLON_SPACE.len() + charset_with_key_ref.len(),
        );
        result.push_str(content_type_ref);
        result.push_str(SEMICOLON_SPACE);
        result.push_str(charset_with_key_ref);
        result
    }
}

/// Implements `FromStr` for `ContentType`.
impl FromStr for ContentType {
    type Err = ();

    /// Parses string into ContentType.
    ///
    /// # Arguments
    ///
    /// - `&str` - String to parse
    ///
    /// # Returns
    ///
    /// - `Result<Self, Self::Err>` - Parse result
    fn from_str(data: &str) -> Result<Self, Self::Err> {
        match data.to_ascii_lowercase().as_str() {
            APPLICATION_JSON => Ok(Self::ApplicationJson),
            APPLICATION_XML => Ok(Self::ApplicationXml),
            TEXT_PLAIN => Ok(Self::TextPlain),
            TEXT_HTML => Ok(Self::TextHtml),
            FORM_URLENCODED => Ok(Self::FormUrlEncoded),
            _ => Ok(Self::Unknown),
        }
    }
}
