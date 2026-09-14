use super::*;

#[test]
fn test_hash_set_xx_hash3_64_new() {
    let set: HashSetXxHash3_64<String> = hash_set_xx_hash3_64();
    assert!(set.is_empty());
    assert_eq!(set.len(), 0);
}

#[test]
fn test_hash_set_xx_hash3_64_insert_and_contains() {
    let mut set: HashSetXxHash3_64<String> = hash_set_xx_hash3_64();
    set.insert("item1".to_string());
    set.insert("item2".to_string());
    assert!(set.contains("item1"));
    assert!(set.contains("item2"));
    assert!(!set.contains("item3"));
}

#[test]
fn test_hash_set_xx_hash3_64_remove() {
    let mut set: HashSetXxHash3_64<String> = hash_set_xx_hash3_64();
    set.insert("item1".to_string());
    assert!(set.remove("item1"));
    assert!(!set.contains("item1"));
}

#[test]
fn test_hash_set_xx_hash3_64_with_integer_elements() {
    let mut set: HashSetXxHash3_64<i32> = hash_set_xx_hash3_64();
    set.insert(1);
    set.insert(2);
    set.insert(3);
    assert!(set.contains(&1));
    assert!(set.contains(&2));
    assert!(set.contains(&3));
    assert!(!set.contains(&4));
}

#[test]
fn test_hash_set_xx_hash3_64_iter() {
    let mut set: HashSetXxHash3_64<String> = hash_set_xx_hash3_64();
    set.insert("a".to_string());
    set.insert("b".to_string());
    let mut count: usize = 0;
    for _item in &set {
        count += 1;
    }
    assert_eq!(count, 2);
}
