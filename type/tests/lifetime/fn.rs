use super::*;

#[test]
fn test_lifetime_trait_leak() {
    let data: TestLifetimeStruct = TestLifetimeStruct { value: 42 };
    let leaked: &'static TestLifetimeStruct = unsafe { data.leak() };
    assert_eq!(leaked.value, 42);
}

#[test]
fn test_lifetime_trait_leak_mut() {
    let data: TestLifetimeStruct = TestLifetimeStruct { value: 10 };
    let leaked: &'static mut TestLifetimeStruct = unsafe { data.leak_mut() };
    assert_eq!(leaked.value, 10);
    leaked.value = 99;
    assert_eq!(leaked.value, 99);
}

#[test]
fn test_lifetime_trait_leak_preserves_value() {
    let data: TestLifetimeStruct = TestLifetimeStruct { value: 100 };
    let leaked: &'static TestLifetimeStruct = unsafe { data.leak() };
    assert_eq!(leaked.value, 100);
}
