use super::*;

#[test]
fn test_body_empty_has_no_bytes() {
    let body: Body = Body::empty();
    assert!(body.get_bytes_ref().is_empty());
    assert!(body.as_slice().is_empty());
    assert!(body.get_bytes().is_empty());
    assert_eq!(body.as_str(), Some(""));
}

#[test]
fn test_body_default_equals_empty() {
    let body: Body = Body::default();
    assert_eq!(body, Body::empty());
    assert!(body.get_bytes_ref().is_empty());
}

#[test]
fn test_body_from_bytes_accepts_str_and_vec() {
    let from_str: Body = Body::from_bytes("hi");
    let from_vec: Body = Body::from_bytes(vec![b'h', b'i']);
    assert_eq!(from_str, from_vec);
    assert_eq!(from_str.get_bytes_ref(), from_vec.get_bytes_ref());
    assert_eq!(from_str.as_str(), Some("hi"));
    assert_eq!(from_str.get_bytes_ref().len(), 2usize);
}

#[test]
fn test_body_from_bytes_accepts_slice() {
    let body: Body = Body::from_bytes(&[1u8, 2, 3][..]);
    assert_eq!(body.get_bytes_ref(), &[1u8, 2, 3]);
    assert_eq!(body.as_str(), Some("\u{1}\u{2}\u{3}"));
}

#[test]
fn test_body_as_str_is_none_for_invalid_utf8() {
    let body: Body = Body::from_bytes(vec![0xffu8, 0xfe]);
    assert_eq!(body.as_str(), None);
    assert_eq!(body.as_slice().len(), 2usize);
    assert_eq!(body.get_bytes_ref(), &[0xffu8, 0xfe]);
}

#[test]
fn test_body_display_writes_utf8_text() {
    let body: Body = Body::from_bytes("hello");
    assert_eq!(format!("{body}"), "hello");
}

#[test]
fn test_body_display_falls_back_to_debug_bytes() {
    let body: Body = Body::from_bytes(vec![0xffu8, 0xfe]);
    assert_eq!(format!("{body}"), "[255, 254]");
}

#[test]
fn test_body_clone_is_equal_and_independent() {
    let body: Body = Body::from_bytes("abc");
    let cloned: Body = body.clone();
    assert_eq!(body, cloned);
    assert_eq!(cloned.get_bytes_ref(), b"abc");
}

#[test]
fn test_body_serializes_bytes_field() {
    let body: Body = Body::from_bytes("hi");
    let encoded: String = serde_json::to_string(&body).unwrap_or_default();
    assert_eq!(encoded, "{\"bytes\":[104,105]}");
}
