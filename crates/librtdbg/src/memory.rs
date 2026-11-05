use std::ffi::{CStr, c_void};

use crate::error::Error;

#[derive(Debug, Clone)]
pub struct MemoryBlock {
    base: *mut c_void,
}

impl MemoryBlock {
    pub fn at(base: u64) -> MemoryBlock {
        MemoryBlock {
            base: base as *mut c_void,
        }
    }

    pub fn load<T: Copy>(&self, offset: i64) -> Result<T, Error> {
        unsafe {
            Ok((self.base as *mut T)
                .byte_offset(offset as isize)
                .read_unaligned())
        }
    }

    pub fn store<T>(&self, offset: i64, value: T) -> Result<(), Error> {
        unsafe {
            ((self.base as *mut T).byte_offset(offset as isize)).write_unaligned(value);
        }

        Ok(())
    }

    pub fn load_cstr(&self, offset: i64) -> Result<&'static CStr, Error> {
        unsafe {
            Ok(CStr::from_ptr(
                (self.base as *const i8).byte_offset(offset as isize),
            ))
        }
    }

    pub fn store_cstr(&self, offset: i64, value: &CStr) -> Result<(), Error> {
        let ptr = unsafe { (self.base as *mut u8).byte_offset(offset as isize) };

        for (i, cstr_char) in value.to_bytes_with_nul().iter().enumerate() {
            unsafe {
                *(ptr.byte_offset(i as isize)) = *cstr_char;
            }
        }

        Ok(())
    }
}
