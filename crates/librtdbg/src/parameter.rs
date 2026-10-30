use std::{
    collections::HashMap,
    ffi::{CStr, CString},
};

use rhai::{CustomType, Dynamic, EvalAltResult, TypeBuilder, export_module};

use crate::{
    error::Error,
    memory::Memory,
    register::{Register, RegisterSnapshot},
    utils::{FromU128, ToU128},
};

#[derive(Debug, Clone, PartialEq, CustomType)]
#[rhai_type(extra = Self::build_extra)]
pub struct Parameter {
    #[rhai_type(skip)]
    pub location: ParameterLocation,
    #[rhai_type(skip)]
    pub value: ParameterValue,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ParameterValue {
    U8(Option<u8>),
    U16(Option<u16>),
    U32(Option<u32>),
    U64(Option<u64>),
    I8(Option<i8>),
    I16(Option<i16>),
    I32(Option<i32>),
    I64(Option<i64>),
    F32(Option<f32>),
    F64(Option<f64>),
    CStr(Option<String>),
    Ptr(Option<u64>, Vec<Parameter>),
    // I love this
    NamedPtr(Option<u64>, HashMap<String, Parameter>),
    NullPtr,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ParameterLocation {
    // This is used for stack and pointer-relative addressing
    Relative(i64),
    Reg(Register),
}

impl Parameter {
    fn build_extra(builder: &mut TypeBuilder<Self>) {
        builder
            .with_indexer_get_set(
                Self::rhai_integer_indexer_get,
                Self::rhai_integer_indexer_set,
            )
            .with_indexer_get_set(Self::rhai_string_indexer_get, Self::rhai_string_indexer_set)
            .on_print(|v| format!("{v:?}"))
            .on_debug(|v| format!("{v:?}"));
    }

    fn rhai_integer_indexer_get(&mut self, index: i64) -> Result<Dynamic, Box<EvalAltResult>> {
        if let ParameterValue::Ptr(_, ref p) = self.value {
            return match p.get(index as usize) {
                Some(v) => Ok(Dynamic::from(v.clone())),
                None => Err("index out of bounds".into()),
            };
        }

        Err("cannot index non-Ptr Parameter".into())
    }

    fn rhai_integer_indexer_set(
        &mut self,
        index: i64,
        value: Dynamic,
    ) -> Result<(), Box<EvalAltResult>> {
        if let ParameterValue::Ptr(_, ref mut p) = self.value {
            let index = index as usize;

            if index >= p.len() {
                return Err("index out of bounds".into());
            }

            p[index] = value.try_cast().ok_or("expected Parameter")?;

            return Ok(());
        }

        Err("cannot index non-Ptr Parameter".into())
    }

    fn rhai_string_indexer_get(&mut self, index: &str) -> Result<Dynamic, Box<EvalAltResult>> {
        if index == "parameter_location" {
            return Ok(Dynamic::from(self.location));
        }

        if index == "parameter_value" {
            return Ok(Dynamic::from(self.value.clone()));
        }

        Ok(match self.value {
            ParameterValue::NamedPtr(_, ref h) => match h.get(index) {
                Some(p) => Dynamic::from(p.clone()),
                None => return Err(format!("no such field `{}`", index).into()),
            },
            ParameterValue::Ptr(_, ref mut v) => match v.as_mut_slice() {
                [only] => {
                    return only.rhai_string_indexer_get(index);
                }
                _ => {
                    return Err(format!(
                        "Ptr passthrough need a single pointee, found {} children",
                        v.len()
                    )
                    .into());
                }
            },
            ParameterValue::U8(Some(v)) if index == "val" => Dynamic::from(v),
            ParameterValue::U16(Some(v)) if index == "val" => Dynamic::from(v),
            ParameterValue::U32(Some(v)) if index == "val" => Dynamic::from(v),
            ParameterValue::U64(Some(v)) if index == "val" => Dynamic::from(v),
            ParameterValue::I8(Some(v)) if index == "val" => Dynamic::from(v),
            ParameterValue::I16(Some(v)) if index == "val" => Dynamic::from(v),
            ParameterValue::I32(Some(v)) if index == "val" => Dynamic::from(v),
            ParameterValue::I64(Some(v)) if index == "val" => Dynamic::from(v),
            ParameterValue::F32(Some(v)) if index == "val" => Dynamic::from(v),
            ParameterValue::F64(Some(v)) if index == "val" => Dynamic::from(v),
            ParameterValue::CStr(Some(ref v)) if index == "val" => Dynamic::from(v.to_string()),
            _ if index == "val" => return Err("value not resolved".into()),
            _ => return Err("cannot string index non-NamedPtr Parameter".into()),
        })
    }

    fn rhai_string_indexer_set(
        &mut self,
        index: &str,
        value: Dynamic,
    ) -> Result<(), Box<EvalAltResult>> {
        if index == "parameter_location" {
            self.location = value.try_cast().ok_or("expected ParameterLocation")?;

            return Ok(());
        }

        if index == "parameter_value" {
            self.value = value.try_cast().ok_or("expected ParameterValue")?;

            return Ok(());
        }

        match self.value {
            ParameterValue::NamedPtr(_, ref mut h) => {
                let parameter = value.try_cast().ok_or("expected Parameter")?;

                match h.entry(index.to_string()) {
                    std::collections::hash_map::Entry::Occupied(mut e) => {
                        *e.get_mut() = parameter;
                    }
                    std::collections::hash_map::Entry::Vacant(e) => {
                        e.insert(parameter);
                    }
                }
            }
            ParameterValue::Ptr(_, ref mut v) => match v.as_mut_slice() {
                [only] => return only.rhai_string_indexer_set(index, value),
                _ => {
                    return Err(format!(
                        "Ptr passthrough need a single pointee, found {} children",
                        v.len()
                    )
                    .into());
                }
            },
            ref mut a if index == "val" => {
                *a = value.try_cast().ok_or("expected ParameterValue")?
            }
            _ => return Err("cannot string index non-NamedPtr Parameter".into()),
        }

        Ok(())
    }

    pub fn try_resolve<R: RegisterSnapshot>(
        &mut self,
        register_snapshot: &R,
        base: u64,
    ) -> Result<(), Error> {
        match &mut self.value {
            ParameterValue::U8(v) => {
                *v = Some(Self::acquire_value(&self.location, register_snapshot, base))
            }
            ParameterValue::U16(v) => {
                *v = Some(Self::acquire_value(&self.location, register_snapshot, base))
            }
            ParameterValue::U32(v) => {
                *v = Some(Self::acquire_value(&self.location, register_snapshot, base))
            }
            ParameterValue::U64(v) => {
                *v = Some(Self::acquire_value(&self.location, register_snapshot, base))
            }
            ParameterValue::I8(v) => {
                *v = Some(Self::acquire_value(&self.location, register_snapshot, base))
            }
            ParameterValue::I16(v) => {
                *v = Some(Self::acquire_value(&self.location, register_snapshot, base))
            }
            ParameterValue::I32(v) => {
                *v = Some(Self::acquire_value(&self.location, register_snapshot, base))
            }
            ParameterValue::I64(v) => {
                *v = Some(Self::acquire_value(&self.location, register_snapshot, base))
            }
            ParameterValue::F32(v) => {
                *v = Some(Self::acquire_value(&self.location, register_snapshot, base))
            }
            ParameterValue::F64(v) => {
                *v = Some(Self::acquire_value(&self.location, register_snapshot, base))
            }
            ParameterValue::CStr(v) => {
                *v = Some(
                    Self::acquire_cstr(&self.location, base)?
                        .to_string_lossy()
                        .into_owned(),
                )
            }
            ParameterValue::Ptr(v, parameters) => {
                let new_base = Self::acquire_value(&self.location, register_snapshot, base);

                if new_base == 0 {
                    self.value = ParameterValue::NullPtr;
                    return Ok(());
                }

                *v = Some(new_base);

                for parameter in parameters {
                    parameter.try_resolve(register_snapshot, new_base)?;
                }
            }
            ParameterValue::NamedPtr(v, parameters) => {
                let new_base = Self::acquire_value(&self.location, register_snapshot, base);

                if new_base == 0 {
                    self.value = ParameterValue::NullPtr;
                    return Ok(());
                }

                *v = Some(new_base);

                for parameter in parameters.values_mut() {
                    parameter.try_resolve(register_snapshot, new_base)?;
                }
            }
            ParameterValue::NullPtr => (),
        }

        Ok(())
    }

    pub fn try_commit<R: RegisterSnapshot>(
        &self,
        register_snapshot: &mut R,
        base: u64,
    ) -> Result<(), Error> {
        match &self.value {
            ParameterValue::U8(Some(v)) => {
                Self::commit_value(*v, &self.location, register_snapshot, base);
            }
            ParameterValue::U16(Some(v)) => {
                Self::commit_value(*v, &self.location, register_snapshot, base);
            }
            ParameterValue::U32(Some(v)) => {
                Self::commit_value(*v, &self.location, register_snapshot, base);
            }
            ParameterValue::U64(Some(v)) => {
                Self::commit_value(*v, &self.location, register_snapshot, base);
            }
            ParameterValue::I8(Some(v)) => {
                Self::commit_value(*v, &self.location, register_snapshot, base);
            }
            ParameterValue::I16(Some(v)) => {
                Self::commit_value(*v, &self.location, register_snapshot, base);
            }
            ParameterValue::I32(Some(v)) => {
                Self::commit_value(*v, &self.location, register_snapshot, base);
            }
            ParameterValue::I64(Some(v)) => {
                Self::commit_value(*v, &self.location, register_snapshot, base);
            }
            ParameterValue::F32(Some(v)) => {
                Self::commit_value(v.to_bits(), &self.location, register_snapshot, base);
            }
            ParameterValue::F64(Some(v)) => {
                Self::commit_value(v.to_bits(), &self.location, register_snapshot, base);
            }
            ParameterValue::CStr(Some(string)) => {
                let cstring = CString::new(string.clone()).map_err(|_| Error::InvalidCString)?;

                Self::commit_cstr(&cstring, &self.location, base)?;
            }
            ParameterValue::Ptr(Some(addy), parameters) => {
                // We first construct, and only after do we write the pointer
                for parameter in parameters {
                    parameter.try_commit(register_snapshot, *addy)?;
                }

                Self::commit_value(*addy, &self.location, register_snapshot, base);
            }
            ParameterValue::NamedPtr(Some(addy), parameters) => {
                for parameter in parameters.values() {
                    parameter.try_commit(register_snapshot, *addy)?;
                }

                Self::commit_value(*addy, &self.location, register_snapshot, base);
            }
            _ => (),
        }

        Ok(())
    }

    pub fn acquire_value<T: Copy + FromU128, R: RegisterSnapshot>(
        location: &ParameterLocation,
        register_snapshot: &R,
        base: u64,
    ) -> T {
        match location {
            ParameterLocation::Relative(o) => {
                let memory = Memory::at(base);

                memory.load(*o)
            }
            ParameterLocation::Reg(reg) => T::from_u128(register_snapshot.acquire_value(reg)),
        }
    }

    pub fn commit_value<T: Copy + FromU128 + ToU128 + PartialEq, R: RegisterSnapshot>(
        value: T,
        location: &ParameterLocation,
        register_snapshot: &mut R,
        base: u64,
    ) {
        if Self::acquire_value::<T, R>(location, register_snapshot, base) == value {
            return;
        }

        match location {
            ParameterLocation::Relative(o) => {
                let memory = Memory::at(base);

                memory.store(*o, value);
            }
            ParameterLocation::Reg(reg) => {
                register_snapshot.set_value(reg, value.to_u128());
            }
        }
    }

    pub fn acquire_cstr(location: &ParameterLocation, base: u64) -> Result<&'static CStr, Error> {
        match location {
            ParameterLocation::Relative(o) => {
                let memory = Memory::at(base);

                Ok(memory.load_cstr(*o))
            }
            ParameterLocation::Reg(_) => Err(Error::CannotLoadCStrFromRegister),
        }
    }

    pub fn commit_cstr(value: &CStr, location: &ParameterLocation, base: u64) -> Result<(), Error> {
        if Self::acquire_cstr(location, base)? == value {
            return Ok(());
        }

        match location {
            ParameterLocation::Relative(o) => {
                let memory = Memory::at(base);

                memory.store_cstr(*o, value);
            }
            ParameterLocation::Reg(_) => return Err(Error::CannotStoreCStrToRegister),
        }

        Ok(())
    }
}

// Expose Parameter::create() to rhai
#[export_module]
pub mod parameter_module {
    use crate::parameter::{Parameter, ParameterLocation, ParameterValue};

