use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use rhai::{AST, CustomType, Dynamic, EvalAltResult, FnPtr, Position, TypeBuilder};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, CustomType, Hash, PartialEq, Eq)]
pub struct ScriptId(pub u64);

// Script related apis
#[derive(Debug, Clone)]
pub struct Script {
    pub src: String,
    pub ast: AST,
}

#[derive(Debug, Clone)]
pub struct ScriptCallback {
    pub script_id: ScriptId,
    pub callback: FnPtr,
}

#[derive(Debug, Clone, CustomType)]
#[rhai_type(extra = Self::build_extra)]
pub struct Store {
    inner: Arc<Mutex<HashMap<String, Dynamic>>>,
}

#[derive(Debug, Clone)]
pub struct ScriptContext {
    pub script: Arc<Script>,
    pub store: Store,
}

impl Store {
    fn build_extra(builder: &mut TypeBuilder<Self>) {
        builder
            .with_indexer_get(Self::string_indexer_get)
            .with_indexer_set(Self::string_indexer_set)
            .with_fn("get", |s: &mut Store, k: String| Store::get(s, k))
            .with_fn("set", |s: &mut Store, k: String, v: Dynamic| {
                Store::set(s, k, v)
            });
    }

    fn string_indexer_get(&mut self, index: String) -> Result<Dynamic, Box<EvalAltResult>> {
        self.get(index)
    }

    fn string_indexer_set(
        &mut self,
        index: String,
        value: Dynamic,
    ) -> Result<(), Box<EvalAltResult>> {
        self.set(index, value)
    }

    pub fn new() -> Store {
        Store {
            inner: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn get(&self, key: String) -> Result<Dynamic, Box<EvalAltResult>> {
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

    pub fn set(&self, key: String, value: Dynamic) -> Result<(), Box<EvalAltResult>> {
        self.inner
            .lock()
            .map_err(|e| e.to_string())?
            .insert(key, value);

        Ok(())
    }
}

impl Default for Store {
    fn default() -> Self {
        Self::new()
    }
}
