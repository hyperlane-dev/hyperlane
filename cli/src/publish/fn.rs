use super::*;

/// Discover all packages in the workspace: every `[workspace.members]`
/// entry in declaration order, with the root package (when the workspace
/// root manifest also declares `[package]`) appended last.
///
/// # Arguments
///
/// - `&Path`: Path to workspace root Cargo.toml
///
/// # Returns
///
/// - `Result<Vec<Package>, PublishError>`: List of packages or error
async fn discover_packages(workspace_manifest: &Path) -> Result<Vec<Package>, PublishError> {
    let content: String = read_to_string(workspace_manifest).await?;
    let doc: Value = toml::from_str(&content).map_err(|_| PublishError::ManifestParseError)?;
    let mut packages: Vec<Package> = Vec::new();
    if let Some(workspace) = doc.get("workspace")
        && let Some(members) = workspace
            .get("members")
            .and_then(|members_value: &Value| members_value.as_array())
    {
        for member in members {
            if let Some(pattern) = member.as_str() {
                let base_path: &Path = workspace_manifest.parent().unwrap_or(workspace_manifest);
                expand_pattern(base_path, pattern, &mut packages).await?;
            }
        }
    }
    if doc.get("package").is_some() {
        let root_package: Package = read_package_manifest(workspace_manifest).await?;
        packages.push(root_package);
    }
    Ok(packages)
}

/// Expand glob pattern to find package directories
///
/// # Arguments
///
/// - `&Path`: Base path for expansion
/// - `&str`: Glob pattern
/// - `&mut Vec<Package>`: Output vector for found packages
///
/// # Returns
///
/// - `Result<(), PublishError>`: Success or error
async fn expand_pattern(
    base_path: &Path,
    pattern: &str,
    packages: &mut Vec<Package>,
) -> Result<(), PublishError> {
    if pattern.contains('*') {
        let parent: &Path = Path::new(pattern).parent().unwrap_or(Path::new("."));
        let full_parent: PathBuf = base_path.join(parent);
        if full_parent.is_dir() {
            let mut entries: ReadDir = read_dir(&full_parent).await?;
            while let Some(entry) = entries.next_entry().await? {
                let path: PathBuf = entry.path();
                if path.is_dir() {
                    let cargo_toml: PathBuf = path.join("Cargo.toml");
                    if cargo_toml.exists() {
                        let package: Package = read_package_manifest(&cargo_toml).await?;
                        packages.push(package);
                    }
                }
            }
        }
    } else {
        let cargo_toml: PathBuf = base_path.join(pattern).join("Cargo.toml");
        if cargo_toml.exists() {
            let package: Package = read_package_manifest(&cargo_toml).await?;
            packages.push(package);
        }
    }
    Ok(())
}

/// Read package manifest and extract information
///
/// # Arguments
///
/// - `&Path`: Path to package Cargo.toml
///
/// # Returns
///
/// - `Result<Package, PublishError>`: Package info or error
async fn read_package_manifest(manifest_path: &Path) -> Result<Package, PublishError> {
    let content: String = read_to_string(manifest_path).await?;
    let doc: Value = toml::from_str(&content).map_err(|_| PublishError::ManifestParseError)?;
    let package_table: &Value = doc.get("package").ok_or(PublishError::ManifestParseError)?;
    let name: String = package_table
        .get("name")
        .and_then(|n: &Value| n.as_str())
        .ok_or(PublishError::ManifestParseError)?
        .to_string();
    let version: String = package_table
        .get("version")
        .and_then(|v: &Value| v.as_str())
        .ok_or(PublishError::ManifestParseError)?
        .to_string();
    let path: PathBuf = manifest_path
        .parent()
        .filter(|p: &&Path| !p.as_os_str().is_empty())
        .map_or_else(|| PathBuf::from("."), |p: &Path| p.to_path_buf());
    let local_dependencies: Vec<String> = extract_local_dependencies(&doc, manifest_path)?;
    Ok(Package {
        name,
        version,
        path,
        local_dependencies,
    })
}

/// Extract local workspace dependencies from manifest
///
/// # Arguments
///
/// - `&Value`: Parsed manifest
/// - `&Path`: Path to manifest for resolving relative paths
///
/// # Returns
///
/// - `Result<Vec<String>, PublishError>`: List of local dependency names
fn extract_local_dependencies(
    doc: &Value,
    _manifest_path: &Path,
) -> Result<Vec<String>, PublishError> {
    let mut deps: Vec<String> = Vec::new();
    let dep_sections: [&str; 3] = ["dependencies", "dev-dependencies", "build-dependencies"];
    for section in &dep_sections {
        if let Some(table) = doc
            .get(section)
            .and_then(|section_value: &Value| section_value.as_table())
        {
            for (dep_name, dep_value) in table {
                let is_local: bool = match dep_value {
                    Value::Table(t) => {
                        t.get("path").is_some()
                            || t.get("workspace")
                                .and_then(|workspace_value: &Value| workspace_value.as_bool())
                                .unwrap_or(false)
                    }
                    _ => false,
                };
                if is_local {
                    deps.push(dep_name.clone());
                }
            }
        }
    }
    Ok(deps)
}

