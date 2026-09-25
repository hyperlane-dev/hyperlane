use super::*;

/// Execute cargo run command and capture output through log.
///
/// # Returns
///
/// - `Result<(), io::Error>`: Success or error
async fn run_cargo_run() -> Result<(), io::Error> {
    let output: std::process::Output = Command::new("cargo")
        .arg("run")
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
            log::error!("cargo run failed");
        }
    }
    Ok(())
}

/// Execute watch command using notify crate to monitor file changes
/// and re-run cargo run when source files are modified.
///
/// # Returns
///
/// - `Result<(), io::Error>`: Success or error
pub async fn execute_watch() -> Result<(), io::Error> {
    let src_path: PathBuf = PathBuf::from("src");
    if !src_path.exists() {
        return Err(io::Error::other(
            "src directory not found in current directory",
        ));
    }
    run_cargo_run().await?;
    let (tx, mut rx): (Sender<Event>, Receiver<Event>) = channel(Event::new(EventKind::Any));
    let mut watcher: RecommendedWatcher =
        recommended_watcher(move |result: Result<Event, notify::Error>| {
            if let Ok(event) = result {
                let _: Result<(), tokio::sync::watch::error::SendError<Event>> = tx.send(event);
            }
        })
        .map_err(|error: notify::Error| io::Error::other(error.to_string()))?;
    watcher
        .watch(&src_path, RecursiveMode::Recursive)
        .map_err(|error: notify::Error| io::Error::other(error.to_string()))?;
    log::info!("Watching src/ for changes...");
    let mut debounce: Interval = interval(Duration::from_millis(500));
    debounce.tick().await;
    while rx.changed().await.is_ok() {
        let event: Event = rx.borrow().clone();
        let has_rust_change: bool = event
            .paths
            .iter()
            .any(|path: &PathBuf| path.extension().is_some_and(|ext: &OsStr| ext == "rs"));
        if !has_rust_change {
            continue;
        }
        log::warn!("File change detected: {}", event.paths[0].display());
        debounce.reset();
        sleep(Duration::from_millis(300)).await;
        run_cargo_run().await?;
    }
    Ok(())
}
