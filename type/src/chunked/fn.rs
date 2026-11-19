use super::*;

/// Decodes one HTTP/1.1 chunked body, discarding trailers without consuming the next message.
///
/// # Arguments
///
/// - `&mut R` - The buffered reader positioned at the first chunk size line.
/// - `&mut RequestBody` - The reusable destination for the decoded data.
/// - `&RequestConfig` - The request limits for decoded data and trailers.
///
/// # Returns
///
/// - `Result<(), RequestError>` - Success after the final CRLF, or a framing, limit or I/O error.
pub(crate) async fn fill_chunked_body<R>(
    reader: &mut R,
    body: &mut RequestBody,
    config: &RequestConfig,
) -> Result<(), RequestError>
where
    R: AsyncBufReadExt + Unpin,
{
    body.clear();
    let mut line: Vec<u8> = Vec::new();
    let mut extension_budget: usize = MAX_CHUNK_EXTENSION_BYTES;
    let max_body_size: usize = config.get_max_body_size();
    loop {
        read_chunk_line(reader, &mut line, MAX_CHUNK_LINE_BYTES).await?;
        let size_end: usize = line
            .iter()
            .position(|byte: &u8| !byte.is_ascii_hexdigit())
            .unwrap_or(line.len());
        let (size, extensions): (&[u8], &[u8]) = line.split_at(size_end);
        extension_budget = extension_budget
            .checked_sub(extensions.len())
            .ok_or(CHUNK_METADATA_TOO_LARGE)?;
        let size: &str = from_utf8(size).map_err(|_| INVALID_CHUNK)?;
        let size: usize = usize::from_str_radix(size, 16).map_err(|_| INVALID_CHUNK)?;
        validate_chunk_extensions(extensions)?;
        if size == 0 {
            return discard_chunk_trailers(reader, &mut line, config).await;
        }
        if size > max_body_size.saturating_sub(body.len()) {
            return Err(RequestError::ContentLengthTooLarge(
                HttpStatus::PayloadTooLarge,
            ));
        }
        read_chunk_data(reader, body, size).await?;
    }
}

/// Reads a bounded metadata line and removes its required CRLF.
///
/// # Arguments
///
/// - `&mut R` - The buffered reader.
/// - `&mut Vec<u8>` - Scratch space reused across chunks and trailers.
/// - `usize` - The byte budget for the whole line including CRLF.
///
/// # Returns
///
/// - `Result<(), RequestError>` - Success after consuming just the line, or a read error.
async fn read_chunk_line<R>(
    reader: &mut R,
    line: &mut Vec<u8>,
    max_len: usize,
) -> Result<(), RequestError>
where
    R: AsyncBufReadExt + Unpin,
{
    line.clear();
    while line.len() < max_len {
        let available: &[u8] = reader.fill_buf().await?;
        if available.is_empty() {
            return Err(INVALID_CHUNK);
        }
        let scan_len: usize = available.len().min(max_len - line.len());
        let newline: Option<usize> = available[..scan_len]
            .iter()
            .position(|byte: &u8| *byte == b'\n');
        let consumed: usize = newline.map_or(scan_len, |index: usize| index + 1);
        line.extend_from_slice(&available[..consumed]);
        reader.consume(consumed);
        if newline.is_some() {
            if !line.ends_with(HTTP_BR_BYTES) {
                return Err(INVALID_CHUNK);
            }
            line.truncate(line.len() - HTTP_BR_BYTES.len());
            return Ok(());
        }
    }
    Err(CHUNK_METADATA_TOO_LARGE)
}

