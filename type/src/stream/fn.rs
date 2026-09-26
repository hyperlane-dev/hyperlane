use super::*;

thread_local! {
    static READ_BUFFER_POOL: RefCell<Vec<Vec<u8>>> = const { RefCell::new(Vec::new()) };
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
    if buffer.len() > MAX_POOLED_READ_BUFFER_SIZE {
        return;
    }
    READ_BUFFER_POOL.with(|pool: &RefCell<Vec<Vec<u8>>>| {
        let mut pool: RefMut<'_, Vec<Vec<u8>>> = pool.borrow_mut();
        if pool.len() < MAX_POOLED_READ_BUFFERS {
            pool.push(buffer);
        }
    });
}
