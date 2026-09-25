use super::*;

#[test]
fn test_content_type_from_str() {
    assert_eq!(
        APPLICATION_JSON.parse::<ContentType>().unwrap(),
        ContentType::ApplicationJson
    );
    assert_eq!(
        APPLICATION_XML.parse::<ContentType>().unwrap(),
        ContentType::ApplicationXml
    );
    assert_eq!(
        TEXT_PLAIN.parse::<ContentType>().unwrap(),
        ContentType::TextPlain
    );
    assert_eq!(
        TEXT_HTML.parse::<ContentType>().unwrap(),
        ContentType::TextHtml
    );
    assert_eq!(
        FORM_URLENCODED.parse::<ContentType>().unwrap(),
        ContentType::FormUrlEncoded
    );
    assert_eq!(
        "unknown/type".parse::<ContentType>().unwrap(),
        ContentType::Unknown
    );
    assert_eq!("".parse::<ContentType>().unwrap(), ContentType::Unknown);
}

#[test]
fn test_content_type_from_str_case_insensitive() {
    assert_eq!(
        "APPLICATION/JSON".parse::<ContentType>().unwrap(),
        ContentType::ApplicationJson
    );
    assert_eq!(
        "Application/Json".parse::<ContentType>().unwrap(),
        ContentType::ApplicationJson
    );
    assert_eq!(
        "TEXT/PLAIN".parse::<ContentType>().unwrap(),
        ContentType::TextPlain
    );
    assert_eq!(
        "Text/Plain".parse::<ContentType>().unwrap(),
        ContentType::TextPlain
    );
    assert_eq!(
        "text/HTML".parse::<ContentType>().unwrap(),
        ContentType::TextHtml
    );
}

#[test]
fn test_content_type_default() {
    assert_eq!(ContentType::default(), ContentType::Unknown);
}

#[test]
fn test_content_type_clone() {
    let content_type: ContentType = ContentType::ApplicationJson;
    let cloned_content_type: ContentType = content_type;
    assert_eq!(content_type, cloned_content_type);
}

#[test]
fn test_content_type_debug() {
    let content_type: ContentType = ContentType::ApplicationJson;
    let debug_str: String = format!("{content_type:?}");
    assert_eq!(debug_str, "ApplicationJson");
}

#[test]
fn test_content_type_equality() {
    assert_eq!(ContentType::ApplicationJson, ContentType::ApplicationJson);
    assert_ne!(ContentType::ApplicationJson, ContentType::ApplicationXml);
    assert_eq!(ContentType::Unknown, ContentType::Unknown);
    assert_ne!(ContentType::TextPlain, ContentType::TextHtml);
}

#[test]
fn test_content_type_get_body_string_with_simple_string() {
    let data: String = "Hello, World!".to_string();
    let json_result: String = ContentType::ApplicationJson.get_body_string(&data);
    assert_eq!(json_result, "\"Hello, World!\"");
    let plain_result: String = ContentType::TextPlain.get_body_string(&data);
    assert_eq!(plain_result, "Hello, World!");
    let html_result: String = ContentType::TextHtml.get_body_string(&data);
    assert!(html_result.contains("Hello, World!"));
    assert!(html_result.starts_with("<table><tr><td>"));
}

#[test]
fn test_content_type_get_body_string_with_number() {
    let data: i32 = 42;
    let json_result: String = ContentType::ApplicationJson.get_body_string(&data);
    assert_eq!(json_result, "42");
    let plain_result: String = ContentType::TextPlain.get_body_string(&data);
    assert_eq!(plain_result, "42");
    let html_result: String = ContentType::TextHtml.get_body_string(&data);
    assert!(html_result.contains("42"));
}

#[test]
fn test_content_type_format_content_type_with_charset() {
    let result: String = ContentType::format_content_type_with_charset("text/html", "utf-8");
    assert_eq!(result, "text/html; charset=utf-8");
    let result2: String =
        ContentType::format_content_type_with_charset("application/json", "iso-8859-1");
    assert_eq!(result2, "application/json; charset=iso-8859-1");
}

