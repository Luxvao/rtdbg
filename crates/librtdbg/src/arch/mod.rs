pub mod amd64;

#[derive(Debug, Clone, Copy)]
pub struct TrampolineGenDesc<const SIZE: usize> {
    pub trampoline_src: &'static [u8; SIZE],
    pub function_id_offset: u64,
    pub dispatcher_address_offset: u64,
    pub original_function_address_offset: u64,
}

#[cfg(target_arch = "x86_64")]
pub type NativeRegisterStruct = amd64::Amd64RegisterSnapshot;

#[cfg(target_arch = "x86_64")]
pub const NATIVE_TRAMPOLINE_SIZE: u64 = amd64::TRAMPOLINE_SIZE as u64;

#[cfg(target_arch = "x86_64")]
pub const NATIVE_TRAMPOLINE_GEN_DESC: TrampolineGenDesc<{ amd64::TRAMPOLINE_SIZE }> =
    amd64::TRAMPOLINE_GEN_DESC;
