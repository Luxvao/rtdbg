pub mod amd64;

#[derive(Debug, Clone, Copy)]
pub struct TrampolineGenDesc<const SIZE: usize> {
    pub trampoline_src: &'static [u8; SIZE],
    pub function_id_offset: u64,
    pub dispatcher_address_offset: u64,
    pub original_function_address_offset: u64,
}

#[cfg(target_arch = "x86_64")]
pub const NATIVE_TRAMPOLINE_SIZE: usize = amd64::TRAMPOLINE_SIZE;
