use super::*;

/// Get directory name for template type
///
/// # Arguments
///
/// - `&TemplateType` - The template type
///
/// # Returns
///
/// - `String` - Directory name
fn get_directory_name(template_type: &TemplateType) -> String {
    match template_type {
        TemplateType::Controller => TEMPLATE_TYPE_CONTROLLER.to_string(),
        TemplateType::Domain => TEMPLATE_TYPE_DOMAIN.to_string(),
        TemplateType::Exception => TEMPLATE_TYPE_EXCEPTION.to_string(),
        TemplateType::Mapper => TEMPLATE_TYPE_MAPPER.to_string(),
        TemplateType::Model => TEMPLATE_TYPE_MODEL.to_string(),
        TemplateType::Repository => TEMPLATE_TYPE_REPOSITORY.to_string(),
        TemplateType::Service => TEMPLATE_TYPE_SERVICE.to_string(),
        TemplateType::Utils => TEMPLATE_TYPE_UTILS.to_string(),
        TemplateType::View => TEMPLATE_TYPE_VIEW.to_string(),
    }
}

/// Get model subtype directory name
///
/// # Arguments
///
/// - `&ModelSubType` - The model subtype
///
/// # Returns
///
/// - `String` - Directory name
fn get_model_sub_type_name(sub_type: &ModelSubType) -> String {
    match sub_type {
        ModelSubType::Application => MODEL_SUB_TYPE_APPLICATION.to_string(),
        ModelSubType::Request => MODEL_SUB_TYPE_REQUEST.to_string(),
        ModelSubType::Response => MODEL_SUB_TYPE_RESPONSE.to_string(),
    }
}

/// Create directory if it does not exist
///
/// # Arguments
///
/// - `&Path` - Path to the directory
///
/// # Returns
///
/// - `Result<(), TemplateError>` - Success or error
async fn ensure_directory(path: &Path) -> Result<(), TemplateError> {
    if !path.exists() {
        create_dir_all(path).await?;
    }
    Ok(())
}

/// Write mod.rs content with module declarations
///
/// # Arguments
///
/// - `&Path` - Path to mod.rs file
/// - `&[&str]` - List of modules to include
///
/// # Returns
///
/// - `Result<(), TemplateError>` - Success or error
async fn write_mod_rs(path: &Path, modules: &[&str]) -> Result<(), TemplateError> {
    let mut content: String = String::new();
    for module in modules {
        let mod_name: String = if module.starts_with("r#") {
            module.to_string()
        } else {
            format!("r#{module}")
        };
        content.push_str(&format!("mod {mod_name};\n"));
    }
    content.push('\n');
    let mut pub_use_parts: Vec<String> = Vec::new();
    for module in modules {
        let raw_name: &str = if let Some(stripped) = module.strip_prefix("r#") {
            stripped
        } else {
            module
        };
        let mod_name: String = if module.starts_with("r#") {
            module.to_string()
        } else {
            format!("r#{module}")
        };
        if raw_name == MODULE_NAME_CONST || raw_name == MODULE_NAME_STATIC {
            pub_use_parts.push(mod_name);
        } else if raw_name == MODULE_NAME_ENUM || raw_name == MODULE_NAME_FN {
            pub_use_parts.push(format!("{mod_name}::*"));
        } else if raw_name == MODULE_NAME_STRUCT {
            pub_use_parts.push(mod_name);
        }
    }
    if !pub_use_parts.is_empty() {
        content.push_str(PUB_USE_PREFIX);
        content.push_str(&pub_use_parts.join(", "));
        content.push_str("};\n");
    }
    content.push('\n');
    content.push_str(KEYWORD_FILE_HEADER);
    write(path, content).await?;
    Ok(())
}

/// Write empty mod.rs
///
/// # Arguments
///
/// - `&Path` - Path to mod.rs file
///
/// # Returns
///
/// - `Result<(), TemplateError>` - Success or error
async fn write_empty_mod_rs(path: &Path) -> Result<(), TemplateError> {
    write(path, "\n").await?;
    Ok(())
}

