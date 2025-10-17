use crate::*;

#[tokio::test]
async fn lifecycle_new() {
    let lifecycle: RequestLifecycle = RequestLifecycle::new(true);
    assert_eq!(lifecycle, RequestLifecycle::Continuing(true));
    assert!(lifecycle.is_keep_alive());
    assert!(!lifecycle.is_aborted());
}

#[tokio::test]
async fn lifecycle_update_status() {
    let mut lifecycle: RequestLifecycle = RequestLifecycle::new(true);
    lifecycle.update_status(true, true);
    assert_eq!(lifecycle, RequestLifecycle::Aborted(true));
    assert!(lifecycle.is_aborted());
    assert!(lifecycle.is_keep_alive());
    lifecycle.update_status(true, false);
    assert_eq!(lifecycle, RequestLifecycle::Aborted(false));
    assert!(lifecycle.is_aborted());
    assert!(!lifecycle.is_keep_alive());
    lifecycle.update_status(false, true);
    assert_eq!(lifecycle, RequestLifecycle::Continuing(true));
    assert!(!lifecycle.is_aborted());
    assert!(lifecycle.is_keep_alive());
    lifecycle.update_status(false, false);
    assert_eq!(lifecycle, RequestLifecycle::Continuing(false));
    assert!(!lifecycle.is_aborted());
    assert!(!lifecycle.is_keep_alive());
}

#[tokio::test]
async fn lifecycle_is_aborted() {
    let abort_true: RequestLifecycle = RequestLifecycle::Aborted(true);
    assert!(abort_true.is_aborted());
    let abort_false: RequestLifecycle = RequestLifecycle::Aborted(false);
    assert!(abort_false.is_aborted());
    let continue_true: RequestLifecycle = RequestLifecycle::Continuing(true);
    assert!(!continue_true.is_aborted());
    let continue_false: RequestLifecycle = RequestLifecycle::Continuing(false);
    assert!(!continue_false.is_aborted());
}

#[tokio::test]
async fn lifecycle_is_keep_alive() {
    let abort_true: RequestLifecycle = RequestLifecycle::Aborted(true);
    assert!(abort_true.is_keep_alive());
    let abort_false: RequestLifecycle = RequestLifecycle::Aborted(false);
    assert!(!abort_false.is_keep_alive());
    let continue_true: RequestLifecycle = RequestLifecycle::Continuing(true);
    assert!(continue_true.is_keep_alive());
    let continue_false: RequestLifecycle = RequestLifecycle::Continuing(false);
    assert!(!continue_false.is_keep_alive());
}

#[tokio::test]
async fn lifecycle_keep_alive() {
    let abort_true: RequestLifecycle = RequestLifecycle::Aborted(true);
    assert!(abort_true.keep_alive());
    let abort_false: RequestLifecycle = RequestLifecycle::Aborted(false);
    assert!(!abort_false.keep_alive());
    let continue_true: RequestLifecycle = RequestLifecycle::Continuing(true);
    assert!(continue_true.keep_alive());
    let continue_false: RequestLifecycle = RequestLifecycle::Continuing(false);
    assert!(!continue_false.keep_alive());
}
