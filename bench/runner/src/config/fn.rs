use super::*;

/// Reads one string environment variable or returns its default.
///
/// # Arguments
///
/// - `&str` - The environment variable name.
/// - `&str` - The default value.
///
/// # Returns
///
/// - `String` - The configured value.
pub(crate) fn env_string(name: &str, default: &str) -> String {
    match env::var(name) {
        Ok(value) if !value.is_empty() => value,
        _ => default.to_owned(),
    }
}

/// Reads one path environment variable or returns its default.
///
/// # Arguments
///
/// - `&str` - The environment variable name.
/// - `&Path` - The default path.
///
/// # Returns
///
/// - `PathBuf` - The configured path.
pub(crate) fn env_path(name: &str, default: &Path) -> PathBuf {
    match env::var_os(name) {
        Some(value) if !value.is_empty() => PathBuf::from(value),
        _ => default.to_path_buf(),
    }
}

/// Parses a whitespace-separated list of concurrency levels.
///
/// # Arguments
///
/// - `&str` - The environment variable name.
/// - `&str` - The configured levels.
///
/// # Returns
///
/// - `Result<Vec<u32>, String>` - The levels, or the invalid level.
pub(crate) fn parse_levels(name: &str, value: &str) -> Result<Vec<u32>, String> {
    let mut levels: Vec<u32> = Vec::new();
    for level in value.split_whitespace() {
        match level.parse::<u32>() {
            Ok(parsed) => levels.push(parsed),
            Err(error) => return Err(format!("invalid {name} value {level}: {error}")),
        }
    }
    Ok(levels)
}

/// Builds the runner configuration from the process environment.
///
/// # Returns
///
/// - `Result<Config, String>` - The configuration, or an invalid numeric setting.
pub(crate) fn config_from_env() -> Result<Config, String> {
    let manifest_dir: PathBuf = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let root: PathBuf = match manifest_dir.parent() {
        Some(parent) => parent.to_path_buf(),
        None => manifest_dir.clone(),
    };
    let certs: PathBuf = env_path(ENV_CERTS, &root.join(DEFAULT_CERTS_DIR));
    let connections_value: String = env_string(ENV_CONNECTIONS, DEFAULT_CONNECTIONS);
    let short_value: String = env_string(ENV_CONNECTIONS_SHORT, DEFAULT_CONNECTIONS_SHORT);
    let repeats_value: String = env_string(ENV_REPEATS, DEFAULT_REPEATS);
    let repeats: u32 = match repeats_value.parse::<u32>() {
        Ok(value) => value,
        Err(error) => {
            return Err(format!(
                "invalid {ENV_REPEATS} value {repeats_value}: {error}"
            ));
        }
    };
    let cooldown_value: String = env_string(ENV_COOLDOWN_SHORT, DEFAULT_COOLDOWN_SHORT);
    let cooldown_short: u64 = match cooldown_value.parse::<u64>() {
        Ok(value) => value,
        Err(error) => {
            return Err(format!(
                "invalid {ENV_COOLDOWN_SHORT} value {cooldown_value}: {error}"
            ));
        }
    };
    Ok(Config {
        results: env_path(ENV_RESULTS, &root.join(DEFAULT_RESULTS_DIR)),
        network: env_string(ENV_NETWORK, DEFAULT_NETWORK),
        image_prefix: env_string(ENV_IMAGE_PREFIX, DEFAULT_IMAGE_PREFIX),
        server_cpus: env_string(ENV_SERVER_CPUS, DEFAULT_SERVER_CPUS),
        loadgen_cpus: env_string(ENV_LOADGEN_CPUS, DEFAULT_LOADGEN_CPUS),
        memory: env_string(ENV_MEMORY, DEFAULT_MEMORY),
        connections: parse_levels(ENV_CONNECTIONS, &connections_value)?,
        connections_short: parse_levels(ENV_CONNECTIONS_SHORT, &short_value)?,
        duration: env_string(ENV_DURATION, DEFAULT_DURATION),
        warmup: env_string(ENV_WARMUP, DEFAULT_WARMUP),
        repeats,
        protocols: env_string(ENV_PROTOCOLS, DEFAULT_PROTOCOLS)
            .split_whitespace()
            .map(str::to_owned)
            .collect(),
        modes: env_string(ENV_MODES, DEFAULT_MODES)
            .split_whitespace()
            .map(str::to_owned)
            .collect(),
        cooldown_short,
        rate_short: env_string(ENV_RATE_SHORT, DEFAULT_RATE_SHORT),
        ca_cert: env_path(ENV_CA_CERT, &certs.join(CA_FILE_NAME)),
        tls_certificate: env_path(ENV_TLS_CERT, &certs.join(LEAF_CERT_FILE_NAME)),
        tls_key: env_path(ENV_TLS_KEY, &certs.join(LEAF_KEY_FILE_NAME)),
        root,
        certs,
    })
}
