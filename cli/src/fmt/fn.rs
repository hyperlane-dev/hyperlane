use super::*;

/// Sort derive traits in a single line
///
/// # Arguments
///
/// - `&str`: The line containing derive attribute
///
/// # Returns
///
/// - `Option<String>`: Sorted line if derive found, None otherwise
fn sort_derive_in_line(line: &str) -> Option<String> {
    let captures: Captures<'_> = DERIVE_REGEX.captures(line)?;
    let derive_content: &str = captures.get(1)?.as_str();
    let mut traits: Vec<String> = derive_content
        .split(',')
        .map(|s: &str| s.trim().to_string())
        .filter(|s: &String| !s.is_empty())
        .collect();
    traits.sort_by_key(|a: &String| a.to_lowercase());
    let sorted_traits: String = traits.join(", ");
    let result: String = line.replace(derive_content, &sorted_traits);
    Some(result)
}

/// Format derive attributes in a file
///
/// # Arguments
///
/// - `&Path`: Path to the Rust file
///
/// # Returns
///
/// - `Result<bool, io::Error>`: True if file was modified, false otherwise
async fn format_derive_in_file(file_path: &Path) -> Result<bool, io::Error> {
    let content: String = read_to_string(file_path).await?;
    let lines: std::str::Lines<'_> = content.lines();
    let mut modified: bool = false;
    let mut new_content: String = String::new();
    for line in lines {
        let trimmed: &str = line.trim();
        let new_line: String = if trimmed.starts_with("#[derive(") {
            if let Some(sorted) = sort_derive_in_line(line) {
                if sorted != line {
                    modified = true;
                }
                sorted
            } else {
                line.to_string()
            }
        } else {
            line.to_string()
        };
        new_content.push_str(&new_line);
        new_content.push('\n');
    }
    if modified {
        write(file_path, new_content).await?;
    }
    Ok(modified)
}

/// Find all Rust files in workspace
///
/// # Arguments
///
/// - `&Path`: Path to Cargo.toml
///
/// # Returns
///
/// - `Result<Vec<PathBuf>, io::Error>`: List of Rust file paths
async fn find_rust_files(manifest_path: &Path) -> Result<Vec<PathBuf>, io::Error> {
    let mut files: Vec<PathBuf> = Vec::new();
    let workspace_root: &Path = manifest_path.parent().unwrap_or(Path::new("."));
    let src_dir: PathBuf = workspace_root.join("src");
    if src_dir.exists() {
        find_rust_files_in_dir(&src_dir, &mut files).await?;
    }
    let content: String = read_to_string(manifest_path).await?;
    if let Ok(doc) = toml::from_str::<Value>(&content)
        && let Some(workspace) = doc.get("workspace")
        && let Some(members) = workspace.get("members").and_then(|m: &Value| m.as_array())
    {
        for member in members {
            if let Some(pattern) = member.as_str() {
                let member_src: PathBuf = workspace_root.join(pattern).join("src");
                if member_src.exists() {
                    find_rust_files_in_dir(&member_src, &mut files).await?;
                }
            }
        }
    }
    Ok(files)
}

/// Recursively find Rust files in directory
///
/// # Arguments
///
/// - `&Path`: Directory to search
/// - `&mut Vec<PathBuf>`: Vector to collect file paths
///
/// # Returns
///
/// - `Result<(), io::Error>`: Success or error
async fn find_rust_files_in_dir(dir: &Path, files: &mut Vec<PathBuf>) -> Result<(), io::Error> {
    let mut entries: ReadDir = read_dir(dir).await?;
    while let Some(entry) = entries.next_entry().await? {
        let path: PathBuf = entry.path();
        if path.is_file() && path.extension().is_some_and(|ext: &OsStr| ext == "rs") {
            files.push(path);
        } else if path.is_dir() {
            Box::pin(find_rust_files_in_dir(&path, files)).await?;
        }
    }
    Ok(())
}

