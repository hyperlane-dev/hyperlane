use super::*;

/// An alias for `HashSet` that hashes with `xxHash3_64`.
///
/// The hasher keeps its secret on the stack, so a lookup costs no
/// allocation. The hasher that ships with `twox_hash` allocates its secret on
/// every hash operation, which made every header lookup on the request path
/// allocate.
pub type HashSetXxHash3_64<K> = HashSet<K, BuildHasherDefault<XxHash3_64>>;
