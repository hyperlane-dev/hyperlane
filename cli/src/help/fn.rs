/// Print help message
pub fn print_help() {
    log::info!("hyperlane-cli [COMMAND] [OPTIONS]");
    log::info!("");
    log::info!("Commands:");
    log::info!("  watch     Watch files and run cargo run using cargo-watch");
    log::info!("  new       Create a new project from template");
    log::info!(
        "  template  Generate template components (controller|domain|exception|mapper|model|repository|service|utils|view)"
    );
    log::info!("  -h, --help      Print this help message");
    log::info!("  -v, --version   Print version information");
    log::info!("");
    log::info!("New Options:");
    log::info!("  <PROJECT_NAME>  Name of the project to create");
}
