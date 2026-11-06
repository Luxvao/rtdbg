use crate::{arch::NativeRelocator, error::Error};

pub trait Relocator {
    fn rewrite(opcode: &[u8], old_pc: u64, new_pc: u64) -> Result<Vec<Instruction>, Error>;
}

#[derive(Debug, Copy, Clone)]
pub enum RelType {
    Rel32,
    Rel16,
    Rel8,
}

#[derive(Debug, Clone)]
pub struct Instruction {
    pub opcode: Vec<u8>,
    pub bytes: Vec<u8>,
    pub rel_type: RelType,
    pub immediate_offset: usize,
}

impl Instruction {
    pub fn relocate(&self, old_pc: u64, new_pc: u64) -> Result<Vec<Instruction>, Error> {
        let rel = match self.rel_type {
            RelType::Rel32 => i32::from_ne_bytes(
                self.bytes
                    .get(self.immediate_offset..self.immediate_offset + size_of::<u32>())
                    .ok_or(Error::MalformedInstruction)?
                    .try_into()?,
            ),
            RelType::Rel16 => i16::from_ne_bytes(
                self.bytes
                    .get(self.immediate_offset..self.immediate_offset + size_of::<u16>())
                    .ok_or(Error::MalformedInstruction)?
                    .try_into()?,
            )
            .into(),
            RelType::Rel8 => i32::from(
                self.bytes
                    .get(self.immediate_offset)
                    .ok_or(Error::MalformedInstruction)?
                    .cast_signed(),
            ),
        };

        let target_address = old_pc
            .checked_add_signed(rel as i64)
            .ok_or(Error::MalformedInstruction)?;

        if let RelType::Rel32 = self.rel_type {
            // If we're here we can just edit the rel
            let new_rel: i32 = target_address
                .checked_signed_diff(new_pc)
                .ok_or(Error::MalformedInstruction)?
                .try_into()
                .map_err(|_| Error::RelocationTooFar)?;

            let new_rel_bytes = new_rel.to_ne_bytes();

            let mut new_instruction = self.clone();

            if new_instruction.bytes.len() < new_instruction.immediate_offset + 4 {
                return Err(Error::MalformedInstruction);
            }

            new_instruction.bytes.splice(
                new_instruction.immediate_offset..new_instruction.immediate_offset + 4,
                new_rel_bytes,
            );

            return Ok(vec![new_instruction]);
        }

        NativeRelocator::rewrite(&self.opcode, target_address, new_pc)
    }
}