/// Copies only received chunk data into the body and checks the data delimiter.
///
/// # Arguments
///
/// - `&mut R` - The buffered reader positioned at the chunk data.
/// - `&mut RequestBody` - The destination for the payload.
/// - `usize` - The validated size of the current chunk.
///
/// # Returns
///
/// - `Result<(), RequestError>` - Success after the chunk delimiter, or an error.
async fn read_chunk_data<R>(
    reader: &mut R,
    body: &mut RequestBody,
    mut remaining: usize,
) -> Result<(), RequestError>
where
    R: AsyncBufReadExt + Unpin,
{
    while remaining != 0 {
        let available: &[u8] = reader.fill_buf().await?;
        if available.is_empty() {
            return Err(INVALID_CHUNK);
        }
        let consumed: usize = available.len().min(remaining);
        body.try_reserve(consumed)
            .map_err(|_| RequestError::ContentLengthTooLarge(HttpStatus::PayloadTooLarge))?;
        body.extend_from_slice(&available[..consumed]);
        reader.consume(consumed);
        remaining -= consumed;
    }
    let mut delimiter: [u8; HTTP_BR_BYTES.len()] = [0; HTTP_BR_BYTES.len()];
    reader.read_exact(&mut delimiter).await?;
    if delimiter != HTTP_BR_BYTES {
        return Err(INVALID_CHUNK);
    }
    Ok(())
}

/// Validates chunk extension syntax without retaining names or values.
///
/// # Arguments
///
/// - `&[u8]` - The bytes after the chunk size digits.
///
/// # Returns
///
/// - `Result<(), RequestError>` - Success if the extension syntax is valid, or a framing error.
fn validate_chunk_extensions(mut input: &[u8]) -> Result<(), RequestError> {
    while !input.is_empty() {
        let name: &[u8] = trim_chunk_ows(input)
            .strip_prefix(b";")
            .ok_or(INVALID_CHUNK)?;
        input = skip_chunk_token(trim_chunk_ows(name))?;
        if let Some(value) = trim_chunk_ows(input).strip_prefix(b"=") {
            let value: &[u8] = trim_chunk_ows(value);
            input = match value.strip_prefix(b"\"") {
                Some(quoted) => skip_chunk_quoted_string(quoted)?,
                None => skip_chunk_token(value)?,
            };
        }
    }
    Ok(())
}

/// Skips the contents and closing quote of one extension value.
///
/// # Arguments
///
/// - `&[u8]` - The bytes after the opening quote.
///
/// # Returns
///
/// - `Result<&[u8], RequestError>` - The suffix after the closing quote, or a framing error.
fn skip_chunk_quoted_string(input: &[u8]) -> Result<&[u8], RequestError> {
    let mut escaped: bool = false;
    for (index, &byte) in input.iter().enumerate() {
        if !is_chunk_field_value_byte(byte) {
            return Err(INVALID_CHUNK);
        }
        if escaped {
            escaped = false;
        } else if byte == b'\\' {
            escaped = true;
        } else if byte == b'"' {
            return Ok(&input[index + 1..]);
        }
    }
    Err(INVALID_CHUNK)
}

/// Validates and discards trailers through the terminating empty line.
///
/// # Arguments
///
/// - `&mut R` - The buffered reader positioned immediately after the zero chunk line.
/// - `&mut Vec<u8>` - The reused metadata line buffer.
/// - `&RequestConfig` - The request header count, key and value limits.
///
/// # Returns
///
/// - `Result<(), RequestError>` - Success after the empty line, or a read, syntax or limit error.
async fn discard_chunk_trailers<R>(
    reader: &mut R,
    line: &mut Vec<u8>,
    config: &RequestConfig,
) -> Result<(), RequestError>
where
    R: AsyncBufReadExt + Unpin,
{
    let mut budget: usize = MAX_CHUNK_TRAILER_BYTES;
    let mut count: usize = 0;
    loop {
        read_chunk_line(reader, line, budget.min(MAX_CHUNK_LINE_BYTES)).await?;
        if line.is_empty() {
            return Ok(());
        }
        budget -= line.len() + HTTP_BR_BYTES.len();
        count += 1;
        Request::check_http_header_count(count, config.get_max_header_count())?;
        validate_chunk_trailer(line, config)?;
    }
}

