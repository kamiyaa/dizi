mod audio;
mod client;
mod config;
mod context;
mod events;
mod playlist;
mod server;
mod server_commands;
mod server_util;
mod traits;
mod util;

use std::path::PathBuf;

use clap::Parser;

use dizi::error::DiziResult;
use lazy_static::lazy_static;
use tracing_subscriber::{prelude::*, EnvFilter};

use crate::config::{AppConfig, TomlConfigFile};

const PROGRAM_NAME: &str = "dizi";
const CONFIG_HOME: &str = "DIZI_CONFIG_HOME";
const CONFIG_FILE: &str = "server.toml";

lazy_static! {
    // dynamically builds the config hierarchy
    static ref CONFIG_HIERARCHY: Vec<PathBuf> = {
        let mut config_dirs = vec![];

        if let Ok(p) = std::env::var(CONFIG_HOME) {
            let p = PathBuf::from(p);
            if p.is_dir() {
                config_dirs.push(p);
            }
        }

        if let Ok(dirs) = xdg::BaseDirectories::with_prefix(PROGRAM_NAME) {
            config_dirs.push(dirs.get_config_home());
        }

        if let Ok(p) = std::env::var("HOME") {
            let mut p = PathBuf::from(p);
            p.push(format!(".config/{}", PROGRAM_NAME));
            if p.is_dir() {
                config_dirs.push(p);
            }
        }

        // adds the default config files to the config hierarchy if running through cargo
        if cfg!(debug_assertions) {
            config_dirs.push(PathBuf::from("./config"));
        }
        config_dirs
    };

    static ref HOME_DIR: Option<PathBuf> = dirs_next::home_dir();
}

#[derive(Clone, Debug, Parser)]
pub struct CommandArgs {
    #[arg(short = 'v', long = "version")]
    version: bool,
}

fn run_server(args: CommandArgs) -> DiziResult {
    if args.version {
        let version = env!("CARGO_PKG_VERSION");
        println!("{}", version);
        return Ok(());
    }

    let config = AppConfig::get_config(CONFIG_FILE);

    let env_filter = EnvFilter::from_default_env();
    let fmt_layer = tracing_subscriber::fmt::layer();
    tracing_subscriber::registry()
        .with(env_filter)
        .with(fmt_layer)
        .init();

    tracing::debug!(?config, "Config");
    server::run(config)
}

fn main() {
    let args = CommandArgs::parse();
    let res = run_server(args);

    match res {
        Ok(_) => {}
        Err(err) => eprintln!("Error: {:?}", err),
    }
}
