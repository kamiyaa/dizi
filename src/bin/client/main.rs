mod commands;
mod config;
mod context;
mod event;
mod fs;
mod history;
mod key_command;
mod preview;
mod run;
mod tab;
mod traits;
mod ui;
mod util;

use std::os::unix::net::UnixStream;
use std::path::{Path, PathBuf};
use std::process;
use std::thread;
use std::time;

use dizi::error::DiziResult;
use lazy_static::lazy_static;
use structopt::StructOpt;

use crate::config::{
    AppConfig, AppKeyMapping, AppLayout, AppTheme, JsonConfigFile, TomlConfigFile,
};
use crate::context::AppContext;
use crate::tab::JoshutoTab;

const PROGRAM_NAME: &str = "dizi";
const CONFIG_HOME: &str = "DIZI_CONFIG_HOME";
const CONFIG_FILE: &str = "client.toml";
const KEYMAP_FILE: &str = "keymap.toml";
const THEME_FILE: &str = "theme.toml";
const LAYOUT_FILE: &str = "layout.json";

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
    #[structopt(short = "Q", long = "query")]
    query: Option<String>,
    // query
    #[structopt(long = "query-all")]
    query_all: bool,

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

fn start_server() -> DiziResult {
    println!("Server is not running");
    println!("Starting server...");
    process::Command::new("dizi-server")
        .stdout(process::Stdio::null())
        .stderr(process::Stdio::null())
        .spawn()?;
    Ok(())
}

fn create_context(config: AppConfig, cwd: &Path, stream: UnixStream) -> AppContext {
    AppContext::new(config, cwd.to_path_buf(), stream)
}

fn run_app(args: Args) -> DiziResult {
    // print version
    if args.version {
        let version = env!("CARGO_PKG_VERSION");
        println!("{}", version);
        return Ok(());
    }

    let config = AppConfig::get_config(CONFIG_FILE);
    if let Some(home_dir) = config.client_ref().home_dir.as_ref() {
        std::env::set_current_dir(home_dir)?;
    }
    let cwd = std::env::current_dir()?;

    // query
    if args.query_all {
        // connect to stream
        let stream = UnixStream::connect(config.client_ref().socket_ref())?;
        let mut context = create_context(config, &cwd, stream);
        run::run_query_all(&mut context)?;
        return Ok(());
    } else if let Some(query) = args.query {
        // connect to stream
        let stream = UnixStream::connect(config.client_ref().socket_ref())?;
        let mut context = create_context(config, &cwd, stream);
        run::run_query(&mut context, query)?;
        return Ok(());
    } else if args.exit
        || args.next
        || args.previous
        || args.pause
        || args.resume
        || args.toggle_play
    {
        // connect to stream
        let stream = UnixStream::connect(config.client_ref().socket_ref())?;
        let mut context = create_context(config, &cwd, stream);
        run::run_control(&mut context, &args)?;
    } else {
        lazy_static::initialize(&HOME_DIR);
        lazy_static::initialize(&THEME_T);
        lazy_static::initialize(&LAYOUT_T);

        let mut stream = UnixStream::connect(config.client_ref().socket_ref());
        if stream.is_err() {
            start_server()?;
        }
        println!("Connecting to server ...");
        for i in 1..11 {
            stream = UnixStream::connect(config.client_ref().socket_ref());
            if stream.is_ok() {
                break;
            }
            let wait_interval = time::Duration::from_millis(500);
            thread::sleep(wait_interval);
            println!("Retrying #{} ...", i);
        }

        match stream {
            Err(_) => eprintln!("Error: Failed to connect to server after 10 retries"),
            Ok(stream) => {
                let mut context = create_context(config, &cwd, stream);

                let keymap = AppKeyMapping::get_config(KEYMAP_FILE);
                // eprintln!("keymap: {:#?}", keymap);

                let tab = JoshutoTab::new(
                    cwd,
                    context.ui_context_ref(),
                    context.config_ref().display_options_ref(),
                )?;
                context.tab_context_mut().push_tab(tab);

                let mut backend: ui::AppBackend = ui::AppBackend::new()?;
                run::run_ui(&mut backend, &mut context, keymap)?;
            }
        }
    }
    Ok(())
}

fn main() {
    let args = Args::from_args();

    match run_app(args) {
        Ok(_) => {}
        Err(e) => {
            eprintln!("{}", e);
            process::exit(1);
        }
    }
}
