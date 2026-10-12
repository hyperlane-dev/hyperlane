use super::*;

/// Implementation for `MultipartFormData`.
impl MultipartFormData {
    /// Reads the `boundary` parameter out of a `multipart/form-data` content type.
    ///
    /// The parameter is matched case-insensitively and accepts both a quoted and
    /// a bare value, so the clients that emit `boundary="abc"` and the clients
    /// that emit `boundary=abc` are both accepted. A parameter that carries no
    /// `=` is skipped rather than aborting the scan.
    ///
    /// # Arguments
    ///
    /// - `C` - The content type header value, which requires the `AsRef<str>` bound.
    ///
    /// # Returns
    ///
    /// - `Option<MultipartFormDataBoundary>` - The declared boundary, or `None` when absent.
    pub fn get_boundary<C>(content_type: C) -> Option<MultipartFormDataBoundary>
    where
        C: AsRef<str>,
    {
        for parameter in content_type.as_ref().split(SEMICOLON).skip(1) {
            let Some((key, value)) = parameter.trim().split_once(EQUAL) else {
                continue;
            };
            if !key
                .trim()
                .eq_ignore_ascii_case(MULTIPART_BOUNDARY_PARAMETER)
            {
                continue;
            }
            let value: &str = Self::unquote(value.trim());
            if !value.is_empty() {
                return Some(value.to_string());
            }
        }
        None
    }

    /// Checks whether a content type declares a `multipart/form-data` body.
    ///
    /// # Arguments
    ///
    /// - `C` - The content type header value, which requires the `AsRef<str>` bound.
    ///
    /// # Returns
    ///
    /// - `bool` - Whether the content type is `multipart/form-data`.
    pub fn is_form_data<C>(content_type: C) -> bool
    where
        C: AsRef<str>,
    {
        content_type
            .as_ref()
            .split(SEMICOLON)
            .next()
            .unwrap_or(EMPTY_STR)
            .trim()
            .eq_ignore_ascii_case(MULTIPART_FORM_DATA)
    }

    /// Strips one pair of surrounding quotes from a header parameter value.
    ///
    /// # Arguments
    ///
    /// - `&str` - The raw parameter value.
    ///
    /// # Returns
    ///
    /// - `&str` - The value without its surrounding quotes.
    fn unquote(value: &str) -> &str {
        value
            .strip_prefix(MULTIPART_PARAMETER_QUOTE)
            .and_then(|value: &str| value.strip_suffix(MULTIPART_PARAMETER_QUOTE))
            .unwrap_or(value)
    }

    /// Finds the first occurrence of a needle inside a haystack.
    ///
    /// # Arguments
    ///
    /// - `&[u8]` - The bytes to search in.
    /// - `&[u8]` - The bytes to search for.
    ///
    /// # Returns
    ///
    /// - `Option<usize>` - The offset of the first occurrence, or `None` when absent.
    fn find_subslice(haystack: &[u8], needle: &[u8]) -> Option<usize> {
        if needle.is_empty() || needle.len() > haystack.len() {
            return None;
        }
        haystack
            .windows(needle.len())
            .position(|window: &[u8]| window == needle)
    }

    /// Builds the byte delimiter that marks a boundary line in the body.
    ///
    /// # Arguments
    ///
    /// - `&str` - The declared boundary, without its leading dash pair.
    ///
    /// # Returns
    ///
    /// - `Vec<u8>` - The delimiter bytes, dash pair included.
    fn build_delimiter(boundary: &str) -> Vec<u8> {
        let mut delimiter: Vec<u8> = Vec::with_capacity(boundary.len() + 4);
        delimiter.extend_from_slice(MULTIPART_DELIMITER_DASH.as_bytes());
        delimiter.extend_from_slice(boundary.as_bytes());
        delimiter
    }

    /// Checks whether the bytes after a delimiter are a closing dash pair.
    ///
    /// The closing boundary writes a second dash pair right after the boundary
    /// token, optionally followed by the line break that ends the body.
    ///
    /// # Arguments
    ///
    /// - `&[u8]` - The bytes that follow the delimiter.
    ///
    /// # Returns
    ///
    /// - `bool` - Whether the delimiter is closed by a dash pair.
    fn is_closing_delimiter(remainder: &[u8]) -> bool {
        let Some(after_dash) = remainder.strip_prefix(MULTIPART_DELIMITER_DASH.as_bytes()) else {
            return false;
        };
        after_dash.is_empty()
            || after_dash.starts_with(HTTP_BR.as_bytes())
            || after_dash.starts_with(MULTIPART_LINE_FEED)
    }

