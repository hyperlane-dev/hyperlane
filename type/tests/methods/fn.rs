use super::*;

#[test]
fn test_method_display() {
    assert_eq!(Method::Get.to_string(), GET);
    assert_eq!(Method::Post.to_string(), POST);
    assert_eq!(Method::Put.to_string(), PUT);
    assert_eq!(Method::Delete.to_string(), DELETE);
    assert_eq!(Method::Patch.to_string(), PATCH);
    assert_eq!(Method::Head.to_string(), HEAD);
    assert_eq!(Method::Options.to_string(), OPTIONS);
    assert_eq!(Method::Connect.to_string(), CONNECT);
    assert_eq!(Method::Trace.to_string(), TRACE);
    assert_eq!(Method::Unknown("CUSTOM".to_string()).to_string(), "CUSTOM");
}

#[test]
fn test_method_from_str() {
    assert_eq!(GET.parse::<Method>().unwrap(), Method::Get);
    assert_eq!(POST.parse::<Method>().unwrap(), Method::Post);
    assert_eq!(PUT.parse::<Method>().unwrap(), Method::Put);
    assert_eq!(DELETE.parse::<Method>().unwrap(), Method::Delete);
    assert_eq!(PATCH.parse::<Method>().unwrap(), Method::Patch);
    assert_eq!(HEAD.parse::<Method>().unwrap(), Method::Head);
    assert_eq!(OPTIONS.parse::<Method>().unwrap(), Method::Options);
    assert_eq!(CONNECT.parse::<Method>().unwrap(), Method::Connect);
    assert_eq!(TRACE.parse::<Method>().unwrap(), Method::Trace);
    assert_eq!(
        "CUSTOM".parse::<Method>().unwrap(),
        Method::Unknown("CUSTOM".to_string())
    );
    assert_eq!(
        "".parse::<Method>().unwrap(),
        Method::Unknown("".to_string())
    );
}

#[test]
fn test_method_default() {
    assert_eq!(Method::default(), Method::Unknown(String::new()));
}

#[test]
fn test_method_is_get() {
    assert!(Method::Get.is_get());
    assert!(!Method::Post.is_get());
    assert!(!Method::Put.is_get());
    assert!(!Method::Delete.is_get());
    assert!(!Method::Patch.is_get());
    assert!(!Method::Head.is_get());
    assert!(!Method::Options.is_get());
    assert!(!Method::Connect.is_get());
    assert!(!Method::Trace.is_get());
    assert!(!Method::Unknown("GET".to_string()).is_get());
}

#[test]
fn test_method_is_post() {
    assert!(!Method::Get.is_post());
    assert!(Method::Post.is_post());
    assert!(!Method::Put.is_post());
    assert!(!Method::Delete.is_post());
    assert!(!Method::Patch.is_post());
    assert!(!Method::Head.is_post());
    assert!(!Method::Options.is_post());
    assert!(!Method::Connect.is_post());
    assert!(!Method::Trace.is_post());
    assert!(!Method::Unknown("POST".to_string()).is_post());
}

#[test]
fn test_method_is_put() {
    assert!(!Method::Get.is_put());
    assert!(!Method::Post.is_put());
    assert!(Method::Put.is_put());
    assert!(!Method::Delete.is_put());
    assert!(!Method::Patch.is_put());
    assert!(!Method::Head.is_put());
    assert!(!Method::Options.is_put());
    assert!(!Method::Connect.is_put());
    assert!(!Method::Trace.is_put());
    assert!(!Method::Unknown("PUT".to_string()).is_put());
}

#[test]
fn test_method_is_delete() {
    assert!(!Method::Get.is_delete());
    assert!(!Method::Post.is_delete());
    assert!(!Method::Put.is_delete());
    assert!(Method::Delete.is_delete());
    assert!(!Method::Patch.is_delete());
    assert!(!Method::Head.is_delete());
    assert!(!Method::Options.is_delete());
    assert!(!Method::Connect.is_delete());
    assert!(!Method::Trace.is_delete());
    assert!(!Method::Unknown("DELETE".to_string()).is_delete());
}

#[test]
fn test_method_is_patch() {
    assert!(!Method::Get.is_patch());
    assert!(!Method::Post.is_patch());
    assert!(!Method::Put.is_patch());
    assert!(!Method::Delete.is_patch());
    assert!(Method::Patch.is_patch());
    assert!(!Method::Head.is_patch());
    assert!(!Method::Options.is_patch());
    assert!(!Method::Connect.is_patch());
    assert!(!Method::Trace.is_patch());
    assert!(!Method::Unknown("PATCH".to_string()).is_patch());
}

#[test]
fn test_method_is_head() {
    assert!(!Method::Get.is_head());
    assert!(!Method::Post.is_head());
    assert!(!Method::Put.is_head());
    assert!(!Method::Delete.is_head());
    assert!(!Method::Patch.is_head());
    assert!(Method::Head.is_head());
    assert!(!Method::Options.is_head());
    assert!(!Method::Connect.is_head());
    assert!(!Method::Trace.is_head());
    assert!(!Method::Unknown("HEAD".to_string()).is_head());
}

