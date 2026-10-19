use serde::{Deserialize, Serialize};
use serde_json::Value;

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
        params: Option<Value>,
    },
    Status,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Response {
    Success,
    LoadSuccess { script_id: ScriptId },
    RunSuccess { response: Value },
    Status { loaded_scripts: Vec<ScriptId> },
    Error(String),
}
