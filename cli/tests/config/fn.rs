use super::*;

#[test]
fn test_args_default_values() {
    let args: Args = Args {
        command: CommandType::Help,
        check: false,
        manifest_path: None,
        bump_type: None,
        max_retries: 3,
        project_name: None,
        template_type: None,
        model_sub_type: None,
        component_name: None,
    };
    assert!(!args.check);
    assert_eq!(args.max_retries, 3);
    assert!(args.manifest_path.is_none());
    assert!(args.bump_type.is_none());
    assert!(args.project_name.is_none());
    assert!(args.template_type.is_none());
    assert!(args.model_sub_type.is_none());
    assert!(args.component_name.is_none());
}

#[test]
fn test_args_with_values() {
    let args: Args = Args {
        command: CommandType::Bump,
        check: true,
        manifest_path: Some("./test/Cargo.toml".to_string()),
        bump_type: Some(BumpVersionType::Minor),
        max_retries: 5,
        project_name: Some("test-project".to_string()),
        template_type: Some(TemplateType::Controller),
        model_sub_type: None,
        component_name: Some("test".to_string()),
    };
    assert!(args.check);
    assert_eq!(args.max_retries, 5);
    assert_eq!(args.manifest_path, Some("./test/Cargo.toml".to_string()));
    assert_eq!(args.bump_type, Some(BumpVersionType::Minor));
    assert_eq!(args.project_name, Some("test-project".to_string()));
    assert_eq!(args.template_type, Some(TemplateType::Controller));
    assert_eq!(args.component_name, Some("test".to_string()));
}

#[test]
fn test_args_with_model_subtype() {
    let args: Args = Args {
        command: CommandType::Template,
        check: false,
        manifest_path: None,
        bump_type: None,
        max_retries: 3,
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
    let _: CommandType = CommandType::Fmt;
    let _: CommandType = CommandType::Watch;
    let _: CommandType = CommandType::Bump;
    let _: CommandType = CommandType::Publish;
    let _: CommandType = CommandType::New;
    let _: CommandType = CommandType::Template;
    let _: CommandType = CommandType::Help;
    let _: CommandType = CommandType::Version;
}

#[test]
fn test_args_clone() {
    let args: Args = Args {
        command: CommandType::Bump,
        check: true,
        manifest_path: Some("./test/Cargo.toml".to_string()),
        bump_type: Some(BumpVersionType::Minor),
        max_retries: 5,
        project_name: Some("test-project".to_string()),
        template_type: Some(TemplateType::Controller),
        model_sub_type: None,
        component_name: Some("test".to_string()),
    };
    let cloned: Args = args.clone();
    assert_eq!(cloned.check, args.check);
    assert_eq!(cloned.max_retries, args.max_retries);
    assert_eq!(cloned.manifest_path, args.manifest_path);
    assert_eq!(cloned.bump_type, args.bump_type);
    assert_eq!(cloned.project_name, args.project_name);
    assert_eq!(cloned.template_type, args.template_type);
    assert_eq!(cloned.component_name, args.component_name);
}
