use super::*;

/// Get directory name for template type
///
/// # Arguments
///
/// - `&TemplateType`: The template type
///
/// # Returns
///
/// - `String`: Directory name
fn get_directory_name(template_type: &TemplateType) -> String {
    match template_type {
        TemplateType::Controller => "controller".to_string(),
        TemplateType::Domain => "domain".to_string(),
        TemplateType::Exception => "exception".to_string(),
        TemplateType::Mapper => "mapper".to_string(),
        TemplateType::Model => "model".to_string(),
        TemplateType::Repository => "repository".to_string(),
        TemplateType::Service => "service".to_string(),
        TemplateType::Utils => "utils".to_string(),
        TemplateType::View => "view".to_string(),
    }
}

/// Get model subtype directory name
///
/// # Arguments
///
/// - `&ModelSubType`: The model subtype
///
/// # Returns
///
/// - `String`: Directory name
fn get_model_sub_type_name(sub_type: &ModelSubType) -> String {
    match sub_type {
        ModelSubType::Application => "application".to_string(),
        ModelSubType::Request => "request".to_string(),
        ModelSubType::Response => "response".to_string(),
    }
}

/// Create directory if it does not exist
///
/// # Arguments
///
/// - `&Path`: Path to the directory
///
/// # Returns
///
/// - `Result<(), TemplateError>`: Success or error
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
/// - `&Path`: Path to mod.rs file
/// - `&[&str]`: List of modules to include
///
/// # Returns
///
/// - `Result<(), TemplateError>`: Success or error
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
        if raw_name == "const" || raw_name == "static" {
            pub_use_parts.push(mod_name);
        } else if raw_name == "enum" || raw_name == "fn" {
            pub_use_parts.push(format!("{mod_name}::*"));
        } else if raw_name == "struct" {
            pub_use_parts.push(mod_name);
        }
    }
    if !pub_use_parts.is_empty() {
        content.push_str("pub use {");
        content.push_str(&pub_use_parts.join(", "));
        content.push_str("};\n");
    }
    content.push('\n');
    content.push_str("use super::*;\n");
    write(path, content).await?;
    Ok(())
}

/// Write empty mod.rs
///
/// # Arguments
///
/// - `&Path`: Path to mod.rs file
///
/// # Returns
///
/// - `Result<(), TemplateError>`: Success or error
async fn write_empty_mod_rs(path: &Path) -> Result<(), TemplateError> {
    write(path, "\n").await?;
    Ok(())
}

/// Create controller template files
///
/// # Arguments
///
/// - `&Path`: Target directory path
/// - `&str`: Name of the component
///
/// # Returns
///
/// - `Result<(), TemplateError>`: Success or error
async fn create_controller_template(
    target_dir: &Path,
    _component_name: &str,
) -> Result<(), TemplateError> {
    ensure_directory(target_dir).await?;
    let mod_rs: PathBuf = target_dir.join("mod.rs");
    write_mod_rs(&mod_rs, &["fn", "impl", "struct"]).await?;
    let fn_rs: PathBuf = target_dir.join("fn.rs");
    write(&fn_rs, "use super::*;\n").await?;
    let impl_rs: PathBuf = target_dir.join("impl.rs");
    write(&impl_rs, "use super::*;\n").await?;
    let struct_rs: PathBuf = target_dir.join("struct.rs");
    write(&struct_rs, "use super::*;\n").await?;
    Ok(())
}

/// Create view template files
///
/// # Arguments
///
/// - `&Path`: Target directory path
/// - `&str`: Name of the component
///
/// # Returns
///
/// - `Result<(), TemplateError>`: Success or error
async fn create_view_template(
    target_dir: &Path,
    _component_name: &str,
) -> Result<(), TemplateError> {
    ensure_directory(target_dir).await?;
    let mod_rs: PathBuf = target_dir.join("mod.rs");
    write_mod_rs(&mod_rs, &["fn", "impl", "struct"]).await?;
    let fn_rs: PathBuf = target_dir.join("fn.rs");
    write(&fn_rs, "use super::*;\n").await?;
    let impl_rs: PathBuf = target_dir.join("impl.rs");
    write(&impl_rs, "use super::*;\n").await?;
    let struct_rs: PathBuf = target_dir.join("struct.rs");
    write(&struct_rs, "use super::*;\n").await?;
    Ok(())
}

/// Create service template files
///
/// # Arguments
///
/// - `&Path`: Target directory path
/// - `&str`: Name of the component
///
/// # Returns
///
/// - `Result<(), TemplateError>`: Success or error
async fn create_service_template(
    target_dir: &Path,
    _component_name: &str,
) -> Result<(), TemplateError> {
    ensure_directory(target_dir).await?;
    let mod_rs: PathBuf = target_dir.join("mod.rs");
    write_mod_rs(&mod_rs, &["impl", "struct"]).await?;
    let impl_rs: PathBuf = target_dir.join("impl.rs");
    write(&impl_rs, "use super::*;\n").await?;
    let struct_rs: PathBuf = target_dir.join("struct.rs");
    write(&struct_rs, "use super::*;\n").await?;
    Ok(())
}

