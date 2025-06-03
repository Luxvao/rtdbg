set positional-arguments

alias b := build

# Build everything
build: runtime launcher

# Build runtime
runtime:
    cargo build --release -p rtdbg_runtime

# Build the launcher
launcher:
    cargo build --release -p rtdbg

# Run the project, passing the parameters
run: build
    cargo run --release -p rtdbg

