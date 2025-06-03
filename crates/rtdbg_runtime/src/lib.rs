mod rhai_lib;
mod runtime;
mod sock;

use std::{
    collections::VecDeque,
    sync::{Condvar, Mutex},
};

use ctor::ctor;

pub static SCRIPT_QUEUE: (Mutex<VecDeque<String>>, Condvar) =
    (Mutex::new(VecDeque::new()), Condvar::new());

#[ctor]
fn init() {
    std::thread::spawn(sock::io);
    std::thread::spawn(runtime::runtime);
}
