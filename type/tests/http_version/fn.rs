use super::*;

#[test]
fn test_http_version_display() {
    assert_eq!(HttpVersion::Http0_9.to_string(), HTTP_VERSION_0_9);
    assert_eq!(HttpVersion::Http1_0.to_string(), HTTP_VERSION_1_0);
    assert_eq!(HttpVersion::Http1_1.to_string(), HTTP_VERSION_1_1);
    assert_eq!(HttpVersion::Http2.to_string(), HTTP_VERSION_2);
    assert_eq!(HttpVersion::Http3.to_string(), HTTP_VERSION_3);
    assert_eq!(
        HttpVersion::Unknown("HTTP/4.0".to_string()).to_string(),
        "HTTP/4.0"
    );
}

#[test]
fn test_http_version_default() {
    assert_eq!(HttpVersion::default(), HttpVersion::Http1_1);
}

#[test]
fn test_http_version_clone() {
    let version: HttpVersion = HttpVersion::Http1_1;
    let cloned_version: HttpVersion = version.clone();
    assert_eq!(version, cloned_version);
}

#[test]
fn test_http_version_debug() {
    let version: HttpVersion = HttpVersion::Http1_1;
    let debug_str: String = format!("{version:?}");
    assert_eq!(debug_str, "Http1_1");
}

#[test]
fn test_http_version_equality() {
    assert_eq!(HttpVersion::Http1_1, HttpVersion::Http1_1);
    assert_ne!(HttpVersion::Http1_1, HttpVersion::Http2);
    assert_eq!(HttpVersion::Http0_9, HttpVersion::Http0_9);
    assert_ne!(HttpVersion::Http1_0, HttpVersion::Http1_1);
    assert_eq!(
        HttpVersion::Unknown("test".to_string()),
        HttpVersion::Unknown("test".to_string())
    );
    assert_ne!(
        HttpVersion::Unknown("test1".to_string()),
        HttpVersion::Unknown("test2".to_string())
    );
}

#[test]
fn test_http_version_all_variants() {
    let versions: Vec<HttpVersion> = vec![
        HttpVersion::Http0_9,
        HttpVersion::Http1_0,
        HttpVersion::Http1_1,
        HttpVersion::Http2,
        HttpVersion::Http3,
        HttpVersion::Unknown("HTTP/4.0".to_string()),
    ];
    for version in versions {
        let display_str: String = version.to_string();
        assert!(!display_str.is_empty());
        let debug_str: String = format!("{version:?}");
        assert!(!debug_str.is_empty());
    }
}

#[test]
fn test_http_version_unknown_with_empty_string() {
    let version: HttpVersion = HttpVersion::Unknown("".to_string());
    assert_eq!(version.to_string(), "");
    assert_eq!(format!("{version:?}"), "Unknown(\"\")");
}

#[test]
fn test_http_version_unknown_with_special_characters() {
    let version: HttpVersion = HttpVersion::Unknown("HTTP/1.1-custom".to_string());
    assert_eq!(version.to_string(), "HTTP/1.1-custom");
    assert_eq!(format!("{version:?}"), "Unknown(\"HTTP/1.1-custom\")");
}

#[test]
fn test_http_version_pattern_matching() {
    let version: HttpVersion = HttpVersion::Http1_1;
    match version {
        HttpVersion::Http0_9 => panic!("Should not match HTTP0_9"),
        HttpVersion::Http1_0 => panic!("Should not match HTTP1_0"),
        HttpVersion::Http1_1 => {}
        HttpVersion::Http2 => panic!("Should not match HTTP2"),
        HttpVersion::Http3 => panic!("Should not match HTTP3"),
        HttpVersion::Unknown(_) => panic!("Should not match Unknown"),
    }
}

#[test]
fn test_http_version_unknown_pattern_matching() {
    let version: HttpVersion = HttpVersion::Unknown("custom".to_string());
    match version {
        HttpVersion::Http0_9 => panic!("Should not match HTTP0_9"),
        HttpVersion::Http1_0 => panic!("Should not match HTTP1_0"),
        HttpVersion::Http1_1 => panic!("Should not match HTTP1_1"),
        HttpVersion::Http2 => panic!("Should not match HTTP2"),
        HttpVersion::Http3 => panic!("Should not match HTTP3"),
        HttpVersion::Unknown(ref custom_version) => {
            assert_eq!(custom_version, "custom");
        }
    }
}

#[test]
fn test_http_version_is_http1() {
    assert!(matches!(HttpVersion::Http1_0, HttpVersion::Http1_0));
    assert!(matches!(HttpVersion::Http1_1, HttpVersion::Http1_1));
    assert!(!matches!(
        HttpVersion::Http0_9,
        HttpVersion::Http1_0 | HttpVersion::Http1_1
    ));
    assert!(!matches!(
        HttpVersion::Http2,
        HttpVersion::Http1_0 | HttpVersion::Http1_1
    ));
    assert!(!matches!(
        HttpVersion::Http3,
        HttpVersion::Http1_0 | HttpVersion::Http1_1
    ));
}

#[test]
fn test_http_version_is_modern() {
    assert!(matches!(
        HttpVersion::Http2,
        HttpVersion::Http2 | HttpVersion::Http3
    ));
    assert!(matches!(
        HttpVersion::Http3,
        HttpVersion::Http2 | HttpVersion::Http3
    ));
    assert!(!matches!(
        HttpVersion::Http0_9,
        HttpVersion::Http2 | HttpVersion::Http3
    ));
    assert!(!matches!(
        HttpVersion::Http1_0,
        HttpVersion::Http2 | HttpVersion::Http3
    ));
    assert!(!matches!(
        HttpVersion::Http1_1,
        HttpVersion::Http2 | HttpVersion::Http3
    ));
}

#[test]
fn test_http_version_ordering() {
    let mut versions: Vec<HttpVersion> = vec![
        HttpVersion::Http3,
        HttpVersion::Http1_0,
        HttpVersion::Http2,
        HttpVersion::Http0_9,
        HttpVersion::Http1_1,
    ];
    versions.sort_by(|a, b| {
        let order_a: u8 = match a {
            HttpVersion::Http0_9 => 0,
            HttpVersion::Http1_0 => 1,
            HttpVersion::Http1_1 => 2,
            HttpVersion::Http2 => 3,
            HttpVersion::Http3 => 4,
            HttpVersion::Unknown(_) => 255,
        };
        let order_b: u8 = match b {
            HttpVersion::Http0_9 => 0,
            HttpVersion::Http1_0 => 1,
            HttpVersion::Http1_1 => 2,
            HttpVersion::Http2 => 3,
            HttpVersion::Http3 => 4,
            HttpVersion::Unknown(_) => 255,
        };
        order_a.cmp(&order_b)
    });
    assert_eq!(versions[0], HttpVersion::Http0_9);
    assert_eq!(versions[1], HttpVersion::Http1_0);
    assert_eq!(versions[2], HttpVersion::Http1_1);
    assert_eq!(versions[3], HttpVersion::Http2);
    assert_eq!(versions[4], HttpVersion::Http3);
}

#[test]
fn test_http_version_memory_size() {
    use std::mem;
    let size: usize = mem::size_of::<HttpVersion>();
    assert!(size > 0);
    let http11_size: usize = mem::size_of_val(&HttpVersion::Http1_1);
    let unknown_size: usize = mem::size_of_val(&HttpVersion::Unknown("test".to_string()));
    assert_eq!(http11_size, unknown_size);
}
