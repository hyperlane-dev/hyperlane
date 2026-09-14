use super::*;

/// Parse a version string into Version struct
///
/// # Arguments
///
/// - `&str`: The version string to parse (e.g., "0.1.0" or "0.1.0-alpha")
///
/// # Returns
///
/// - `Option<Version>`: Parsed version if successful, None otherwise
fn parse_version(version_str: &str) -> Option<Version> {
    let parts: Vec<&str> = version_str.split('-').collect();
    let version_part: &str = parts.first()?;
    let prerelease: Option<String> = parts.get(1).map(|s: &&str| s.to_string());
    let nums: Vec<&str> = version_part.split('.').collect();
    if nums.len() != 3 {
        return None;
    }
    let major: u64 = nums.first()?.parse().ok()?;
    let minor: u64 = nums.get(1)?.parse().ok()?;
    let patch: u64 = nums.get(2)?.parse().ok()?;
    Some(Version {
        major,
        minor,
        patch,
        prerelease,
    })
}

/// Parse pre-release identifier to extract type and number
///
/// # Arguments
///
/// - `&str`: The pre-release string (e.g., "alpha", "alpha.1", "beta.2")
///
/// # Returns
///
/// - `Option<(&str, u64)>`: Tuple of (pre_release_type, number) if parsed successfully
fn parse_prerelease(prerelease: &str) -> Option<(&str, u64)> {
    let parts: Vec<&str> = prerelease.split('.').collect();
    let pre_type: &str = parts.first()?;
    let number: u64 = parts
        .get(1)
        .and_then(|s: &&str| s.parse().ok())
        .unwrap_or(0);
    Some((pre_type, number))
}

/// Get the next pre-release version string
///
/// # Arguments
///
/// - `Option<&String>`: Current pre-release identifier
/// - `&str`: Target pre-release type ("alpha", "beta", "rc")
///
/// # Returns
///
/// - `String`: The new pre-release identifier
fn get_next_prerelease(current: Option<&String>, target_type: &str) -> String {
    match current {
        Some(pre) => {
            if let Some((pre_type, number)) = parse_prerelease(pre)
                && pre_type == target_type
                && number > 0
            {
                return format!("{}.{}", target_type, number + 1);
            }
            format!("{target_type}.1")
        }
        None => target_type.to_string(),
    }
}

/// Convert Version back to string representation
///
/// # Arguments
///
/// - `&Version`: The Version struct to convert
///
/// # Returns
///
/// - `String`: Version string (e.g., "0.1.0" or "0.1.0-alpha")
fn version_to_string(version: &Version) -> String {
    let base: String = format!("{}.{}.{}", version.major, version.minor, version.patch);
    match &version.prerelease {
        Some(pre) => format!("{base}-{pre}"),
        None => base,
    }
}

/// Apply version bump according to the specified type
///
/// # Arguments
///
/// - `&Version`: The current version
/// - `&BumpVersionType`: The type of version bump to apply
///
/// # Returns
///
/// - `Version`: The new version after bumping
fn bump_version(version: &Version, bump_type: &BumpVersionType) -> Version {
    match bump_type {
        BumpVersionType::Patch => Version {
            major: version.major,
            minor: version.minor,
            patch: version.patch + 1,
            prerelease: None,
        },
        BumpVersionType::Minor => Version {
            major: version.major,
            minor: version.minor + 1,
            patch: 0,
            prerelease: None,
        },
        BumpVersionType::Major => Version {
            major: version.major + 1,
            minor: 0,
            patch: 0,
            prerelease: None,
        },
        BumpVersionType::Release => Version {
            major: version.major,
            minor: version.minor,
            patch: version.patch,
            prerelease: None,
        },
        BumpVersionType::Alpha => {
            let prerelease: String = get_next_prerelease(version.prerelease.as_ref(), "alpha");
            Version {
                major: version.major,
                minor: version.minor,
                patch: version.patch,
                prerelease: Some(prerelease),
            }
        }
        BumpVersionType::Beta => {
            let prerelease: String = get_next_prerelease(version.prerelease.as_ref(), "beta");
            Version {
                major: version.major,
                minor: version.minor,
                patch: version.patch,
                prerelease: Some(prerelease),
            }
        }
        BumpVersionType::Rc => {
            let prerelease: String = get_next_prerelease(version.prerelease.as_ref(), "rc");
            Version {
                major: version.major,
                minor: version.minor,
                patch: version.patch,
                prerelease: Some(prerelease),
            }
        }
    }
}

/// Find version value position in a line
///
/// # Arguments
///
/// - `&str`: The line to search
///
/// # Returns
///
/// - `Option<(usize, usize)>`: Start and end positions of version string within quotes
fn find_version_position(line: &str) -> Option<(usize, usize)> {
    let trimmed: &str = line.trim();
    if !trimmed.starts_with("version") || !trimmed.contains('=') {
        return None;
    }
    let eq_pos: usize = line.find('=')?;
    let after_eq: &str = &line[eq_pos + 1..];
    let quote_start: usize = after_eq.find('"')?;
    let after_first_quote: &str = &after_eq[quote_start + 1..];
    let quote_end: usize = after_first_quote.find('"')?;
    let version_start: usize = eq_pos + 1 + quote_start + 1;
    let version_end: usize = version_start + quote_end;
    Some((version_start, version_end))
}

/// Read and update version in Cargo.toml
///
/// # Arguments
///
/// - `&str`: Path to Cargo.toml file
/// - `&BumpVersionType`: Type of version bump to apply
///
/// # Returns
///
/// - `Result<String, Box<dyn std::error::Error>>`: The new version string or an error
pub async fn execute_bump(
    manifest_path: &str,
    bump_type: &BumpVersionType,
) -> Result<String, Box<dyn std::error::Error>> {
    let path: &Path = Path::new(manifest_path);
    let content: String = read_to_string(path).await?;
    let mut new_version: Option<String> = None;
    let mut found_version: bool = false;
    let mut updated_content: String = content.clone();
    for line in content.lines() {
        if found_version {
            break;
        }
        if let Some((version_start, version_end)) = find_version_position(line) {
            let version_str: &str = &line[version_start..version_end];
            if let Some(version) = parse_version(version_str) {
                let bumped: Version = bump_version(&version, bump_type);
                let version_string: String = version_to_string(&bumped);
                new_version = Some(version_string.clone());
                let new_line: String = format!(
                    "{}{version_string}{}",
                    &line[..version_start],
                    &line[version_end..]
                );
                updated_content = updated_content.replacen(line, &new_line, 1);
                found_version = true;
            }
        }
    }
    if !found_version {
        return Err("version field not found in Cargo.toml".into());
    }
    write(path, updated_content).await?;
    match new_version {
        Some(v) => Ok(v),
        None => Err("failed to bump version".into()),
    }
}