/// Validate that the publish order satisfies every package's local
/// dependency constraints: a package must never appear before a
/// workspace-local dependency of its own.
///
/// # Arguments
///
/// - `&[Package]`: Packages in intended publish order
///
/// # Returns
///
/// - `Result<(), PublishError>`: `InvalidPublishOrder` naming the first
///   offending pair when the order violates a local dependency.
fn validate_publish_order(packages: &[Package]) -> Result<(), PublishError> {
    let position: HashMap<String, usize> = packages
        .iter()
        .enumerate()
        .map(|(index, package): (usize, &Package)| (package.name.clone(), index))
        .collect();
    for package in packages {
        let Some(package_position) = position.get(&package.name) else {
            continue;
        };
        for dep in &package.local_dependencies {
            if let Some(dep_position) = position.get(dep)
                && dep_position > package_position
            {
                return Err(PublishError::InvalidPublishOrder(format!(
                    "{} depends on {} but is listed before it in [workspace.members]",
                    package.name, dep
                )));
            }
        }
    }
    Ok(())
}

/// Resolve the publish order for a workspace: `[workspace.members]`
/// declaration order with the root package (if any) appended last,
/// validated against local dependency constraints.
///
/// # Arguments
///
/// - `&str`: Path to the workspace root Cargo.toml
///
/// # Returns
///
/// - `Result<Vec<Package>, PublishError>`: Ordered packages, or an
///   error when the members order violates a local dependency.
pub async fn resolve_publish_order(manifest_path: &str) -> Result<Vec<Package>, PublishError> {
    let workspace_manifest: &Path = Path::new(manifest_path);
    let packages: Vec<Package> = discover_packages(workspace_manifest).await?;
    validate_publish_order(&packages)?;
    Ok(packages)
}

/// Check whether `cargo publish` stderr indicates the package version is
/// already present on the registry (a success case for idempotent
/// re-runs).
///
/// # Arguments
///
/// - `&str`: cargo publish stderr output
///
/// # Returns
///
/// - `bool`: True when the output means "already published"
pub fn is_already_published(stderr: &str) -> bool {
    stderr.contains("already been uploaded") || stderr.contains("is already published")
}

/// Publish a single package with retry logic
///
/// # Arguments
///
/// - `&Package`: Package to publish
/// - `u32`: Maximum retry attempts
///
/// # Returns
///
/// - `PublishResult`: Result with success status and retry count
async fn publish_package_with_retry(package: &Package, max_retries: u32) -> PublishResult {
    let mut attempt: u32 = 0;
    let mut last_error: Option<String> = None;
    while attempt <= max_retries {
        match publish_single_package(package).await {
            Ok(()) => {
                return PublishResult {
                    package_name: package.name.clone(),
                    success: true,
                    error: None,
                    retries: attempt,
                };
            }
            Err(error) => {
                last_error = Some(error.to_string());
                attempt += 1;
                if attempt <= max_retries {
                    sleep(Duration::from_secs(2_u64.pow(attempt))).await;
                }
            }
        }
    }
    PublishResult {
        package_name: package.name.clone(),
        success: false,
        error: last_error,
        retries: attempt - 1,
    }
}

/// Execute cargo publish command for a single package
///
/// # Arguments
///
/// - `&Package`: Package to publish
///
/// # Returns
///
/// - `Result<(), Box<dyn std::error::Error>>`: Success or error
async fn publish_single_package(package: &Package) -> Result<(), Box<dyn std::error::Error>> {
    let output: std::process::Output = Command::new("cargo")
        .arg("publish")
        .arg("--allow-dirty")
        .arg("--no-verify")
        .current_dir(&package.path)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .await?;
    if output.status.success() {
        return Ok(());
    }
    let stderr: String = String::from_utf8_lossy(&output.stderr).to_string();
    if is_already_published(&stderr) {
        log::info!("{} is already published, treating as success", package.name);
        return Ok(());
    }
    Err(stderr.into())
}

/// Execute publish command for all packages in workspace
///
/// Publishes in `[workspace.members]` declaration order with the root
/// package (if any) last, after validating the order against local
/// dependency constraints.
///
/// # Arguments
///
/// - `&str`: Path to workspace Cargo.toml
/// - `u32`: Maximum retry attempts per package
///
/// # Returns
///
/// - `Result<Vec<PublishResult>, PublishError>`: Results for all packages
pub async fn execute_publish(
    manifest_path: &str,
    max_retries: u32,
) -> Result<Vec<PublishResult>, PublishError> {
    let path: &Path = Path::new(manifest_path);
    let path: &Path = match path.parent() {
        Some(parent) if !parent.as_os_str().is_empty() => parent,
        _ => Path::new("."),
    };
    let workspace_manifest: PathBuf = path.join("Cargo.toml");
    let sync_report: SyncReport =
        match execute_sync(workspace_manifest.to_str().unwrap_or("Cargo.toml")).await {
            Ok(report) => report,
            Err(error) => return Err(PublishError::SyncFailed(error)),
        };
    if sync_report.file_changed {
        log::info!(
            "publish: synced workspace dependencies ({} renamed, {} versioned) to v{}",
            sync_report.renamed_entries.len(),
            sync_report.versioned_entries.len(),
            sync_report.workspace_version,
        );
    }
    let ordered_packages: Vec<Package> =
        resolve_publish_order(workspace_manifest.to_str().unwrap_or("Cargo.toml")).await?;
    if ordered_packages.is_empty() {
        return Ok(Vec::new());
    }
    let mut results: Vec<PublishResult> = Vec::new();
    for package in ordered_packages {
        log::info!("Publishing {} v{}...", package.name, package.version);
        let result: PublishResult = publish_package_with_retry(&package, max_retries).await;
        if result.success {
            if result.retries == 0 {
                log::info!("Successfully published {}", result.package_name,);
            } else {
                log::info!(
                    "Successfully published {} (retried {} times)",
                    result.package_name,
                    result.retries
                );
            }
        } else if let Some(error) = &result.error {
            log::error!("Failed to publish {}: {error}", result.package_name);
        } else {
            log::error!("Failed to publish {}", result.package_name);
        }
        results.push(result);
    }
    Ok(results)
}
