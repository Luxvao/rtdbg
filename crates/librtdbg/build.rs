use std::process::Command;

fn main() {
    if cfg!(feature = "include_runtime") {
        Command::new("cargo")
            .args(["build", "--release", "-p", "rtdbg_runtime"])
            .status()
            .unwrap();
    }
}
