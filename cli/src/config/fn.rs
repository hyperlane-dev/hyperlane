use super::*;

/// Parse command line arguments
///
/// # Returns
///
/// - `Args` - Parsed arguments
pub fn parse_args() -> Args {
    let raw_args: Vec<String> = args().collect();
    parse_args_from(&raw_args)
}

/// Parse command line arguments from an explicit slice (starting with argv0).
///
/// # Arguments
///
/// - `&[String]` - Raw command line argument slice.
///
/// # Returns
///
/// - `Args` - Parsed arguments.
pub fn parse_args_from(raw_args: &[String]) -> Args {
    let mut command: CommandType = CommandType::Help;
    let mut project_name: Option<String> = None;
    let mut template_type: Option<TemplateType> = None;
    let mut model_sub_type: Option<ModelSubType> = None;
    let mut component_name: Option<String> = None;
    let mut fmt_check: bool = false;
    let mut fmt_path: Option<PathBuf> = None;
    let mut i: usize = 1;
    while i < raw_args.len() {
        let arg: &str = raw_args[i].as_str();
        let default_cmd: bool = matches!(command, CommandType::Help | CommandType::Version);
        match arg {
            "-h" | FLAG_LONG_HELP => {
                command = CommandType::Help;
            }
            "-v" | FLAG_LONG_VERSION => {
                command = CommandType::Version;
            }
            COMMAND_WATCH if default_cmd => {
                command = CommandType::Watch;
            }
            COMMAND_FMT if default_cmd => {
                command = CommandType::Fmt;
            }
            FLAG_FMT_CHECK if command == CommandType::Fmt => {
                fmt_check = true;
            }
            FLAG_FMT_PATH if command == CommandType::Fmt => {
                i += 1;
                if i < raw_args.len() {
                    fmt_path = Some(PathBuf::from(&raw_args[i]));
                } else {
                    i -= 1;
                }
            }
            "new" if default_cmd => {
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
            COMMAND_TEMPLATE if default_cmd => {
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
            other
                if command == CommandType::Fmt && fmt_path.is_none() && !other.starts_with('-') =>
            {
                fmt_path = Some(PathBuf::from(other));
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
        fmt_check,
        fmt_path,
    }
}