    #[rhai_fn(name = "create")]
    pub fn create(location: ParameterLocation, value: ParameterValue) -> Parameter {
        Parameter { location, value }
    }
}

// To make the `ParameterValue` enum work properly
#[export_module]
pub mod parametervalue_module {
    use rhai::{Array, Dynamic, EvalAltResult, Map};

    use crate::rhai_utils::{parse_array, parse_map};

    // Convenience
    #[rhai_fn(global, get = "enum_type", pure)]
    pub fn get_type(paramv: &mut ParameterValue) -> String {
        match paramv {
            ParameterValue::U8(_) => "U8".to_string(),
            ParameterValue::U16(_) => "U16".to_string(),
            ParameterValue::U32(_) => "U32".to_string(),
            ParameterValue::U64(_) => "U64".to_string(),
            ParameterValue::I8(_) => "I8".to_string(),
            ParameterValue::I16(_) => "I16".to_string(),
            ParameterValue::I32(_) => "I32".to_string(),
            ParameterValue::I64(_) => "I64".to_string(),
            ParameterValue::F32(_) => "F32".to_string(),
            ParameterValue::F64(_) => "F64".to_string(),
            ParameterValue::CStr(_) => "CStr".to_string(),
            ParameterValue::Ptr(_, _) => "Ptr".to_string(),
            ParameterValue::NamedPtr(_, _) => "NamedPtr".to_string(),
            ParameterValue::NullPtr => "NullPtr".to_string(),
        }
    }

