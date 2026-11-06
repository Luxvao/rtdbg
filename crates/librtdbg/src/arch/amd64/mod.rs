pub mod reg;
pub mod rel;

use crate::arch::TrampolineGenDesc;

pub const TRAMPOLINE_SIZE: usize = 448;

const TRAMPOLINE_HOOK: &[u8; TRAMPOLINE_SIZE] =
    include_bytes!("../../../../../artefacts/amd64_trampoline.bin");

const FUNCTION_ID_OFFSET: u16 = 0x1a8;
const DISPATCHER_ADDRESS_OFFSET: u16 = FUNCTION_ID_OFFSET + 8;
const ORIGINAL_FUNCTION_OFFSET: u16 = FUNCTION_ID_OFFSET + 16;

// Consistency check
const _: () = assert!(
    TRAMPOLINE_HOOK[FUNCTION_ID_OFFSET as usize..FUNCTION_ID_OFFSET as usize + 8]
        == [0xEF, 0xBE, 0xAD, 0xDE, 0xEF, 0xBE, 0xAD, 0xDE]
);

const _: () = assert!(
    TRAMPOLINE_HOOK[DISPATCHER_ADDRESS_OFFSET as usize..DISPATCHER_ADDRESS_OFFSET as usize + 8]
        == [0xEF, 0xBE, 0xAD, 0xDE, 0xEF, 0xBE, 0xAD, 0xDE]
);

const _: () = assert!(
    TRAMPOLINE_HOOK[ORIGINAL_FUNCTION_OFFSET as usize..ORIGINAL_FUNCTION_OFFSET as usize + 8]
        == [0xEF, 0xBE, 0xAD, 0xDE, 0xEF, 0xBE, 0xAD, 0xDE]
);

pub const TRAMPOLINE_GEN_DESC: TrampolineGenDesc<TRAMPOLINE_SIZE> = TrampolineGenDesc {
    trampoline_src: TRAMPOLINE_HOOK,
    import_id_offset: FUNCTION_ID_OFFSET as u64,
    dispatcher_address_offset: DISPATCHER_ADDRESS_OFFSET as u64,
    original_function_address_offset: ORIGINAL_FUNCTION_OFFSET as u64,
};
