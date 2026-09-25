use super::*;

/// Parse command line arguments
///
/// # Returns
///
/// - `Args`: Parsed arguments
pub fn parse_args() -> Args {
    let raw_args: Vec<String> = args().collect();
    let mut command: CommandType = CommandType::Help;
    let mut project_name: Option<String> = None;
    let mut template_type: Option<TemplateType> = None;
    let mut model_sub_type: Option<ModelSubType> = None;
    let mut component_name: Option<String> = None;
    let mut i: usize = 1;
    while i < raw_args.len() {
        let arg: &str = raw_args[i].as_str();
        match arg {
            "-h" | "--help" => {
                command = CommandType::Help;
            }
            "-v" | "--version" => {
                command = CommandType::Version;
            }
            "watch" if (command == CommandType::Help || command == CommandType::Version) => {
                command = CommandType::Watch;
            }
            "new" if (command == CommandType::Help || command == CommandType::Version) => {
                command = CommandType::New;
                i += 1;
                if i < raw_args.len()
                    && !raw_args[i].starts_with("--")
                    && !raw_args[i].starts_with("-")
                {
                    project_name = Some(raw_args[i].clone());
                } else {
                    i -= 1;
                }
            }
            "template" if (command == CommandType::Help || command == CommandType::Version) => {
                command = CommandType::Template;
                i += 1;
                if i < raw_args.len()
                    && !raw_args[i].starts_with("--")
                    && !raw_args[i].starts_with("-")
                {
                    let type_str: &str = &raw_args[i];
                    template_type = TemplateType::from_str(type_str).ok();
                    i += 1;
                    if template_type == Some(TemplateType::Model)
                        && i < raw_args.len()
                        && !raw_args[i].starts_with("--")
                        && !raw_args[i].starts_with("-")
                    {
                        let sub_type_str: &str = &raw_args[i];
                        model_sub_type = ModelSubType::from_str(sub_type_str).ok();
                        i += 1;
                    }
                    if i < raw_args.len()
                        && !raw_args[i].starts_with("--")
                        && !raw_args[i].starts_with("-")
                    {
                        component_name = Some(raw_args[i].clone());
                        i += 1;
                    }
                    i -= 1;
                }
            }
            _ => {}
        }
        i += 1;
    }
    Args {
        command,
        project_name,
        template_type,
        model_sub_type,
        component_name,
    }
}
