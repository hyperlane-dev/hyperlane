use super::*;

/// Prints a section heading.
///
/// # Arguments
///
/// - `&str` - The heading text.
fn log(message: &str) {
    println!("\n\x1b[1m==> {message}\x1b[0m");
}

/// Captures one child process without invoking a shell.
///
/// # Arguments
///
/// - `&str` - The executable.
/// - `&[String]` - The arguments.
///
/// # Returns
///
/// - `Result<Output, String>` - The captured output, or a spawn error.
fn command_output(program: &str, args: &[String]) -> Result<Output, String> {
    match Command::new(program).args(args).output() {
        Ok(output) => Ok(output),
        Err(error) => Err(format!("could not run {program}: {error}")),
    }
}

/// Runs a command with standard output suppressed and standard error inherited.
///
/// # Arguments
///
/// - `&str` - The executable.
/// - `&[String]` - The arguments.
///
/// # Returns
///
/// - `Result<(), String>` - Success, or a spawn or exit-status error.
fn run_command(program: &str, args: &[String]) -> Result<(), String> {
    let status: ExitStatus = match Command::new(program)
        .args(args)
        .stdout(Stdio::null())
        .status()
    {
        Ok(status) => status,
        Err(error) => return Err(format!("could not run {program}: {error}")),
    };
    if !status.success() {
        return Err(format!("{program} failed with status {status}"));
    }
    Ok(())
}

/// Runs a command with both output streams suppressed.
///
/// # Arguments
///
/// - `&str` - The executable.
/// - `&[String]` - The arguments.
///
/// # Returns
///
/// - `Result<(), String>` - Success, or a spawn or exit-status error.
fn run_silent(program: &str, args: &[String]) -> Result<(), String> {
    let status: ExitStatus = match Command::new(program)
        .args(args)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
    {
        Ok(status) => status,
        Err(error) => return Err(format!("could not run {program}: {error}")),
    };
    if !status.success() {
        return Err(format!("{program} failed with status {status}"));
    }
    Ok(())
}

/// Removes a file or directory tree.
///
/// # Arguments
///
/// - `&Path` - The path to remove.
///
/// # Returns
///
/// - `Result<(), String>` - Success, or the removal error.
fn remove_path(path: &Path) -> Result<(), String> {
    if !path.exists() {
        return Ok(());
    }
    let file_type: fs::FileType = match fs::symlink_metadata(path) {
        Ok(metadata) => metadata.file_type(),
        Err(error) => return Err(format!("could not inspect {}: {error}", path.display())),
    };
    let result: Result<(), io::Error> = if file_type.is_dir() {
        fs::remove_dir_all(path)
    } else {
        fs::remove_file(path)
    };
    match result {
        Ok(()) => Ok(()),
        Err(error) => Err(format!("could not remove {}: {error}", path.display())),
    }
}

/// Recursively copies the measuring instrument into a build context.
///
/// # Arguments
///
/// - `&Path` - The source directory.
/// - `&Path` - The destination directory.
///
/// # Returns
///
/// - `Result<(), String>` - Success, or the copy error.
fn copy_tree(source: &Path, target: &Path) -> Result<(), String> {
    match fs::create_dir_all(target) {
        Ok(()) => {}
        Err(error) => return Err(format!("could not create {}: {error}", target.display())),
    }
    let entries: fs::ReadDir = match fs::read_dir(source) {
        Ok(entries) => entries,
        Err(error) => return Err(format!("could not read {}: {error}", source.display())),
    };
    for entry in entries {
        let entry: fs::DirEntry = match entry {
            Ok(entry) => entry,
            Err(error) => return Err(format!("could not read {}: {error}", source.display())),
        };
        let source_path: PathBuf = entry.path();
        let target_path: PathBuf = target.join(entry.file_name());
        let file_type: fs::FileType = match entry.file_type() {
            Ok(file_type) => file_type,
            Err(error) => {
                return Err(format!(
                    "could not inspect {}: {error}",
                    source_path.display()
                ));
            }
        };
        if file_type.is_dir() {
            copy_tree(&source_path, &target_path)?;
        } else if file_type.is_file() {
            match fs::copy(&source_path, &target_path) {
                Ok(_) => {}
                Err(error) => {
                    return Err(format!(
                        "could not copy {} to {}: {error}",
                        source_path.display(),
                        target_path.display()
                    ));
                }
            }
        } else {
            return Err(format!(
                "unsupported file type at {}",
                source_path.display()
            ));
        }
    }
    Ok(())
}

