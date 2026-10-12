use super::*;

#[test]
fn multipart_is_form_data() {
    assert!(MultipartFormData::is_form_data(
        "multipart/form-data; boundary=abc"
    ));
    assert!(MultipartFormData::is_form_data("MULTIPART/FORM-DATA"));
    assert!(MultipartFormData::is_form_data("  multipart/form-data  "));
    assert!(!MultipartFormData::is_form_data("application/json"));
    assert!(!MultipartFormData::is_form_data(""));
}

#[test]
fn multipart_get_boundary_unquoted() {
    let boundary: Option<MultipartFormDataBoundary> =
        MultipartFormData::get_boundary("multipart/form-data; boundary=----WebKitFormBoundary");
    assert_eq!(boundary, Some("----WebKitFormBoundary".to_string()));
}

#[test]
fn multipart_get_boundary_quoted() {
    let boundary: Option<MultipartFormDataBoundary> =
        MultipartFormData::get_boundary("multipart/form-data; boundary=\"abc123\"");
    assert_eq!(boundary, Some("abc123".to_string()));
}

#[test]
fn multipart_get_boundary_case_insensitive_and_extra_parameters() {
    let boundary: Option<MultipartFormDataBoundary> =
        MultipartFormData::get_boundary("multipart/form-data; charset=utf-8; BOUNDARY=xyz");
    assert_eq!(boundary, Some("xyz".to_string()));
}

#[test]
fn multipart_get_boundary_skips_parameter_without_equals() {
    let boundary: Option<MultipartFormDataBoundary> =
        MultipartFormData::get_boundary("multipart/form-data; broken; boundary=late");
    assert_eq!(boundary, Some("late".to_string()));
}

#[test]
fn multipart_get_boundary_missing() {
    let boundary: Option<MultipartFormDataBoundary> =
        MultipartFormData::get_boundary("multipart/form-data");
    assert_eq!(boundary, None);
    let boundary_empty: Option<MultipartFormDataBoundary> =
        MultipartFormData::get_boundary("multipart/form-data; boundary=");
    assert_eq!(boundary_empty, None);
}

#[test]
fn multipart_parse_single_text_field() {
    let body: String = concat!(
        "--BOUND\r\n",
        "Content-Disposition: form-data; name=\"title\"\r\n",
        "\r\n",
        "hello world\r\n",
        "--BOUND--\r\n"
    )
    .to_string();
    let parts: MultipartFormDataParts = MultipartFormData::parse(body, "BOUND").unwrap();
    assert_eq!(parts.len(), 1);
    let part: &BodyPart = &parts[0];
    assert_eq!(part.try_get_name(), Some(&"title".to_string()));
    assert_eq!(part.try_get_file_name(), None);
    assert_eq!(part.try_get_data(), Some(b"hello world".as_slice()));
}

#[test]
fn multipart_parse_file_upload() {
    let body: String = concat!(
        "--BOUND\r\n",
        "Content-Disposition: form-data; name=\"upload\"; filename=\"a.txt\"\r\n",
        "Content-Type: text/plain\r\n",
        "\r\n",
        "file body\r\n",
        "--BOUND--\r\n"
    )
    .to_string();
    let parts: MultipartFormDataParts = MultipartFormData::parse(body, "BOUND").unwrap();
    assert_eq!(parts.len(), 1);
    let part: &BodyPart = &parts[0];
    assert_eq!(part.try_get_name(), Some(&"upload".to_string()));
    assert_eq!(part.try_get_file_name(), Some(&"a.txt".to_string()));
    assert_eq!(part.try_get_content_type(), Some(&"text/plain".to_string()));
    assert!(part.try_get_header("content-type").is_some());
    assert_eq!(part.try_get_header("x-absent"), None);
    assert_eq!(part.try_get_data(), Some(b"file body".as_slice()));
}

#[test]
fn multipart_parse_multiple_parts() {
    let body: String = concat!(
        "--BOUND\r\n",
        "Content-Disposition: form-data; name=\"field\"\r\n",
        "\r\n",
        "value\r\n",
        "--BOUND\r\n",
        "Content-Disposition: form-data; name=\"file\"; filename=\"b.bin\"\r\n",
        "Content-Type: application/octet-stream\r\n",
        "\r\n",
        "\u{0}\u{1}\u{2}\r\n",
        "--BOUND--\r\n"
    )
    .to_string();
    let parts: MultipartFormDataParts = MultipartFormData::parse(body, "BOUND").unwrap();
    assert_eq!(parts.len(), 2);
    assert_eq!(parts[0].try_get_name(), Some(&"field".to_string()));
    assert_eq!(parts[1].try_get_name(), Some(&"file".to_string()));
    assert_eq!(parts[1].try_get_data(), Some([0u8, 1, 2].as_slice()));
}