    /// Checks whether the bytes after a delimiter close a boundary line.
    ///
    /// A boundary line ends at a line break or at the end of the body, so the
    /// dash pair of the closing boundary is matched here rather than being part
    /// of the delimiter itself.
    ///
    /// # Arguments
    ///
    /// - `&[u8]` - The bytes that follow the delimiter.
    ///
    /// # Returns
    ///
    /// - `bool` - Whether the delimiter is followed by a boundary line ending.
    fn is_boundary_tail(remainder: &[u8]) -> bool {
        remainder.starts_with(HTTP_BR.as_bytes())
            || remainder.starts_with(MULTIPART_LINE_FEED)
            || Self::is_closing_delimiter(remainder)
    }

    /// Finds every byte offset at which the boundary delimiter starts.
    ///
    /// # Arguments
    ///
    /// - `&[u8]` - The raw request body.
    /// - `&[u8]` - The boundary delimiter, dash pair included.
    ///
    /// # Returns
    ///
    /// - `Vec<usize>` - The ascending offsets of every delimiter occurrence.
    fn find_delimiter_positions(body: &[u8], delimiter: &[u8]) -> Vec<usize> {
        let mut positions: Vec<usize> = Vec::new();
        let mut offset: usize = 0;
        while offset < body.len() {
            let Some(found) = Self::find_subslice(&body[offset..], delimiter) else {
                break;
            };
            let position: usize = offset + found;
            let after: usize = position + delimiter.len();
            if after == body.len() || Self::is_boundary_tail(&body[after..]) {
                positions.push(position);
            }
            offset = position + 1;
        }
        positions
    }

    /// Removes the line breaks that belong to a boundary line from a segment.
    ///
    /// The break that follows the opening delimiter is stripped from the front,
    /// and the break that precedes the closing delimiter is stripped from the
    /// back: per the multipart grammar both belong to the boundary line, not to
    /// the part content.
    ///
    /// # Arguments
    ///
    /// - `&[u8]` - The raw segment between two boundary lines.
    ///
    /// # Returns
    ///
    /// - `&[u8]` - The segment without its surrounding boundary line breaks.
    fn trim_boundary_breaks(segment: &[u8]) -> &[u8] {
        let segment: &[u8] = segment.strip_prefix(HTTP_BR.as_bytes()).unwrap_or(segment);
        if let Some(trimmed) = segment.strip_suffix(HTTP_BR.as_bytes()) {
            return trimmed;
        }
        segment.strip_suffix(MULTIPART_LINE_FEED).unwrap_or(segment)
    }

