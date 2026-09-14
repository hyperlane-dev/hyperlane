use super::*;

#[test]
fn test_status_is_continue() {
    let status: Status = Status::Continue;
    assert!(status.is_continue());
    assert!(!status.is_reject());
}

#[test]
fn test_status_is_reject() {
    let status: Status = Status::Reject;
    assert!(status.is_reject());
    assert!(!status.is_continue());
}

#[test]
fn test_status_default_is_reject() {
    let status: Status = Status::default();
    assert!(status.is_reject());
    assert_eq!(status, Status::Reject);
}

#[test]
fn test_status_equality() {
    assert_eq!(Status::Continue, Status::Continue);
    assert_eq!(Status::Reject, Status::Reject);
    assert_ne!(Status::Continue, Status::Reject);
}

#[test]
fn test_status_clone() {
    let status: Status = Status::Continue;
    let new_status: Status = status;
    assert_eq!(status, new_status);
}

#[test]
fn test_status_copy() {
    let status: Status = Status::Reject;
    let copied: Status = status;
    assert_eq!(status, copied);
}

#[test]
fn test_status_serialize_deserialize() {
    let status: Status = Status::Continue;
    let json: String = serde_json::to_string(&status).unwrap();
    let deserialized: Status = serde_json::from_str(&json).unwrap();
    assert_eq!(status, deserialized);
}

#[test]
fn test_status_serialize_deserialize_reject() {
    let status: Status = Status::Reject;
    let json: String = serde_json::to_string(&status).unwrap();
    let deserialized: Status = serde_json::from_str(&json).unwrap();
    assert_eq!(status, deserialized);
}

#[test]
fn test_status_debug_format() {
    let status: Status = Status::Continue;
    let debug_str: String = format!("{:?}", status);
    assert_eq!(debug_str, "Continue");
    let status: Status = Status::Reject;
    let debug_str: String = format!("{:?}", status);
    assert_eq!(debug_str, "Reject");
}
