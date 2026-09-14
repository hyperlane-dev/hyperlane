use super::*;

/// Creates a new `HashSetXxHash3_64` with the default hasher.
///
/// # Arguments
///
/// - `K: Eq + Hash` - The type of the elements in the hash set.
///
/// # Returns
///
/// A new `HashSetXxHash3_64` instance.
#[inline(always)]
pub fn hash_set_xx_hash3_64<K: Eq + Hash>() -> HashSetXxHash3_64<K> {
    HashSet::with_hasher(BuildHasherDefault::default())
}
