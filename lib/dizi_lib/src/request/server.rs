use serde_derive::{Deserialize, Serialize};

use crate::macros::dizi_json_stub;
use crate::request::constants::*;
use crate::traits::DiziJsonCommand;

dizi_json_stub!(ServerQuit, API_SERVER_QUIT);
