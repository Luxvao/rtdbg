#![feature(const_cmp)]
#![feature(const_index)]
#![feature(const_trait_impl)]

use std::sync::atomic::AtomicU64;

pub mod api;
pub mod arch;
pub mod bits;
pub mod either;
pub mod error;
pub mod hook;
pub mod memory;
pub mod parameter;
pub mod proc_utils;
pub mod register;
pub mod rhai_utils;
pub mod runtime_connection;
pub mod runtime_extract;
pub mod script;
#[cfg(test)]
mod tests;
pub mod utils;

pub static ID_GEN: AtomicU64 = AtomicU64::new(0);
