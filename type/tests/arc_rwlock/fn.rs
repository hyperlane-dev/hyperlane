use super::*;

#[tokio::test]
async fn test_arc_rwlock_type() {
    let data: ArcRwLock<i32> = arc_rwlock(42);
    let guard: RwLockReadGuard<'_, i32> = data.read().await;
    assert_eq!(*guard, 42);
}

#[tokio::test]
async fn test_arc_rwlock_write() {
    let data: ArcRwLock<i32> = arc_rwlock(42);
    {
        let mut guard: RwLockWriteGuard<'_, i32> = data.write().await;
        *guard = 100;
    }
    let guard: RwLockReadGuard<'_, i32> = data.read().await;
    assert_eq!(*guard, 100);
}

#[tokio::test]
async fn test_arc_rwlock_with_string() {
    let data: ArcRwLock<String> = arc_rwlock("hello".to_string());
    {
        let mut guard: RwLockWriteGuard<'_, String> = data.write().await;
        guard.push_str(" world");
    }
    let guard: RwLockReadGuard<'_, String> = data.read().await;
    assert_eq!(*guard, "hello world");
}

#[tokio::test]
async fn test_arc_rwlock_clone() {
    let data: ArcRwLock<i32> = arc_rwlock(10);
    let cloned_data: ArcRwLock<i32> = Arc::clone(&data);
    {
        let mut guard: RwLockWriteGuard<'_, i32> = data.write().await;
        *guard = 20;
    }
    let guard: RwLockReadGuard<'_, i32> = cloned_data.read().await;
    assert_eq!(*guard, 20);
}

#[test]
fn test_arc_rwlock_is_send_sync() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<ArcRwLock<i32>>();
    assert_send_sync::<ArcRwLock<String>>();
}
