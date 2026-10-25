use super::*;

/// The number of header entries the pool keeps for reuse.
pub(crate) const MAX_POOLED_HEADER_ENTRIES: usize = B_128;

/// Takes a recycled header entry from the pool.
///
/// # Returns
///
/// - `Option<(String, VecDeque<String>)>` - The recycled entry, or `None` when
///   the pool is empty.
pub(crate) fn take_header_entry() -> Option<(String, VecDeque<String>)> {
    HEADER_ENTRY_POOL
        .with(|pool: &RefCell<Vec<(String, VecDeque<String>)>>| pool.borrow_mut().pop())
}

/// Returns a header entry to the pool for reuse.
///
/// # Arguments
///
/// - `(String, VecDeque<String>)` - The entry to recycle.
pub(crate) fn return_header_entry(entry: (String, VecDeque<String>)) {
    HEADER_ENTRY_POOL.with(|pool: &RefCell<Vec<(String, VecDeque<String>)>>| {
        let mut pool: RefMut<'_, Vec<(String, VecDeque<String>)>> = pool.borrow_mut();
        if pool.len() < MAX_POOLED_HEADER_ENTRIES {
            pool.push(entry);
        }
    });
}

/// Takes a recycled string from the pool.
///
/// # Returns
///
/// - `Option<String>` - The recycled string, or `None` when the pool is empty.
pub(crate) fn take_recycled_string() -> Option<String> {
    STRING_POOL.with(|pool: &RefCell<Vec<String>>| pool.borrow_mut().pop())
}

/// Returns a string to the pool for reuse.
///
/// # Arguments
///
/// - `String` - The string to recycle.
pub(crate) fn return_recycled_string(value: String) {
    STRING_POOL.with(|pool: &RefCell<Vec<String>>| {
        let mut pool: RefMut<'_, Vec<String>> = pool.borrow_mut();
        if pool.len() < MAX_POOLED_HEADER_ENTRIES {
            pool.push(value);
        }
    });
}

/// Polls a future exactly once.
///
/// # Arguments
///
/// - `Pin<&mut F>` - The future to poll.
///
/// # Returns
///
/// - `Option<F::Output>` - `Some` with the value when the future completed on
///   that first poll, `None` when it is still pending. A pending future stays
///   registered with its waker, so it can be awaited normally afterwards.
pub(crate) async fn poll_once<F>(mut future: Pin<&mut F>) -> Option<F::Output>
where
    F: Future,
{
    future::poll_fn(
        |context: &mut Context<'_>| match future.as_mut().poll(context) {
            Poll::Ready(value) => Poll::Ready(Some(value)),
            Poll::Pending => Poll::Ready(None),
        },
    )
    .await
}

/// Takes a read buffer from the thread-local pool or allocates a new one.
///
/// # Arguments
///
/// - `usize` - The required buffer length in bytes.
///
/// # Returns
///
/// - `Vec<u8>` - A buffer with at least the requested length.
pub(crate) fn take_read_buffer(capacity: usize) -> Vec<u8> {
    READ_BUFFER_POOL.with(|pool: &RefCell<Vec<Vec<u8>>>| {
        let mut pool: RefMut<'_, Vec<Vec<u8>>> = pool.borrow_mut();
        match pool.pop() {
            Some(mut buffer) => {
                if buffer.len() < capacity {
                    buffer.resize(capacity, 0);
                }
                buffer
            }
            None => vec![0; capacity],
        }
    })
}

/// Returns a read buffer to the thread-local pool for reuse.
///
/// Buffers larger than `MAX_POOLED_READ_BUFFER_SIZE` are dropped instead
/// of being retained, and the pool is capped at `MAX_POOLED_READ_BUFFERS`.
///
/// # Arguments
///
/// - `Vec<u8>` - The buffer to recycle.
pub(crate) fn return_read_buffer(buffer: Vec<u8>) {
    if buffer.is_empty() || buffer.capacity() > MAX_POOLED_READ_BUFFER_SIZE {
        return;
    }
    READ_BUFFER_POOL.with(|pool: &RefCell<Vec<Vec<u8>>>| {
        let mut pool: RefMut<'_, Vec<Vec<u8>>> = pool.borrow_mut();
        if pool.len() < MAX_POOLED_READ_BUFFERS {
            pool.push(buffer);
        }
    });
}
