use serde::Deserialize;

use std::collections::{hash_map::Entry, HashMap};
use std::convert::{AsMut, AsRef, From};

use termion::event::Event;
#[cfg(feature = "mouse")]
use termion::event::MouseEvent;

use dizi::error::DiziResult;
use dizi::request::client::ClientRequest;

use crate::config::{parse_toml_to_config, TomlConfigFile};
use crate::key_command::{AppCommand, Command, CommandKeybind};
use crate::traits::ToString;
use crate::util::keyparse::str_to_event;

use super::default_keymap::DEFAULT_KEYMAP;

#[derive(Clone, Copy, Debug)]
enum KeymapError {
    Conflict,
}

pub type KeyMapping = HashMap<Event, CommandKeybind>;

#[derive(Debug, Deserialize)]
pub struct CommandKeymap {
    pub keys: Vec<String>,
    pub command: String,
    pub request: Option<ClientRequest>,
}

#[derive(Debug, Deserialize)]
struct AppKeyMappingRaw {
    #[serde(default)]
    pub keymap: Vec<CommandKeymap>,
}

#[derive(Debug)]
pub struct AppKeyMapping {
    map: HashMap<Event, CommandKeybind>,
}

impl AppKeyMapping {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    pub fn default_res() -> DiziResult<Self> {
        let raw: AppKeyMappingRaw = toml::from_str(DEFAULT_KEYMAP)?;
        let keymapping: Self = Self::from(raw);
        Ok(keymapping)
    }
}

impl AsRef<HashMap<Event, CommandKeybind>> for AppKeyMapping {
    fn as_ref(&self) -> &HashMap<Event, CommandKeybind> {
        &self.map
    }
}

impl AsMut<HashMap<Event, CommandKeybind>> for AppKeyMapping {
    fn as_mut(&mut self) -> &mut HashMap<Event, CommandKeybind> {
        &mut self.map
    }
}

impl From<AppKeyMappingRaw> for AppKeyMapping {
    fn from(raw: AppKeyMappingRaw) -> Self {
        let mut keymaps = Self::new();
        keymaps.map = vec_to_map(&raw.keymap);
        keymaps
    }
}

impl TomlConfigFile for AppKeyMapping {
    fn get_config(file_name: &str) -> Self {
        match parse_toml_to_config::<AppKeyMappingRaw, AppKeyMapping>(file_name) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("Failed to parse keymap config: {}", e);
                Self::default()
            }
        }
    }
}

impl std::default::Default for AppKeyMapping {
    fn default() -> Self {
        AppKeyMapping::default_res().unwrap()
    }
}

fn vec_to_map(vec: &[CommandKeymap]) -> HashMap<Event, CommandKeybind> {
    let mut hashmap = HashMap::new();

    for m in vec {
        match Command::from_keymap(m) {
            Ok(command) => {
                let events: Vec<Event> = m
                    .keys
                    .iter()
                    .filter_map(|s| str_to_event(s.as_str()))
                    .collect();

                if events.len() != m.keys.len() {
                    eprintln!("Failed to parse events: {:?}", m.keys);
                    continue;
                }

                let command_str = command.command();
                let result = insert_keycommand(&mut hashmap, command, &events);
                match result {
                    Ok(_) => {}
                    Err(e) => match e {
                        KeymapError::Conflict => {
                            let events_str: Vec<String> =
                                events.iter().map(|e| e.to_string()).collect();
                            eprintln!("Error: Ambiguous Keymapping: Multiple commands mapped to key sequence {:?} {}", events_str, command_str);
                        }
                    },
                }
            }
            Err(e) => eprintln!("Keymap error: {}", e),
        }
    }
    hashmap
}

fn insert_keycommand(
    keymap: &mut KeyMapping,
    keycommand: Command,
    events: &[Event],
) -> Result<(), KeymapError> {
    let num_events = events.len();
    if num_events == 0 {
        return Ok(());
    }

    let event = events[0].clone();
    if num_events == 1 {
        match keymap.entry(event) {
            Entry::Occupied(_) => return Err(KeymapError::Conflict),
            Entry::Vacant(entry) => entry.insert(CommandKeybind::SimpleKeybind(keycommand)),
        };
        return Ok(());
    }

    match keymap.entry(event) {
        Entry::Occupied(mut entry) => match entry.get_mut() {
            CommandKeybind::CompositeKeybind(ref mut m) => {
                insert_keycommand(m, keycommand, &events[1..])
            }
            _ => Err(KeymapError::Conflict),
        },
        Entry::Vacant(entry) => {
            let mut new_map = KeyMapping::new();
            let result = insert_keycommand(&mut new_map, keycommand, &events[1..]);
            if result.is_ok() {
                let composite_command = CommandKeybind::CompositeKeybind(new_map);
                entry.insert(composite_command);
            }
            result
        }
    }
}