    #[rhai_fn(global, get = "field_0", pure)]
    pub fn get_field_0(paramv: &mut ParameterValue) -> Dynamic {
        match paramv {
            ParameterValue::U8(Some(v)) => Dynamic::from_int(*v as i64),
            ParameterValue::U16(Some(v)) => Dynamic::from_int(*v as i64),
            ParameterValue::U32(Some(v)) => Dynamic::from_int(*v as i64),
            ParameterValue::U64(Some(v)) => Dynamic::from_int(*v as i64),
            ParameterValue::I8(Some(v)) => Dynamic::from_int(*v as i64),
            ParameterValue::I16(Some(v)) => Dynamic::from_int(*v as i64),
            ParameterValue::I32(Some(v)) => Dynamic::from_int(*v as i64),
            ParameterValue::I64(Some(v)) => Dynamic::from_int(*v),
            ParameterValue::F32(Some(v)) => Dynamic::from_float(*v as f64),
            ParameterValue::F64(Some(v)) => Dynamic::from_float(*v),
            ParameterValue::CStr(Some(v)) => v.clone().into(),
            ParameterValue::Ptr(Some(v), _) => Dynamic::from_int(*v as i64),
            ParameterValue::NamedPtr(Some(v), _) => Dynamic::from_int(*v as i64),
            _ => Dynamic::UNIT,
        }
    }

