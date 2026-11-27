use super::*;

/// Executes unified `crate-cli` formatting first (Markdown, YAML, JSON/JSONC, TOML, plain text, derives, `cargo fmt`), then applies Hyperlane-specific attribute formatting.
///
/// # Arguments
///
/// - `&Path` - Workspace root directory, `Cargo.toml` manifest path, or single file path.
/// - `bool` - When `true`, verifies formatting without writing files.
///
/// # Returns
///
/// - `io::Result<HyperlaneFmtReport>` - Aggregated formatting report or error.
pub async fn execute_fmt(target_root: &Path, check: bool) -> io::Result<HyperlaneFmtReport> {
    let is_non_manifest_file: bool = target_root.is_file()
        && target_root.file_name().and_then(OsStr::to_str) != Some(FMT_CARGO_TOML);
    let text_report: UnifiedFmtReport = if is_non_manifest_file {
        format_workspace_text_files(target_root, check)?
    } else {
        execute_unified_fmt(target_root, check).await?
    };
    let workspace_dir: PathBuf = if target_root.is_file() {
        target_root
            .parent()
            .map(Path::to_path_buf)
            .unwrap_or_else(|| PathBuf::from("."))
    } else {
        target_root.to_path_buf()
    };
    let has_manifest: bool = workspace_dir.join(FMT_CARGO_TOML).exists();
    if has_manifest && !is_non_manifest_file {
        run_workspace_cargo_fmt(&workspace_dir, check).await?;
    }
    let (rust_scanned, rust_changed): (usize, Vec<String>) =
        format_hyperlane_attributes_in_workspace(target_root, check)?;
    let rust_formatted: usize = rust_changed.len();
    if !check && has_manifest && !is_non_manifest_file && rust_formatted > 0 {
        run_workspace_cargo_fmt(&workspace_dir, false).await?;
    }
    let mut changed_files: Vec<String> = text_report.get_changed_files().clone();
    changed_files.extend(rust_changed);
    Ok(HyperlaneFmtReport::new(
        text_report.get_files_scanned(),
        text_report.get_files_formatted(),
        rust_scanned,
        rust_formatted,
        check,
        changed_files,
    ))
}

/// Normalizes single-line Hyperlane attribute macros (`#[route(...)]`, `#[methods(...)]`, `#[hyperlane::...(...)]`, etc.) in a Rust source string.
///
/// # Arguments
///
/// - `&str` - Raw Rust source code.
///
/// # Returns
///
/// - `(String, usize)` - Tuple of `(formatted_source, normalized_attribute_count)`.
#[must_use]
pub fn format_hyperlane_attributes_in_source(source: &str) -> (String, usize) {
    let mut out: String = String::with_capacity(source.len());
    let mut hits: usize = 0;
    let ends_with_newline: bool = source.ends_with('\n');
    let lines: Vec<&str> = source.lines().collect();
    for (idx, line) in lines.iter().enumerate() {
        let formatted_line: String = match format_hyperlane_attr_line(line) {
            Some(next) if next != *line => {
                hits += 1;
                next
            }
            _ => (*line).to_string(),
        };
        out.push_str(&formatted_line);
        if idx + 1 < lines.len() || ends_with_newline {
            out.push('\n');
        }
    }
    (out, hits)
}

/// Formats Hyperlane attributes across all `.rs` files under `root` (or a single `.rs` file).
///
/// # Arguments
///
/// - `&Path` - Root directory or individual `.rs` file path.
/// - `bool` - When `true`, returns an error if any file needs formatting without modifying disk.
///
/// # Returns
///
/// - `io::Result<(usize, Vec<String>)>` - Tuple of `(scanned_count, changed_file_paths)`.
pub fn format_hyperlane_attributes_in_workspace(
    root: &Path,
    check: bool,
) -> io::Result<(usize, Vec<String>)> {
    let mut files: Vec<PathBuf> = Vec::new();
    collect_rs_files(root, true, &mut files)?;
    files.sort();
    let scanned: usize = files.len();
    let mut changed: Vec<String> = Vec::new();
    for file_path in files {
        let original: String = fs::read_to_string(&file_path)?;
        let (formatted, _): (String, usize) = format_hyperlane_attributes_in_source(&original);
        if formatted != original {
            changed.push(file_path.display().to_string());
            if !check {
                fs::write(&file_path, formatted)?;
            }
        }
    }
    if check && !changed.is_empty() {
        return Err(io::Error::other(format!(
            "{ERR_HYPERLANE_ATTR_CHECK_FAILED}: {}",
            changed.join(", ")
        )));
    }
    Ok((scanned, changed))
}

