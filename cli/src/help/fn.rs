/// Print help message
pub fn print_help() {
    log::info!("hyperlane-cli [COMMAND] [OPTIONS]");
    log::info!("");
    log::info!("Commands:");
    log::info!("  bump      Bump version in Cargo.toml");
    log::info!("  fmt       Format Rust code using cargo fmt");
    log::info!("  watch     Watch files and run cargo run using cargo-watch");
    log::info!("  publish   Publish packages in monorepo with topological ordering");
    log::info!("  new       Create a new project from template");
    log::info!(
        "  template  Generate template components (controller|domain|exception|mapper|model|repository|service|utils|view)"
    );
    log::info!("  -h, --help      Print this help message");
    log::info!("  -v, --version   Print version information");
    log::info!("");
    log::info!("New Options:");
    log::info!("  <PROJECT_NAME>  Name of the project to create");
    log::info!("");
    log::info!("Bump Options:");
    log::info!("  --patch         Bump patch version (0.1.0 -> 0.1.1) [default]");
    log::info!("  --minor         Bump minor version (0.1.0 -> 0.2.0)");
    log::info!("  --major         Bump major version (0.1.0 -> 1.0.0)");
    log::info!(
        "  --alpha         Add or bump alpha version (0.1.0 -> 0.1.0-alpha, 0.1.0-alpha -> 0.1.0-alpha.1)"
    );
    log::info!(
        "  --beta          Add or bump beta version (0.1.0 -> 0.1.0-beta, 0.1.0-alpha.2 -> 0.1.0-beta.1)"
    );
    log::info!(
        "  --rc            Add or bump rc version (0.1.0 -> 0.1.0-rc, 0.1.0-beta.1 -> 0.1.0-rc.1)"
    );
    log::info!("  --release       Remove pre-release identifier (0.1.0-alpha -> 0.1.0)");
    log::info!("  --manifest-path <PATH>  Path to Cargo.toml [default: Cargo.toml]");
    log::info!("");
    log::info!("Fmt Options:");
    log::info!("  --check         Check formatting without making changes");
    log::info!("  --manifest-path <PATH>  Path to Cargo.toml");
    log::info!("");
    log::info!("Publish Options:");
    log::info!("  --manifest-path <PATH>  Path to workspace Cargo.toml [default: Cargo.toml]");
    log::info!("  --max-retries <N>       Maximum retry attempts per package [default: 3]");
}
