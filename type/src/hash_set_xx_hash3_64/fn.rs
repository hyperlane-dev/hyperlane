use super::*;

/// Creates a new `HashSetXxHash3_64` with the default hasher.
///
/// The element type is fixed by the return type and must satisfy the `Eq`
/// and `Hash` bounds.
///
/// # Returns
///
/// A new `HashSetXxHash3_64` instance.
#[inline(always)]
pub fn hash_set_xx_hash3_64<K: Eq + Hash>() -> HashSetXxHash3_64<K> {
    HashSet::with_hasher(BuildHasherDefault::default())
}
