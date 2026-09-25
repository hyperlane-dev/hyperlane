use super::*;

#[test]
fn test_any_send_impl_for_i32() {
    fn assert_any_send<T: AnySend>() {}
    assert_any_send::<i32>();
}

#[test]
fn test_any_send_impl_for_string() {
    fn assert_any_send<T: AnySend>() {}
    assert_any_send::<String>();
}

#[test]
fn test_any_send_clone_impl_for_i32() {
    fn assert_any_send_clone<T: AnySendClone>() {}
    assert_any_send_clone::<i32>();
}

#[test]
fn test_any_send_clone_impl_for_string() {
    fn assert_any_send_clone<T: AnySendClone>() {}
    assert_any_send_clone::<String>();
}

#[test]
fn test_any_sync_impl_for_i32() {
    fn assert_any_sync<T: AnySync>() {}
    assert_any_sync::<i32>();
}

#[test]
fn test_any_sync_impl_for_string() {
    fn assert_any_sync<T: AnySync>() {}
    assert_any_sync::<String>();
}

#[test]
fn test_any_sync_clone_impl_for_i32() {
    fn assert_any_sync_clone<T: AnySyncClone>() {}
    assert_any_sync_clone::<i32>();
}

#[test]
fn test_any_sync_clone_impl_for_string() {
    fn assert_any_sync_clone<T: AnySyncClone>() {}
    assert_any_sync_clone::<String>();
}

#[test]
fn test_any_send_sync_impl_for_i32() {
    fn assert_any_send_sync<T: AnySendSync>() {}
    assert_any_send_sync::<i32>();
}

#[test]
fn test_any_send_sync_impl_for_string() {
    fn assert_any_send_sync<T: AnySendSync>() {}
    assert_any_send_sync::<String>();
}

#[test]
fn test_any_send_sync_clone_impl_for_i32() {
    fn assert_any_send_sync_clone<T: AnySendSyncClone>() {}
    assert_any_send_sync_clone::<i32>();
}

#[test]
fn test_any_send_sync_clone_impl_for_string() {
    fn assert_any_send_sync_clone<T: AnySendSyncClone>() {}
    assert_any_send_sync_clone::<String>();
}

#[test]
fn test_box_any_downcast() {
    let boxed: BoxAny = Box::new(42_i32);
    let result: Option<&i32> = boxed.downcast_ref::<i32>();
    assert_eq!(result, Some(&42));
}

#[test]
fn test_box_any_send_downcast() {
    let boxed: BoxAnySend = Box::new(42_i32);
    let result: Option<&i32> = boxed.downcast_ref::<i32>();
    assert_eq!(result, Some(&42));
}

#[test]
fn test_box_any_send_sync_downcast() {
    let boxed: BoxAnySendSync = Box::new(42_i32);
    let result: Option<&i32> = boxed.downcast_ref::<i32>();
    assert_eq!(result, Some(&42));
}

#[test]
fn test_rc_any_downcast() {
    let rc: RcAny = Rc::new(42_i32);
    let result: Option<&i32> = rc.downcast_ref::<i32>();
    assert_eq!(result, Some(&42));
}

#[test]
fn test_arc_any_downcast() {
    let arc: ArcAny = Arc::new(42_i32);
    let result: Option<&i32> = arc.downcast_ref::<i32>();
    assert_eq!(result, Some(&42));
}

#[test]
fn test_arc_any_send_downcast() {
    let arc: ArcAnySend = Arc::new(42_i32);
    let result: Option<&i32> = arc.downcast_ref::<i32>();
    assert_eq!(result, Some(&42));
}

#[test]
fn test_arc_any_send_sync_downcast() {
    let arc: ArcAnySendSync = Arc::new(42_i32);
    let result: Option<&i32> = arc.downcast_ref::<i32>();
    assert_eq!(result, Some(&42));
}

#[test]
fn test_box_any_downcast_wrong_type() {
    let boxed: BoxAny = Box::new(42_i32);
    let result: Option<&String> = boxed.downcast_ref::<String>();
    assert!(result.is_none());
}

#[test]
fn test_rc_any_downcast_wrong_type() {
    let rc: RcAny = Rc::new(42_i32);
    let result: Option<&String> = rc.downcast_ref::<String>();
    assert!(result.is_none());
}

#[test]
fn test_arc_any_downcast_wrong_type() {
    let arc: ArcAny = Arc::new(42_i32);
    let result: Option<&String> = arc.downcast_ref::<String>();
    assert!(result.is_none());
}

#[test]
fn test_box_any_with_string() {
    let boxed: BoxAny = Box::new("hello".to_string());
    let result: Option<&String> = boxed.downcast_ref::<String>();
    assert_eq!(result, Some(&"hello".to_string()));
}

#[test]
fn test_arc_any_send_with_string() {
    let arc: ArcAnySend = Arc::new("hello".to_string());
    let result: Option<&String> = arc.downcast_ref::<String>();
    assert_eq!(result, Some(&"hello".to_string()));
}
