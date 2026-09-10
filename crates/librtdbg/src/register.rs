use paste::paste;
use rhai::export_module;

use crate::create_enum_module;

pub trait RegisterSnapshot {
    fn acquire_value(&self, register: &Register) -> u128;
    fn set_value(&mut self, register: &Register, value: u128);
}

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
