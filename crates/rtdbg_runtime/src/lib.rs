pub mod rhai_lib;
mod runtime;
mod sock;

use std::{
    collections::VecDeque,
    sync::{Condvar, Mutex},
};

use ctor::ctor;
use librtdbg::script::Script;

#[ctor]
fn init() {
    std::thread::spawn(sock::io);
    std::thread::spawn(runtime::runtime);
}
