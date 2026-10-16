pub mod rhai_lib;
mod runtime;
mod sock;


use ctor::ctor;

#[ctor]
fn init() {
    std::thread::spawn(sock::io);
    std::thread::spawn(runtime::runtime);
}