/// Runs `cargo fmt --all` (or `--all -- --check`) inside `workspace_dir`.
///
/// # Arguments
///
/// - `&Path` - Workspace directory containing `Cargo.toml`.
/// - `bool` - Whether to pass `--check`.
///
/// # Returns
///
/// - `io::Result<()>` - Execution status.
async fn run_workspace_cargo_fmt(workspace_dir: &Path, check: bool) -> io::Result<()> {
    let mut cmd: Command = Command::new(FMT_BIN_CARGO);
    cmd.current_dir(workspace_dir)
        .arg(FMT_ARG_FMT)
        .arg(FMT_ARG_ALL);
    if check {
        cmd.arg(FMT_ARG_DOUBLE_DASH).arg(FMT_ARG_CHECK);
    }
    cmd.stdout(Stdio::piped()).stderr(Stdio::piped());
    let output: Output = cmd.output().await?;
    if !output.status.success() {
        let stderr: String = String::from_utf8_lossy(&output.stderr).trim().to_string();
        return Err(io::Error::other(format!(
            "{ERR_CARGO_FMT_FAILED}: {stderr}"
        )));
    }
    Ok(())
}

/// Recursively collects `.rs` files under `path` while skipping generated directories.
///
/// # Arguments
///
/// - `&Path` - Directory or file path to traverse.
/// - `bool` - Whether `path` is the top-level target.
/// - `&mut Vec<PathBuf>` - Output vector of Rust source paths.
///
/// # Returns
///
/// - `io::Result<()>` - Traversal result.
fn collect_rs_files(path: &Path, is_root: bool, out: &mut Vec<PathBuf>) -> io::Result<()> {
    if path.is_file() {
        if path.extension().and_then(OsStr::to_str) == Some(FMT_EXT_RS) {
            out.push(path.to_path_buf());
        }
        return Ok(());
    }
    if !path.is_dir() {
        return Ok(());
    }
    if !is_root
        && let Some(name) = path.file_name().and_then(OsStr::to_str)
        && FMT_SKIP_DIRS.contains(&name)
    {
        return Ok(());
    }
    for entry_res in fs::read_dir(path)? {
        let entry: DirEntry = entry_res?;
        let child: PathBuf = entry.path();
        if child.is_dir() {
            collect_rs_files(&child, false, out)?;
        } else if child.extension().and_then(OsStr::to_str) == Some(FMT_EXT_RS) {
            out.push(child);
        }
    }
    Ok(())
}

/// Normalizes a single-line Hyperlane attribute if `line` matches `#[<hyperlane_attr>(...)]`.
///
/// # Arguments
///
/// - `&str` - Single source line.
///
/// # Returns
///
/// - `Option<String>` - Normalized line when it is a recognized single-line Hyperlane attribute.
fn format_hyperlane_attr_line(line: &str) -> Option<String> {
    let trimmed_start: &str = line.trim_start();
    let indent_len: usize = line.len() - trimmed_start.len();
    let indent: &str = &line[..indent_len];
    let trimmed: &str = trimmed_start.trim_end();
    if !trimmed.starts_with("#[") || !trimmed.ends_with(']') {
        return None;
    }
    let body: &str = &trimmed[2..trimmed.len() - 1];
    let open_paren: usize = body.find('(')?;
    if !body.ends_with(')') {
        return None;
    }
    let raw_path: &str = body[..open_paren].trim();
    let attr_ident: &str = raw_path.rsplit("::").next().unwrap_or(raw_path).trim();
    if !HYPERLANE_ATTR_NAMES.contains(&attr_ident) {
        return None;
    }
    let raw_args: &str = &body[open_paren + 1..body.len() - 1];
    let norm_args: String = normalize_attr_arg_list(raw_args);
    Some(format!("{indent}#[{raw_path}({norm_args})]"))
}

/// Normalizes spacing around commas and outer parentheses in a single-line attribute argument list while preserving string literals verbatim.
///
/// # Arguments
///
/// - `&str` - Raw argument slice inside `(...)`.
///
/// # Returns
///
/// - `String` - Normalized argument list.
fn normalize_attr_arg_list(raw: &str) -> String {
    let trimmed: &str = raw.trim();
    if trimmed.is_empty() {
        return String::new();
    }
    let mut parts: Vec<String> = Vec::new();
    let mut cur: String = String::new();
    let mut in_string: bool = false;
    let mut escaped: bool = false;
    let mut depth: usize = 0;
    for ch in trimmed.chars() {
        if in_string {
            cur.push(ch);
            if escaped {
                escaped = false;
            } else if ch == '\\' {
                escaped = true;
            } else if ch == '"' {
                in_string = false;
            }
            continue;
        }
        match ch {
            '"' => {
                in_string = true;
                cur.push(ch);
            }
            '(' | '[' | '{' => {
                depth += 1;
                cur.push(ch);
            }
            ')' | ']' | '}' => {
                depth = depth.saturating_sub(1);
                cur.push(ch);
            }
            ',' if depth == 0 => {
                let piece: String = cur.trim().to_string();
                if !piece.is_empty() {
                    parts.push(piece);
                }
                cur.clear();
            }
            _ => cur.push(ch),
        }
    }
    let tail: String = cur.trim().to_string();
    if !tail.is_empty() {
        parts.push(tail);
    }
    parts.join(", ")
}
