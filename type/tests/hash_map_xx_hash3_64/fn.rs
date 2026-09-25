use super::*;

#[test]
fn test_hash_map_xx_hash3_64_new() {
    let map: HashMapXxHash3_64<String, i32> = hash_map_xx_hash3_64();
    assert!(map.is_empty());
    assert_eq!(map.len(), 0);
}

#[test]
fn test_hash_map_xx_hash3_64_insert_and_get() {
    let mut map: HashMapXxHash3_64<String, i32> = hash_map_xx_hash3_64();
    map.insert("key1".to_string(), 100);
    map.insert("key2".to_string(), 200);
    assert_eq!(map.get("key1"), Some(&100));
    assert_eq!(map.get("key2"), Some(&200));
    assert_eq!(map.get("key3"), None);
}

#[test]
fn test_hash_map_xx_hash3_64_remove() {
    let mut map: HashMapXxHash3_64<String, i32> = hash_map_xx_hash3_64();
    map.insert("key1".to_string(), 100);
    assert_eq!(map.remove("key1"), Some(100));
    assert_eq!(map.get("key1"), None);
}

#[test]
fn test_hash_map_xx_hash3_64_with_integer_keys() {
    let mut map: HashMapXxHash3_64<i32, String> = hash_map_xx_hash3_64();
    map.insert(1, "one".to_string());
    map.insert(2, "two".to_string());
    assert_eq!(map.get(&1), Some(&"one".to_string()));
    assert_eq!(map.get(&2), Some(&"two".to_string()));
}

#[test]
fn test_hash_map_xx_hash3_64_iter() {
    let mut map: HashMapXxHash3_64<String, i32> = hash_map_xx_hash3_64();
    map.insert("a".to_string(), 1);
    map.insert("b".to_string(), 2);
    let mut count: usize = 0;
    for _value in map.values() {
        count += 1;
    }
    assert_eq!(count, 2);
}