/// Create controller template files
///
/// # Arguments
///
/// - `&Path` - Target directory path
/// - `&str` - Name of the component
///
/// # Returns
///
/// - `Result<(), TemplateError>` - Success or error
async fn create_controller_template(
    target_dir: &Path,
    _component_name: &str,
) -> Result<(), TemplateError> {
    ensure_directory(target_dir).await?;
    let mod_rs: PathBuf = target_dir.join(FILE_NAME_MOD_RS);
    write_mod_rs(
        &mod_rs,
        &[MODULE_NAME_FN, MODULE_NAME_IMPL, MODULE_NAME_STRUCT],
    )
    .await?;
    let fn_rs: PathBuf = target_dir.join(FILE_NAME_FN_RS);
    write(&fn_rs, KEYWORD_FILE_HEADER).await?;
    let impl_rs: PathBuf = target_dir.join(FILE_NAME_IMPL_RS);
    write(&impl_rs, KEYWORD_FILE_HEADER).await?;
    let struct_rs: PathBuf = target_dir.join(FILE_NAME_STRUCT_RS);
    write(&struct_rs, KEYWORD_FILE_HEADER).await?;
    Ok(())
}

/// Create view template files
///
/// # Arguments
///
/// - `&Path` - Target directory path
/// - `&str` - Name of the component
///
/// # Returns
///
/// - `Result<(), TemplateError>` - Success or error
async fn create_view_template(
    target_dir: &Path,
    _component_name: &str,
) -> Result<(), TemplateError> {
    ensure_directory(target_dir).await?;
    let mod_rs: PathBuf = target_dir.join(FILE_NAME_MOD_RS);
    write_mod_rs(
        &mod_rs,
        &[MODULE_NAME_FN, MODULE_NAME_IMPL, MODULE_NAME_STRUCT],
    )
    .await?;
    let fn_rs: PathBuf = target_dir.join(FILE_NAME_FN_RS);
    write(&fn_rs, KEYWORD_FILE_HEADER).await?;
    let impl_rs: PathBuf = target_dir.join(FILE_NAME_IMPL_RS);
    write(&impl_rs, KEYWORD_FILE_HEADER).await?;
    let struct_rs: PathBuf = target_dir.join(FILE_NAME_STRUCT_RS);
    write(&struct_rs, KEYWORD_FILE_HEADER).await?;
    Ok(())
}

/// Create service template files
///
/// # Arguments
///
/// - `&Path` - Target directory path
/// - `&str` - Name of the component
///
/// # Returns
///
/// - `Result<(), TemplateError>` - Success or error
async fn create_service_template(
    target_dir: &Path,
    _component_name: &str,
) -> Result<(), TemplateError> {
    ensure_directory(target_dir).await?;
    let mod_rs: PathBuf = target_dir.join(FILE_NAME_MOD_RS);
    write_mod_rs(&mod_rs, &[MODULE_NAME_IMPL, MODULE_NAME_STRUCT]).await?;
    let impl_rs: PathBuf = target_dir.join(FILE_NAME_IMPL_RS);
    write(&impl_rs, KEYWORD_FILE_HEADER).await?;
    let struct_rs: PathBuf = target_dir.join(FILE_NAME_STRUCT_RS);
    write(&struct_rs, KEYWORD_FILE_HEADER).await?;
    Ok(())
}

/// Create domain template files
///
/// # Arguments
///
/// - `&Path` - Target directory path
/// - `&str` - Name of the component
///
/// # Returns
///
/// - `Result<(), TemplateError>` - Success or error
async fn create_domain_template(
    target_dir: &Path,
    _component_name: &str,
) -> Result<(), TemplateError> {
    ensure_directory(target_dir).await?;
    let mod_rs: PathBuf = target_dir.join(FILE_NAME_MOD_RS);
    write_mod_rs(&mod_rs, &[MODULE_NAME_IMPL, MODULE_NAME_STRUCT]).await?;
    let impl_rs: PathBuf = target_dir.join(FILE_NAME_IMPL_RS);
    write(&impl_rs, KEYWORD_FILE_HEADER).await?;
    let struct_rs: PathBuf = target_dir.join(FILE_NAME_STRUCT_RS);
    write(&struct_rs, KEYWORD_FILE_HEADER).await?;
    Ok(())
}

