use super::*;

#[tokio::test]
async fn test_rc_rwlock_type() {
    let data: RcRwLock<i32> = rc_rwlock(42);
    let guard: RwLockReadGuard<'_, i32> = data.read().await;
    assert_eq!(*guard, 42);
}

#[tokio::test]
async fn test_rc_rwlock_write() {
    let data: RcRwLock<i32> = rc_rwlock(42);
    {
        let mut guard: RwLockWriteGuard<'_, i32> = data.write().await;
        *guard = 100;
    }
    let guard: RwLockReadGuard<'_, i32> = data.read().await;
    assert_eq!(*guard, 100);
}

#[tokio::test]
async fn test_rc_rwlock_with_string() {
    let data: RcRwLock<String> = rc_rwlock("hello".to_string());
    {
        let mut guard: RwLockWriteGuard<'_, String> = data.write().await;
        guard.push_str(" world");
    }
    let guard: RwLockReadGuard<'_, String> = data.read().await;
    assert_eq!(*guard, "hello world");
}

#[test]
fn test_rc_rwlock_clone() {
    let data: RcRwLock<i32> = rc_rwlock(10);
    let cloned_data: RcRwLock<i32> = Rc::clone(&data);
    assert_eq!(Rc::strong_count(&data), 2);
    assert_eq!(Rc::strong_count(&cloned_data), 2);
}

#[test]
fn test_rc_rwlock_not_send() {
    fn assert_not_send<T>() {}
    assert_not_send::<RcRwLock<i32>>();
}
