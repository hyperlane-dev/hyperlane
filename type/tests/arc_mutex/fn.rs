use super::*;

#[tokio::test]
async fn test_arc_mutex_creation() {
    let mutex: ArcMutex<i32> = arc_mutex(42);
    let guard: tokio::sync::MutexGuard<'_, i32> = mutex.lock().await;
    assert_eq!(*guard, 42);
}

#[tokio::test]
async fn test_arc_mutex_with_string() {
    let mutex: ArcMutex<String> = arc_mutex("hello".to_string());
    let guard: tokio::sync::MutexGuard<'_, String> = mutex.lock().await;
    assert_eq!(*guard, "hello");
}

#[tokio::test]
async fn test_arc_mutex_mutation() {
    let mutex: ArcMutex<i32> = arc_mutex(0);
    {
        let mut guard: tokio::sync::MutexGuard<'_, i32> = mutex.lock().await;
        *guard = 100;
    }
    let guard: tokio::sync::MutexGuard<'_, i32> = mutex.lock().await;
    assert_eq!(*guard, 100);
}

#[tokio::test]
async fn test_arc_mutex_clone_shares_data() {
    let mutex1: ArcMutex<i32> = arc_mutex(42);
    let mutex2: ArcMutex<i32> = Arc::clone(&mutex1);
    {
        let mut guard: tokio::sync::MutexGuard<'_, i32> = mutex2.lock().await;
        *guard = 99;
    }
    let guard: tokio::sync::MutexGuard<'_, i32> = mutex1.lock().await;
    assert_eq!(*guard, 99);
}

#[tokio::test]
async fn test_arc_mutex_with_vec() {
    let mutex: ArcMutex<Vec<i32>> = arc_mutex(vec![1, 2, 3]);
    {
        let mut guard: tokio::sync::MutexGuard<'_, Vec<i32>> = mutex.lock().await;
        guard.push(4);
    }
    let guard: tokio::sync::MutexGuard<'_, Vec<i32>> = mutex.lock().await;
    assert_eq!(*guard, vec![1, 2, 3, 4]);
}

#[test]
fn test_arc_mutex_reference_count() {
    let mutex: ArcMutex<i32> = arc_mutex(42);
    assert_eq!(Arc::strong_count(&mutex), 1);
    let mutex2: ArcMutex<i32> = Arc::clone(&mutex);
    assert_eq!(Arc::strong_count(&mutex), 2);
    assert_eq!(Arc::strong_count(&mutex2), 2);
}