    #[rhai_fn(global, get = "field_1", pure)]
    pub fn get_field_1(paramv: &mut ParameterValue) -> Dynamic {
        match paramv {
            ParameterValue::Ptr(_, parameters) => parameters.clone().into(),
            ParameterValue::NamedPtr(_, parameters) => parameters.clone().into(),
            _ => Dynamic::UNIT,
        }
    }

    // Constructors
    #[allow(nonstandard_style)]
    pub const NullPtr: ParameterValue = ParameterValue::NullPtr;

    pub const U8: ParameterValue = ParameterValue::U8(None);

    #[rhai_fn(name = "U8")]
    pub fn u8_val(value: i64) -> ParameterValue {
        ParameterValue::U8(Some(value as u8))
    }

    pub const U16: ParameterValue = ParameterValue::U16(None);

    #[rhai_fn(name = "U16")]
    pub fn u16_val(value: i64) -> ParameterValue {
        ParameterValue::U16(Some(value as u16))
    }

    pub const U32: ParameterValue = ParameterValue::U32(None);

    #[rhai_fn(name = "U32")]
    pub fn u32_val(value: i64) -> ParameterValue {
        ParameterValue::U32(Some(value as u32))
    }

    pub const U64: ParameterValue = ParameterValue::U64(None);

    #[rhai_fn(name = "U64")]
    pub fn u64_val(value: i64) -> ParameterValue {
        ParameterValue::U64(Some(value as u64))
    }

