use paste::paste;
use rhai::{CustomType, export_module};

use crate::create_enum_module;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Register {
    Rax,
    Rbx,
    Rcx,
    Rdx,
    Rsi,
    Rdi,
    Rbp,
    R8,
    R9,
    R10,
    R11,
    R12,
    R13,
    R14,
    R15,
    Xmm0,
    Xmm1,
    Xmm2,
    Xmm3,
    Xmm4,
    Xmm5,
    Xmm6,
    Xmm7,
    Xmm8,
    Xmm9,
    Xmm10,
    Xmm11,
    Xmm12,
    Xmm13,
    Xmm14,
    Xmm15,
}

create_enum_module!(Register => Rax, Rbx, Rcx, Rdx, Rsi, Rbp, R8, R9, R10, R11, R12, R13, R14, R15, Xmm0, Xmm1, Xmm2, Xmm3, Xmm4, Xmm5, Xmm6, Xmm7, Xmm8, Xmm9, Xmm10, Xmm11, Xmm12, Xmm13, Xmm14, Xmm15);

// This is a snapshot of the registers when entering a hook
#[repr(C)]
#[derive(Debug, Clone, Copy, CustomType)]
pub struct RegisterSnapshot {
    pub r15: u64,
    pub r14: u64,
    pub r13: u64,
    pub r12: u64,
    pub r11: u64,
    pub r10: u64,
    pub r9: u64,
    pub r8: u64,
    pub rbp: u64,
    pub rdi: u64,
    pub rsi: u64,
    pub rdx: u64,
    pub rcx: u64,
    pub rbx: u64,
    pub rax: u64,
    pub xmm: [u128; 16],
}

impl RegisterSnapshot {
    pub fn acquire_value(&self, register: &Register) -> u128 {
        match register {
            Register::Rax => self.rax as u128,
            Register::Rbx => self.rbx as u128,
            Register::Rcx => self.rcx as u128,
            Register::Rdx => self.rdx as u128,
            Register::Rsi => self.rsi as u128,
            Register::Rdi => self.rdi as u128,
            Register::Rbp => self.rbp as u128,
            Register::R8 => self.r8 as u128,
            Register::R9 => self.r9 as u128,
            Register::R10 => self.r10 as u128,
            Register::R11 => self.r11 as u128,
            Register::R12 => self.r12 as u128,
            Register::R13 => self.r13 as u128,
            Register::R14 => self.r14 as u128,
            Register::R15 => self.r15 as u128,

            Register::Xmm0 => self.xmm[0],
            Register::Xmm1 => self.xmm[1],
            Register::Xmm2 => self.xmm[2],
            Register::Xmm3 => self.xmm[3],
            Register::Xmm4 => self.xmm[4],
            Register::Xmm5 => self.xmm[5],
            Register::Xmm6 => self.xmm[6],
            Register::Xmm7 => self.xmm[7],
            Register::Xmm8 => self.xmm[8],
            Register::Xmm9 => self.xmm[9],
            Register::Xmm10 => self.xmm[10],
            Register::Xmm11 => self.xmm[11],
            Register::Xmm12 => self.xmm[12],
            Register::Xmm13 => self.xmm[13],
            Register::Xmm14 => self.xmm[14],
            Register::Xmm15 => self.xmm[15],
        }
    }

    pub fn set_value(&mut self, register: &Register, value: u128) {
        match register {
            Register::Rax => self.rax = value as u64,
            Register::Rbx => self.rbx = value as u64,
            Register::Rcx => self.rcx = value as u64,
            Register::Rdx => self.rdx = value as u64,
            Register::Rsi => self.rsi = value as u64,
            Register::Rdi => self.rdi = value as u64,
            Register::Rbp => self.rbp = value as u64,
            Register::R8 => self.r8 = value as u64,
            Register::R9 => self.r9 = value as u64,
            Register::R10 => self.r10 = value as u64,
            Register::R11 => self.r11 = value as u64,
            Register::R12 => self.r12 = value as u64,
            Register::R13 => self.r13 = value as u64,
            Register::R14 => self.r14 = value as u64,
            Register::R15 => self.r15 = value as u64,

            Register::Xmm0 => self.xmm[0] = value,
            Register::Xmm1 => self.xmm[1] = value,
            Register::Xmm2 => self.xmm[2] = value,
            Register::Xmm3 => self.xmm[3] = value,
            Register::Xmm4 => self.xmm[4] = value,
            Register::Xmm5 => self.xmm[5] = value,
            Register::Xmm6 => self.xmm[6] = value,
            Register::Xmm7 => self.xmm[7] = value,
            Register::Xmm8 => self.xmm[8] = value,
            Register::Xmm9 => self.xmm[9] = value,
            Register::Xmm10 => self.xmm[10] = value,
            Register::Xmm11 => self.xmm[11] = value,
            Register::Xmm12 => self.xmm[12] = value,
            Register::Xmm13 => self.xmm[13] = value,
            Register::Xmm14 => self.xmm[14] = value,
            Register::Xmm15 => self.xmm[15] = value,
        }
    }
}