/// Create domain template files
///
/// # Arguments
///
/// - `&Path`: Target directory path
/// - `&str`: Name of the component
///
/// # Returns
///
/// - `Result<(), TemplateError>`: Success or error
async fn create_domain_template(
    target_dir: &Path,
    _component_name: &str,
) -> Result<(), TemplateError> {
    ensure_directory(target_dir).await?;
    let mod_rs: PathBuf = target_dir.join("mod.rs");
    write_mod_rs(&mod_rs, &["impl", "struct"]).await?;
    let impl_rs: PathBuf = target_dir.join("impl.rs");
    write(&impl_rs, "use super::*;\n").await?;
    let struct_rs: PathBuf = target_dir.join("struct.rs");
    write(&struct_rs, "use super::*;\n").await?;
    Ok(())
}

/// Create mapper template files
///
/// # Arguments
///
/// - `&Path`: Target directory path
/// - `&str`: Name of the component
///
/// # Returns
///
/// - `Result<(), TemplateError>`: Success or error
async fn create_mapper_template(
    target_dir: &Path,
    _component_name: &str,
) -> Result<(), TemplateError> {
    ensure_directory(target_dir).await?;
    let mod_rs: PathBuf = target_dir.join("mod.rs");
    write_mod_rs(
        &mod_rs,
        &["const", "enum", "fn", "impl", "static", "struct"],
    )
    .await?;
    let const_rs: PathBuf = target_dir.join("const.rs");
    write(&const_rs, "use super::*;\n").await?;
    let enum_rs: PathBuf = target_dir.join("enum.rs");
    write(&enum_rs, "use super::*;\n").await?;
    let fn_rs: PathBuf = target_dir.join("fn.rs");
    write(&fn_rs, "use super::*;\n").await?;
    let impl_rs: PathBuf = target_dir.join("impl.rs");
    write(&impl_rs, "use super::*;\n").await?;
    let static_rs: PathBuf = target_dir.join("static.rs");
    write(&static_rs, "use super::*;\n").await?;
    let struct_rs: PathBuf = target_dir.join("struct.rs");
    write(&struct_rs, "use super::*;\n").await?;
    Ok(())
}

/// Create utils template files
///
/// # Arguments
///
/// - `&Path`: Target directory path
/// - `&str`: Name of the component
///
/// # Returns
///
/// - `Result<(), TemplateError>`: Success or error
async fn create_utils_template(
    target_dir: &Path,
    _component_name: &str,
) -> Result<(), TemplateError> {
    ensure_directory(target_dir).await?;
    let mod_rs: PathBuf = target_dir.join("mod.rs");
    write_mod_rs(&mod_rs, &["fn"]).await?;
    let fn_rs: PathBuf = target_dir.join("fn.rs");
    write(&fn_rs, "use super::*;\n").await?;
    Ok(())
}

/// Create exception template files
///
/// # Arguments
///
/// - `&Path`: Target directory path
/// - `&str`: Name of the component
///
/// # Returns
///
/// - `Result<(), TemplateError>`: Success or error
async fn create_exception_template(
    target_dir: &Path,
    _component_name: &str,
) -> Result<(), TemplateError> {
    ensure_directory(target_dir).await?;
    let mod_rs: PathBuf = target_dir.join("mod.rs");
    write_empty_mod_rs(&mod_rs).await?;
    Ok(())
}

/// Create repository template files
///
/// # Arguments
///
/// - `&Path`: Target directory path
/// - `&str`: Name of the component
///
/// # Returns
///
/// - `Result<(), TemplateError>`: Success or error
async fn create_repository_template(
    target_dir: &Path,
    _component_name: &str,
) -> Result<(), TemplateError> {
    ensure_directory(target_dir).await?;
    let mod_rs: PathBuf = target_dir.join("mod.rs");
    write_mod_rs(&mod_rs, &["impl", "struct"]).await?;
    let impl_rs: PathBuf = target_dir.join("impl.rs");
    write(&impl_rs, "use super::*;\n").await?;
    let struct_rs: PathBuf = target_dir.join("struct.rs");
    write(&struct_rs, "use super::*;\n").await?;
    Ok(())
}

/// Create model template files
///
/// # Arguments
///
/// - `&Path`: Target directory path
/// - `&str`: Name of the component
/// - `&ModelSubType`: Model subtype
///
/// # Returns
///
/// - `Result<(), TemplateError>`: Success or error
async fn create_model_template(
    target_dir: &Path,
    _component_name: &str,
    sub_type: &ModelSubType,
) -> Result<(), TemplateError> {
    let sub_type_name: String = get_model_sub_type_name(sub_type);
    let model_dir: PathBuf = target_dir.join(&sub_type_name);
    ensure_directory(&model_dir).await?;
    let mod_rs: PathBuf = model_dir.join("mod.rs");
    write_mod_rs(&mod_rs, &["struct"]).await?;
    let struct_rs: PathBuf = model_dir.join("struct.rs");
    write(&struct_rs, "use super::*;\n").await?;
    Ok(())
}

/// Execute template generation
///
/// # Arguments
///
/// - `&TemplateType`: Type of template component
/// - `&str`: Name of the component
/// - `model_sub_type`: Optional model subtype
///
/// # Returns
///
/// - `Result<(), TemplateError>`: Success or error
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
                TemplateError::InvalidModelSubType("Missing model subtype".to_string())
            })?;
            create_model_template(&target_dir, &config.component_name, &sub_type).await?;
        }
    }
    let _: Result<(), io::Error> = crate::fmt::format_path(&target_dir).await;
    log::info!(
        "Created {dir_name} '{}' at {}",
        config.component_name,
        target_dir.display()
    );
    Ok(())
}
