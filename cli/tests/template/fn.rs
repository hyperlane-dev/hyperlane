use super::*;

#[test]
fn test_template_type_from_str_lowercase_names() {
    assert_eq!(
        "controller".parse::<TemplateType>().ok(),
        Some(TemplateType::Controller)
    );
    assert_eq!(
        "domain".parse::<TemplateType>().ok(),
        Some(TemplateType::Domain)
    );
    assert_eq!(
        "exception".parse::<TemplateType>().ok(),
        Some(TemplateType::Exception)
    );
    assert_eq!(
        "mapper".parse::<TemplateType>().ok(),
        Some(TemplateType::Mapper)
    );
    assert_eq!(
        "model".parse::<TemplateType>().ok(),
        Some(TemplateType::Model)
    );
    assert_eq!(
        "repository".parse::<TemplateType>().ok(),
        Some(TemplateType::Repository)
    );
    assert_eq!(
        "service".parse::<TemplateType>().ok(),
        Some(TemplateType::Service)
    );
    assert_eq!(
        "utils".parse::<TemplateType>().ok(),
        Some(TemplateType::Utils)
    );
    assert_eq!(
        "view".parse::<TemplateType>().ok(),
        Some(TemplateType::View)
    );
}

#[test]
fn test_template_type_from_str_is_case_insensitive() {
    assert_eq!(
        "CONTROLLER".parse::<TemplateType>().ok(),
        Some(TemplateType::Controller)
    );
    assert_eq!(
        "Model".parse::<TemplateType>().ok(),
        Some(TemplateType::Model)
    );
}

#[test]
fn test_template_type_from_str_rejects_unknown() {
    let result: Result<TemplateType, TemplateError> = "nope".parse::<TemplateType>();
    assert!(result.is_err());
    let error: TemplateError = result.unwrap_err();
    assert!(error.to_string().contains("Invalid template type: nope"));
}

#[test]
fn test_template_type_from_str_rejects_empty() {
    let result: Result<TemplateType, TemplateError> = "".parse::<TemplateType>();
    assert!(result.is_err());
    assert!(
        result
            .unwrap_err()
            .to_string()
            .contains("Invalid template type")
    );
}

#[test]
fn test_template_type_variants_are_distinct() {
    assert_ne!(TemplateType::Controller, TemplateType::Model);
    assert_ne!(TemplateType::Domain, TemplateType::Service);
    assert_ne!(TemplateType::Mapper, TemplateType::Repository);
    assert_ne!(TemplateType::Exception, TemplateType::Utils);
    assert_ne!(TemplateType::View, TemplateType::Controller);
}

#[test]
fn test_template_type_is_copy() {
    let value: TemplateType = TemplateType::Domain;
    let copied: TemplateType = value;
    assert_eq!(value, copied);
}

#[test]
fn test_model_sub_type_from_str_lowercase_names() {
    assert_eq!(
        "application".parse::<ModelSubType>().ok(),
        Some(ModelSubType::Application)
    );
    assert_eq!(
        "request".parse::<ModelSubType>().ok(),
        Some(ModelSubType::Request)
    );
    assert_eq!(
        "response".parse::<ModelSubType>().ok(),
        Some(ModelSubType::Response)
    );
}

#[test]
fn test_model_sub_type_from_str_is_case_insensitive() {
    assert_eq!(
        "Request".parse::<ModelSubType>().ok(),
        Some(ModelSubType::Request)
    );
    assert_eq!(
        "RESPONSE".parse::<ModelSubType>().ok(),
        Some(ModelSubType::Response)
    );
}

#[test]
fn test_model_sub_type_from_str_rejects_template_names() {
    let result: Result<ModelSubType, TemplateError> = "model".parse::<ModelSubType>();
    assert!(result.is_err());
    assert!(
        result
            .unwrap_err()
            .to_string()
            .contains("Invalid model subtype: model")
    );
}

#[test]
fn test_model_sub_type_variants_are_distinct() {
    assert_ne!(ModelSubType::Application, ModelSubType::Request);
    assert_ne!(ModelSubType::Request, ModelSubType::Response);
    assert_ne!(ModelSubType::Application, ModelSubType::Response);
}

#[test]
fn test_template_config_new_defaults_base_directory() {
    let config: TemplateConfig = TemplateConfig::new(
        TemplateType::Controller,
        "user".to_string(),
        None::<ModelSubType>,
    );
    assert_eq!(config.template_type, TemplateType::Controller);
    assert_eq!(config.component_name, "user");
    assert!(config.model_sub_type.is_none());
    assert_eq!(config.base_directory, "./application");
}

#[test]
fn test_template_config_new_keeps_model_sub_type() {
    let config: TemplateConfig = TemplateConfig::new(
        TemplateType::Model,
        "order".to_string(),
        Some(ModelSubType::Request),
    );
    assert_eq!(config.template_type, TemplateType::Model);
    assert_eq!(config.model_sub_type, Some(ModelSubType::Request));
    assert_eq!(config.component_name, "order");
}

#[test]
fn test_template_config_fields_are_mutable() {
    let mut config: TemplateConfig = TemplateConfig::new(
        TemplateType::Utils,
        "helper".to_string(),
        None::<ModelSubType>,
    );
    config.template_type = TemplateType::View;
    config.component_name = "page".to_string();
    config.model_sub_type = Some(ModelSubType::Response);
    config.base_directory = "./custom".to_string();
    assert_eq!(config.template_type, TemplateType::View);
    assert_eq!(config.component_name, "page");
    assert_eq!(config.model_sub_type, Some(ModelSubType::Response));
    assert_eq!(config.base_directory, "./custom");
}

#[test]
fn test_template_config_clone_and_debug() {
    let config: TemplateConfig = TemplateConfig::new(
        TemplateType::Service,
        "svc".to_string(),
        None::<ModelSubType>,
    );
    let cloned: TemplateConfig = config.clone();
    assert_eq!(cloned.component_name, "svc");
    assert_eq!(cloned.template_type, TemplateType::Service);
    let debugged: String = format!("{config:?}");
    assert!(debugged.contains("svc"));
    assert!(debugged.contains("./application"));
}

#[test]
fn test_template_error_variants_display_messages() {
    let invalid_type: TemplateError = TemplateError::InvalidTemplateType("bogus".to_string());
    assert_eq!(invalid_type.to_string(), "Invalid template type: bogus");
    let invalid_sub: TemplateError = TemplateError::InvalidModelSubType("bogus".to_string());
    assert_eq!(invalid_sub.to_string(), "Invalid model subtype: bogus");
    let exists: TemplateError = TemplateError::DirectoryExists("./a/b".to_string());
    assert_eq!(exists.to_string(), "Directory './a/b' already exists");
}

#[test]
fn test_template_error_from_io() {
    let io_error: io::Error = io::Error::new(io::ErrorKind::PermissionDenied, "denied");
    let template_error: TemplateError = TemplateError::from(io_error);
    assert_eq!(template_error.to_string(), "IO error: denied");
}

#[test]
fn test_template_error_debug_names_variant() {
    let error: TemplateError = TemplateError::InvalidTemplateType("x".to_string());
    let debugged: String = format!("{error:?}");
    assert!(debugged.contains("InvalidTemplateType"));
}
