use super::*;

/// Parse command line arguments
///
/// # Returns
///
/// - `Args`: Parsed arguments
pub fn parse_args() -> Args {
    let raw_args: Vec<String> = args().collect();
    let mut command: CommandType = CommandType::Help;
    let mut check: bool = false;
    let mut manifest_path: Option<String> = None;
    let mut bump_type: Option<BumpVersionType> = None;
    let mut max_retries: u32 = 8;
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
            "fmt" if (command == CommandType::Help || command == CommandType::Version) => {
                command = CommandType::Fmt;
            }
            "watch" if (command == CommandType::Help || command == CommandType::Version) => {
                command = CommandType::Watch;
            }
            "bump" if (command == CommandType::Help || command == CommandType::Version) => {
                command = CommandType::Bump;
            }
            "publish" if (command == CommandType::Help || command == CommandType::Version) => {
                command = CommandType::Publish;
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
            "--patch" => {
                bump_type = Some(BumpVersionType::Patch);
            }
            "--minor" => {
                bump_type = Some(BumpVersionType::Minor);
            }
            "--major" => {
                bump_type = Some(BumpVersionType::Major);
            }
            "--release" => {
                bump_type = Some(BumpVersionType::Release);
            }
            "--alpha" => {
                bump_type = Some(BumpVersionType::Alpha);
            }
            "--beta" => {
                bump_type = Some(BumpVersionType::Beta);
            }
            "--rc" => {
                bump_type = Some(BumpVersionType::Rc);
            }
            "--check" => {
                check = true;
            }
            "--manifest-path" => {
                i += 1;
                if i < raw_args.len() {
                    manifest_path = Some(raw_args[i].clone());
                }
            }
            "--max-retries" => {
                i += 1;
                if i < raw_args.len()
                    && let Ok(n) = raw_args[i].parse::<u32>()
                {
                    max_retries = n;
                }
            }
            _ => {}
        }
        i += 1;
    }
    Args {
        command,
        check,
        manifest_path,
        bump_type,
        max_retries,
        project_name,
        template_type,
        model_sub_type,
        component_name,
    }
}
