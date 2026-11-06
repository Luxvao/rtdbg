use crate::{
    arch::relocator::{Instruction, RelType, Relocator},
    error::Error,
};

pub struct Amd64Relocator {}

impl Relocator for Amd64Relocator {
    fn rewrite(opcode: &[u8], target: u64, new_pc: u64) -> Result<Vec<Instruction>, Error> {
        match opcode {
            // Jmp
            [0xEB] => {
                let rel = target
                    .checked_signed_diff(new_pc.saturating_add(5))
                    .ok_or(Error::MalformedInstruction)?;

                let rel_bytes = rel.to_le_bytes();

                Ok(vec![Instruction {
                    opcode: vec![0xE9],
                    bytes: [[0xE9].as_slice(), &rel_bytes].concat(),
                    rel_type: RelType::Rel32,
                    immediate_offset: 1,
                }])
            }
            // Branch instructions
            [a] if 0x70 <= *a && *a <= 0x7F => {
                let rel = target
                    .checked_signed_diff(new_pc.saturating_add(6))
                    .ok_or(Error::MalformedInstruction)?;

                let rel_bytes = rel.to_le_bytes();

                let new_opcode = [0x0F, a + 0x10];

                Ok(vec![Instruction {
                    opcode: new_opcode.into(),
                    bytes: [new_opcode.as_slice(), &rel_bytes].concat(),
                    rel_type: RelType::Rel32,
                    immediate_offset: 2,
                }])
            }
            _ => Err(Error::UnknownOpcode),
        }
    }
}