#[test]
fn multipart_parse_binary_payload_is_byte_exact() {
    let mut body: Vec<u8> = Vec::new();
    body.extend_from_slice(b"--BOUND\r\n");
    body.extend_from_slice(
        b"Content-Disposition: form-data; name=\"blob\"; filename=\"raw.bin\"\r\n\r\n",
    );
    body.extend_from_slice(&[0u8, 13, 10, 255, 128, 0, 65]);
    body.extend_from_slice(b"\r\n--BOUND--\r\n");
    let parts: MultipartFormDataParts = MultipartFormData::parse(&body, "BOUND").unwrap();
    assert_eq!(parts.len(), 1);
    assert_eq!(
        parts[0].try_get_data(),
        Some([0u8, 13, 10, 255, 128, 0, 65].as_slice())
    );
}

#[test]
fn multipart_parse_accepts_bare_line_feeds() {
    let body: String = concat!(
        "--BOUND\n",
        "Content-Disposition: form-data; name=\"title\"\n",
        "\n",
        "lenient\n",
        "--BOUND--\n"
    )
    .to_string();
    let parts: MultipartFormDataParts = MultipartFormData::parse(body, "BOUND").unwrap();
    assert_eq!(parts.len(), 1);
    assert_eq!(parts[0].try_get_name(), Some(&"title".to_string()));
}

#[test]
fn multipart_parse_ignores_boundary_like_inside_payload() {
    let body: String = concat!(
        "--BOUND\r\n",
        "Content-Disposition: form-data; name=\"text\"\r\n",
        "\r\n",
        "--BOUNDARY is not a boundary\r\n",
        "--BOUND--\r\n"
    )
    .to_string();
    let parts: MultipartFormDataParts = MultipartFormData::parse(body, "BOUND").unwrap();
    assert_eq!(parts.len(), 1);
    assert_eq!(
        parts[0].try_get_data(),
        Some(b"--BOUNDARY is not a boundary".as_slice())
    );
}

#[test]
fn multipart_parse_header_keys_are_lowercased() {
    let body: String = concat!(
        "--BOUND\r\n",
        "Content-Disposition: form-data; name=\"title\"\r\n",
        "X-Custom-Header: value\r\n",
        "\r\n",
        "data\r\n",
        "--BOUND--\r\n"
    )
    .to_string();
    let parts: MultipartFormDataParts = MultipartFormData::parse(body, "BOUND").unwrap();
    let part: &BodyPart = &parts[0];
    assert_eq!(
        part.try_get_header("x-custom-header"),
        Some(&"value".to_string())
    );
}

#[test]
fn multipart_parse_missing_boundary_returns_error() {
    assert_eq!(MultipartFormData::parse("no boundary here", "BOUND"), None);
}

#[test]
fn multipart_parse_missing_name_returns_error() {
    let body: String = concat!(
        "--BOUND\r\n",
        "Content-Type: text/plain\r\n",
        "\r\n",
        "data\r\n",
        "--BOUND--\r\n"
    )
    .to_string();
    assert_eq!(MultipartFormData::parse(body, "BOUND"), None);
}

#[test]
fn multipart_parse_unterminated_closing_boundary_is_rejected() {
    let body: String = concat!(
        "--BOUND\r\n",
        "Content-Disposition: form-data; name=\"title\"\r\n",
        "\r\n",
        "value\r\n",
        "--BOUND\r\n"
    )
    .to_string();
    assert_eq!(MultipartFormData::parse(body, "BOUND"), None);
}

#[test]
fn multipart_try_get_part() {
    let body: String = concat!(
        "--BOUND\r\n",
        "Content-Disposition: form-data; name=\"first\"\r\n",
        "\r\n",
        "one\r\n",
        "--BOUND\r\n",
        "Content-Disposition: form-data; name=\"second\"\r\n",
        "\r\n",
        "two\r\n",
        "--BOUND--\r\n"
    )
    .to_string();
    let parts: MultipartFormDataParts = MultipartFormData::parse(body, "BOUND").unwrap();
    let found: Option<&BodyPart> = MultipartFormData::try_get_part(&parts, "second");
    assert_eq!(
        found.and_then(|part: &BodyPart| part.try_get_text()),
        Some("two".to_string())
    );
    assert!(MultipartFormData::try_get_part(&parts, "missing").is_none());
}

#[test]
fn multipart_try_get_part_text_skips_files() {
    let body: String = concat!(
        "--BOUND\r\n",
        "Content-Disposition: form-data; name=\"field\"\r\n",
        "\r\n",
        "plain value\r\n",
        "--BOUND\r\n",
        "Content-Disposition: form-data; name=\"upload\"; filename=\"c.txt\"\r\n",
        "\r\n",
        "file content\r\n",
        "--BOUND--\r\n"
    )
    .to_string();
    let parts: MultipartFormDataParts = MultipartFormData::parse(body, "BOUND").unwrap();
    assert_eq!(
        MultipartFormData::try_get_part_text(&parts, "field"),
        Some("plain value".to_string())
    );
    assert_eq!(MultipartFormData::try_get_part_text(&parts, "upload"), None);
    assert_eq!(
        MultipartFormData::try_get_part_text(&parts, "missing"),
        None
    );
}