/// Copies the current harness and refreshes only its workspace lock members.
///
/// # Arguments
///
/// - `&Config` - The runner configuration.
/// - `&Path` - The build context.
///
/// # Returns
///
/// - `Result<(), String>` - Success, or the synchronization error.
fn sync_harness(config: &Config, context: &Path) -> Result<(), String> {
    if !config.root.join(DOCKERFILE_PATH).is_file() {
        return Err(format!(
            "{} is not a performance source tree; set PERF_ROOT to the checkout's performance directory",
            config.root.display()
        ));
    }
    let context_real: PathBuf = match fs::canonicalize(context) {
        Ok(path) => path,
        Err(error) => return Err(format!("could not resolve {}: {error}", context.display())),
    };
    let root_real: PathBuf = match fs::canonicalize(&config.root) {
        Ok(path) => path,
        Err(error) => {
            return Err(format!(
                "could not resolve {}: {error}",
                config.root.display()
            ));
        }
    };
    if context_real == root_real {
        return Ok(());
    }
    let target: PathBuf = context_real.join(HARNESS_DIR);
    if target == root_real {
        return Ok(());
    }
    remove_path(&target)?;
    copy_tree(&root_real, &target)?;
    prepare_context_manifest(&root_real, &context_real)?;
    let manifest: PathBuf = context_real.join(LIBRARY_MANIFEST);
    let args: Vec<String> = vec![
        CARGO_UPDATE.to_owned(),
        WORKSPACE_FLAG.to_owned(),
        OFFLINE_FLAG.to_owned(),
        MANIFEST_PATH_FLAG.to_owned(),
        manifest.display().to_string(),
    ];
    let output: Output = command_output(CARGO_PROGRAM, &args)?;
    if !output.status.success() {
        return Err(format!(
            "the lock of {} does not match its source: {}",
            context.display(),
            String::from_utf8_lossy(&output.stderr).trim()
        ));
    }
    Ok(())
}

/// Generates certificate material when any required file is missing.
///
/// # Arguments
///
/// - `&Config` - The runner configuration.
///
/// # Returns
///
/// - `Result<(), String>` - Success, or the generation error.
fn ensure_certificates(config: &Config) -> Result<(), String> {
    if config.ca_cert.is_file() && config.tls_certificate.is_file() && config.tls_key.is_file() {
        return Ok(());
    }
    log("generating the benchmark certificate material");
    generate_certificates(&config.certs)
}

/// Builds one Docker image from a harness target.
///
/// # Arguments
///
/// - `&Config` - The runner configuration.
/// - `&Path` - The build context.
/// - `&str` - The Dockerfile target.
/// - `&str` - The image label.
/// - `&str` - The image tag.
///
/// # Returns
///
/// - `Result<(), String>` - Success, or the build error.
fn build_image(
    config: &Config,
    context: &Path,
    target: &str,
    label: &str,
    tag: &str,
) -> Result<(), String> {
    let dockerfile: PathBuf = context.join(HARNESS_DIR).join(DOCKERFILE_PATH);
    if !dockerfile.is_file() {
        return Err(format!("the context has no {DOCKERFILE_PATH}"));
    }
    let image: String = format!("{}-{label}:{tag}", config.image_prefix);
    let args: Vec<String> = vec![
        DOCKER_BUILD.to_owned(),
        "-q".to_owned(),
        "-f".to_owned(),
        dockerfile.display().to_string(),
        TARGET_FLAG.to_owned(),
        target.to_owned(),
        "-t".to_owned(),
        image,
        context.display().to_string(),
    ];
    run_command(DOCKER_PROGRAM, &args)
}

