use serde_derive::{Deserialize, Serialize};

use crate::macros::dizi_json_stub;
use crate::traits::DiziJsonCommand;

use super::constants::*;

dizi_json_stub!(ServerQuit, RESP_SERVER_QUIT);
