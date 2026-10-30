#![feature(const_cmp)]
#![feature(const_index)]
#![feature(const_trait_impl)]

use std::sync::atomic::AtomicU64;

pub mod api;
pub mod arch;
pub mod bits;
pub mod comms;
pub mod either;
pub mod error;
#[cfg(feature = "include_runtime")]
pub mod extract;
pub mod hook;
pub mod memory;
pub mod os;
pub mod parameter;
pub mod register;
pub mod rhai_utils;
pub mod script;
#[cfg(test)]
mod tests;
pub mod utils;

pub static ID_GEN: AtomicU64 = AtomicU64::new(0);