    /// Splits a body into the raw segments carried between boundary lines.
    ///
    /// Both `CRLF` and bare `LF` line breaks are accepted, and the breaks that
    /// belong to a boundary line are not part of a segment. The final boundary
    /// must be the closing one: a body whose last boundary opens a part that is
    /// never closed is truncated, and reporting it as `None` keeps a truncated
    /// upload from passing as a complete one.
    ///
    /// # Arguments
    ///
    /// - `&'a [u8]` - The raw request body.
    /// - `&str` - The declared boundary, without its leading dash pair.
    ///
    /// # Returns
    ///
    /// - `Option<MultipartFormDataSegments<'a>>` - The body segments, or `None` when malformed.
    fn split_segments<'a>(body: &'a [u8], boundary: &str) -> Option<MultipartFormDataSegments<'a>> {
        let delimiter: Vec<u8> = Self::build_delimiter(boundary);
        let positions: Vec<usize> = Self::find_delimiter_positions(body, &delimiter);
        let last_position: usize = positions.last().copied()?;
        let last_end: usize = last_position + delimiter.len();
        if !Self::is_closing_delimiter(&body[last_end..]) {
            return None;
        }
        let mut segments: MultipartFormDataSegments = Vec::with_capacity(positions.len());
        for index in 0..positions.len().saturating_sub(1) {
            let start: usize = positions[index] + delimiter.len();
            let end: usize = positions[index + 1];
            segments.push(Self::trim_boundary_breaks(&body[start..end]));
        }
        Some(segments)
    }

    /// Splits a part into its header block and its content.
    ///
    /// # Arguments
    ///
    /// - `&[u8]` - The raw segment of a single part.
    ///
    /// # Returns
    ///
    /// - `(&[u8], &[u8])` - The header block and the content of the part.
    fn split_part(segment: &[u8]) -> (&[u8], &[u8]) {
        let fallback: &[u8] = &segment[segment.len()..];
        if let Some(position) = Self::find_subslice(segment, HTTP_DOUBLE_BR.as_bytes()) {
            let data_start: usize = position + HTTP_DOUBLE_BR.len();
            return (&segment[..position], &segment[data_start..]);
        }
        if let Some(position) = Self::find_subslice(segment, MULTIPART_HEADER_SEPARATOR) {
            let data_start: usize = position + MULTIPART_HEADER_SEPARATOR_LEN;
            return (&segment[..position], &segment[data_start..]);
        }
        (segment, fallback)
    }

    /// Decodes the header block of a single part.
    ///
    /// # Arguments
    ///
    /// - `&[u8]` - The header block of the part, terminated by a blank line.
    ///
    /// # Returns
    ///
    /// - `Option<BodyPartHeaders>` - The decoded headers, or `None` when not valid text.
    fn parse_part_headers(header_block: &[u8]) -> Option<BodyPartHeaders> {
        let header_text: &str = from_utf8(header_block).ok()?;
        let mut headers: BodyPartHeaders = hash_map_xx_hash3_64();
        for line in header_text.split(HTTP_BR) {
            let line: &str = line.trim_end_matches('\n').trim();
            if line.is_empty() {
                continue;
            }
            let (key, value) = line.split_once(COLON)?;
            let key: String = key.trim().to_ascii_lowercase();
            if key.is_empty() {
                continue;
            }
            headers.insert(key, value.trim().to_string());
        }
        Some(headers)
    }

    /// Reads the `name` and `filename` parameters of a part content disposition.
    ///
    /// # Arguments
    ///
    /// - `&str` - The part `Content-Disposition` header value.
    ///
    /// # Returns
    ///
    /// - `(BodyPartName, BodyPartFileName)` - The declared field name and file name.
    fn parse_part_disposition(disposition: &str) -> (BodyPartName, BodyPartFileName) {
        let mut name: BodyPartName = String::new();
        let mut file_name: BodyPartFileName = String::new();
        for parameter in disposition.split(SEMICOLON).skip(1) {
            let Some((key, value)) = parameter.trim().split_once(EQUAL) else {
                continue;
            };
            let value: &str = Self::unquote(value.trim());
            match key.trim().to_ascii_lowercase().as_str() {
                MULTIPART_NAME_PARAMETER => name = value.to_string(),
                FILENAME => file_name = value.to_string(),
                _ => {}
            }
        }
        (name, file_name)
    }

    /// Decodes every part of a `multipart/form-data` body.
    ///
    /// A body that carries no declared boundary, a body whose final boundary is
    /// not the closing one, and a part that declares no `name` all yield `None`.
    ///
    /// # Arguments
    ///
    /// - `B` - The raw request body, which requires the `AsRef<[u8]>` bound.
    /// - `&str` - The declared boundary, without its leading dash pair.
    ///
    /// # Returns
    ///
    /// - `Option<MultipartFormDataParts>` - The decoded parts, or `None` when malformed.
    pub fn parse<B>(body: B, boundary: &str) -> Option<MultipartFormDataParts>
    where
        B: AsRef<[u8]>,
    {
        let segments: MultipartFormDataSegments = Self::split_segments(body.as_ref(), boundary)?;
        let mut parts: MultipartFormDataParts = Vec::with_capacity(segments.len());
        for segment in segments {
            if segment.is_empty() {
                continue;
            }
            let (header_block, data) = Self::split_part(segment);
            let headers: BodyPartHeaders = Self::parse_part_headers(header_block)?;
            let disposition: String = headers
                .get(MULTIPART_CONTENT_DISPOSITION)
                .cloned()
                .unwrap_or_default();
            let (name, file_name) = Self::parse_part_disposition(&disposition);
            if name.is_empty() || parts.len() >= MULTIPART_MAX_PARTS {
                return None;
            }
            let content_type: BodyPartContentType =
                headers.get(CONTENT_TYPE).cloned().unwrap_or_default();
            parts.push(BodyPart::new(
                name,
                file_name,
                content_type,
                headers,
                data.to_vec(),
            ));
        }
        Some(parts)
    }

    /// Tries to get a part by its form field name.
    ///
    /// # Arguments
    ///
    /// - `&[BodyPart]` - The decoded parts of the body.
    /// - `K` - The form field name, which requires the `AsRef<str>` bound.
    ///
    /// # Returns
    ///
    /// - `Option<&BodyPart>` - The first part carrying that name, or `None`.
    pub fn try_get_part<K>(parts: &[BodyPart], name: K) -> Option<&BodyPart>
    where
        K: AsRef<str>,
    {
        let name: &str = name.as_ref();
        parts
            .iter()
            .find(|part: &&BodyPart| part.get_name() == name)
    }

    /// Tries to get a plain text field by its form field name.
    ///
    /// A part that uploads a file is skipped, so a file field never silently
    /// decodes as text.
    ///
    /// # Arguments
    ///
    /// - `&[BodyPart]` - The decoded parts of the body.
    /// - `K` - The form field name, which requires the `AsRef<str>` bound.
    ///
    /// # Returns
    ///
    /// - `Option<BodyFormValue>` - The field value, or `None` when absent or a file.
    pub fn try_get_part_text<K>(parts: &[BodyPart], name: K) -> Option<BodyFormValue>
    where
        K: AsRef<str>,
    {
        let part: &BodyPart = Self::try_get_part(parts, name)?;
        if !part.get_file_name().is_empty() {
            return None;
        }
        Some(String::from_utf8_lossy(part.get_data()).into_owned())
    }

    /// Decodes an `application/x-www-form-urlencoded` body into its fields.
    ///
    /// # Arguments
    ///
    /// - `B` - The raw request body, which requires the `AsRef<[u8]>` bound.
    ///
    /// # Returns
    ///
    /// - `BodyForm` - The decoded form fields.
    pub fn parse_url_encoded<B>(body: B) -> BodyForm
    where
        B: AsRef<[u8]>,
    {
        let decoded: String = String::from_utf8_lossy(body.as_ref()).into_owned();
        let mut form: BodyForm = hash_map_xx_hash3_64();
        for pair in decoded.split(AND) {
            if pair.is_empty() {
                continue;
            }
            let (key, value) = pair.split_once(EQUAL).unwrap_or((pair, EMPTY_STR));
            let key: BodyFormKey = Self::decode_form_component(key);
            if key.is_empty() {
                continue;
            }
            form.insert(key, Self::decode_form_component(value));
        }
        form
    }

    /// Decodes one percent-encoded form component.
    ///
    /// `+` is decoded as a space, `%XX` as the matching byte, and an invalid
    /// escape is kept verbatim instead of dropping the component.
    ///
    /// # Arguments
    ///
    /// - `&str` - The raw component of the form body.
    ///
    /// # Returns
    ///
    /// - `String` - The decoded component.
    fn decode_form_component(component: &str) -> String {
        let bytes: Vec<u8> = component.bytes().collect();
        let mut decoded: Vec<u8> = Vec::with_capacity(bytes.len());
        let mut index: usize = 0;
        while index < bytes.len() {
            let byte: u8 = bytes[index];
            match byte {
                URL_ENCODED_PLUS => {
                    decoded.push(URL_ENCODED_SPACE);
                    index += 1;
                }
                byte if byte == URL_ENCODED_PERCENT && index + 2 < bytes.len() => {
                    match Self::parse_hex_pair(&bytes[index + 1], &bytes[index + 2]) {
                        Some(value) => {
                            decoded.push(value);
                            index += 3;
                        }
                        None => {
                            decoded.push(byte);
                            index += 1;
                        }
                    }
                }
                _ => {
                    decoded.push(byte);
                    index += 1;
                }
            }
        }
        String::from_utf8_lossy(&decoded).into_owned()
    }

    /// Reads the two hexadecimal digits of a percent escape.
    ///
    /// # Arguments
    ///
    /// - `&u8` - The high digit of the escape.
    /// - `&u8` - The low digit of the escape.
    ///
    /// # Returns
    ///
    /// - `Option<u8>` - The decoded byte, or `None` when a digit is not hexadecimal.
    fn parse_hex_pair(high: &u8, low: &u8) -> Option<u8> {
        let high_digit: u8 = Self::hex_digit(high)?;
        let low_digit: u8 = Self::hex_digit(low)?;
        Some(high_digit * HEX_RADIX + low_digit)
    }

    /// Converts one hexadecimal digit into its numeric value.
    ///
    /// # Arguments
    ///
    /// - `&u8` - The ASCII byte of the digit.
    ///
    /// # Returns
    ///
    /// - `Option<u8>` - The numeric value, or `None` when the byte is not hexadecimal.
    fn hex_digit(byte: &u8) -> Option<u8> {
        match byte {
            HEX_DIGIT_ZERO..=HEX_DIGIT_NINE => Some(byte - HEX_DIGIT_ZERO),
            HEX_DIGIT_LOWER_A..=HEX_DIGIT_LOWER_F => {
                Some(byte - HEX_DIGIT_LOWER_A + HEX_LOWER_OFFSET)
            }
            HEX_DIGIT_UPPER_A..=HEX_DIGIT_UPPER_F => {
                Some(byte - HEX_DIGIT_UPPER_A + HEX_LOWER_OFFSET)
            }
            _ => None,
        }
    }
}

