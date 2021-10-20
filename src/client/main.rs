mod commands;
mod config;
mod context;
mod event;
mod fs;
mod history;
mod key_command;
mod preview;
mod run;
mod ui;
mod util;

use std::os::unix::net::UnixStream;
use std::path::PathBuf;
use std::process;
use std::thread;
use std::time;

use dizi_lib::error::DiziResult;
use lazy_static::lazy_static;
use structopt::StructOpt;

use crate::config::{AppConfig, AppKeyMapping, AppTheme, ConfigStructure};
use crate::context::AppContext;
use crate::history::DirectoryHistory;
use crate::run::run;

const PROGRAM_NAME: &str = "dizi";
const CONFIG_FILE: &str = "client.toml";
const KEYMAP_FILE: &str = "keymap.toml";
const THEME_FILE: &str = "theme.toml";

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

    static ref THEME_T: AppTheme = AppTheme::get_config(THEME_FILE);
    static ref HOME_DIR: Option<PathBuf> = dirs_next::home_dir();
}

#[derive(Clone, Debug, StructOpt)]
pub struct Args {
    #[structopt(short = "v", long = "version")]
    version: bool,
}

fn run_app(args: Args) -> DiziResult<()> {
    if args.version {
        let version = env!("CARGO_PKG_VERSION");
        println!("{}", version);
        return Ok(());
    }

    let config = AppConfig::get_config(CONFIG_FILE);
    let keymap = AppKeyMapping::get_config(KEYMAP_FILE);

    eprintln!("keymap: {:#?}", keymap);

    if UnixStream::connect(&config.client_ref().socket).is_err() {
        println!("Server is not running");
        return Ok(()); // don't start server, still need to iron things out
        println!("Starting server...");
        process::Command::new("dizi-server")
            .stdout(process::Stdio::null())
            .stderr(process::Stdio::null())
            .spawn()?;
        let ten_millis = time::Duration::from_millis(300);
        thread::sleep(ten_millis);
    }
    let stream = UnixStream::connect(&config.client_ref().socket)?;

    let cwd = std::env::current_dir()?;
    let mut context = AppContext::new(config, cwd.clone(), stream);

    let display_options = context
        .config_ref()
        .client_ref()
        .display_options_ref()
        .clone();
    context
        .history_mut()
        .populate_to_root(cwd.as_path(), &display_options)?;

    let mut backend: ui::TuiBackend = ui::TuiBackend::new()?;
    run(&mut backend, &mut context, keymap)?;

    Ok(())
}

fn main() {
    let args = Args::from_args();

    match run_app(args) {
        Ok(_) => {}
        Err(e) => {
            eprintln!("{}", e.to_string());
            process::exit(1);
        }
    }
}
