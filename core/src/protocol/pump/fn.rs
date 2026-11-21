use super::*;

/// Takes a recycled context from the worker-local pool, or builds a fresh one.
///
/// # Returns
///
/// - `Context` - A clean context ready for one request or connection.
pub(crate) fn take_pooled_context() -> Context {
    CONTEXT_POOL.with(|pool: &RefCell<Vec<Context>>| pool.borrow_mut().pop().unwrap_or_default())
}

/// Resets and returns a context to the worker-local pool for reuse.
///
/// Oversized buffers are released before pooling so a single large request or
/// response never pins excessive memory on a worker thread.
///
/// # Arguments
///
/// - `Context` - The context to recycle.
pub(crate) fn return_pooled_context(mut ctx: Context) {
    {
        let headers: &mut RequestHeaders = ctx.get_mut_request().get_mut_headers();
        if !headers.is_empty() {
            PROTOCOL_HEADER_POOL.with(|pool: &RefCell<Vec<(String, VecDeque<String>)>>| {
                let mut pool: RefMut<'_, Vec<(String, VecDeque<String>)>> = pool.borrow_mut();
                for (key, values) in headers.drain() {
                    if pool.len() < MAX_POOLED_PROTOCOL_HEADERS
                        && key.capacity() <= MAX_POOLED_STRING_CAPACITY
                        && values.capacity() <= B_4
                        && values
                            .iter()
                            .all(|value: &String| value.capacity() <= MAX_POOLED_STRING_CAPACITY)
                    {
                        pool.push((key, values));
                    }
                }
            });
        }
    }
    ctx.get_mut_request().reset();
    if ctx.get_request().get_headers().capacity() > MAX_POOLED_PROTOCOL_HEADERS {
        *ctx.get_mut_request().get_mut_headers() = RequestHeaders::default();
    }
    if ctx.get_request().get_querys().capacity() > MAX_POOLED_PROTOCOL_HEADERS {
        *ctx.get_mut_request().get_mut_querys() = RequestQuerys::default();
    }
    if ctx.get_request().get_body().capacity() > MAX_POOLED_BUFFER_CAPACITY {
        *ctx.get_mut_request().get_mut_body() = Vec::new();
    }
    if ctx.get_request().get_path().capacity() > MAX_POOLED_STRING_CAPACITY {
        *ctx.get_mut_request().get_mut_path() = String::new();
    }
    if ctx.get_request().get_host().capacity() > MAX_POOLED_STRING_CAPACITY {
        *ctx.get_mut_request().get_mut_host() = String::new();
    }
    ctx.get_mut_response().reset();
    if ctx.get_response().get_headers().capacity() > MAX_POOLED_PROTOCOL_HEADERS {
        *ctx.get_mut_response().get_mut_headers() = ResponseHeaders::default();
    }
    if ctx.get_response().get_reason_phrase().capacity() > MAX_POOLED_STRING_CAPACITY {
        *ctx.get_mut_response().get_mut_reason_phrase() = String::new();
    }
    if ctx.get_response().get_body().capacity() > MAX_POOLED_BUFFER_CAPACITY {
        *ctx.get_mut_response().get_mut_body() = Vec::new();
    }
    ctx.clear_route_params();
    ctx.reset_request_attributes();
    ctx.set_original_head(None);
    if let Some(target) = ctx.take_original_target() {
        return_target_buffer(target);
    }
    CONTEXT_POOL.with(|pool: &RefCell<Vec<Context>>| {
        let mut pool: RefMut<'_, Vec<Context>> = pool.borrow_mut();
        if pool.len() < MAX_POOLED_CONTEXTS {
            pool.push(ctx);
        }
    });
}

/// Takes a recycled protocol header entry from the worker-local pool.
///
/// # Returns
///
/// - `Option<(String, VecDeque<String>)>` - A recycled header name and value list.
pub(crate) fn take_protocol_header_entry() -> Option<(String, VecDeque<String>)> {
    PROTOCOL_HEADER_POOL
        .with(|pool: &RefCell<Vec<(String, VecDeque<String>)>>| pool.borrow_mut().pop())
}

/// Takes a recycled target string buffer from the worker-local pool.
///
/// # Returns
///
/// - `String` - An empty string with retained capacity when available.
pub(crate) fn take_target_buffer() -> String {
    TARGET_BUFFER_POOL.with(|pool: &RefCell<Vec<String>>| {
        let mut buffer: String = pool.borrow_mut().pop().unwrap_or_default();
        buffer.clear();
        buffer
    })
}

/// Returns a target string buffer to the worker-local pool.
///
/// # Arguments
///
/// - `String` - The target buffer to recycle.
pub(crate) fn return_target_buffer(mut buffer: String) {
    if buffer.capacity() == 0 || buffer.capacity() > MAX_POOLED_STRING_CAPACITY {
        return;
    }
    buffer.clear();
    TARGET_BUFFER_POOL.with(|pool: &RefCell<Vec<String>>| {
        let mut pool: RefMut<'_, Vec<String>> = pool.borrow_mut();
        if pool.len() < MAX_POOLED_CONTEXTS {
            pool.push(buffer);
        }
    });
}

/// Takes a recycled route parameter key/value pair from the worker-local pool.
///
/// # Returns
///
/// - `(String, String)` - A pair of cleared strings retaining previous capacity.
pub(crate) fn take_route_param_entry() -> (String, String) {
    ROUTE_PARAM_POOL.with(|pool: &RefCell<Vec<(String, String)>>| {
        let mut pair: (String, String) = pool.borrow_mut().pop().unwrap_or_default();
        pair.0.clear();
        pair.1.clear();
        pair
    })
}

/// Returns a route parameter key/value pair to the worker-local pool.
///
/// # Arguments
///
/// - `(String, String)` - The key/value strings to recycle.
pub(crate) fn return_route_param_entry(mut pair: (String, String)) {
    if pair.0.capacity() > MAX_POOLED_STRING_CAPACITY
        || pair.1.capacity() > MAX_POOLED_STRING_CAPACITY
    {
        return;
    }
    pair.0.clear();
    pair.1.clear();
    ROUTE_PARAM_POOL.with(|pool: &RefCell<Vec<(String, String)>>| {
        let mut pool: RefMut<'_, Vec<(String, String)>> = pool.borrow_mut();
        if pool.len() < MAX_POOLED_PROTOCOL_HEADERS {
            pool.push(pair);
        }
    });
}

/// Polls a future once before arming a timeout on the hot path.
///
/// # Arguments
///
/// - `Pin<&mut F>` - The future to poll.
///
/// # Returns
///
/// - `Option<F::Output>` - `Some` when the future completed immediately, `None` when still pending.
pub(crate) async fn poll_once_protocol<F>(mut future: Pin<&mut F>) -> Option<F::Output>
where
    F: Future,
{
    poll_fn(
        |context: &mut task::Context<'_>| match future.as_mut().poll(context) {
            Poll::Ready(value) => Poll::Ready(Some(value)),
            Poll::Pending => Poll::Ready(None),
        },
    )
    .await
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
