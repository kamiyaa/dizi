mod audio;
mod client;
mod client_command;
mod client_commands;
mod config;
mod context;
mod events;
mod server;
mod server_command;
mod server_commands;

use std::path::PathBuf;

use lazy_static::lazy_static;
use structopt::StructOpt;

use dizi_lib::error::DiziResult;

use crate::config::{AppConfig, ConfigStructure};

const PROGRAM_NAME: &str = "dizi";
const CONFIG_FILE: &str = "server.toml";

lazy_static! {
    // dynamically builds the config hierarchy
    static ref CONFIG_HIERARCHY: Vec<PathBuf> = {
        let mut config_dirs = vec![];

        if let Ok(p) = std::env::var("DIZI_CONFIG_HOME") {
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
}

#[derive(Clone, Debug, StructOpt)]
pub struct Args {
    #[structopt(short = "v", long = "version")]
    version: bool,
}

fn run_server(args: Args) -> DiziResult<()> {
    if args.version {
        let version = env!("CARGO_PKG_VERSION");
        println!("{}", version);
        return Ok(());
    }

    let config = AppConfig::get_config(CONFIG_FILE);

    eprintln!("{:#?}", config);

    server::serve(config)
}

fn main() {
    let args = Args::from_args();
    let res = run_server(args);

    match res {
        Ok(_) => {
            println!("TODO: saving playlist...");
            println!("Exiting server");
        }
        Err(e) => eprintln!("Error: {}", e),
    }
}
