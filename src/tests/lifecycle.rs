use crate::*;

#[tokio::test]
async fn test_lifecycle_new() {
    let lifecycle: Lifecycle = Lifecycle::new(true);
    assert_eq!(lifecycle, Lifecycle::Continue(true));
    assert!(lifecycle.is_keep_alive());
    assert!(!lifecycle.is_abort());
}

#[tokio::test]
async fn test_lifecycle_update_status() {
    let mut lifecycle: Lifecycle = Lifecycle::new(true);
    lifecycle.update_status(true, true);
    assert_eq!(lifecycle, Lifecycle::Abort(true));
    assert!(lifecycle.is_abort());
    assert!(lifecycle.is_keep_alive());
    lifecycle.update_status(true, false);
    assert_eq!(lifecycle, Lifecycle::Abort(false));
    assert!(lifecycle.is_abort());
    assert!(!lifecycle.is_keep_alive());
    lifecycle.update_status(false, true);
    assert_eq!(lifecycle, Lifecycle::Continue(true));
    assert!(!lifecycle.is_abort());
    assert!(lifecycle.is_keep_alive());
    lifecycle.update_status(false, false);
    assert_eq!(lifecycle, Lifecycle::Continue(false));
    assert!(!lifecycle.is_abort());
    assert!(!lifecycle.is_keep_alive());
}

#[tokio::test]
async fn test_lifecycle_is_abort() {
    let abort_true: Lifecycle = Lifecycle::Abort(true);
    assert!(abort_true.is_abort());
    let abort_false: Lifecycle = Lifecycle::Abort(false);
    assert!(abort_false.is_abort());
    let continue_true: Lifecycle = Lifecycle::Continue(true);
    assert!(!continue_true.is_abort());
    let continue_false: Lifecycle = Lifecycle::Continue(false);
    assert!(!continue_false.is_abort());
}

#[tokio::test]
async fn test_lifecycle_is_keep_alive() {
    let abort_true: Lifecycle = Lifecycle::Abort(true);
    assert!(abort_true.is_keep_alive());
    let abort_false: Lifecycle = Lifecycle::Abort(false);
    assert!(!abort_false.is_keep_alive());
    let continue_true: Lifecycle = Lifecycle::Continue(true);
    assert!(continue_true.is_keep_alive());
    let continue_false: Lifecycle = Lifecycle::Continue(false);
    assert!(!continue_false.is_keep_alive());
}

#[tokio::test]
async fn test_lifecycle_keep_alive() {
    let abort_true: Lifecycle = Lifecycle::Abort(true);
    assert!(abort_true.keep_alive());
    let abort_false: Lifecycle = Lifecycle::Abort(false);
    assert!(!abort_false.keep_alive());
    let continue_true: Lifecycle = Lifecycle::Continue(true);
    assert!(continue_true.keep_alive());
    let continue_false: Lifecycle = Lifecycle::Continue(false);
    assert!(!continue_false.keep_alive());
}
