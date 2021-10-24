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

use crate::config::{
    AppConfig, AppKeyMapping, AppLayout, AppTheme, JsonConfigFile, TomlConfigFile,
};
use crate::context::AppContext;
use crate::history::DirectoryHistory;

const PROGRAM_NAME: &str = "dizi";
const CONFIG_FILE: &str = "client.toml";
const KEYMAP_FILE: &str = "keymap.toml";
const THEME_FILE: &str = "theme.toml";
const LAYOUT_FILE: &str = "layout.json";

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
    static ref LAYOUT_T: AppLayout = AppLayout::get_config(LAYOUT_FILE);
}

#[derive(Clone, Debug, StructOpt)]
pub struct Args {
    // version
    #[structopt(short = "v", long = "version")]
    version: bool,

    // query
    #[structopt(short = "q", long = "query")]
    query: Option<String>,

    // controls
    #[structopt(long = "exit")]
    exit: bool,
    #[structopt(long = "next")]
    next: bool,
    #[structopt(long = "previous")]
    previous: bool,
    #[structopt(long = "pause")]
    pause: bool,
    #[structopt(long = "resume")]
    resume: bool,
    #[structopt(long = "toggle-pause")]
    toggle_play: bool,
}

fn run_app(args: Args) -> DiziResult<()> {
    // print version
    if args.version {
        let version = env!("CARGO_PKG_VERSION");
        println!("{}", version);
        return Ok(());
    }

    let config = AppConfig::get_config(CONFIG_FILE);
    if UnixStream::connect(&config.client_ref().socket).is_err() {
        println!("Server is not running");
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

    // query
    if let Some(query) = args.query {
        run::run_query(&mut context, query)?;
        return Ok(());
    } else if args.exit
        || args.next
        || args.previous
        || args.pause
        || args.resume
        || args.toggle_play
    {
        run::run_control(&mut context, &args);
    } else {
        let keymap = AppKeyMapping::get_config(KEYMAP_FILE);
        // eprintln!("keymap: {:#?}", keymap);

        let display_options = context
            .config_ref()
            .client_ref()
            .display_options_ref()
            .clone();
        context
            .history_mut()
            .populate_to_root(cwd.as_path(), &display_options)?;

        let mut backend: ui::TuiBackend = ui::TuiBackend::new()?;
        run::run_ui(&mut backend, &mut context, keymap)?;
    }
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
