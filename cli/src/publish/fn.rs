use super::*;

/// Discover all packages in the workspace
///
/// # Arguments
///
/// - `&Path`: Path to workspace root Cargo.toml
///
/// # Returns
///
/// - `Result<Vec<Package>, PublishError>`: List of packages or error
async fn discover_packages(workspace_root: &Path) -> Result<Vec<Package>, PublishError> {
    let content: String = read_to_string(workspace_root).await?;
    let doc: Value = toml::from_str(&content).map_err(|_| PublishError::ManifestParseError)?;
    let mut packages: Vec<Package> = Vec::new();
    if let Some(workspace) = doc.get("workspace")
        && let Some(members) = workspace
            .get("members")
            .and_then(|members_value: &Value| members_value.as_array())
    {
        for member in members {
            if let Some(pattern) = member.as_str() {
                let base_path: &Path = workspace_root.parent().unwrap_or(workspace_root);
                expand_pattern(base_path, pattern, &mut packages).await?;
            }
        }
    }
    if packages.is_empty() {
        let package: Package = read_single_package(workspace_root).await?;
        packages.push(package);
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

/// Read a single package (non-workspace mode)
///
/// # Arguments
///
/// - `&Path`: Path to Cargo.toml
///
/// # Returns
///
/// - `Result<Package, PublishError>`: Package info or error
async fn read_single_package(manifest_path: &Path) -> Result<Package, PublishError> {
    read_package_manifest(manifest_path).await
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

/// Perform topological sort on packages based on dependencies
///
/// # Arguments
///
/// - `&[Package]`: List of packages to sort
///
/// # Returns
///
/// - `Result<Vec<Package>, PublishError>`: Sorted packages or error if circular
fn topological_sort(packages: &[Package]) -> Result<Vec<Package>, PublishError> {
    let mut in_degree: HashMap<String, usize> = HashMap::new();
    let mut graph: HashMap<String, Vec<String>> = HashMap::new();
    let package_map: HashMap<String, Package> = packages
        .iter()
        .map(|package: &Package| (package.name.clone(), package.clone()))
        .collect();
    for package in packages {
        in_degree.entry(package.name.clone()).or_insert(0);
        for dep in &package.local_dependencies {
            if package_map.contains_key(dep) {
                graph
                    .entry(dep.clone())
                    .or_default()
                    .push(package.name.clone());
                *in_degree.entry(package.name.clone()).or_insert(0) += 1;
            }
        }
    }
    let mut queue: VecDeque<String> = VecDeque::new();
    for (name, degree) in &in_degree {
        if *degree == 0 {
            queue.push_back(name.clone());
        }
    }
    let mut result: Vec<Package> = Vec::new();
    while let Some(name) = queue.pop_front() {
        if let Some(package) = package_map.get(&name) {
            result.push(package.clone());
        }
        if let Some(dependents) = graph.get(&name) {
            for dependent in dependents {
                if let Some(degree) = in_degree.get_mut(dependent) {
                    *degree -= 1;
                    if *degree == 0 {
                        queue.push_back(dependent.clone());
                    }
                }
            }
        }
    }
    if result.len() != packages.len() {
        return Err(PublishError::CircularDependency);
    }
    Ok(result)
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
        .current_dir(&package.path)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .await?;
    if output.status.success() {
        Ok(())
    } else {
        let stderr: String = String::from_utf8_lossy(&output.stderr).to_string();
        Err(stderr.into())
    }
}

/// Execute publish command for all packages in workspace
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
    let packages: Vec<Package> = discover_packages(path).await?;
    if packages.is_empty() {
        return Ok(Vec::new());
    }
    let sorted_packages: Vec<Package> = topological_sort(&packages)?;
    let mut results: Vec<PublishResult> = Vec::new();
    for package in sorted_packages {
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
