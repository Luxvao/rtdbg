pub mod amd64;

#[derive(Debug, Clone, Copy)]
pub struct TrampolineGenDesc<const SIZE: usize> {
    pub trampoline_src: [u8; SIZE],
    pub function_id_offset: u64,
    pub dispatcher_address_offset: u64,
    pub original_function_address_offset: u64,
}