/// Create mapper template files
///
/// # Arguments
///
/// - `&Path` - Target directory path
/// - `&str` - Name of the component
///
/// # Returns
///
/// - `Result<(), TemplateError>` - Success or error
async fn create_mapper_template(
    target_dir: &Path,
    _component_name: &str,
) -> Result<(), TemplateError> {
    ensure_directory(target_dir).await?;
    let mod_rs: PathBuf = target_dir.join(FILE_NAME_MOD_RS);
    write_mod_rs(
        &mod_rs,
        &[
            MODULE_NAME_CONST,
            MODULE_NAME_ENUM,
            MODULE_NAME_FN,
            MODULE_NAME_IMPL,
            MODULE_NAME_STATIC,
            MODULE_NAME_STRUCT,
        ],
    )
    .await?;
    let const_rs: PathBuf = target_dir.join(FILE_NAME_CONST_RS);
    write(&const_rs, KEYWORD_FILE_HEADER).await?;
    let enum_rs: PathBuf = target_dir.join(FILE_NAME_ENUM_RS);
    write(&enum_rs, KEYWORD_FILE_HEADER).await?;
    let fn_rs: PathBuf = target_dir.join(FILE_NAME_FN_RS);
    write(&fn_rs, KEYWORD_FILE_HEADER).await?;
    let impl_rs: PathBuf = target_dir.join(FILE_NAME_IMPL_RS);
    write(&impl_rs, KEYWORD_FILE_HEADER).await?;
    let static_rs: PathBuf = target_dir.join(FILE_NAME_STATIC_RS);
    write(&static_rs, KEYWORD_FILE_HEADER).await?;
    let struct_rs: PathBuf = target_dir.join(FILE_NAME_STRUCT_RS);
    write(&struct_rs, KEYWORD_FILE_HEADER).await?;
    Ok(())
}

/// Create utils template files
///
/// # Arguments
///
/// - `&Path` - Target directory path
/// - `&str` - Name of the component
///
/// # Returns
///
/// - `Result<(), TemplateError>` - Success or error
async fn create_utils_template(
    target_dir: &Path,
    _component_name: &str,
) -> Result<(), TemplateError> {
    ensure_directory(target_dir).await?;
    let mod_rs: PathBuf = target_dir.join(FILE_NAME_MOD_RS);
    write_mod_rs(&mod_rs, &[MODULE_NAME_FN]).await?;
    let fn_rs: PathBuf = target_dir.join(FILE_NAME_FN_RS);
    write(&fn_rs, KEYWORD_FILE_HEADER).await?;
    Ok(())
}

/// Create exception template files
///
/// # Arguments
///
/// - `&Path` - Target directory path
/// - `&str` - Name of the component
///
/// # Returns
///
/// - `Result<(), TemplateError>` - Success or error
async fn create_exception_template(
    target_dir: &Path,
    _component_name: &str,
) -> Result<(), TemplateError> {
    ensure_directory(target_dir).await?;
    let mod_rs: PathBuf = target_dir.join(FILE_NAME_MOD_RS);
    write_empty_mod_rs(&mod_rs).await?;
    Ok(())
}

/// Create repository template files
///
/// # Arguments
///
/// - `&Path` - Target directory path
/// - `&str` - Name of the component
///
/// # Returns
///
/// - `Result<(), TemplateError>` - Success or error
async fn create_repository_template(
    target_dir: &Path,
    _component_name: &str,
) -> Result<(), TemplateError> {
    ensure_directory(target_dir).await?;
    let mod_rs: PathBuf = target_dir.join(FILE_NAME_MOD_RS);
    write_mod_rs(&mod_rs, &[MODULE_NAME_IMPL, MODULE_NAME_STRUCT]).await?;
    let impl_rs: PathBuf = target_dir.join(FILE_NAME_IMPL_RS);
    write(&impl_rs, KEYWORD_FILE_HEADER).await?;
    let struct_rs: PathBuf = target_dir.join(FILE_NAME_STRUCT_RS);
    write(&struct_rs, KEYWORD_FILE_HEADER).await?;
    Ok(())
}

