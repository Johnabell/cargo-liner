//! Main module: regroups parsing CLI arguments, deserializing configuration,
//! and execution of `cargo install` with the required settings.
use std::env;
use std::process;

use anyhow::Result;
use log::error;

mod cargo;
mod cli;
use cli::{LinerArgs, LinerCommands};
mod config;
use config::{CargoCratesToml, UserConfig};

fn main() -> Result<()> {
    let args = LinerArgs::parse_env();

    // Use INFO as a default.
    if env::var_os("RUST_LOG").is_none() {
        env::set_var("RUST_LOG", format!("{}=info", env!("CARGO_CRATE_NAME")));
    }

    pretty_env_logger::try_init()?;

    match &args.command {
        Some(LinerCommands::Import(import_args)) => {
            if !import_args.force && UserConfig::file_path()?.try_exists()? {
                error!("Configuration file already exists, use -f/--force to overwrite.");
                // HACK: consider wrapping a function that `main` would call
                // and filter for errors itself, instead of doing this.
                process::exit(1);
            }
            // Clap conflict settings ensure the options are mutually exclusive.
            (if import_args.exact {
                CargoCratesToml::into_exact_version_config
            } else if import_args.compatible {
                CargoCratesToml::into_comp_version_config
            } else if import_args.patch {
                CargoCratesToml::into_patch_version_config
            } else {
                CargoCratesToml::into_star_version_config
            })(CargoCratesToml::parse_file()?, import_args.keep_self)
            .save_file()?;
        }
        Some(LinerCommands::Ship(ship_args)) => {
            let config = UserConfig::parse_file()?
                .self_update(!ship_args.no_self)
                .update_others(!ship_args.only_self);
            cargo::install_all(&config.packages)?;
        }
        None => cargo::install_all(&UserConfig::parse_file()?.packages)?,
    }

    Ok(())
}
