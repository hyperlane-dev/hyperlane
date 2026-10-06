use super::*;

#[test]
fn test_command_type_is_copy() {
    let value: CommandType = CommandType::Template;
    let copied: CommandType = value;
    assert_eq!(value, copied);
}

#[test]
fn test_command_type_variants_are_distinct() {
    assert_ne!(CommandType::Watch, CommandType::New);
    assert_ne!(CommandType::New, CommandType::Template);
    assert_ne!(CommandType::Template, CommandType::Help);
    assert_ne!(CommandType::Help, CommandType::Version);
    assert_ne!(CommandType::Version, CommandType::Watch);
}

#[test]
fn test_command_type_debug_names_variant() {
    let watch: String = format!("{:?}", CommandType::Watch);
    let version: String = format!("{:?}", CommandType::Version);
    assert_eq!(watch, "Watch");
    assert_eq!(version, "Version");
}

#[test]
fn test_command_type_copy_preserves_value() {
    let original: CommandType = CommandType::New;
    let copied: CommandType = original;
    assert_eq!(copied, CommandType::New);
    assert_eq!(original, CommandType::New);
}

#[test]
fn test_args_debug_lists_optional_fields() {
    let args: Args = Args {
        command: CommandType::Template,
        project_name: Some("proj".to_string()),
        template_type: Some(TemplateType::Model),
        model_sub_type: Some(ModelSubType::Application),
        component_name: Some("thing".to_string()),
    };
    let debugged: String = format!("{args:?}");
    assert!(debugged.contains("proj"));
    assert!(debugged.contains("thing"));
    assert!(debugged.contains("Model"));
}
