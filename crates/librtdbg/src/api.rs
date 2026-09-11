use serde::{Deserialize, Serialize};

use crate::script::ScriptId;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Request {
    LoadScript {
        script: String,
    },
    UnloadScript {
        script_id: ScriptId,
    },
    RunFunction {
        script_id: ScriptId,
        function: String,
    },
    Status,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Response {
    Success,
    LoadSuccess { script_id: ScriptId },
    RunSuccess { response: String },
    Status { loaded_scripts: Vec<ScriptId> },
    Error(String),
}
