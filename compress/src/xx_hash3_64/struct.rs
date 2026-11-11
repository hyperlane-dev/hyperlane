use super::*;

/// A XXH3-64 hasher that keeps its secret on the stack.
///
/// The hasher `twox_hash` ships by default allocates its 192-byte secret on
/// the heap, so every hash operation costs one allocation. A request that
/// looks a header up in a map pays that allocation per lookup, which is why
/// the allocating hasher is the single largest source of allocations on the
/// request path.
///
/// This hasher keeps the same secret and produces the same hash values, but
/// holds the secret in the hasher itself, so hashing allocates nothing.
///
/// # Examples
///
/// ```
/// use http_compress::XxHash3_64;
/// use std::hash::Hasher;
///
/// let mut hasher: XxHash3_64 = XxHash3_64::default();
/// hasher.write(b"connection");
/// let _: u64 = hasher.finish();
/// ```
#[derive(Clone)]
pub struct XxHash3_64 {
    /// The hasher that holds the secret, the seed and the buffer.
    pub(super) inner: RawHasher<&'static [u8; DEFAULT_SECRET_LENGTH]>,
}
