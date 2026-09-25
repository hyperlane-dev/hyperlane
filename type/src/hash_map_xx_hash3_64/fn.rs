use super::*;

/// Creates a new `HashMapXxHash3_64` with the default hasher.
///
/// This function initializes a hash map that uses `xxHash3_64` as its hashing algorithm,
/// providing efficient hashing for keys.
///
/// # Returns
///
/// A new `HashMapXxHash3_64` instance.
#[inline(always)]
pub fn hash_map_xx_hash3_64<K: Eq + Hash, V>() -> HashMapXxHash3_64<K, V> {
    HashMap::with_hasher(BuildHasherDefault::default())
}
