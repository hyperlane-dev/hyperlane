use super::*;

#[tokio::test]
async fn test_box_rwlock_type() {
    let data: BoxRwLock<i32> = box_rwlock(42);
    let guard: RwLockReadGuard<'_, i32> = data.read().await;
    assert_eq!(*guard, 42);
}

#[tokio::test]
async fn test_box_rwlock_write() {
    let data: BoxRwLock<i32> = box_rwlock(42);
    {
        let mut guard: RwLockWriteGuard<'_, i32> = data.write().await;
        *guard = 100;
    }
    let guard: RwLockReadGuard<'_, i32> = data.read().await;
    assert_eq!(*guard, 100);
}

#[tokio::test]
async fn test_box_rwlock_with_string() {
    let data: BoxRwLock<String> = box_rwlock("hello".to_string());
    {
        let mut guard: RwLockWriteGuard<'_, String> = data.write().await;
        guard.push_str(" world");
    }
    let guard: RwLockReadGuard<'_, String> = data.read().await;
    assert_eq!(*guard, "hello world");
}

#[test]
fn test_box_rwlock_is_send() {
    fn assert_send<T: Send>() {}
    assert_send::<BoxRwLock<i32>>();
    assert_send::<BoxRwLock<String>>();
}
