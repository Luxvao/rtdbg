use std::{
    any::type_name,
    collections::HashMap,
    sync::{Arc, Mutex},
};

use rhai::{AST, CustomType, Dynamic, EvalAltResult, FnPtr, TypeBuilder};
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
            })
            .with_fn("fetch_add", Self::fetch_add)
            .with_fn("add_fetch", Self::add_fetch);
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
            .map_err(|e| e.to_string())?
            .get(&key)
            .ok_or("value not initialised")?
            .clone())
    }

    pub fn set(&self, key: String, value: Dynamic) -> Result<(), Box<EvalAltResult>> {
        self.inner
            .lock()
            .map_err(|e| e.to_string())?
            .insert(key, value);

        Ok(())
    }

    pub fn fetch_add(&self, key: String, value: i64) -> Result<i64, Box<EvalAltResult>> {
        let mut lock = self.inner.lock().map_err(|e| e.to_string())?;

        let entry = lock.get_mut(&key).ok_or("value not initialised")?;

        let val = cast_wrapper::<i64>(entry)?;

        *entry = Dynamic::from_int(val + value);

        Ok(val)
    }

    pub fn add_fetch(&self, key: String, value: i64) -> Result<i64, Box<EvalAltResult>> {
        let mut lock = self.inner.lock().map_err(|e| e.to_string())?;

        let entry = lock.get_mut(&key).ok_or("value not initialised")?;

        let val = cast_wrapper::<i64>(entry)? + value;

        *entry = Dynamic::from_int(val);

        Ok(val)
    }
}

fn cast_wrapper<T: 'static>(dynamic: &mut Dynamic) -> Result<T, Box<EvalAltResult>> {
    match std::mem::take(dynamic).try_cast_result::<T>() {
        Ok(val) => Ok(val),
        Err(val) => {
            *dynamic = val;
            Err(format!("expected {}", type_name::<T>()).into())
        }
    }
}

impl Default for Store {
    fn default() -> Self {
        Self::new()
    }
}
