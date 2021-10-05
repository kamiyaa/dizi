use std::path::PathBuf;
use serde_derive::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PlayerPlay {
    pub command: String,
    pub path: PathBuf,
}
