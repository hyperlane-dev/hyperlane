use super::*;

/// Spawns `cargo run` as a managed child process that terminates cleanly on restart or drop.
///
/// # Returns
///
/// - `Result<Child, io::Error>` - The spawned child process handle.
fn spawn_cargo_run() -> Result<Child, io::Error> {
    Command::new(CARGO_RUN_PROGRAM)
        .arg("run")
        .kill_on_drop(true)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
}

/// Terminates any active watched child process before restarting.
///
/// # Arguments
///
/// - `&mut Option<Child>` - The active child process handle, if one is running.
async fn stop_cargo_run(child: &mut Option<Child>) {
    if let Some(mut process) = child.take() {
        let _: Result<(), io::Error> = process.kill().await;
        let _: Result<_, io::Error> = process.wait().await;
    }
}

/// Execute watch command using notify crate to monitor file changes
/// and restart `cargo run` when source files are modified.
///
/// # Returns
///
/// - `Result<(), io::Error>` - Success or error
pub async fn execute_watch() -> Result<(), io::Error> {
    let src_path: PathBuf = PathBuf::from("src");
    if !src_path.exists() {
        return Err(io::Error::other(ERROR_SRC_DIRECTORY_NOT_FOUND));
    }
    let mut active_child: Option<Child> = Some(spawn_cargo_run()?);
    let (tx, mut rx): (Sender<Event>, Receiver<Event>) = channel(Event::new(EventKind::Any));
    let mut watcher: RecommendedWatcher =
        recommended_watcher(move |result: Result<Event, Error>| {
            if let Ok(event) = result {
                let _: Result<(), SendError<Event>> = tx.send(event);
            }
        })
        .map_err(|error: Error| io::Error::other(error.to_string()))?;
    watcher
        .watch(&src_path, RecursiveMode::Recursive)
        .map_err(|error: Error| io::Error::other(error.to_string()))?;
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
        stop_cargo_run(&mut active_child).await;
        active_child = Some(spawn_cargo_run()?);
    }
    stop_cargo_run(&mut active_child).await;
    Ok(())
}