/// Builds the server image of one variant.
///
/// # Arguments
///
/// - `&Config` - The runner configuration.
/// - `&str` - The variant name.
/// - `&Path` - The build context.
///
/// # Returns
///
/// - `Result<(), String>` - Success, or the build error.
fn build_variant(config: &Config, variant: &str, context: &Path) -> Result<(), String> {
    if !context.join(LIBRARY_MANIFEST).is_file() {
        return Err(format!(
            "the context {} has no library tree",
            context.display()
        ));
    }
    sync_harness(config, context)?;
    ensure_certificates(config)?;
    log(&format!("building the {variant} server image"));
    build_image(config, context, SERVER_RUNTIME, SERVER_IMAGE, variant)
}

/// Builds the load generator and control images.
///
/// # Arguments
///
/// - `&Config` - The runner configuration.
/// - `&Path` - The build context.
///
/// # Returns
///
/// - `Result<(), String>` - Success, or the build error.
fn build_instrument(config: &Config, context: &Path) -> Result<(), String> {
    sync_harness(config, context)?;
    ensure_certificates(config)?;
    log("building the load generator image");
    build_image(
        config,
        context,
        LOADGEN_RUNTIME,
        LOADGEN_IMAGE,
        INSTRUMENT_TAG,
    )?;
    log("building the control server image");
    build_image(
        config,
        context,
        CONTROL_RUNTIME,
        CONTROL_IMAGE,
        INSTRUMENT_TAG,
    )
}

/// Creates the benchmark network when it does not exist.
///
/// # Arguments
///
/// - `&Config` - The runner configuration.
///
/// # Returns
///
/// - `Result<(), String>` - Success, or the Docker error.
fn ensure_network(config: &Config) -> Result<(), String> {
    let inspect_args: Vec<String> = vec![
        DOCKER_NETWORK.to_owned(),
        DOCKER_INSPECT.to_owned(),
        config.network.clone(),
    ];
    let output: Output = command_output(DOCKER_PROGRAM, &inspect_args)?;
    if output.status.success() {
        return Ok(());
    }
    let create_args: Vec<String> = vec![
        DOCKER_NETWORK.to_owned(),
        DOCKER_CREATE.to_owned(),
        config.network.clone(),
    ];
    run_silent(DOCKER_PROGRAM, &create_args)
}

/// Maps a client protocol name to its server-side name.
///
/// # Arguments
///
/// - `&str` - The client protocol name.
///
/// # Returns
///
/// - `&str` - The server protocol name.
fn server_protocol_of(protocol: &str) -> &str {
    match protocol {
        H1_PROTOCOL => SERVER_H1_PROTOCOL,
        H1_TLS_PROTOCOL => SERVER_H1_TLS_PROTOCOL,
        H2C_PROTOCOL => SERVER_H2_PROTOCOL,
        H2_TLS_PROTOCOL => SERVER_H2_TLS_PROTOCOL,
        H3_PROTOCOL => SERVER_H3_PROTOCOL,
        other => other,
    }
}

/// Reports whether a protocol variant terminates TLS.
///
/// # Arguments
///
/// - `&str` - The protocol variant.
///
/// # Returns
///
/// - `bool` - Whether the variant needs certificate material.
fn variant_uses_tls(protocol: &str) -> bool {
    protocol == H1_TLS_PROTOCOL || protocol == H2_TLS_PROTOCOL || protocol == H3_PROTOCOL
}

/// Reads one Docker image ID.
///
/// # Arguments
///
/// - `&str` - The image name.
///
/// # Returns
///
/// - `Result<String, String>` - The image ID, or an inspection error.
fn image_id(image: &str) -> Result<String, String> {
    let args: Vec<String> = vec![
        DOCKER_IMAGE.to_owned(),
        DOCKER_INSPECT.to_owned(),
        FORMAT_FLAG.to_owned(),
        IMAGE_ID_FORMAT.to_owned(),
        image.to_owned(),
    ];
    let output: Output = command_output(DOCKER_PROGRAM, &args)?;
    if !output.status.success() {
        return Err(format!("an arm image is missing: {image}"));
    }
    Ok(String::from_utf8_lossy(&output.stdout).trim().to_owned())
}