    pub const I8: ParameterValue = ParameterValue::I8(None);

    #[rhai_fn(name = "I8")]
    pub fn i8_val(value: i64) -> ParameterValue {
        ParameterValue::I8(Some(value as i8))
    }

    pub const I16: ParameterValue = ParameterValue::I16(None);

    #[rhai_fn(name = "I16")]
    pub fn i16_val(value: i64) -> ParameterValue {
        ParameterValue::I16(Some(value as i16))
    }

    pub const I32: ParameterValue = ParameterValue::I32(None);

    #[rhai_fn(name = "I32")]
    pub fn i32_val(value: i64) -> ParameterValue {
        ParameterValue::I32(Some(value as i32))
    }

    pub const I64: ParameterValue = ParameterValue::I64(None);

    #[rhai_fn(name = "I64")]
    pub fn i64_val(value: i64) -> ParameterValue {
        ParameterValue::I64(Some(value))
    }

    pub const F32: ParameterValue = ParameterValue::F32(None);

    #[rhai_fn(name = "F32")]
    pub fn f32_val(value: f64) -> ParameterValue {
        ParameterValue::F32(Some(value as f32))
    }

    pub const F64: ParameterValue = ParameterValue::F64(None);

    #[rhai_fn(name = "F64")]
    pub fn f64_val(value: f64) -> ParameterValue {
        ParameterValue::F64(Some(value))
    }

    #[allow(nonstandard_style)]
    pub const CStr: ParameterValue = ParameterValue::CStr(None);

    #[rhai_fn(name = "CStr", return_raw)]
    pub fn cstr_val(value: String) -> Result<ParameterValue, Box<EvalAltResult>> {
        Ok(ParameterValue::CStr(Some(value)))
    }

    #[rhai_fn(name = "Ptr", return_raw)]
    pub fn ptr_no_addy(to_what: Array) -> Result<ParameterValue, Box<EvalAltResult>> {
        Ok(ParameterValue::Ptr(
            None,
            parse_array(to_what).ok_or("Array was not of type Parameter")?,
        ))
    }

    #[rhai_fn(name = "Ptr", return_raw)]
    pub fn ptr_with_addy(addy: i64, to_what: Array) -> Result<ParameterValue, Box<EvalAltResult>> {
        Ok(ParameterValue::Ptr(
            Some(addy as u64),
            parse_array(to_what).ok_or("Array was not of type Parameter")?,
        ))
    }

    #[rhai_fn(name = "NamedPtr", return_raw)]
    pub fn named_ptr_no_addy(to_what: Map) -> Result<ParameterValue, Box<EvalAltResult>> {
        Ok(ParameterValue::NamedPtr(
            None,
            parse_map(to_what).ok_or("Map was not of type <String, Parameter>!")?,
        ))
    }

    #[rhai_fn(name = "NamedPtr", return_raw)]
    pub fn named_ptr_with_addy(
        addy: i64,
        to_what: Map,
    ) -> Result<ParameterValue, Box<EvalAltResult>> {
        Ok(ParameterValue::NamedPtr(
            Some(addy as u64),
            parse_map(to_what).ok_or("Map was not of type <String, Parameter>!")?,
        ))
    }
}

#[export_module]
pub mod parameterlocation_module {
    use rhai::Dynamic;

    // Convenience
    #[rhai_fn(global, get = "enum_type", pure)]
    pub fn get_type(paraml: &mut ParameterLocation) -> String {
        match paraml {
            ParameterLocation::Relative(_) => "Relative".to_string(),
            ParameterLocation::Reg(_) => "Reg".to_string(),
        }
    }

    #[rhai_fn(global, get = "value", pure)]
    pub fn get_value(paraml: &mut ParameterLocation) -> Dynamic {
        match paraml {
            ParameterLocation::Relative(v) => Dynamic::from(*v),
            ParameterLocation::Reg(register) => Dynamic::from(*register),
        }
    }

    // Constructors
    #[allow(nonstandard_style)]
    pub fn Relative(value: i64) -> ParameterLocation {
        ParameterLocation::Relative(value)
    }

    #[allow(nonstandard_style)]
    pub fn Reg(value: Register) -> ParameterLocation {
        ParameterLocation::Reg(value)
    }
}
