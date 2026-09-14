use super::*;

/// A type alias for `HashSet` using `XxHash3_64` as the hasher.
pub type HashSetXxHash3_64<K> = HashSet<K, BuildHasherDefault<XxHash3_64>>;