/// Create model template files
///
/// # Arguments
///
/// - `&Path` - Target directory path
/// - `&str` - Name of the component
/// - `&ModelSubType` - Model subtype
///
/// # Returns
///
/// - `Result<(), TemplateError>` - Success or error
async fn create_model_template(
    target_dir: &Path,
    _component_name: &str,
    sub_type: &ModelSubType,
) -> Result<(), TemplateError> {
    let sub_type_name: String = get_model_sub_type_name(sub_type);
    let model_dir: PathBuf = target_dir.join(&sub_type_name);
    ensure_directory(&model_dir).await?;
    let mod_rs: PathBuf = model_dir.join(FILE_NAME_MOD_RS);
    write_mod_rs(&mod_rs, &[MODULE_NAME_STRUCT]).await?;
    let struct_rs: PathBuf = model_dir.join(FILE_NAME_STRUCT_RS);
    write(&struct_rs, KEYWORD_FILE_HEADER).await?;
    Ok(())
}

/// Execute template generation
///
/// # Arguments
///
/// - `TemplateType` - Type of template component
/// - `&str` - Name of the component
/// - `Option<ModelSubType>` - Optional model subtype for model components
///
/// # Returns
///
/// - `Result<(), TemplateError>` - Success or error
pub async fn execute_template(
    template_type: TemplateType,
    component_name: &str,
    model_sub_type: Option<ModelSubType>,
) -> Result<(), TemplateError> {
    let config: TemplateConfig =
        TemplateConfig::new(template_type, component_name.to_string(), model_sub_type);
    let base_path: PathBuf = PathBuf::from(&config.base_directory);
    let dir_name: String = get_directory_name(&config.template_type);
    let type_dir: PathBuf = base_path.join(&dir_name);
    let target_dir: PathBuf = type_dir.join(&config.component_name);
    if target_dir.exists() {
        return Err(TemplateError::DirectoryExists(
            target_dir.to_string_lossy().to_string(),
        ));
    }
    ensure_directory(&type_dir).await?;
    match config.template_type {
        TemplateType::Controller => {
            create_controller_template(&target_dir, &config.component_name).await?
        }
        TemplateType::View => create_view_template(&target_dir, &config.component_name).await?,
        TemplateType::Service => {
            create_service_template(&target_dir, &config.component_name).await?
        }
        TemplateType::Domain => create_domain_template(&target_dir, &config.component_name).await?,
        TemplateType::Mapper => create_mapper_template(&target_dir, &config.component_name).await?,
        TemplateType::Utils => create_utils_template(&target_dir, &config.component_name).await?,
        TemplateType::Exception => {
            create_exception_template(&target_dir, &config.component_name).await?
        }
        TemplateType::Repository => {
            create_repository_template(&target_dir, &config.component_name).await?
        }
        TemplateType::Model => {
            let sub_type: ModelSubType = config.model_sub_type.ok_or_else(|| {
                TemplateError::InvalidModelSubType(ERROR_MISSING_MODEL_SUB_TYPE.to_string())
            })?;
            create_model_template(&target_dir, &config.component_name, &sub_type).await?;
        }
    }
    let _: Result<(), io::Error> = format_generated_path(&target_dir).await;
    log::info!(
        "Created {dir_name} '{}' at {}",
        config.component_name,
        target_dir.display()
    );
    Ok(())
}

/// Run `cargo fmt` on freshly generated template code so the scaffolded
/// files match the project formatting out of the box.
///
/// # Arguments
///
/// - `&Path` - Path to format
///
/// # Returns
///
/// - `Result<(), io::Error>` - Success or error
async fn format_generated_path(path: &Path) -> Result<(), io::Error> {
    let mut cmd: Command = Command::new(CARGO_FMT_PROGRAM);
    cmd.arg("fmt");
    cmd.arg("--");
    cmd.arg(path);
    cmd.stdout(Stdio::null()).stderr(Stdio::null());
    cmd.status().await?;
    Ok(())
}
