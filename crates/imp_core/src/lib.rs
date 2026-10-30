pub mod error;
pub mod rhai_enum_impl;

use rhai::{CustomType, Dynamic, TypeBuilder};

pub trait Parser {
    fn import_slots<T>(extra: T) -> impl Iterator<Item = ImportSlot>;
    fn modules<T>(extra: T) -> impl Iterator<Item = Module>;
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, CustomType)]
#[rhai_type(extra = Self::build_extra)]
pub struct ImportSlot {
    pub import_name: String,
    #[rhai_type(skip)]
    pub module_path: Option<String>,
    pub ptr: u64,
}

#[derive(Clone, Debug, CustomType)]
#[rhai_type(extra = Self::build_extra)]
pub struct Module {
    #[rhai_type(skip)]
    pub path: Option<String>,
    #[rhai_type(get = Self::get_base, set = Self::set_base)]
    pub base: u64,
    #[rhai_type(get = Self::get_bias, set = Self::set_bias)]
    pub bias: u64,
    #[rhai_type(get = Self::get_size, set = Self::set_size)]
    pub size: u64,
    pub format: ObjectFormat,
    pub is_main: bool,
}

#[derive(Clone, Debug)]
pub enum ObjectFormat {
    Elf,
    Pe,
    Macho,
}

impl ImportSlot {
    fn build_extra(builder: &mut TypeBuilder<Self>) {
        builder
            .with_get("module_path", Self::get_module_path)
            .with_set("module_path", Self::set_module_path)
            .on_print(|v| format!("{v:?}"))
            .on_debug(|v| format!("{v:?}"));
    }

    fn get_module_path(&mut self) -> Dynamic {
        self.module_path
            .as_ref()
            .map(|v| Dynamic::from(v.clone()))
            .unwrap_or_default()
    }

    fn set_module_path(&mut self, path: String) {
        self.module_path = Some(path);
    }
}

impl Module {
    fn build_extra(builder: &mut TypeBuilder<Self>) {
        builder
            .with_get("path", Self::get_path)
            .with_set("path", Self::set_path)
            .on_print(|v| format!("{v:?}"))
            .on_debug(|v| format!("{v:?}"));
    }

    fn get_path(&mut self) -> Dynamic {
        self.path
            .as_ref()
            .map(|v| Dynamic::from(v.clone()))
            .unwrap_or_default()
    }

    fn get_base(&self) -> i64 {
        self.base as i64
    }

    fn get_bias(&self) -> i64 {
        self.bias as i64
    }

    fn get_size(&self) -> i64 {
        self.size as i64
    }

    fn set_path(&mut self, path: String) {
        self.path = Some(path);
    }

    fn set_base(&mut self, v: i64) {
        self.base = v as u64;
    }

    fn set_bias(&mut self, v: i64) {
        self.bias = v as u64;
    }

    fn set_size(&mut self, v: i64) {
        self.size = v as u64;
    }
}
