use std::ffi::{CStr, c_void};

#[derive(Debug, Clone)]
pub struct Memory {
    base: *mut c_void,
}

impl Memory {
    pub fn at(base: u64) -> Memory {
        Memory {
            base: base as *mut c_void,
        }
    }

    pub fn load<T: Copy>(&self, offset: i64) -> T {
        unsafe {
            (self.base as *mut T)
                .byte_offset(offset as isize)
                .read_unaligned()
        }
    }

    pub fn store<T>(&self, offset: i64, value: T) {
        unsafe {
            ((self.base as *mut T).byte_offset(offset as isize)).write_unaligned(value);
        }
    }

    pub fn load_cstr(&self, offset: i64) -> &'static CStr {
        unsafe { CStr::from_ptr((self.base as *const i8).byte_offset(offset as isize)) }
    }

    pub fn store_cstr(&self, offset: i64, value: &CStr) {
        let ptr = unsafe { (self.base as *mut u8).byte_offset(offset as isize) };

        for (i, cstr_char) in value.to_bytes_with_nul().iter().enumerate() {
            unsafe {
                *(ptr.byte_offset(i as isize)) = *cstr_char;
            }
        }
    }
}
