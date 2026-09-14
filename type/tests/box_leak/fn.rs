use super::*;

#[test]
fn test_box_leak_new_with_integer() {
    let leaked: &'static mut i32 = box_leak_new(42);
    assert_eq!(*leaked, 42);
    *leaked = 100;
    assert_eq!(*leaked, 100);
}

#[test]
fn test_box_leak_new_with_string() {
    let leaked: &'static mut String = box_leak_new("hello".to_string());
    assert_eq!(*leaked, "hello");
    leaked.push_str(" world");
    assert_eq!(*leaked, "hello world");
}

#[test]
fn test_box_leak_new_with_vec() {
    let leaked: &'static mut Vec<i32> = box_leak_new(vec![1, 2, 3]);
    assert_eq!(*leaked, vec![1, 2, 3]);
    leaked.push(4);
    assert_eq!(*leaked, vec![1, 2, 3, 4]);
}

#[test]
fn test_box_leak_new_static_lifetime() {
    fn assert_static<T: 'static>(_: T) {}
    let leaked: &'static mut i32 = box_leak_new(42);
    assert_static(*leaked);
}
