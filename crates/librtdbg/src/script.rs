use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use rhai::{AST, CustomType, Dynamic, EvalAltResult, FuncArgs, Position};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, CustomType, Hash, PartialEq, Eq)]
pub struct ScriptId(pub u64);

// Script related apis
#[derive(Debug, Clone)]
pub struct Script {
    pub ast: AST,
}

#[derive(Debug, Clone)]
pub struct ScriptCallback {
    pub script_id: ScriptId,
    pub callback: String,
}

#[derive(Debug, Clone, CustomType)]
pub struct Store {
    inner: Arc<Mutex<HashMap<String, Dynamic>>>,
}

#[derive(Debug, Clone)]
pub struct ScriptContext {
    pub script: Arc<Script>,
    pub store: Store,
}

impl Store {
    pub fn new() -> Store {
        Store {
            inner: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn load(&self, key: String) -> Result<Dynamic, Box<EvalAltResult>> {
        Ok(self
            .inner
            .lock()
            .map_err(|e| {
                Box::new(EvalAltResult::ErrorRuntime(
                    e.to_string().into(),
                    Position::NONE,
                ))
            })?
            .get(&key)
            .ok_or(Box::new(EvalAltResult::ErrorRuntime(
                "Value not found".into(),
                Position::NONE,
            )))?
            .clone())
    }

    pub fn store(&self, key: String, value: Dynamic) -> Result<(), Box<EvalAltResult>> {
        self.inner
            .lock()
            .map_err(|e| {
                Box::new(EvalAltResult::ErrorRuntime(
                    e.to_string().into(),
                    Position::NONE,
                ))
            })?
            .insert(key, value);

        Ok(())
    }
}

impl Default for Store {
    fn default() -> Self {
        Self::new()
    }
}
