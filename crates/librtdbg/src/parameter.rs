use std::{
    collections::HashMap,
    ffi::{CStr, CString},
};

use rhai::{CustomType, export_module};

use crate::{
    error::Error,
    memory::Memory,
    register::{Register, RegisterSnapshot},
    utils::{FromU128, ToU128},
};

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
    CStr(Option<CString>),
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

#[derive(Debug, Clone, PartialEq, CustomType)]
pub struct Parameter {
    pub location: ParameterLocation,
    pub value: ParameterValue,
}

impl Parameter {
    pub fn try_resolve(
        &mut self,
        register_snapshot: &RegisterSnapshot,
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
                *v = Some(Self::acquire_cstr(&self.location, base)?.to_owned())
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

    pub fn try_commit(
        &self,
        register_snapshot: &mut RegisterSnapshot,
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
            ParameterValue::CStr(Some(cstring)) => {
                Self::commit_cstr(&cstring, &self.location, base)?;
            }
            ParameterValue::Ptr(addy, parameters) => {
                // We first construct, and only after do we write the pointer
                for parameter in parameters {
                    parameter.try_commit(register_snapshot, base)?;
                }

                if let Some(addy) = addy {
                    Self::commit_value(*addy, &self.location, register_snapshot, base);
                }
            }
            ParameterValue::NamedPtr(addy, parameters) => {
                for parameter in parameters.values() {
                    parameter.try_commit(register_snapshot, base)?;
                }

                if let Some(addy) = addy {
                    Self::commit_value(*addy, &self.location, register_snapshot, base);
                }
            }
            _ => (),
        }

        Ok(())
    }

    pub fn acquire_value<T: Copy + FromU128>(
        location: &ParameterLocation,
        register_snapshot: &RegisterSnapshot,
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

    pub fn commit_value<T: ToU128>(
        value: T,
        location: &ParameterLocation,
        register_snapshot: &mut RegisterSnapshot,
        base: u64,
    ) {
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

// To make the `ParameterValue` enum work properly
#[export_module]
pub mod parametervalue_module {
    use rhai::{Dynamic, EvalAltResult, Map};

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
            ParameterValue::U8(Some(v)) => Dynamic::from(*v),
            ParameterValue::U16(Some(v)) => Dynamic::from(*v),
            ParameterValue::U32(Some(v)) => Dynamic::from(*v),
            ParameterValue::U64(Some(v)) => Dynamic::from(*v),
            ParameterValue::I8(Some(v)) => Dynamic::from(*v),
            ParameterValue::I16(Some(v)) => Dynamic::from(*v),
            ParameterValue::I32(Some(v)) => Dynamic::from(*v),
            ParameterValue::I64(Some(v)) => Dynamic::from(*v),
            ParameterValue::F32(Some(v)) => Dynamic::from(*v),
            ParameterValue::F64(Some(v)) => Dynamic::from(*v),
            ParameterValue::CStr(Some(v)) => Dynamic::from(v.to_string_lossy().into_owned()),
            ParameterValue::Ptr(Some(v), _) => Dynamic::from(*v),
            ParameterValue::NamedPtr(Some(v), _) => Dynamic::from(*v),
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
        Ok(ParameterValue::CStr(Some(
            CString::new(value).map_err(|e| e.to_string())?,
        )))
    }

    #[rhai_fn(name = "Ptr")]
    pub fn ptr_no_addy(to_what: Vec<Parameter>) -> ParameterValue {
        ParameterValue::Ptr(None, to_what)
    }

    #[rhai_fn(name = "Ptr")]
    pub fn ptr_with_addy(addy: i64, to_what: Vec<Parameter>) -> ParameterValue {
        ParameterValue::Ptr(Some(addy as u64), to_what)
    }

    #[rhai_fn(name = "NamedPtr", return_raw)]
    pub fn named_ptr_no_addy(to_what: Map) -> Result<ParameterValue, Box<EvalAltResult>> {
        let map = to_what
            .into_iter()
            .map(|(key, value)| {
                Ok((
                    key.to_string(),
                    value
                        .try_cast::<Parameter>()
                        .ok_or(Box::new(EvalAltResult::from("NamedPtr must take a Map!")))?,
                ))
            })
            .collect::<Result<HashMap<String, Parameter>, Box<EvalAltResult>>>()?;

        Ok(ParameterValue::NamedPtr(None, map))
    }

    #[rhai_fn(name = "NamedPtr", return_raw)]
    pub fn named_ptr_with_addy(
        addy: i64,
        to_what: Map,
    ) -> Result<ParameterValue, Box<EvalAltResult>> {
        let map = to_what
            .into_iter()
            .map(|(key, value)| {
                Ok((
                    key.to_string(),
                    value
                        .try_cast::<Parameter>()
                        .ok_or(Box::new(EvalAltResult::from("NamedPtr must take a Map!")))?,
                ))
            })
            .collect::<Result<HashMap<String, Parameter>, Box<EvalAltResult>>>()?;

        Ok(ParameterValue::NamedPtr(Some(addy as u64), map))
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