#[test]
fn test_method_is_options() {
    assert!(!Method::Get.is_options());
    assert!(!Method::Post.is_options());
    assert!(!Method::Put.is_options());
    assert!(!Method::Delete.is_options());
    assert!(!Method::Patch.is_options());
    assert!(!Method::Head.is_options());
    assert!(Method::Options.is_options());
    assert!(!Method::Connect.is_options());
    assert!(!Method::Trace.is_options());
    assert!(!Method::Unknown("OPTIONS".to_string()).is_options());
}

#[test]
fn test_method_is_connect() {
    assert!(!Method::Get.is_connect());
    assert!(!Method::Post.is_connect());
    assert!(!Method::Put.is_connect());
    assert!(!Method::Delete.is_connect());
    assert!(!Method::Patch.is_connect());
    assert!(!Method::Head.is_connect());
    assert!(!Method::Options.is_connect());
    assert!(Method::Connect.is_connect());
    assert!(!Method::Trace.is_connect());
    assert!(!Method::Unknown("CONNECT".to_string()).is_connect());
}

#[test]
fn test_method_is_trace() {
    assert!(!Method::Get.is_trace());
    assert!(!Method::Post.is_trace());
    assert!(!Method::Put.is_trace());
    assert!(!Method::Delete.is_trace());
    assert!(!Method::Patch.is_trace());
    assert!(!Method::Head.is_trace());
    assert!(!Method::Options.is_trace());
    assert!(!Method::Connect.is_trace());
    assert!(Method::Trace.is_trace());
    assert!(!Method::Unknown("TRACE".to_string()).is_trace());
}

#[test]
fn test_method_is_unknown() {
    assert!(!Method::Get.is_unknown());
    assert!(!Method::Post.is_unknown());
    assert!(!Method::Put.is_unknown());
    assert!(!Method::Delete.is_unknown());
    assert!(!Method::Patch.is_unknown());
    assert!(!Method::Head.is_unknown());
    assert!(!Method::Options.is_unknown());
    assert!(!Method::Connect.is_unknown());
    assert!(!Method::Trace.is_unknown());
    assert!(Method::Unknown("CUSTOM".to_string()).is_unknown());
    assert!(Method::Unknown("".to_string()).is_unknown());
}

#[test]
fn test_method_clone() {
    let method: Method = Method::Get;
    let cloned_method: Method = method.clone();
    assert_eq!(method, cloned_method);
    let unknown_method: Method = Method::Unknown("CUSTOM".to_string());
    let cloned_unknown: Method = unknown_method.clone();
    assert_eq!(unknown_method, cloned_unknown);
}

#[test]
fn test_method_debug() {
    let method: Method = Method::Get;
    let debug_str: String = format!("{method:?}");
    assert_eq!(debug_str, "Get");
    let unknown_method: Method = Method::Unknown("CUSTOM".to_string());
    let unknown_debug_str: String = format!("{unknown_method:?}");
    assert_eq!(unknown_debug_str, "Unknown(\"CUSTOM\")");
}

#[test]
fn test_method_equality() {
    assert_eq!(Method::Get, Method::Get);
    assert_ne!(Method::Get, Method::Post);
    assert_eq!(
        Method::Unknown("CUSTOM".to_string()),
        Method::Unknown("CUSTOM".to_string())
    );
    assert_ne!(
        Method::Unknown("CUSTOM1".to_string()),
        Method::Unknown("CUSTOM2".to_string())
    );
    assert_ne!(Method::Get, Method::Unknown("GET".to_string()));
}

#[test]
fn test_method_case_sensitivity() {
    assert_eq!(
        "get".parse::<Method>().unwrap(),
        Method::Unknown("get".to_string())
    );
    assert_eq!(
        "Get".parse::<Method>().unwrap(),
        Method::Unknown("Get".to_string())
    );
    assert_eq!("POST".parse::<Method>().unwrap(), Method::Post);
    assert_eq!(
        "post".parse::<Method>().unwrap(),
        Method::Unknown("post".to_string())
    );
}

#[test]
fn test_method_all_variants() {
    let methods: Vec<Method> = vec![
        Method::Get,
        Method::Post,
        Method::Put,
        Method::Delete,
        Method::Patch,
        Method::Head,
        Method::Options,
        Method::Connect,
        Method::Trace,
        Method::Unknown("CUSTOM".to_string()),
    ];
    for method in methods {
        let display_str: String = method.to_string();
        assert!(!display_str.is_empty());
        let debug_str: String = format!("{method:?}");
        assert!(!debug_str.is_empty());
    }
}

#[test]
fn test_method_unknown_with_empty_string() {
    let method: Method = Method::Unknown("".to_string());
    assert_eq!(method.to_string(), "");
    assert!(method.is_unknown());
    assert_eq!(format!("{method:?}"), "Unknown(\"\")");
}

#[test]
fn test_method_unknown_with_special_characters() {
    let method: Method = Method::Unknown("CUSTOM-METHOD".to_string());
    assert_eq!(method.to_string(), "CUSTOM-METHOD");
    assert!(method.is_unknown());
    assert_eq!(format!("{method:?}"), "Unknown(\"CUSTOM-METHOD\")");
}
