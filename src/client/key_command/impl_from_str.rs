use std::path;
use std::time;

use dirs_next::home_dir;
use shellexpand::tilde_with_context;

use dizi_lib::error::{DiziError, DiziErrorKind};
use dizi_lib::request::client::ClientRequest;

use crate::util::select::SelectOption;
use crate::util::sort_type::SortType;

use crate::HOME_DIR;

use super::constants::*;
use super::Command;

macro_rules! simple_command_conversion_case {
    ($command: ident, $command_match: ident, $enum_name: expr) => {
        if $command == $command_match {
            return Ok($enum_name);
        }
    };
}

impl std::str::FromStr for Command {
    type Err = DiziError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(stripped) = s.strip_prefix(':') {
            return Ok(Self::CommandLine(stripped.to_owned(), "".to_owned()));
        }

        let (command, arg) = match s.find(' ') {
            Some(i) => (&s[..i], s[i..].trim_start()),
            None => (s, ""),
        };

        if command.starts_with("/") {
            match ClientRequest::parse_str(command, arg) {
                Ok(s) => return Ok(Self::Request(s)),
                Err(e) => return Err(e),
            }
        }

        simple_command_conversion_case!(command, CMD_CLOSE, Self::Close);
        simple_command_conversion_case!(command, CMD_CURSOR_MOVE_HOME, Self::CursorMoveHome);
        simple_command_conversion_case!(command, CMD_CURSOR_MOVE_END, Self::CursorMoveEnd);
        simple_command_conversion_case!(command, CMD_CURSOR_MOVE_PAGEUP, Self::CursorMovePageUp);
        simple_command_conversion_case!(
            command,
            CMD_CURSOR_MOVE_PAGEDOWN,
            Self::CursorMovePageDown
        );
        simple_command_conversion_case!(command, CMD_OPEN_FILE, Self::OpenFile);

        simple_command_conversion_case!(command, CMD_SEARCH_SKIM, Self::SearchSkim);
        simple_command_conversion_case!(command, CMD_SEARCH_NEXT, Self::SearchNext);
        simple_command_conversion_case!(command, CMD_SEARCH_PREV, Self::SearchPrev);

        simple_command_conversion_case!(command, CMD_TOGGLE_HIDDEN, Self::ToggleHiddenFiles);

        if command == CMD_CHANGE_DIRECTORY {
            match arg {
                "" => match HOME_DIR.as_ref() {
                    Some(s) => Ok(Self::ChangeDirectory(s.clone())),
                    None => Err(DiziError::new(
                        DiziErrorKind::EnvVarNotPresent,
                        format!("{}: Cannot find home directory", command),
                    )),
                },
                ".." => Ok(Self::ParentDirectory),
                arg => Ok({
                    let path_accepts_tilde = tilde_with_context(arg, home_dir);
                    Self::ChangeDirectory(path::PathBuf::from(path_accepts_tilde.as_ref()))
                }),
            }
        } else if command == CMD_CURSOR_MOVE_DOWN {
            match arg {
                "" => Ok(Self::CursorMoveDown(1)),
                arg => match arg.trim().parse::<usize>() {
                    Ok(s) => Ok(Self::CursorMoveDown(s)),
                    Err(e) => Err(DiziError::new(DiziErrorKind::ParseError, e.to_string())),
                },
            }
        } else if command == CMD_CURSOR_MOVE_UP {
            match arg {
                "" => Ok(Self::CursorMoveUp(1)),
                arg => match arg.trim().parse::<usize>() {
                    Ok(s) => Ok(Self::CursorMoveUp(s)),
                    Err(e) => Err(DiziError::new(DiziErrorKind::ParseError, e.to_string())),
                },
            }
        } else if command == CMD_RELOAD_DIRECTORY_LIST {
            Ok(Self::ReloadDirList)
        } else if command == CMD_SEARCH_STRING {
            match arg {
                "" => Err(DiziError::new(
                    DiziErrorKind::InvalidParameters,
                    format!("{}: Expected 1, got 0", command),
                )),
                arg => Ok(Self::SearchString(arg.to_string())),
            }
        } else if command == CMD_SEARCH_GLOB {
            match arg {
                "" => Err(DiziError::new(
                    DiziErrorKind::InvalidParameters,
                    format!("{}: Expected 1, got 0", command),
                )),
                arg => Ok(Self::SearchGlob(arg.to_string())),
            }
        } else if command == CMD_SELECT_FILES {
            let mut options = SelectOption::default();
            let mut pattern = "";
            match shell_words::split(arg) {
                Ok(args) => {
                    for arg in args.iter() {
                        match arg.as_str() {
                            "--toggle=true" => options.toggle = true,
                            "--all=true" => options.all = true,
                            "--toggle=false" => options.toggle = false,
                            "--all=false" => options.all = false,
                            "--deselect=true" => options.reverse = true,
                            "--deselect=false" => options.reverse = false,
                            s => pattern = s,
                        }
                    }
                    Ok(Self::SelectFiles(pattern.to_string(), options))
                }
                Err(e) => Err(DiziError::new(
                    DiziErrorKind::InvalidParameters,
                    format!("{}: {}", arg, e),
                )),
            }
        } else if command == CMD_SORT {
            match arg {
                "reverse" => Ok(Self::SortReverse),
                arg => match SortType::parse(arg) {
                    Some(s) => Ok(Self::Sort(s)),
                    None => Err(DiziError::new(
                        DiziErrorKind::InvalidParameters,
                        format!("{}: Unknown option '{}'", command, arg),
                    )),
                },
            }
        } else {
            Err(DiziError::new(
                DiziErrorKind::UnrecognizedCommand,
                format!("Unrecognized command '{}'", command),
            ))
        }
    }
}
