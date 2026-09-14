use super::*;

pub type HashMapXxHash3_64<K, V> = HashMap<K, V, BuildHasherDefault<XxHash3_64>>;
