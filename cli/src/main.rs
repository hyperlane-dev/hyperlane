//! Hyperlane CLI
//!
//! A command-line tool for Hyperlane framework.

use hyperlane_cli::*;

use std::process::exit;

#[tokio::main]
async fn main() {
    Logger::init(log::LevelFilter::Info);
    let args: Args = parse_args();
    match args.command {
        CommandType::Fmt => {
            if let Err(error) = execute_fmt(&args).await {
                log::error!("fmt failed: {error}");
                exit(1);
            }
        }
        CommandType::Watch => {
            if let Err(error) = execute_watch().await {
                log::error!("watch failed: {error}");
                exit(1);
            }
        }
        CommandType::Bump => {
            let manifest_path: String = args
                .manifest_path
                .unwrap_or_else(|| "Cargo.toml".to_string());
            let bump_type: BumpVersionType = args.bump_type.unwrap_or(BumpVersionType::Patch);
            match execute_bump(&manifest_path, &bump_type).await {
                Ok(new_version) => {
                    log::info!("Version bumped to {new_version}");
                }
                Err(error) => {
                    log::error!("bump failed: {error}");
                    exit(1);
                }
            }
        }
        CommandType::Publish => {
            let manifest_path: String = args
                .manifest_path
                .unwrap_or_else(|| "Cargo.toml".to_string());
            let max_retries: u32 = args.max_retries;
            match execute_publish(&manifest_path, max_retries).await {
                Ok(results) => {
                    let failed_count: usize = results
                        .iter()
                        .filter(|r: &&PublishResult| !r.success)
                        .count();
                    if failed_count > 0 {
                        log::error!("Publish completed with {failed_count} failures");
                        exit(1);
                    } else {
                        log::info!("All packages published successfully");
                    }
                }
                Err(error) => {
                    log::error!("publish failed: {error}");
                    exit(1);
                }
            }
        }
        CommandType::New => {
            if let Some(project_name) = args.project_name {
                if let Err(error) = execute_new(&project_name).await {
                    log::error!("new failed: {error}");
                    exit(1);
                }
            } else {
                log::error!(
                    "Error: Project name is required. Usage: hyperlane-cli new <PROJECT_NAME>"
                );
                exit(1);
            }
        }
        CommandType::Template => {
            let template_type: TemplateType = match args.template_type {
                Some(tt) => tt,
                None => {
                    log::error!(
                        "Error: Template type is required. Usage: hyperlane-cli template <TYPE> [SUBTYPE] <NAME>"
                    );
                    exit(1);
                }
            };
            let component_name: String = match args.component_name {
                Some(cn) => cn,
                None => {
                    log::error!(
                        "Error: Component name is required. Usage: hyperlane-cli template <TYPE> [SUBTYPE] <NAME>"
                    );
                    exit(1);
                }
            };
            if template_type == TemplateType::Model && args.model_sub_type.is_none() {
                log::error!("Error: Model type requires subtype (application|request|response)");
                exit(1);
            }
            if let Err(error) =
                execute_template(template_type, &component_name, args.model_sub_type).await
            {
                log::error!("template failed: {error}");
                exit(1);
            }
        }
        CommandType::Help => print_help(),
        CommandType::Version => print_version(),
    }
}
