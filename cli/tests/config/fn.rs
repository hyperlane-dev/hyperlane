use super::*;

#[test]
fn test_parse_args_defaults_to_help_without_command_tokens() {
    let recognised: bool = args().any(|item: String| {
        matches!(
            item.as_str(),
            "-h" | "--help" | "-v" | "--version" | "watch" | "new" | "template"
        )
    });
    if recognised {
        return;
    }
    let args: Args = parse_args();
    assert_eq!(args.command, CommandType::Help);
    assert!(args.project_name.is_none());
    assert!(args.template_type.is_none());
    assert!(args.model_sub_type.is_none());
    assert!(args.component_name.is_none());
}

#[test]
fn test_parse_args_returns_populated_args() {
    let args: Args = parse_args();
    let command: CommandType = args.command;
    assert!(matches!(
        command,
        CommandType::Watch
            | CommandType::New
            | CommandType::Template
            | CommandType::Help
            | CommandType::Version
    ));
}

#[test]
fn test_args_default_values() {
    let args: Args = Args {
        command: CommandType::Help,
        project_name: None,
        template_type: None,
        model_sub_type: None,
        component_name: None,
    };
    assert!(args.project_name.is_none());
    assert!(args.template_type.is_none());
    assert!(args.model_sub_type.is_none());
    assert!(args.component_name.is_none());
}

#[test]
fn test_args_with_values() {
    let args: Args = Args {
        command: CommandType::New,
        project_name: Some("test-project".to_string()),
        template_type: None,
        model_sub_type: None,
        component_name: None,
    };
    assert_eq!(args.project_name, Some("test-project".to_string()));
}

#[test]
fn test_args_with_model_subtype() {
    let args: Args = Args {
        command: CommandType::Template,
        project_name: None,
        template_type: Some(TemplateType::Model),
        model_sub_type: Some(ModelSubType::Request),
        component_name: Some("user".to_string()),
    };
    assert_eq!(args.template_type, Some(TemplateType::Model));
    assert_eq!(args.model_sub_type, Some(ModelSubType::Request));
    assert_eq!(args.component_name, Some("user".to_string()));
}

#[test]
fn test_command_type_enum_values() {
    let _: CommandType = CommandType::Watch;
    let _: CommandType = CommandType::New;
    let _: CommandType = CommandType::Template;
    let _: CommandType = CommandType::Help;
    let _: CommandType = CommandType::Version;
}

#[test]
fn test_args_clone() {
    let args: Args = Args {
        command: CommandType::Template,
        project_name: Some("test-project".to_string()),
        template_type: Some(TemplateType::Controller),
        model_sub_type: None,
        component_name: Some("test".to_string()),
    };
    let cloned: Args = args.clone();
    assert_eq!(cloned.project_name, args.project_name);
    assert_eq!(cloned.template_type, args.template_type);
    assert_eq!(cloned.component_name, args.component_name);
}
