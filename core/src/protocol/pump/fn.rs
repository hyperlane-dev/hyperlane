use super::*;

/// Increments a response head and forwards every body chunk that follows it.
///
/// The pump owns the write side of a protocol stream. It reads the HTTP/1.1
/// shaped bytes the request lifecycle wrote, frames the head exactly once as
/// soon as the blank line ends the header block, and forwards each chunk after
/// it as its own data frame. Running it alongside the lifecycle, rather than
/// after it, is what lets a raw send, a split send or a server sent event
/// stream reach the client before the handler returns.
///
/// The framing is decided as early as the bytes allow. A head that cannot be
/// represented on this protocol at all — one announcing a transfer coding the
/// framing does not carry, one with a status code or a header line the protocol
/// cannot encode — is refused before any frame is emitted, so nothing has been
/// committed to the wire.
///
/// A body that does not match the length it announced is a different case and
/// cannot be caught that early: a short body is only short once the pipe ends.
/// The head and the body bytes read before that point have already been framed,
/// so a truncated or overlong response surfaces as a failed pump with the
/// stream left for the caller to reset, never as a clean `finish()`. The client
/// sees a stream that ends without the bytes the head promised, which is the
/// honest signal; it is not told the response completed.
///
/// # Arguments
///
/// - `&mut F` - The `value` argument.
/// - `DuplexStream` - The read end of the pipe the lifecycle writes onto.
/// - `usize` - The maximum number of head bytes buffered while the head is pending.
/// - `bool` - Whether the request method forbids a response body.
///
/// # Returns
///
/// - `Result<(), ResponseError>` - `Ok(())` once the lifecycle dropped its end
pub(crate) async fn pump_response<F>(
    framer: &mut F,
    mut reader: DuplexStream,
    max_head_size: usize,
    is_head: bool,
) -> Result<(), ResponseError>
where
    F: ResponseFramer,
{
    let mut pending: Vec<u8> = Vec::new();
    let mut read_chunk: Vec<u8> = vec![0; PROTOCOL_BODY_CHUNK];
    let mut head: Option<FramedHead> = None;
    let mut body_len: usize = 0;
    let mut suppress_body: bool = is_head;
    let mut announced_len: Option<usize> = None;
    loop {
        let read: usize = tokio::io::AsyncReadExt::read(&mut reader, &mut read_chunk).await?;
        if read == 0 {
            break;
        }
        if head.is_none() {
            pending.extend_from_slice(&read_chunk[..read]);
            let Some(body_start) = find_head_end(&pending) else {
                if pending.len() > max_head_size {
                    return Err(ResponseError::Send(RESPONSE_HEAD_TOO_LONG.to_owned()));
                }
                continue;
            };
            if body_start > max_head_size {
                return Err(ResponseError::Send(RESPONSE_HEAD_TOO_LONG.to_owned()));
            }
            let (status_code, mut headers): (usize, ProtocolHeaders) =
                parse_response_head(&pending[..body_start])?;
            validate_status_code(status_code)?;
            if has_transfer_encoding(&headers) {
                return Err(ResponseError::Send(
                    UNSUPPORTED_TRANSFER_ENCODING.to_owned(),
                ));
            }
            announced_len = parse_content_length(&headers)?;
            suppress_body = is_bodyless_status(status_code, is_head);
            if hides_entity_length(status_code, is_head) {
                headers.remove(CONTENT_LENGTH);
            }
            let head_body: &[u8] = if suppress_body {
                &[]
            } else {
                &pending[body_start..]
            };
            let parsed: FramedHead = FramedHead {
                status_code,
                headers,
            };
            framer.send_head(&parsed, suppress_body).await?;
            if !head_body.is_empty() {
                let overflowed: bool = check_body_budget(announced_len, body_len, head_body.len());
                body_len += head_body.len();
                if overflowed {
                    return Err(ResponseError::Send(OVERFLOWING_RESPONSE_BODY.to_owned()));
                }
                framer.send_data(head_body).await?;
            }
            head = Some(parsed);
            pending.clear();
            continue;
        }
        if suppress_body {
            continue;
        }
        let overflowed: bool = check_body_budget(announced_len, body_len, read);
        body_len += read;
        if overflowed {
            return Err(ResponseError::Send(OVERFLOWING_RESPONSE_BODY.to_owned()));
        }
        framer.send_data(&read_chunk[..read]).await?;
    }
    let Some(head) = head else {
        return Err(ResponseError::Send(MISSING_RESPONSE_HEAD.to_owned()));
    };
    if !suppress_body
        && let Some(announced) = announced_len
        && body_len != announced
    {
        return Err(ResponseError::Send(TRUNCATED_RESPONSE_BODY.to_owned()));
    }
    framer.finish().await?;
    let _framed: FramedHead = head;
    let _forwarded: usize = body_len;
    Ok(())
}

/// Reports whether a response may not announce an entity length.
///
/// A length may only survive on a response that does promise a representation:
/// a `HEAD` and a `304` both describe the entity a matching `GET` would return
/// and carry no body bytes of their own, so their length is real information
/// about the message and stays. The informational statuses and `204` promise no
/// representation at all, so a length there contradicts the message itself and
/// is dropped — for `HEAD` exactly as it is for every other method, because the
/// status decides this and not the method.
///
/// # Arguments
///
/// - `usize` - The response status code.
/// - `bool` - Whether the request method was `HEAD`.
///
/// # Returns
///
/// - `bool` - `true` when the head must not carry a content length.
pub(crate) fn hides_entity_length(status_code: usize, _is_head: bool) -> bool {
    status_code < HttpStatus::Ok.code() || status_code == HttpStatus::NoContent.code()
}

/// Reports whether one more body chunk runs past the announced length.
///
/// The check happens before the chunk is forwarded, so an overlong body is
/// refused at the byte that exceeded the head rather than after the client has
/// already received a body it will treat as complete.
///
/// # Arguments
///
/// - `Option<usize>` - The length the head announced, if it announced one.
/// - `usize` - The body bytes already forwarded.
///
/// # Returns
///
/// - `bool` - `true` when forwarding them would exceed the announced length.
pub(crate) fn check_body_budget(
    announced_len: Option<usize>,
    forwarded: usize,
    incoming: usize,
) -> bool {
    match announced_len {
        Some(announced) => forwarded.saturating_add(incoming) > announced,
        None => false,
    }
}

/// Rejects a status code the protocol cannot carry on the wire.
///
/// A code outside the three digit range is not a status at all, and the framer
/// would fail on it while building the response — after the head has already
/// been accepted as valid. Checking it here keeps one status decision in one
/// place.
///
/// # Arguments
///
/// - `usize` - The parsed status code.
///
/// # Returns
///
/// - `Result<(), ResponseError>` - `Ok(())` when the protocol may carry the code.
pub(crate) fn validate_status_code(status_code: usize) -> Result<(), ResponseError> {
    let is_code: bool = (MIN_STATUS_CODE..=MAX_STATUS_CODE).contains(&status_code);
    if !is_code {
        return Err(ResponseError::Send(INVALID_RESPONSE_STATUS.to_owned()));
    }
    Ok(())
}