/// Format derive attributes in all workspace files
///
/// # Arguments
///
/// - `&str`: Path to Cargo.toml
///
/// # Returns
///
/// - `Result<(), io::Error>`: Success or error
async fn format_derive_attributes(manifest_path: &str) -> Result<(), io::Error> {
    let path: &Path = Path::new(manifest_path);
    let files: Vec<PathBuf> = find_rust_files(path).await?;
    let modified_count: Arc<Mutex<usize>> = Arc::new(Mutex::new(0));
    let mut handles: Vec<JoinHandle<Result<(), io::Error>>> = Vec::new();
    for file in files {
        let counter: Arc<Mutex<usize>> = Arc::clone(&modified_count);
        let handle: JoinHandle<Result<(), io::Error>> = spawn(async move {
            if format_derive_in_file(&file).await? {
                let mut count: MutexGuard<'_, usize> = counter.lock().await;
                *count += 1;
            }
            Ok(())
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.await??;
    }
    let count: usize = *modified_count.lock().await;
    if count > 0 {
        log::info!("Sorted derive attributes in {count} files");
    }
    Ok(())
}

/// Check if cargo-clippy is installed
///
/// # Returns
///
/// - `bool`: True if cargo-clippy is available
fn is_cargo_clippy_installed() -> bool {
    which("cargo-clippy").is_ok()
}

/// Install cargo-clippy using rustup
///
/// # Returns
///
/// - `Result<(), io::Error>`: Success or error
async fn install_cargo_clippy() -> Result<(), io::Error> {
    log::warn!("cargo-clippy not found, installing...");
    let output: std::process::Output = Command::new("rustup")
        .arg("component")
        .arg("add")
        .arg("clippy")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .await?;
    let stdout: String = String::from_utf8_lossy(&output.stdout).trim().to_string();
    let stderr: String = String::from_utf8_lossy(&output.stderr).trim().to_string();
    if !stdout.is_empty() {
        for line in stdout.lines() {
            log::info!("{line}");
        }
    }
    if !stderr.is_empty() {
        if output.status.success() {
            for line in stderr.lines() {
                if line.is_empty() {
                    continue;
                }
                log::info!("{line}");
            }
        } else {
            for line in stderr.lines() {
                if line.is_empty() {
                    continue;
                }
                log::error!("{line}");
            }
        }
    }
    if !output.status.success() {
        return Err(io::Error::other("failed to install cargo-clippy"));
    }
    Ok(())
}

/// Execute clippy fix command
///
/// # Arguments
///
/// - `&Args`: The parsed arguments
///
/// # Returns
///
/// - `Result<(), io::Error>`: Success or error
async fn execute_clippy_fix(args: &Args) -> Result<(), io::Error> {
    if !is_cargo_clippy_installed() {
        install_cargo_clippy().await?;
    }
    let mut cmd: Command = Command::new("cargo");
    cmd.arg("clippy")
        .arg("--fix")
        .arg("--workspace")
        .arg("--all-targets")
        .arg("--allow-dirty");
    if let Some(ref manifest_path) = args.manifest_path {
        cmd.arg("--manifest-path").arg(manifest_path);
    }
    cmd.stdout(Stdio::piped()).stderr(Stdio::piped());
    let output: std::process::Output = cmd.output().await?;
    let stdout: String = String::from_utf8_lossy(&output.stdout).trim().to_string();
    let stderr: String = String::from_utf8_lossy(&output.stderr).trim().to_string();
    if !stdout.is_empty() {
        for line in stdout.lines() {
            log::info!("{line}");
        }
    }
    if !stderr.is_empty() {
        if output.status.success() {
            for line in stderr.lines() {
                if line.is_empty() {
                    continue;
                }
                log::info!("{line}");
            }
        } else {
            for line in stderr.lines() {
                if line.is_empty() {
                    continue;
                }
                log::error!("{line}");
            }
        }
    }
    if !output.status.success() {
        return Err(io::Error::other("cargo clippy --fix failed"));
    }
    Ok(())
}

/// Execute fmt command
///
/// # Arguments
///
/// - `&Args`: The parsed arguments
///
/// # Returns
///
/// - `Result<(), io::Error>`: Success or error
pub async fn execute_fmt(args: &Args) -> Result<(), io::Error> {
    let manifest_path: String = args
        .manifest_path
        .clone()
        .unwrap_or_else(|| "Cargo.toml".to_string());
    if !args.check {
        format_derive_attributes(&manifest_path).await?;
    }
    let mut cmd: Command = Command::new("cargo");
    cmd.arg("fmt");
    if args.check {
        cmd.arg("--check");
    }
    if let Some(ref manifest_path) = args.manifest_path {
        cmd.arg("--manifest-path").arg(manifest_path);
    }
    cmd.stdout(Stdio::piped()).stderr(Stdio::piped());
    let output: std::process::Output = cmd.output().await?;
    let stdout: String = String::from_utf8_lossy(&output.stdout).trim().to_string();
    let stderr: String = String::from_utf8_lossy(&output.stderr).trim().to_string();
    if !stdout.is_empty() {
        for line in stdout.lines() {
            log::info!("{line}");
        }
    }
    if !stderr.is_empty() {
        if output.status.success() {
            for line in stderr.lines() {
                if line.is_empty() {
                    continue;
                }
                log::info!("{line}");
            }
        } else {
            for line in stderr.lines() {
                if line.is_empty() {
                    continue;
                }
                log::error!("{line}");
            }
        }
    }
    if !output.status.success() {
        return Err(io::Error::other("cargo fmt failed"));
    }
    if !args.check {
        execute_clippy_fix(args).await?;
    }
    Ok(())
}

/// Format code at specific path
///
/// # Arguments
///
/// - `&Path`: Path to format
///
/// # Returns
///
/// - `Result<(), io::Error>`: Success or error
pub async fn format_path(path: &Path) -> Result<(), io::Error> {
    let mut cmd: Command = Command::new("cargo");
    cmd.arg("fmt").arg("--").arg(path);
    cmd.stdout(Stdio::null()).stderr(Stdio::null());
    cmd.status().await?;
    Ok(())
}