#[test]
fn body_part_default_is_empty() {
    let part: BodyPart = BodyPart::default();
    assert_eq!(part.try_get_name(), None);
    assert_eq!(part.try_get_file_name(), None);
    assert_eq!(part.try_get_content_type(), None);
    assert_eq!(part.try_get_header("x-none"), None);
    assert_eq!(part.try_get_data(), None);
}

#[test]
fn url_encoded_parse_basic_fields() {
    let form: BodyForm = MultipartFormData::parse_url_encoded("a=1&b=2");
    assert_eq!(form.get("a"), Some(&"1".to_string()));
    assert_eq!(form.get("b"), Some(&"2".to_string()));
}

#[test]
fn url_encoded_parse_decodes_percent_and_plus() {
    let form: BodyForm =
        MultipartFormData::parse_url_encoded("msg=hello+world%21&name=%E4%B8%AD%E6%96%87");
    assert_eq!(form.get("msg"), Some(&"hello world!".to_string()));
    assert_eq!(form.get("name"), Some(&"中文".to_string()));
}

#[test]
fn url_encoded_parse_handles_empty_and_bare_keys() {
    let form: BodyForm = MultipartFormData::parse_url_encoded("a=&b&&c=1");
    assert_eq!(form.get("a"), Some(&"".to_string()));
    assert_eq!(form.get("b"), Some(&"".to_string()));
    assert_eq!(form.get("c"), Some(&"1".to_string()));
    assert_eq!(form.len(), 3);
}

#[test]
fn url_encoded_parse_keeps_invalid_escape_verbatim() {
    let form: BodyForm = MultipartFormData::parse_url_encoded("a=100%zz");
    assert_eq!(form.get("a"), Some(&"100%zz".to_string()));
}

#[test]
fn url_encoded_parse_empty_body_is_empty_form() {
    let form: BodyForm = MultipartFormData::parse_url_encoded("");
    assert!(form.is_empty());
}

#[test]
fn request_is_multipart_form_data_detects_content_type() {
    let mut values: VecDeque<String> = VecDeque::new();
    values.push_back("multipart/form-data; boundary=BOUND".to_string());
    let mut request: Request = Request::default();
    request
        .get_mut_headers()
        .insert(CONTENT_TYPE.to_string(), values);
    assert!(request.is_multipart_form_data());
    assert!(!Request::default().is_multipart_form_data());
}

#[test]
fn request_try_get_body_form_data_decodes_parts() {
    let body: String = concat!(
        "--BOUND\r\n",
        "Content-Disposition: form-data; name=\"title\"\r\n",
        "\r\n",
        "hello\r\n",
        "--BOUND\r\n",
        "Content-Disposition: form-data; name=\"upload\"; filename=\"d.txt\"\r\n",
        "\r\n",
        "content\r\n",
        "--BOUND--\r\n"
    )
    .to_string();
    let mut values: VecDeque<String> = VecDeque::new();
    values.push_back("multipart/form-data; boundary=BOUND".to_string());
    let mut request: Request = Request::default();
    request
        .get_mut_headers()
        .insert(CONTENT_TYPE.to_string(), values);
    request.set_body(body.into_bytes());
    let parts: MultipartFormDataParts = request.try_get_body_form_data().unwrap();
    assert_eq!(parts.len(), 2);
    assert_eq!(
        request.try_get_body_form_data_text("title"),
        Some("hello".to_string())
    );
    assert_eq!(request.try_get_body_form_data_text("upload"), None);
}

#[test]
fn request_try_get_body_form_data_without_content_type_is_error() {
    let request: Request = Request::default();
    assert_eq!(request.try_get_body_form_data(), None);
}

#[test]
fn request_try_get_body_form_data_without_boundary_is_error() {
    let mut values: VecDeque<String> = VecDeque::new();
    values.push_back("multipart/form-data".to_string());
    let mut request: Request = Request::default();
    request
        .get_mut_headers()
        .insert(CONTENT_TYPE.to_string(), values);
    assert_eq!(request.try_get_body_form_data(), None);
}

#[test]
fn request_get_body_form_decodes_url_encoded() {
    let request: Request = Request {
        body: "a=1&b=two".as_bytes().to_vec(),
        ..Default::default()
    };
    let form: BodyForm = request.get_body_form();
    assert_eq!(form.get("a"), Some(&"1".to_string()));
    assert_eq!(form.get("b"), Some(&"two".to_string()));
}