/// Proves that the two arms are different Docker images.
///
/// # Arguments
///
/// - `&Config` - The runner configuration.
/// - `&str` - The first variant.
/// - `&str` - The second variant.
///
/// # Returns
///
/// - `Result<(), String>` - Success, or a missing or duplicate image error.
fn verify_arms_differ(config: &Config, first: &str, second: &str) -> Result<(), String> {
    let first_image: String = format!("{}-{SERVER_IMAGE}:{first}", config.image_prefix);
    let second_image: String = format!("{}-{SERVER_IMAGE}:{second}", config.image_prefix);
    let first_id: String = image_id(&first_image)?;
    let second_id: String = image_id(&second_image)?;
    if first_id.is_empty() || second_id.is_empty() {
        return Err("an arm image is missing".to_owned());
    }
    if first_id == second_id {
        return Err(format!("the two arms are the same image ({first_id})"));
    }
    println!("  arm images differ:");
    println!("    {first}: {first_id}");
    println!("    {second}: {second_id}");
    Ok(())
}

/// Removes containers on a best-effort basis.
///
/// # Arguments
///
/// - `&[String]` - The container names.
fn remove_containers(containers: &[String]) {
    if containers.is_empty() {
        return;
    }
    let mut args: Vec<String> = vec![DOCKER_REMOVE.to_owned(), "-f".to_owned()];
    args.extend(containers.iter().cloned());
    let _: Result<Output, String> = command_output(DOCKER_PROGRAM, &args);
}

/// Removes benchmark containers when a runner invocation ends.
fn cleanup() {
    let args: Vec<String> = vec![
        DOCKER_PS.to_owned(),
        PS_ALL_QUIET_FLAG.to_owned(),
        FILTER_FLAG.to_owned(),
        PERF_NAME_FILTER.to_owned(),
    ];
    if let Ok(output) = command_output(DOCKER_PROGRAM, &args)
        && output.status.success()
    {
        let containers: Vec<String> = String::from_utf8_lossy(&output.stdout)
            .split_whitespace()
            .map(str::to_owned)
            .collect();
        remove_containers(&containers);
    }
}

