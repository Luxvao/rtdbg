pub mod rhai_lib;
mod runtime;
mod sock;

use ctor::ctor;

#[ctor(unsafe)]
fn init() {
    std::thread::spawn(runtime::runtime);
    std::thread::spawn(sock::io);
}
