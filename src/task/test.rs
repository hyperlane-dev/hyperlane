use crate::*;

#[tokio::test]
async fn task_clone() {
    let task1: Task = Task::default();
    let task2: Task = task1.clone();
    assert_eq!(task1.get_pool().len(), task2.get_pool().len());
    assert_eq!(
        task1.get_counter().load(atomic::Ordering::Relaxed),
        task2.get_counter().load(atomic::Ordering::Relaxed)
    );
    assert_eq!(
        task1.get_shutdown().load(atomic::Ordering::Relaxed),
        task2.get_shutdown().load(atomic::Ordering::Relaxed)
    );
}

#[tokio::test]
async fn task_default() {
    let task: Task = Task::default();
    let worker_count: usize = Handle::current().metrics().num_workers();
    if worker_count > 0 {
        assert_eq!(task.get_pool().len(), worker_count);
    }
    assert_eq!(task.get_counter().load(atomic::Ordering::Relaxed), 0);
    assert!(!task.get_shutdown().load(atomic::Ordering::Relaxed));
}

#[tokio::test]
async fn task_try_spawn_local_with_index() {
    let task: Task = Task::default();
    if task.get_pool().is_empty() {
        return;
    }
    let result: bool = task.try_spawn_local(Some(0), async move {});
    assert!(result);
}

#[tokio::test]
async fn task_try_spawn_local_round_robin() {
    let task: Task = Task::default();
    if task.get_pool().is_empty() {
        return;
    }
    let result: bool = task.try_spawn_local(None, async move {});
    assert!(result);
}

#[tokio::test]
async fn task_try_spawn_local_empty_pool() {
    let task: Task = Task {
        pool: Vec::new(),
        counter: Arc::new(AtomicUsize::new(0)),
        shutdown: Arc::new(AtomicBool::new(false)),
        notifies: Vec::new(),
    };
    let result: bool = task.try_spawn_local(None, async move {});
    assert!(!result);
}

#[tokio::test]
async fn task_try_spawn_local_with_large_index() {
    let task: Task = Task::default();
    if task.get_pool().is_empty() {
        return;
    }
    let large_index: usize = task.get_pool().len() + 100;
    let result: bool = task.try_spawn_local(Some(large_index), async move {});
    assert!(result);
}

#[tokio::test]
async fn task_shutdown() {
    let task: Task = Task::default();
    assert!(!task.get_shutdown().load(atomic::Ordering::Relaxed));
    task.shutdown();
    assert!(task.get_shutdown().load(atomic::Ordering::Relaxed));
}