/// Checks the syntax and limits of a trailer without merging it into the request headers.
///
/// # Arguments
///
/// - `&[u8]` - One complete trailer line excluding CRLF.
/// - `&RequestConfig` - The configured key and value limits.
///
/// # Returns
///
/// - `Result<(), RequestError>` - Success for an allowed trailer, or a syntax or limit error.
fn validate_chunk_trailer(line: &[u8], config: &RequestConfig) -> Result<(), RequestError> {
    let colon: usize = line
        .iter()
        .position(|byte: &u8| *byte == b':')
        .ok_or(INVALID_CHUNK)?;
    let key: &[u8] = &line[..colon];
    let value: &[u8] = &line[colon + 1..];
    if key.is_empty()
        || !key.iter().copied().all(is_chunk_token_byte)
        || !value.iter().copied().all(is_chunk_field_value_byte)
        || key.eq_ignore_ascii_case(CONTENT_LENGTH.as_bytes())
        || key.eq_ignore_ascii_case(HTTP_HEADER_TRANSFER_ENCODING.as_bytes())
        || key.eq_ignore_ascii_case(HOST.as_bytes())
    {
        return Err(INVALID_CHUNK);
    }
    if key.len() > config.get_max_header_key_size() {
        return Err(RequestError::HeaderKeyTooLong(
            HttpStatus::RequestHeaderFieldsTooLarge,
        ));
    }
    let value: &[u8] = trim_chunk_ows(value);
    let end: usize = value
        .iter()
        .rposition(|byte: &u8| !is_chunk_ows(*byte))
        .map_or(0, |index: usize| index + 1);
    if end > config.get_max_header_value_size() {
        return Err(RequestError::HeaderValueTooLong(
            HttpStatus::RequestHeaderFieldsTooLarge,
        ));
    }
    Ok(())
}

/// Skips one nonempty HTTP token.
///
/// # Arguments
///
/// - `&[u8]` - Input positioned at the start of the token.
///
/// # Returns
///
/// - `Result<&[u8], RequestError>` - The suffix after the token, or a framing error.
fn skip_chunk_token(input: &[u8]) -> Result<&[u8], RequestError> {
    let end: usize = input
        .iter()
        .position(|byte: &u8| !is_chunk_token_byte(*byte))
        .unwrap_or(input.len());
    if end == 0 {
        return Err(INVALID_CHUNK);
    }
    Ok(&input[end..])
}

/// Removes leading spaces and horizontal tabs from a metadata field.
///
/// # Arguments
///
/// - `&[u8]` - The bytes to trim.
///
/// # Returns
///
/// - `&[u8]` - The remaining bytes.
fn trim_chunk_ows(input: &[u8]) -> &[u8] {
    let start: usize = input
        .iter()
        .position(|byte: &u8| !is_chunk_ows(*byte))
        .unwrap_or(input.len());
    &input[start..]
}

/// Checks if a byte is HTTP optional whitespace.
///
/// # Arguments
///
/// - `u8` - The byte to check.
///
/// # Returns
///
/// - `bool` - Whether the byte is a space or horizontal tab.
fn is_chunk_ows(byte: u8) -> bool {
    byte == b' ' || byte == HTTP_HEADER_TAB
}

/// Checks if a byte can appear within an HTTP token.
///
/// # Arguments
///
/// - `u8` - The byte to check.
///
/// # Returns
///
/// - `bool` - Whether the byte may appear in a token.
fn is_chunk_token_byte(byte: u8) -> bool {
    byte.is_ascii_alphanumeric() || HTTP_HEADER_TOKEN_PUNCTUATION.contains(&byte)
}

/// Checks if a byte is visible, obsolete text, or allowed whitespace.
///
/// # Arguments
///
/// - `u8` - The byte to check.
///
/// # Returns
///
/// - `bool` - Whether the byte can appear in a trailer value or a quoted extension.
fn is_chunk_field_value_byte(byte: u8) -> bool {
    (byte >= HTTP_HEADER_CONTROL_LIMIT || byte == HTTP_HEADER_TAB) && byte != HTTP_HEADER_DELETE
}