impl BodyPart {
    /// Tries to get the form field name of this part.
    ///
    /// # Returns
    ///
    /// - `Option<&BodyPartName>` - The declared field name, or `None` when
    ///   the part declares none.
    pub fn try_get_name(&self) -> Option<&BodyPartName> {
        (!self.get_name().is_empty()).then_some(self.get_name())
    }

    /// Tries to get the uploaded file name of this part.
    ///
    /// # Returns
    ///
    /// - `Option<&BodyPartFileName>` - The declared file name, or `None`
    ///   when this part is a plain field rather than a file upload.
    pub fn try_get_file_name(&self) -> Option<&BodyPartFileName> {
        (!self.get_file_name().is_empty()).then_some(self.get_file_name())
    }

    /// Tries to get the media type declared by this part.
    ///
    /// # Returns
    ///
    /// - `Option<&BodyPartContentType>` - The declared media type, or
    ///   `None` when the part declares none.
    pub fn try_get_content_type(&self) -> Option<&BodyPartContentType> {
        (!self.get_content_type().is_empty()).then_some(self.get_content_type())
    }

    /// Tries to get the value of one of this part's headers.
    ///
    /// # Arguments
    ///
    /// - `K` - The header name, which must implement `AsRef<str>`.
    ///
    /// # Returns
    ///
    /// - `Option<&BodyPartHeaderValue>` - The header value, or `None`
    ///   when this part carries no such header.
    pub fn try_get_header<K>(&self, key: K) -> Option<&BodyPartHeaderValue>
    where
        K: AsRef<str>,
    {
        self.get_headers().get(key.as_ref())
    }

    /// Tries to get this part's raw content.
    ///
    /// # Returns
    ///
    /// - `Option<&[u8]>` - The raw bytes, or `None` when the part is empty.
    pub fn try_get_data(&self) -> Option<&[u8]> {
        (!self.get_data().is_empty()).then(|| self.get_data().as_slice())
    }

    /// Tries to get this part's raw content as UTF-8 text.
    ///
    /// Invalid UTF-8 is replaced rather than rejected, so a binary upload
    /// read as text yields its readable bytes instead of `None`.
    ///
    /// # Returns
    ///
    /// - `Option<BodyFormValue>` - The decoded text, or `None` when the
    ///   part is empty.
    pub fn try_get_text(&self) -> Option<BodyFormValue> {
        self.try_get_data()
            .map(|data: &[u8]| String::from_utf8_lossy(data).into_owned())
    }
}
