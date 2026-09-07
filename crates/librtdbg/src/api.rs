use serde::{Deserialize, Serialize};

use crate::script::ScriptId;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Request {
    LoadScript { script: String },
    UnloadScript { script_id: ScriptId },
    Status,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Response {
    LoadSuccess { script_id: ScriptId },
    UnloadSuccess,
    Status { loaded_scripts: Vec<ScriptId> },
    Error(String),
}