/// Starts one server container and waits for its readiness announcement.
///
/// # Arguments
///
/// - `&Config` - The runner configuration.
/// - `&str` - The container name.
/// - `&str` - The image name.
/// - `&str` - The client protocol variant.
///
/// # Returns
///
/// - `Result<(), String>` - Success, or a start or readiness error.
fn start_server(
    config: &Config,
    container: &str,
    image: &str,
    protocol: &str,
) -> Result<(), String> {
    remove_containers(&[container.to_owned()]);
    let mut environment: Vec<String> = vec![
        SERVER_ADDRESS_ENV.to_owned(),
        format!("{SERVER_PROTOCOL_ENV}{}", server_protocol_of(protocol)),
    ];
    if variant_uses_tls(protocol) {
        environment.push(SERVER_CERT_ENV.to_owned());
        environment.push(SERVER_KEY_ENV.to_owned());
    }
    let mut args: Vec<String> = vec![
        DOCKER_RUN.to_owned(),
        "-d".to_owned(),
        NAME_FLAG.to_owned(),
        container.to_owned(),
        NETWORK_FLAG.to_owned(),
        config.network.clone(),
        CPUS_FLAG.to_owned(),
        config.server_cpus.clone(),
        MEMORY_FLAG.to_owned(),
        config.memory.clone(),
        "-v".to_owned(),
        format!("{}:/certs:ro", config.certs.display()),
    ];
    for variable in environment {
        args.push("-e".to_owned());
        args.push(variable);
    }
    args.push(image.to_owned());
    run_command(DOCKER_PROGRAM, &args)?;
    let mut ready: bool = false;
    let mut attempt: u32 = 0;
    while attempt < READY_ATTEMPTS {
        let log_args: Vec<String> = vec![DOCKER_LOGS.to_owned(), container.to_owned()];
        if let Ok(output) = command_output(DOCKER_PROGRAM, &log_args) {
            let logs: String = format!(
                "{}{}",
                String::from_utf8_lossy(&output.stdout),
                String::from_utf8_lossy(&output.stderr)
            );
            if logs.contains(READY_MARKER) {
                ready = true;
                break;
            }
        }
        let inspect_args: Vec<String> = vec![DOCKER_INSPECT.to_owned(), container.to_owned()];
        match command_output(DOCKER_PROGRAM, &inspect_args) {
            Ok(output) if output.status.success() => {}
            _ => break,
        }
        attempt += 1;
        sleep(Duration::from_millis(READY_DELAY_MS));
    }
    if ready {
        return Ok(());
    }
    let log_args: Vec<String> = vec![DOCKER_LOGS.to_owned(), container.to_owned()];
    let logs: String = match command_output(DOCKER_PROGRAM, &log_args) {
        Ok(output) => format!(
            "{}{}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        ),
        Err(_) => String::new(),
    };
    Err(format!(
        "the server {container} did not become ready\n{logs}"
    ))
}

/// Constructs the Docker command for one load generator.
///
/// # Arguments
///
/// - `&Config` - The runner configuration.
/// - `&str` - The load generator container name.
/// - `&str` - The server container address.
/// - `u32` - The concurrency level.
/// - `&str` - The protocol variant.
/// - `&str` - The connection mode.
///
/// # Returns
///
/// - `Command` - The configured child process command.
fn drive_command(
    config: &Config,
    container: &str,
    address: &str,
    connections: u32,
    variant: &str,
    mode: &str,
) -> Command {
    let image: String = format!("{}-{LOADGEN_IMAGE}:{INSTRUMENT_TAG}", config.image_prefix);
    let mut args: Vec<String> = vec![
        DOCKER_RUN.to_owned(),
        REMOVE_AFTER_RUN_FLAG.to_owned(),
        NAME_FLAG.to_owned(),
        container.to_owned(),
        NETWORK_FLAG.to_owned(),
        config.network.clone(),
        CPUS_FLAG.to_owned(),
        config.loadgen_cpus.clone(),
        MEMORY_FLAG.to_owned(),
        config.memory.clone(),
        "-v".to_owned(),
        format!("{}:/certs:ro", config.certs.display()),
        image,
        ADDR_FLAG.to_owned(),
        format!("{address}:{CONTAINER_PORT}"),
        CONNECTIONS_FLAG.to_owned(),
        connections.to_string(),
        DURATION_FLAG.to_owned(),
        config.duration.clone(),
        WARMUP_FLAG.to_owned(),
        config.warmup.clone(),
        PROTOCOL_FLAG.to_owned(),
        variant.to_owned(),
        MODE_FLAG.to_owned(),
        mode.to_owned(),
    ];
    if variant_uses_tls(variant) {
        args.push(CA_FLAG.to_owned());
        args.push(CONTAINER_CA_PATH.to_owned());
    }
    if mode == SHORT_MODE {
        args.push(RATE_FLAG.to_owned());
        args.push(config.rate_short.clone());
    }
    let mut command: Command = Command::new(DOCKER_PROGRAM);
    command.args(args);
    command
}

/// Spawns one load generator and redirects its output to a file.
///
/// # Arguments
///
/// - `&Config` - The runner configuration.
/// - `&str` - The load generator container name.
/// - `&str` - The server container address.
/// - `u32` - The concurrency level.
/// - `&str` - The protocol variant.
/// - `&str` - The connection mode.
/// - `&Path` - The output file.
///
/// # Returns
///
/// - `Result<Child, String>` - The child process, or a spawn error.
fn spawn_drive(
    config: &Config,
    container: &str,
    address: &str,
    connections: u32,
    variant: &str,
    mode: &str,
    output_path: &Path,
) -> Result<Child, String> {
    let output_file: File = match File::create(output_path) {
        Ok(file) => file,
        Err(error) => {
            return Err(format!(
                "could not create {}: {error}",
                output_path.display()
            ));
        }
    };
    let mut command: Command =
        drive_command(config, container, address, connections, variant, mode);
    match command
        .stdout(Stdio::from(output_file))
        .stderr(Stdio::null())
        .spawn()
    {
        Ok(child) => Ok(child),
        Err(error) => Err(format!(
            "could not start load generator {container}: {error}"
        )),
    }
}

/// Runs one load generator and captures its standard output.
///
/// # Arguments
///
/// - `&Config` - The runner configuration.
/// - `&str` - The load generator container name.
/// - `&str` - The server container address.
/// - `u32` - The concurrency level.
/// - `&str` - The protocol variant.
/// - `&str` - The connection mode.
///
/// # Returns
///
/// - `Result<String, String>` - The generator output, or a process error.
fn drive_output(
    config: &Config,
    container: &str,
    address: &str,
    connections: u32,
    variant: &str,
    mode: &str,
) -> Result<String, String> {
    let mut command: Command =
        drive_command(config, container, address, connections, variant, mode);
    let output: Output = match command.stderr(Stdio::null()).output() {
        Ok(output) => output,
        Err(error) => {
            return Err(format!(
                "could not start load generator {container}: {error}"
            ));
        }
    };
    if !output.status.success() {
        return Err(format!("load generator {container} failed"));
    }
    Ok(String::from_utf8_lossy(&output.stdout).trim().to_owned())
}

/// Reads one nonempty load generator output file.
///
/// # Arguments
///
/// - `&Path` - The output file.
/// - `&str` - The variant name.
///
/// # Returns
///
/// - `Result<String, String>` - The trimmed output, or a read or empty-output error.
fn read_generator_output(path: &Path, variant: &str) -> Result<String, String> {
    let output: String = match fs::read_to_string(path) {
        Ok(output) => output,
        Err(error) => return Err(format!("could not read {}: {error}", path.display())),
    };
    let trimmed: &str = output.trim();
    if trimmed.is_empty() {
        return Err(format!(
            "the load generator produced no output for {variant}"
        ));
    }
    Ok(trimmed.to_owned())
}

/// Serializes one string for inclusion in a raw measurement row.
///
/// # Arguments
///
/// - `&str` - The string to serialize.
///
/// # Returns
///
/// - `Result<String, String>` - The JSON string, or a serialization error.
fn json_string(value: &str) -> Result<String, String> {
    match to_string(value) {
        Ok(json) => Ok(json),
        Err(error) => Err(format!("could not serialize measurement metadata: {error}")),
    }
}

/// Appends one raw measurement row without reformatting the generator result.
///
/// # Arguments
///
/// - `&Path` - The raw JSON Lines file.
/// - `&str` - The variant name.
/// - `&str` - The protocol variant.
/// - `&str` - The connection mode.
/// - `u32` - The concurrency level.
/// - `u32` - The repeat number.
/// - `&str` - The generator JSON output.
///
/// # Returns
///
/// - `Result<(), String>` - Success, or a file or serialization error.
fn append_measurement(
    path: &Path,
    variant: &str,
    protocol: &str,
    mode: &str,
    connections: u32,
    repeat: u32,
    output: &str,
) -> Result<(), String> {
    let variant_json: String = json_string(variant)?;
    let protocol_json: String = json_string(protocol)?;
    let mode_json: String = json_string(mode)?;
    let line: String = format!(
        "{{\"variant\":{variant_json},\"protocol\":{protocol_json},\"mode\":{mode_json},\"connections\":{connections},\"repeat\":{repeat},\"result\":{result}}}\n",
        result = output.trim()
    );
    let mut file: File = match OpenOptions::new().create(true).append(true).open(path) {
        Ok(file) => file,
        Err(error) => return Err(format!("could not open {}: {error}", path.display())),
    };
    match file.write_all(line.as_bytes()) {
        Ok(()) => Ok(()),
        Err(error) => Err(format!("could not append {}: {error}", path.display())),
    }
}

/// Runs both arms simultaneously and records their paired measurements.
///
/// # Arguments
///
/// - `&Config` - The runner configuration.
/// - `&str` - The first variant.
/// - `&str` - The second variant.
/// - `u32` - The concurrency level.
/// - `u32` - The repeat number.
/// - `&str` - The protocol variant.
/// - `&str` - The connection mode.
///
/// # Returns
///
/// - `Result<(), String>` - Success, or the measurement error.
fn run_pair(
    config: &Config,
    first: &str,
    second: &str,
    connections: u32,
    repeat: u32,
    variant: &str,
    mode: &str,
) -> Result<(), String> {
    let mut first: &str = first;
    let mut second: &str = second;
    if repeat.is_multiple_of(2) {
        swap(&mut first, &mut second);
    }
    let first_container: String =
        format!("{SERVER_CONTAINER_PREFIX}{first}-{variant}-{mode}-{connections}");
    let second_container: String =
        format!("{SERVER_CONTAINER_PREFIX}{second}-{variant}-{mode}-{connections}");
    let first_image: String = format!("{}-{SERVER_IMAGE}:{first}", config.image_prefix);
    let second_image: String = format!("{}-{SERVER_IMAGE}:{second}", config.image_prefix);
    start_server(config, &first_container, &first_image, variant)?;
    start_server(config, &second_container, &second_image, variant)?;
    let first_file: PathBuf = config.results.join(format!(
        "{FIRST_OUTPUT_PREFIX}{variant}-{mode}-{connections}-{repeat}.json"
    ));
    let second_file: PathBuf = config.results.join(format!(
        "{SECOND_OUTPUT_PREFIX}{variant}-{mode}-{connections}-{repeat}.json"
    ));
    let first_loadgen: String =
        format!("{LOADGEN_CONTAINER_PREFIX}{first}-{variant}-{mode}-{connections}-{repeat}");
    let second_loadgen: String =
        format!("{LOADGEN_CONTAINER_PREFIX}{second}-{variant}-{mode}-{connections}-{repeat}");
    let mut first_child: Child = spawn_drive(
        config,
        &first_loadgen,
        &first_container,
        connections,
        variant,
        mode,
        &first_file,
    )?;
    let mut second_child: Child = spawn_drive(
        config,
        &second_loadgen,
        &second_container,
        connections,
        variant,
        mode,
        &second_file,
    )?;
    let _first_status: ExitStatus = match first_child.wait() {
        Ok(status) => status,
        Err(error) => return Err(format!("could not wait for {first_loadgen}: {error}")),
    };
    let _second_status: ExitStatus = match second_child.wait() {
        Ok(status) => status,
        Err(error) => return Err(format!("could not wait for {second_loadgen}: {error}")),
    };
    let first_output: String = read_generator_output(&first_file, first)?;
    let second_output: String = read_generator_output(&second_file, second)?;
    let raw_path: PathBuf = config.results.join(RAW_FILE_NAME);
    append_measurement(
        &raw_path,
        first,
        variant,
        mode,
        connections,
        repeat,
        &first_output,
    )?;
    append_measurement(
        &raw_path,
        second,
        variant,
        mode,
        connections,
        repeat,
        &second_output,
    )?;
    remove_containers(&[first_container, second_container]);
    Ok(())
}

/// Measures the control server after the two arms have stopped.
///
/// # Arguments
///
/// - `&Config` - The runner configuration.
/// - `u32` - The concurrency level.
/// - `u32` - The repeat number.
/// - `&str` - The protocol variant.
/// - `&str` - The connection mode.
///
/// # Returns
///
/// - `Result<(), String>` - Success, or the control measurement error.
fn run_control(
    config: &Config,
    connections: u32,
    repeat: u32,
    variant: &str,
    mode: &str,
) -> Result<(), String> {
    let container: String =
        format!("{SERVER_CONTAINER_PREFIX}{CONTROL_VARIANT}-{variant}-{mode}-{connections}");
    let image: String = format!("{}-{CONTROL_IMAGE}:{INSTRUMENT_TAG}", config.image_prefix);
    start_server(config, &container, &image, variant)?;
    let loadgen: String = format!(
        "{LOADGEN_CONTAINER_PREFIX}{CONTROL_VARIANT}-{variant}-{mode}-{connections}-{repeat}"
    );
    let output: String = drive_output(config, &loadgen, &container, connections, variant, mode)?;
    if output.is_empty() {
        return Err("the control produced no output".to_owned());
    }
    let raw_path: PathBuf = config.results.join(RAW_FILE_NAME);
    append_measurement(
        &raw_path,
        CONTROL_VARIANT,
        variant,
        mode,
        connections,
        repeat,
        &output,
    )?;
    remove_containers(&[container]);
    Ok(())
}

/// Runs the complete paired workload matrix.
///
/// # Arguments
///
/// - `&Config` - The runner configuration.
/// - `&str` - The first variant.
/// - `&str` - The second variant.
///
/// # Returns
///
/// - `Result<(), String>` - Success, or the first broken measurement.
fn run_bench(config: &Config, first: &str, second: &str) -> Result<(), String> {
    match fs::create_dir_all(&config.results) {
        Ok(()) => {}
        Err(error) => {
            return Err(format!(
                "could not create {}: {error}",
                config.results.display()
            ));
        }
    }
    let raw_path: PathBuf = config.results.join(RAW_FILE_NAME);
    match File::create(&raw_path) {
        Ok(_) => {}
        Err(error) => {
            return Err(format!(
                "could not truncate {}: {error}",
                raw_path.display()
            ));
        }
    }
    log("proving the two arms are different images");
    verify_arms_differ(config, first, second)?;
    for variant in &config.protocols {
        for mode in &config.modes {
            let levels: &Vec<u32> = if mode == SHORT_MODE {
                &config.connections_short
            } else {
                &config.connections
            };
            for connections in levels {
                for repeat in 1..=config.repeats {
                    log(&format!(
                        "protocol={variant} mode={mode} conn={connections} repeat={repeat}/{}",
                        config.repeats
                    ));
                    run_pair(config, first, second, *connections, repeat, variant, mode)?;
                    run_control(config, *connections, repeat, variant, mode)?;
                    if mode == SHORT_MODE {
                        sleep(Duration::from_secs(config.cooldown_short));
                    }
                }
            }
        }
    }
    Ok(())
}

/// Dispatches one parsed command line.
///
/// # Arguments
///
/// - `&Config` - The runner configuration.
/// - `&[String]` - The arguments without the program name.
///
/// # Returns
///
/// - `Result<(), String>` - Success, or a usage or operation error.
fn dispatch(config: &Config, args: &[String]) -> Result<(), String> {
    let Some(command) = args.first() else {
        return Err(USAGE.to_owned());
    };
    match command.as_str() {
        INSTRUMENT_COMMAND if args.len() == 2 => {
            ensure_network(config)?;
            build_instrument(config, Path::new(&args[1]))
        }
        BUILD_COMMAND if args.len() == 3 => {
            ensure_network(config)?;
            build_variant(config, &args[1], Path::new(&args[2]))
        }
        BENCH_COMMAND if args.len() == 3 => run_bench(config, &args[1], &args[2]),
        REPORT_COMMAND if args.len() <= 2 => {
            let default_path: PathBuf = config.results.join(RAW_FILE_NAME);
            let path: &Path = match args.get(1) {
                Some(path) => Path::new(path),
                None => &default_path,
            };
            let report: String = render_report(path)?;
            print!("{report}");
            Ok(())
        }
        CERTIFICATES_COMMAND if args.len() == 2 => {
            generate_certificates(Path::new(&args[1]))?;
            println!("the benchmark certificate material is ready in {}", args[1]);
            Ok(())
        }
        _ => Err(USAGE.to_owned()),
    }
}

/// Runs the benchmark orchestrator, reporter or certificate generator.
///
/// # Arguments
///
/// - `&[String]` - The arguments without the program name.
///
/// # Returns
///
/// - `Result<(), String>` - Success, or a usage, configuration or operation error.
pub fn run(args: &[String]) -> Result<(), String> {
    let config: Config = config_from_env()?;
    let result: Result<(), String> = dispatch(&config, args);
    cleanup();
    result
}