#[test]
fn test_content_type_format_content_type_with_charset_declaration() {
    let result: String =
        ContentType::format_content_type_with_charset_declaration("text/html", "charset=utf-8");
    assert_eq!(result, "text/html; charset=utf-8");
    let result2: String = ContentType::format_content_type_with_charset_declaration(
        "application/json",
        "charset=iso-8859-1",
    );
    assert_eq!(result2, "application/json; charset=iso-8859-1");
}

#[test]
fn test_content_type_all_variants() {
    let content_types: Vec<ContentType> = vec![
        ContentType::ApplicationJson,
        ContentType::ApplicationXml,
        ContentType::TextPlain,
        ContentType::TextHtml,
        ContentType::FormUrlEncoded,
        ContentType::Unknown,
    ];
    for content_type in content_types {
        let debug_str: String = format!("{content_type:?}");
        assert!(!debug_str.is_empty());
    }
}

#[test]
fn test_content_type_pattern_matching() {
    let content_type: ContentType = ContentType::ApplicationJson;
    match content_type {
        ContentType::ApplicationJson => {}
        ContentType::ApplicationXml => panic!("Should not match ApplicationXml"),
        ContentType::TextPlain => panic!("Should not match TextPlain"),
        ContentType::TextHtml => panic!("Should not match TextHtml"),
        ContentType::FormUrlEncoded => panic!("Should not match FormUrlEncoded"),
        ContentType::Unknown => panic!("Should not match Unknown"),
    }
}

#[test]
fn test_content_type_charset_formatting_edge_cases() {
    let result_empty: String = ContentType::format_content_type_with_charset("", "");
    assert_eq!(result_empty, "; charset=");
    let result_spaces: String =
        ContentType::format_content_type_with_charset("text/html", "utf-8 ");
    assert_eq!(result_spaces, "text/html; charset=utf-8 ");
    let result_special: String = ContentType::format_content_type_with_charset(
        "application/json",
        "utf-8;boundary=something",
    );
    assert_eq!(
        result_special,
        "application/json; charset=utf-8;boundary=something"
    );
}

#[test]
fn test_content_type_memory_size() {
    use std::mem;
    let size: usize = mem::size_of::<ContentType>();
    assert!(size > 0);
    let json_size: usize = mem::size_of_val(&ContentType::ApplicationJson);
    let unknown_size: usize = mem::size_of_val(&ContentType::Unknown);
    assert_eq!(json_size, unknown_size);
}

#[test]
fn test_content_type_get_body_string_unknown() {
    let data: String = "test_data".to_string();
    let content_type: ContentType = ContentType::Unknown;
    let result: String = content_type.get_body_string(&data);
    assert!(!result.is_empty());
}

#[test]
fn test_content_type_get_body_string_xml() {
    let data: String = "test_data".to_string();
    let content_type: ContentType = ContentType::ApplicationXml;
    let result: String = content_type.get_body_string(&data);
    assert!(result.is_empty());
}

#[test]
fn test_content_type_get_body_string_form_url_encoded() {
    let data: String = "test_data".to_string();
    let content_type: ContentType = ContentType::FormUrlEncoded;
    let result: String = content_type.get_body_string(&data);
    assert_eq!(result, "");
}

#[test]
fn test_content_type_from_str_with_parameters() {
    assert_eq!(
        "application/json; charset=utf-8"
            .parse::<ContentType>()
            .unwrap(),
        ContentType::Unknown
    );
    assert_eq!(
        "text/html; charset=utf-8".parse::<ContentType>().unwrap(),
        ContentType::Unknown
    );
}

#[test]
fn test_content_type_case_variations() {
    assert_eq!(
        "application/JSON".parse::<ContentType>().unwrap(),
        ContentType::ApplicationJson
    );
    assert_eq!(
        "APPLICATION/json".parse::<ContentType>().unwrap(),
        ContentType::ApplicationJson
    );
    assert_eq!(
        "text/PLAIN".parse::<ContentType>().unwrap(),
        ContentType::TextPlain
    );
    assert_eq!(
        "TEXT/plain".parse::<ContentType>().unwrap(),
        ContentType::TextPlain
    );
}
